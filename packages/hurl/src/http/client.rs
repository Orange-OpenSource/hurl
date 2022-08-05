/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::io::Read;
use std::str;

use curl::easy;
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use std::time::Instant;

use super::core::*;
use super::options::ClientOptions;
use super::request::*;
use super::request_spec::*;
use super::response::*;
use super::{Header, HttpError, Verbosity};
use crate::cli::Logger;
use std::str::FromStr;
use url::Url;

#[derive(Debug)]
pub struct Client {
    pub options: ClientOptions,
    pub handle: Box<easy::Easy>,
    pub redirect_count: usize,
    // unfortunately, follow-location feature from libcurl can not be used
    // libcurl returns a single list of headers for the 2 responses
    // hurl needs to keep everything
}

impl Client {
    ///
    /// Init HTTP hurl client
    ///
    pub fn init(options: ClientOptions) -> Client {
        let mut h = easy::Easy::new();

        // Set handle attributes
        // that are not affected by reset

        // Activate cookie storage
        // with or without persistence (empty string)
        h.cookie_file(
            options
                .cookie_input_file
                .clone()
                .unwrap_or_else(|| "".to_string())
                .as_str(),
        )
        .unwrap();

        Client {
            options,
            handle: Box::new(h),
            redirect_count: 0,
        }
    }

    /// Executes an HTTP request and returns a list
    ///
    /// # Arguments
    ///
    /// * `request` - A request specification
    /// * `logger`- A logger
    pub fn execute_with_redirect(
        &mut self,
        request: &RequestSpec,
        logger: &Logger,
    ) -> Result<Vec<(Request, Response)>, HttpError> {
        let mut calls = vec![];

        let mut request_spec = request.clone();
        self.redirect_count = 0;
        loop {
            let (request, response) = self.execute(&request_spec, logger)?;
            calls.push((request, response.clone()));
            if let Some(url) = self.get_follow_location(response.clone()) {
                request_spec = RequestSpec {
                    method: Method::Get,
                    url,
                    headers: vec![],
                    querystring: vec![],
                    form: vec![],
                    multipart: vec![],
                    cookies: vec![],
                    body: Body::Binary(vec![]),
                    content_type: None,
                };

                self.redirect_count += 1;
                if let Some(max_redirect) = self.options.max_redirect {
                    if self.redirect_count > max_redirect {
                        return Err(HttpError::TooManyRedirect);
                    }
                }
            } else {
                break;
            }
        }
        Ok(calls)
    }

