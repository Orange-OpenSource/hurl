/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use std::str;
use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;
use chrono::Utc;
use curl::easy;
use curl::easy::{List, SslOpt};
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, Encoding};
use url::Url;

use crate::http::certificate::Certificate;
use crate::http::core::*;
use crate::http::options::ClientOptions;
use crate::http::request::*;
use crate::http::request_spec::*;
use crate::http::response::*;
use crate::http::timings::Timings;
use crate::http::{easy_ext, Call, Header, HttpError, Verbosity};
use crate::util::logger::Logger;
use crate::util::path::ContextDir;

/// Defines an HTTP client to execute HTTP requests.
///
/// Most of the methods are delegated to libcurl functions, while some
/// features are implemented "by hand" (like retry, redirection etc...)
#[derive(Debug)]
pub struct Client {
    /// The handle to libcurl binding
    handle: Box<easy::Easy>,
}

impl Client {
    /// Creates HTTP Hurl client.
    pub fn new(cookie_input_file: Option<String>) -> Client {
        let mut h = easy::Easy::new();

        // Set handle attributes
        // that are not affected by reset

        // Activate cookie storage
        // with or without persistence (empty string)
        h.cookie_file(cookie_input_file.unwrap_or_default())
            .unwrap();

        Client {
            handle: Box::new(h),
        }
    }

    /// Executes an HTTP request `request_spec`, optionally follows redirection and returns a
    /// list of pair of [`Request`], [`Response`].
    pub fn execute_with_redirect(
        &mut self,
        request_spec: &RequestSpec,
        options: &ClientOptions,
        logger: &Logger,
    ) -> Result<Vec<Call>, HttpError> {
        let mut calls = vec![];

        let mut request_spec = request_spec.clone();

        // Unfortunately, follow-location feature from libcurl can not be used
        // libcurl returns a single list of headers for the 2 responses
        // Hurl needs to keep everything.
        let mut redirect_count = 0;
        loop {
            let call = self.execute(&request_spec, options, logger)?;
            let base_url = call.request.base_url()?;
            let redirect_url = self.get_follow_location(&call.response, &base_url);
            calls.push(call);
            if !options.follow_location || redirect_url.is_none() {
                break;
            }
            let redirect_url = redirect_url.unwrap();
            logger.debug("");
            logger.debug(format!("=> Redirect to {redirect_url}").as_str());
            logger.debug("");
            redirect_count += 1;
            if let Some(max_redirect) = options.max_redirect {
                if redirect_count > max_redirect {
                    return Err(HttpError::TooManyRedirect);
                }
            }
            request_spec = RequestSpec {
                method: Method("GET".to_string()),
                url: redirect_url,
                ..Default::default()
            };
        }
        Ok(calls)
    }