    /// Execute an http request
    pub fn execute(
        &mut self,
        request_spec: &RequestSpec,
        logger: &Logger,
    ) -> Result<(Request, Response), HttpError> {
        // Set handle attributes that have not been set or reset.

        // We force libcurl verbose mode regardless of Hurl verbose option to be able
        // to capture HTTP request headers in libcurl `debug_function`. That's the only
        // way to get access to the outgoing headers.
        self.handle.verbose(true).unwrap();
        self.handle.ssl_verify_host(!self.options.insecure).unwrap();
        self.handle.ssl_verify_peer(!self.options.insecure).unwrap();
        if let Some(cacert_file) = self.options.cacert_file.clone() {
            self.handle.cainfo(cacert_file).unwrap();
            self.handle.ssl_cert_type("PEM").unwrap();
        }

        if let Some(proxy) = self.options.proxy.clone() {
            self.handle.proxy(proxy.as_str()).unwrap();
        }
        if let Some(s) = self.options.no_proxy.clone() {
            self.handle.noproxy(s.as_str()).unwrap();
        }
        self.handle.timeout(self.options.timeout).unwrap();
        self.handle
            .connect_timeout(self.options.connect_timeout)
            .unwrap();

        let url = self.generate_url(&request_spec.url, &request_spec.querystring);
        self.handle.url(url.as_str()).unwrap();
        let method = &request_spec.method;
        self.set_method(method);
        self.set_cookies(&request_spec.cookies);
        self.set_form(&request_spec.form);
        self.set_multipart(&request_spec.multipart);
        let mut request_spec_body: &[u8] = &request_spec.body.bytes();
        self.set_body(request_spec_body);
        self.set_headers(request_spec);

        let start = Instant::now();
        let verbose = self.options.verbosity != None;
        let very_verbose = self.options.verbosity == Some(Verbosity::VeryVerbose);
        let mut request_headers: Vec<Header> = vec![];
        let mut status_lines = vec![];
        let mut response_headers = vec![];
        let has_body_data = !request_spec_body.is_empty()
            || !request_spec.form.is_empty()
            || !request_spec.multipart.is_empty();

        // `request_body` are request body bytes computed by libcurl (the real bytes sent over the wire)
        // whereas`request_spec_body` are request body bytes provided by Hurl user. For instance, if user uses
        // a [FormParam] section, `request_body` is empty whereas libcurl sent a url-form encoded list
        // of key-value.
        let mut request_body = Vec::<u8>::new();
        let mut response_body = Vec::<u8>::new();
        {
            let mut transfer = self.handle.transfer();
            if !request_spec_body.is_empty() {
                transfer
                    .read_function(|buf| Ok(request_spec_body.read(buf).unwrap_or(0)))
                    .unwrap();
            }
            transfer
                .debug_function(|info_type, data| match info_type {
                    // Return all request headers (not one by one)
                    easy::InfoType::HeaderOut => {
                        let mut lines = split_lines(data);
                        if verbose {
                            logger.method_version_out(&lines[0]);
                        }

                        // Extracts request headers from libcurl debug info.
                        lines.pop().unwrap(); // Remove last empty line.
                        lines.remove(0); // Remove method/path/version line.
                        for line in lines {
                            if let Some(header) = Header::parse(line) {
                                request_headers.push(header);
                            }
                        }

                        // If we don't send any data, we log headers and empty body here
                        // instead of relying on libcurl computing body in easy::InfoType::DataOut.
                        // because libcurl dont call easy::InfoType::DataOut if there is no data
                        // to send.
                        if !has_body_data && verbose {
                            let debug_request = Request {
                                url: url.to_string(),
                                method: method.to_string(),
                                headers: request_headers.clone(),
                                body: Vec::new(),
                            };
                            for header in &debug_request.headers {
                                logger.header_out(&header.name, &header.value);
                            }
                            logger.info(">");

                            if very_verbose {
                                debug_request.log_body(logger);
                            }
                        }
                    }
                    // We use this callback to get the real body bytes sent by libcurl.
                    easy::InfoType::DataOut => {
                        // Extracts request body from libcurl debug info.
                        request_body.extend(data);
                        if verbose {
                            let debug_request = Request {
                                url: url.to_string(),
                                method: method.to_string(),
                                headers: request_headers.clone(),
                                body: Vec::from(data),
                            };
                            for header in &debug_request.headers {
                                logger.header_out(&header.name, &header.value);
                            }
                            logger.info(">");

                            if very_verbose {
                                debug_request.log_body(logger);
                            }
                        }
                    }
                    _ => {}
                })
                .unwrap();
            transfer
                .header_function(|h| {
                    if let Some(s) = decode_header(h) {
                        if s.starts_with("HTTP/") {
                            if verbose {
                                logger.debug_important("Response:");
                                logger.debug("");
                                logger.status_version_in(s.trim());
                            }
                            status_lines.push(s);
                        } else {
                            response_headers.push(s)
                        }
                    }
                    true
                })
                .unwrap();

            transfer
                .write_function(|data| {
                    response_body.extend(data);
                    Ok(data.len())
                })
                .unwrap();

            if let Err(e) = transfer.perform() {
                let code = e.code() as i32; // due to windows build
                let description = match e.extra_description() {
                    None => e.description().to_string(),
                    Some(s) => s.to_string(),
                };
                return Err(HttpError::Libcurl {
                    code,
                    description,
                    url: url.to_string(),
                });
            }
        }

        let status = self.handle.response_code().unwrap();
        // TODO: explain why status_lines is Vec ?
        let version = match status_lines.last() {
            None => return Err(HttpError::StatuslineIsMissing { url }),
            Some(status_line) => self.parse_response_version(status_line.to_string())?,
        };
        let headers = self.parse_response_headers(&response_headers);
        let duration = start.elapsed();
        self.handle.reset();

        let request = Request {
            url: url.to_string(),
            method: method.to_string(),
            headers: request_headers,
            body: request_body,
        };
        let response = Response {
            version,
            status,
            headers,
            body: response_body,
            duration,
        };

        if verbose {
            for header in &response.headers {
                logger.header_in(&header.name, &header.value);
            }
            logger.info("<");
            if very_verbose {
                response.log_body(logger);
            }
        }

        Ok((request, response))
    }

    ///
    /// generate url
    ///
    fn generate_url(&mut self, url: &str, params: &[Param]) -> String {
        let url = if params.is_empty() {
            url.to_string()
        } else {
            let url = if url.ends_with('?') {
                url.to_string()
            } else if url.contains('?') {
                format!("{}&", url)
            } else {
                format!("{}?", url)
            };
            let s = self.encode_params(params);
            format!("{}{}", url, s)
        };
        url
    }

    ///
    /// set method
    ///
    fn set_method(&mut self, method: &Method) {
        match method {
            Method::Get => self.handle.custom_request("GET").unwrap(),
            Method::Post => self.handle.custom_request("POST").unwrap(),
            Method::Put => self.handle.custom_request("PUT").unwrap(),
            Method::Head => self.handle.custom_request("HEAD").unwrap(),
            Method::Delete => self.handle.custom_request("DELETE").unwrap(),
            Method::Connect => self.handle.custom_request("CONNECT").unwrap(),
            Method::Options => self.handle.custom_request("OPTIONS").unwrap(),
            Method::Trace => self.handle.custom_request("TRACE").unwrap(),
            Method::Patch => self.handle.custom_request("PATCH").unwrap(),
        }
    }

    ///
    /// set request headers
    ///
    fn set_headers(&mut self, request: &RequestSpec) {
        let mut list = easy::List::new();

        for header in &request.headers {
            list.append(format!("{}: {}", header.name, header.value).as_str())
                .unwrap();
        }

        if request.get_header_values("Content-Type").is_empty() {
            if let Some(ref s) = request.content_type {
                list.append(format!("Content-Type: {}", s).as_str())
                    .unwrap();
            } else {
                list.append("Content-Type:").unwrap(); // remove header Content-Type
            }
        }

        if request.get_header_values("Expect").is_empty() {
            list.append("Expect:").unwrap(); // remove header Expect
        }

        if request.get_header_values("User-Agent").is_empty() {
            let user_agent = match self.options.user_agent {
                Some(ref u) => u.clone(),
                None => format!("hurl/{}", clap::crate_version!()),
            };
            list.append(format!("User-Agent: {}", user_agent).as_str())
                .unwrap();
        }

        if let Some(ref user) = self.options.user {
            let authorization = base64::encode(user.as_bytes());
            if request.get_header_values("Authorization").is_empty() {
                list.append(format!("Authorization: Basic {}", authorization).as_str())
                    .unwrap();
            }
        }
        if self.options.compressed && request.get_header_values("Accept-Encoding").is_empty() {
            list.append("Accept-Encoding: gzip, deflate, br").unwrap();
        }

        self.handle.http_headers(list).unwrap();
    }

    ///
    /// set request cookies
    ///
    fn set_cookies(&mut self, cookies: &[RequestCookie]) {
        let s = cookies
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("; ");
        if !s.is_empty() {
            self.handle.cookie(s.as_str()).unwrap();
        }
    }

    ///
    /// set form
    ///
    fn set_form(&mut self, params: &[Param]) {
        if !params.is_empty() {
            let s = self.encode_params(params);
            self.handle.post_fields_copy(s.as_str().as_bytes()).unwrap();
            //self.handle.write_function(sink);
        }
    }

    ///
    /// set form
    ///
    fn set_multipart(&mut self, params: &[MultipartParam]) {
        if !params.is_empty() {
            let mut form = easy::Form::new();
            for param in params {
                match param {
                    MultipartParam::Param(Param { name, value }) => {
                        form.part(name).contents(value.as_bytes()).add().unwrap()
                    }
                    MultipartParam::FileParam(FileParam {
                        name,
                        filename,
                        data,
                        content_type,
                    }) => form
                        .part(name)
                        .buffer(filename, data.clone())
                        .content_type(content_type)
                        .add()
                        .unwrap(),
                }
            }
            self.handle.httppost(form).unwrap();
        }
    }