    /// Executes an HTTP request `request_spec`, without following redirection and returns a
    /// pair of [`Request`], [`Response`].
    pub fn execute(
        &mut self,
        request_spec: &RequestSpec,
        options: &ClientOptions,
        logger: &Logger,
    ) -> Result<Call, HttpError> {
        // Set handle attributes that have not been set or reset.

        // We force libcurl verbose mode regardless of Hurl verbose option to be able
        // to capture HTTP request headers in libcurl `debug_function`. That's the only
        // way to get access to the outgoing headers.
        self.handle.verbose(true).unwrap();

        // Activates the access of certificates info chain after a transfer has been executed.
        self.handle.certinfo(true).unwrap();

        if !options.connects_to.is_empty() {
            let connects = to_list(&options.connects_to);
            self.handle.connect_to(connects).unwrap();
        }
        if !options.resolves.is_empty() {
            let resolves = to_list(&options.resolves);
            self.handle.resolve(resolves).unwrap();
        }
        self.handle.ssl_verify_host(!options.insecure).unwrap();
        self.handle.ssl_verify_peer(!options.insecure).unwrap();
        if let Some(cacert_file) = options.cacert_file.clone() {
            self.handle.cainfo(cacert_file).unwrap();
            self.handle.ssl_cert_type("PEM").unwrap();
        }
        if let Some(client_cert_file) = options.client_cert_file.clone() {
            self.handle.ssl_cert(client_cert_file).unwrap();
            self.handle.ssl_cert_type("PEM").unwrap();
        }
        if let Some(client_key_file) = options.client_key_file.clone() {
            self.handle.ssl_key(client_key_file).unwrap();
            self.handle.ssl_cert_type("PEM").unwrap();
        }
        self.handle.path_as_is(options.path_as_is).unwrap();
        if let Some(proxy) = options.proxy.clone() {
            self.handle.proxy(proxy.as_str()).unwrap();
        }
        if let Some(s) = options.no_proxy.clone() {
            self.handle.noproxy(s.as_str()).unwrap();
        }
        self.handle.timeout(options.timeout).unwrap();
        self.handle
            .connect_timeout(options.connect_timeout)
            .unwrap();

        self.set_ssl_options(options.ssl_no_revoke);

        let url = self.generate_url(&request_spec.url, &request_spec.querystring);
        self.handle.url(url.as_str()).unwrap();
        let method = &request_spec.method;
        self.set_method(method);
        self.set_cookies(&request_spec.cookies);
        self.set_form(&request_spec.form);
        self.set_multipart(&request_spec.multipart);
        let request_spec_body = &request_spec.body.bytes();
        self.set_body(request_spec_body);
        self.set_headers(request_spec, options);

        let start = Utc::now();
        let verbose = options.verbosity.is_some();
        let very_verbose = options.verbosity == Some(Verbosity::VeryVerbose);
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

        if *method == Method("HEAD".to_string()) {
            self.handle.nobody(true).unwrap();
        }
        {
            let mut transfer = self.handle.transfer();

            transfer
                .debug_function(|info_type, data| match info_type {
                    // Return all request headers (not one by one)
                    easy::InfoType::HeaderOut => {
                        let mut lines = split_lines(data);
                        logger.debug_method_version_out(&lines[0]);

                        // Extracts request headers from libcurl debug info.
                        lines.pop().unwrap(); // Remove last empty line.
                        lines.remove(0); // Remove method/path/version line.
                        for line in lines {
                            if let Some(header) = Header::parse(&line) {
                                request_headers.push(header);
                            }
                        }

                        // If we don't send any data, we log headers and empty body here
                        // instead of relying on libcurl computing body in `easy::InfoType::DataOut`.
                        // because libcurl dont call `easy::InfoType::DataOut` if there is no data
                        // to send.
                        if !has_body_data && verbose {
                            let debug_request = Request {
                                url: url.to_string(),
                                method: method.to_string(),
                                headers: request_headers.clone(),
                                body: Vec::new(),
                            };
                            for header in &debug_request.headers {
                                logger.debug_header_out(&header.name, &header.value);
                            }
                            logger.info(">");

                            if very_verbose {
                                debug_request.log_body(true, logger);
                            }
                        }
                    }
                    // We use this callback to get the real body bytes sent by libcurl.
                    easy::InfoType::DataOut => {
                        // We log request headers with `easy::InfoType::DataOut` using libcurl
                        // debug functions: there is no libcurl function to get the request headers.
                        // As we can be called multiple times in this callback, we only log headers the
                        // first time we send data (marked when the request body is empty).
                        if verbose && request_body.is_empty() {
                            let debug_request = Request {
                                url: url.to_string(),
                                method: method.to_string(),
                                headers: request_headers.clone(),
                                body: Vec::from(data),
                            };
                            for header in &debug_request.headers {
                                logger.debug_header_out(&header.name, &header.value);
                            }
                            logger.info(">");

                            if very_verbose {
                                debug_request.log_body(true, logger);
                            }
                        }

                        // Extracts request body from libcurl debug info.
                        request_body.extend(data);
                    }
                    // Curl debug logs
                    easy::InfoType::Text => {
                        let len = data.len();
                        if very_verbose && len > 0 {
                            let text = str::from_utf8(&data[..len - 1]);
                            if let Ok(text) = text {
                                logger.debug_curl(text);
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
            Some(status_line) => self.parse_response_version(status_line)?,
        };
        let headers = self.parse_response_headers(&response_headers);
        let length = response_body.len();
        let certificate = if let Some(cert_info) = easy_ext::get_certinfo(&self.handle)? {
            match Certificate::try_from(cert_info) {
                Ok(value) => Some(value),
                Err(message) => {
                    logger.error(format!("can not parse certificate - {message}").as_str());
                    None
                }
            }
        } else {
            None
        };
        let stop = Utc::now();
        let duration = (stop - start).to_std().unwrap();
        let timings = Timings::new(&mut self.handle, start, stop);
        self.handle.reset();

        let request = Request {
            url: url.clone(),
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
            url,
            certificate,
        };

        if verbose {
            // FIXME: the cast to u64 seems not necessary.
            //  If we dont cast from u128 and try to format! or println!
            //  we have a segfault on Alpine Docker images and Rust 1.68.0, whereas it was
            //  ok with Rust >= 1.67.0.
            let duration = duration.as_millis() as u64;
            logger.debug_important(
                format!("Response: (received {length} bytes in {duration} ms)").as_str(),
            );
            logger.debug("");

            // FIXME: Explain why there may be multiple status line
            status_lines
                .iter()
                .filter(|s| s.starts_with("HTTP/"))
                .for_each(|s| logger.debug_status_version_in(s.trim()));

            for header in &response.headers {
                logger.debug_header_in(&header.name, &header.value);
            }
            logger.info("<");
            if very_verbose {
                response.log_body(true, logger);
                logger.debug("");
                timings.log(logger);
            }
        }

        Ok(Call {
            request,
            response,
            timings,
        })
    }

    /// Generates URL.
    fn generate_url(&mut self, url: &str, params: &[Param]) -> String {
        if params.is_empty() {
            url.to_string()
        } else {
            let url = if url.ends_with('?') {
                url.to_string()
            } else if url.contains('?') {
                format!("{url}&")
            } else {
                format!("{url}?")
            };
            let s = self.url_encode_params(params);
            format!("{url}{s}")
        }
    }

    /// Sets HTTP method.
    fn set_method(&mut self, method: &Method) {
        self.handle
            .custom_request(method.to_string().as_str())
            .unwrap()
    }

    /// Sets HTTP headers.
    fn set_headers(&mut self, request: &RequestSpec, options: &ClientOptions) {
        let mut list = easy::List::new();

        for header in &request.headers {
            list.append(format!("{}: {}", header.name, header.value).as_str())
                .unwrap();
        }

        if request.get_header_values("Content-Type").is_empty() {
            if let Some(ref s) = request.content_type {
                list.append(format!("Content-Type: {s}").as_str()).unwrap();
            } else {
                // We remove default Content-Type headers added by curl because we want
                // to explicitly manage this header.
                // For instance, with --data option, curl will send a 'Content-type: application/x-www-form-urlencoded'
                // header.
                list.append("Content-Type:").unwrap();
            }
        }

        if request.get_header_values("Expect").is_empty() {
            // We remove default Expect headers added by curl because we want
            // to explicitly manage this header.
            list.append("Expect:").unwrap(); // remove header Expect
        }

        if request.get_header_values("User-Agent").is_empty() {
            let user_agent = match options.user_agent {
                Some(ref u) => u.clone(),
                None => format!("hurl/{}", clap::crate_version!()),
            };
            list.append(format!("User-Agent: {user_agent}").as_str())
                .unwrap();
        }

        if let Some(ref user) = options.user {
            let user = user.as_bytes();
            let authorization = general_purpose::STANDARD.encode(user);
            if request.get_header_values("Authorization").is_empty() {
                list.append(format!("Authorization: Basic {authorization}").as_str())
                    .unwrap();
            }
        }
        if options.compressed && request.get_header_values("Accept-Encoding").is_empty() {
            list.append("Accept-Encoding: gzip, deflate, br").unwrap();
        }

        self.handle.http_headers(list).unwrap();
    }

    /// Sets request cookies.
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

    /// Sets form params.
    fn set_form(&mut self, params: &[Param]) {
        if !params.is_empty() {
            let s = self.url_encode_params(params);
            self.handle.post_fields_copy(s.as_str().as_bytes()).unwrap();
            //self.handle.write_function(sink);
        }
    }

    /// Sets multipart form data.
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

    /// Sets request body.
    fn set_body(&mut self, data: &[u8]) {
        if !data.is_empty() {
            self.handle.post(true).unwrap();
            self.handle.post_fields_copy(data).unwrap();
        }
    }

    /// Sets SSL options
    fn set_ssl_options(&mut self, no_revoke: bool) {
        let mut ssl_opt = SslOpt::new();
        ssl_opt.no_revoke(no_revoke);
        self.handle.ssl_options(&ssl_opt).unwrap();
    }

    /// URL encodes parameters.
    fn url_encode_params(&mut self, params: &[Param]) -> String {
        params
            .iter()
            .map(|p| {
                let value = self.handle.url_encode(p.value.as_bytes());
                format!("{}={}", p.name, value)
            })
            .collect::<Vec<String>>()
            .join("&")
    }

    /// Parses HTTP response version.
    fn parse_response_version(&mut self, line: &str) -> Result<Version, HttpError> {
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

    /// Parse headers from libcurl responses.
    fn parse_response_headers(&mut self, lines: &[String]) -> Vec<Header> {
        let mut headers: Vec<Header> = vec![];
        for line in lines {
            if let Some(header) = Header::parse(line) {
                headers.push(header);
            }
        }
        headers
    }

    /// Retrieves an optional location to follow
    ///
    /// You need:
    /// 1. the option follow_location set to true
    /// 2. a 3xx response code
    /// 3. a header Location
    fn get_follow_location(&mut self, response: &Response, base_url: &str) -> Option<String> {
        let response_code = response.status;
        if !(300..400).contains(&response_code) {
            return None;
        }
        let location = match response.get_header_values("Location").get(0) {
            None => return None,
            Some(value) => get_redirect_url(value, base_url),
        };

        if location.is_empty() {
            None
        } else {
            Some(location)
        }
    }

    /// Returns cookie storage.
    pub fn get_cookie_storage(&mut self) -> Vec<Cookie> {
        let list = self.handle.cookies().unwrap();
        let mut cookies = vec![];
        for cookie in list.iter() {
            let line = str::from_utf8(cookie).unwrap();
            if let Ok(cookie) = Cookie::from_str(line) {
                cookies.push(cookie);
            } else {
                eprintln!("warning: line <{line}> can not be parsed as cookie");
            }
        }
        cookies
    }

    /// Adds a cookie to the cookie jar.
    pub fn add_cookie(&mut self, cookie: &Cookie, options: &ClientOptions) {
        if options.verbosity.is_some() {
            eprintln!("* add to cookie store: {cookie}");
        }
        self.handle
            .cookie_list(cookie.to_string().as_str())
            .unwrap();
    }

    /// Clears cookie storage.
    pub fn clear_cookie_storage(&mut self, options: &ClientOptions) {
        if options.verbosity.is_some() {
            eprintln!("* clear cookie storage");
        }
        self.handle.cookie_list("ALL").unwrap();
    }

    /// Returns curl command-line for the HTTP `request_spec` run by this client.
    pub fn curl_command_line(
        &mut self,
        request_spec: &RequestSpec,
        context_dir: &ContextDir,
        options: &ClientOptions,
    ) -> String {
        let mut arguments = vec!["curl".to_string()];
        arguments.append(&mut request_spec.curl_args(context_dir));

        // We extract the last part of the arguments (the url) to insert it
        // after all the options
        let url = arguments.pop().unwrap();

        let cookies = all_cookies(&self.get_cookie_storage(), request_spec);
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
        arguments.append(&mut options.curl_args());
        arguments.push(url);
        arguments.join(" ")
    }
}

/// Returns the redirect url.
fn get_redirect_url(location: &str, base_url: &str) -> String {
    if location.starts_with('/') {
        format!("{base_url}{location}")
    } else {
        location.to_string()
    }
}

/// Returns cookies from both cookies from the cookie storage and the request.
pub fn all_cookies(cookie_storage: &[Cookie], request_spec: &RequestSpec) -> Vec<RequestCookie> {
    let mut cookies = request_spec.cookies.clone();
    cookies.append(
        &mut cookie_storage
            .iter()
            .filter(|c| c.expires != "1") // cookie expired when libcurl set value to 1?
            .filter(|c| match_cookie(c, request_spec.url.as_str()))
            .map(|c| RequestCookie {
                name: c.name.clone(),
                value: c.value.clone(),
            })
            .collect(),
    );
    cookies
}

/// Matches cookie for a given URL.
pub fn match_cookie(cookie: &Cookie, url: &str) -> bool {
    // FIXME: is it possible to do it with libcurl?
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
    /// Parses an HTTP header line received from the server
    /// It does not panic. Just returns `None` if it can not be parsed.
    pub fn parse(line: &str) -> Option<Header> {
        match line.find(':') {
            Some(index) => {
                let (name, value) = line.split_at(index);
                Some(Header::new(name.trim(), value[1..].trim()))
            }
            None => None,
        }
    }
}

/// Splits an array of bytes into HTTP lines (\r\n separator).
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

/// Decodes optionally header value as text with UTF-8 or ISO-8859-1 encoding.
pub fn decode_header(data: &[u8]) -> Option<String> {
    match str::from_utf8(data) {
        Ok(s) => Some(s.to_string()),
        Err(_) => match ISO_8859_1.decode(data, DecoderTrap::Strict) {
            Ok(s) => Some(s),
            Err(_) => {
                println!("Error decoding header both UTF-8 and ISO-8859-1 {data:?}");
                None
            }
        },
    }
}

/// Converts a list of [`String`] to a libcurl's list of strings.
fn to_list(items: &[String]) -> List {
    let mut list = List::new();
    items.iter().for_each(|l| list.append(l).unwrap());
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        assert_eq!(
            Header::parse("Foo: Bar\r\n").unwrap(),
            Header::new("Foo", "Bar")
        );
        assert_eq!(
            Header::parse("Location: http://localhost:8000/redirected\r\n").unwrap(),
            Header::new("Location", "http://localhost:8000/redirected")
        );
        assert!(Header::parse("Foo").is_none());
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

    #[test]
    fn test_redirect_url() {
        assert_eq!(
            get_redirect_url("http://localhost:8000/redirected", "http://localhost:8000"),
            "http://localhost:8000/redirected".to_string()
        );
        assert_eq!(
            get_redirect_url("/redirected", "http://localhost:8000"),
            "http://localhost:8000/redirected".to_string()
        );
    }
}