    ///
    /// set body
    ///
    fn set_body(&mut self, data: &[u8]) {
        if !data.is_empty() {
            self.handle.post(true).unwrap();
            self.handle.post_field_size(data.len() as u64).unwrap();
        }
    }

    ///
    /// encode parameters
    ///
    fn encode_params(&mut self, params: &[Param]) -> String {
        params
            .iter()
            .map(|p| {
                let value = self.handle.url_encode(p.value.as_bytes());
                format!("{}={}", p.name, value)
            })
            .collect::<Vec<String>>()
            .join("&")
    }

    ///
    /// parse response version
    ///
    fn parse_response_version(&mut self, line: String) -> Result<Version, HttpError> {
        if line.starts_with("HTTP/1.0") {
            Ok(Version::Http10)
        } else if line.starts_with("HTTP/1.1") {
            Ok(Version::Http11)
        } else if line.starts_with("HTTP/2") {
            Ok(Version::Http2)
        } else {
            Err(HttpError::CouldNotParseResponse)
        }
    }

    ///
    /// parse headers from libcurl responses
    ///
    fn parse_response_headers(&mut self, lines: &[String]) -> Vec<Header> {
        let mut headers: Vec<Header> = vec![];
        for line in lines {
            if let Some(header) = Header::parse(line.to_string()) {
                headers.push(header);
            }
        }
        headers
    }

    ///
    /// retrieve an optional location to follow
    /// You need:
    /// 1. the option follow_location set to true
    /// 2. a 3xx response code
    /// 3. a header Location
    ///
    fn get_follow_location(&mut self, response: Response) -> Option<String> {
        if !self.options.follow_location {
            return None;
        }
        let response_code = response.status;
        if !(300..400).contains(&response_code) {
            return None;
        }
        let location = match response.get_header_values("Location").get(0) {
            None => return None,
            Some(value) => value.clone(),
        };

        if location.is_empty() {
            None
        } else {
            Some(location)
        }
    }

    ///
    /// get cookie storage
    ///
    pub fn get_cookie_storage(&mut self) -> Vec<Cookie> {
        let list = self.handle.cookies().unwrap();
        let mut cookies = vec![];
        for cookie in list.iter() {
            let line = str::from_utf8(cookie).unwrap();
            if let Ok(cookie) = Cookie::from_str(line) {
                cookies.push(cookie);
            } else {
                eprintln!("warning: line <{}> can not be parsed as cookie", line);
            }
        }
        cookies
    }

    ///
    /// Add cookie to Cookiejar
    ///
    pub fn add_cookie(&mut self, cookie: Cookie) {
        if self.options.verbosity != None {
            eprintln!("* add to cookie store: {}", cookie);
        }
        self.handle
            .cookie_list(cookie.to_string().as_str())
            .unwrap();
    }

    ///
    /// Clear cookie storage
    ///
    pub fn clear_cookie_storage(&mut self) {
        if self.options.verbosity != None {
            eprintln!("* clear cookie storage");
        }
        self.handle.cookie_list("ALL").unwrap();
    }

    ///
    /// return curl command-line for the http request run by the client
    ///
    pub fn curl_command_line(&mut self, http_request: &RequestSpec) -> String {
        let mut arguments = vec!["curl".to_string()];
        let context_dir = &self.options.context_dir;
        arguments.append(&mut http_request.curl_args(context_dir));

        let cookies = all_cookies(self.get_cookie_storage(), http_request);
        if !cookies.is_empty() {
            arguments.push("--cookie".to_string());
            arguments.push(format!(
                "'{}'",
                cookies
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("; ")
            ));
        }
        arguments.append(&mut self.options.curl_args());
        arguments.join(" ")
    }
}

///
/// return cookies from both cookies from the cookie storage and the request
///
pub fn all_cookies(cookie_storage: Vec<Cookie>, request: &RequestSpec) -> Vec<RequestCookie> {
    let mut cookies = request.cookies.clone();
    cookies.append(
        &mut cookie_storage
            .iter()
            .filter(|c| c.expires != "1") // cookie expired when libcurl set value to 1?
            .filter(|c| match_cookie(c, request.url.as_str()))
            .map(|c| RequestCookie {
                name: (*c).name.clone(),
                value: c.value.clone(),
            })
            .collect(),
    );
    cookies
}

///
/// Match cookie for a given url
///
pub fn match_cookie(cookie: &Cookie, url: &str) -> bool {
    // is it possible to do it with libcurl?
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return false,
    };
    if let Some(domain) = url.domain() {
        if cookie.include_subdomain == "FALSE" {
            if cookie.domain != domain {
                return false;
            }
        } else if !domain.ends_with(cookie.domain.as_str()) {
            return false;
        }
    }
    url.path().starts_with(cookie.path.as_str())
}

impl Header {
    ///
    /// Parse an http header line received from the server
    /// It does not panic. Just return none if it can not be parsed
    ///
    pub fn parse(line: String) -> Option<Header> {
        match line.find(':') {
            Some(index) => {
                let (name, value) = line.split_at(index);
                Some(Header {
                    name: name.to_string().trim().to_string(),
                    value: value[1..].to_string().trim().to_string(),
                })
            }
            None => None,
        }
    }
}

///
/// Split an array of bytes into http lines (\r\n separator)
///
fn split_lines(data: &[u8]) -> Vec<String> {
    let mut lines = vec![];
    let mut start = 0;
    let mut i = 0;
    while i < (data.len() - 1) {
        if data[i] == 13 && data[i + 1] == 10 {
            if let Ok(s) = str::from_utf8(&data[start..i]) {
                lines.push(s.to_string());
            }
            start = i + 2;
            i += 2;
        } else {
            i += 1;
        }
    }
    lines
}

///
/// Decode optionally header value as text with utf8 or iso-8859-1 encoding
///
pub fn decode_header(data: &[u8]) -> Option<String> {
    match str::from_utf8(data) {
        Ok(s) => Some(s.to_string()),
        Err(_) => match ISO_8859_1.decode(data, DecoderTrap::Strict) {
            Ok(s) => Some(s),
            Err(_) => {
                println!("Error decoding header both utf8 and iso-8859-1 {:?}", data);
                None
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        assert_eq!(
            Header::parse("Foo: Bar\r\n".to_string()).unwrap(),
            Header {
                name: "Foo".to_string(),
                value: "Bar".to_string(),
            }
        );
        assert_eq!(
            Header::parse("Location: http://localhost:8000/redirected\r\n".to_string()).unwrap(),
            Header {
                name: "Location".to_string(),
                value: "http://localhost:8000/redirected".to_string(),
            }
        );
        assert!(Header::parse("Foo".to_string()).is_none());
    }

    #[test]
    fn test_split_lines_header() {
        let data = b"GET /hello HTTP/1.1\r\nHost: localhost:8000\r\n\r\n";
        let lines = split_lines(data);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines.get(0).unwrap().as_str(), "GET /hello HTTP/1.1");
        assert_eq!(lines.get(1).unwrap().as_str(), "Host: localhost:8000");
        assert_eq!(lines.get(2).unwrap().as_str(), "");
    }

    #[test]
    fn test_match_cookie() {
        let cookie = Cookie {
            domain: "example.com".to_string(),
            include_subdomain: "FALSE".to_string(),
            path: "/".to_string(),
            https: "".to_string(),
            expires: "".to_string(),
            name: "".to_string(),
            value: "".to_string(),
            http_only: false,
        };
        assert!(match_cookie(&cookie, "http://example.com/toto"));
        assert!(!match_cookie(&cookie, "http://sub.example.com/tata"));
        assert!(!match_cookie(&cookie, "http://toto/tata"));

        let cookie = Cookie {
            domain: "example.com".to_string(),
            include_subdomain: "TRUE".to_string(),
            path: "/toto".to_string(),
            https: "".to_string(),
            expires: "".to_string(),
            name: "".to_string(),
            value: "".to_string(),
            http_only: false,
        };
        assert!(match_cookie(&cookie, "http://example.com/toto"));
        assert!(match_cookie(&cookie, "http://sub.example.com/toto"));
        assert!(!match_cookie(&cookie, "http://example.com/tata"));
    }
}
