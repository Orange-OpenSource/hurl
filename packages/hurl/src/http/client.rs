/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
use std::time::Duration;
use std::time::Instant;

use super::core::*;
use super::request::*;
use super::response::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpError {
    CouldNotResolveProxyName,
    CouldNotResolveHost,
    FailToConnect,
    TooManyRedirect,
    CouldNotParseResponse,
    SslCertificate(Option<String>),
    InvalidUrl,
    Timeout,
    StatuslineIsMissing,
    Other { description: String, code: i32 },
}

#[derive(Debug)]
pub struct Client {
    pub options: ClientOptions,
    pub handle: Box<easy::Easy>,
    pub redirect_count: usize,
    // unfortunately, follow-location feature from libcurl can not be used
    // libcurl returns a single list of headers for the 2 responses
    // hurl needs the return the headers only for the second (last) response)
}

#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub follow_location: bool,
    pub max_redirect: Option<usize>,
    pub cookie_input_file: Option<String>,
    pub proxy: Option<String>,
    pub no_proxy: Option<String>,
    pub verbose: bool,
    pub insecure: bool,
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub user: Option<String>,
    pub compressed: bool,
}

impl Client {
    ///
    /// Init HTTP hurl client
    ///
    pub fn init(options: ClientOptions) -> Client {
        let mut h = easy::Easy::new();

        // Set handle attributes
        // that are not affected by rest

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

    ///
    /// Execute an http request
    ///
    pub fn execute(
        &mut self,
        request: &Request,
        redirect_count: usize,
    ) -> Result<Response, HttpError> {
        // set handle attributes
        // that have not been set or reset

        self.handle.verbose(self.options.verbose).unwrap();
        self.handle.ssl_verify_host(!self.options.insecure).unwrap();
        self.handle.ssl_verify_peer(!self.options.insecure).unwrap();
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

        self.set_url(&request.url, &request.querystring);
        self.set_method(&request.method);

        self.set_cookies(&request.cookies);
        self.set_form(&request.form);
        self.set_multipart(&request.multipart);

        let bytes = request.body.bytes();
        let mut data: &[u8] = bytes.as_ref();
        self.set_body(data);

        self.set_headers(&request);

        self.handle
            .debug_function(|info_type, data| match info_type {
                // return all request headers (not one by one)
                easy::InfoType::HeaderOut => {
                    let lines = split_lines(data);
                    for line in lines {
                        eprintln!("> {}", line);
                    }
                }
                easy::InfoType::HeaderIn => {
                    if let Some(s) = decode_header(data) {
                        eprint!("< {}", s);
                    }
                }
                _ => {}
            })
            .unwrap();

        let start = Instant::now();
        let mut status_lines = vec![];
        let mut headers = vec![];
        let mut body = Vec::<u8>::new();
        {
            let mut transfer = self.handle.transfer();
            if !data.is_empty() {
                transfer
                    .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
                    .unwrap();
            }

            transfer
                .header_function(|h| {
                    if let Some(s) = decode_header(h) {
                        if s.starts_with("HTTP/") {
                            status_lines.push(s);
                        } else {
                            headers.push(s)
                        }
                    }
                    true
                })
                .unwrap();

            transfer
                .write_function(|data| {
                    body.extend(data);
                    Ok(data.len())
                })
                .unwrap();

            if let Err(e) = transfer.perform() {
                return match e.code() {
                    3 => Err(HttpError::InvalidUrl),
                    5 => Err(HttpError::CouldNotResolveProxyName),
                    6 => Err(HttpError::CouldNotResolveHost),
                    7 => Err(HttpError::FailToConnect),
                    28 => Err(HttpError::Timeout),
                    60 => Err(HttpError::SslCertificate(
                        e.extra_description().map(String::from),
                    )),
                    _ => Err(HttpError::Other {
                        code: e.code() as i32, // due to windows build
                        description: e.description().to_string(),
                    }),
                };
            }
        }

        let status = self.handle.response_code().unwrap();
        let version = match status_lines.last() {
            None => return Err(HttpError::StatuslineIsMissing {}),
            Some(status_line) => self.parse_response_version(status_line.clone())?,
        };
        let headers = self.parse_response_headers(&headers);

        if let Some(url) = self.get_follow_location(headers.clone()) {
            let request = Request {
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

            let redirect_count = redirect_count + 1;
            if let Some(max_redirect) = self.options.max_redirect {
                if redirect_count > max_redirect {
                    return Err(HttpError::TooManyRedirect);
                }
            }

            return self.execute(&request, redirect_count);
        }
        let duration = start.elapsed();
        self.redirect_count = redirect_count;
        self.handle.reset();

        Ok(Response {
            version,
            status,
            headers,
            body,
            duration,
        })
    }

    ///
    /// set url
    ///
    fn set_url(&mut self, url: &str, params: &[Param]) {
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
        self.handle.url(url.as_str()).unwrap();
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
    fn set_headers(&mut self, request: &Request) {
        let mut list = easy::List::new();

        for header in request.headers.clone() {
            list.append(format!("{}: {}", header.name, header.value).as_str())
                .unwrap();
        }

        if get_header_values(request.headers.clone(), "Content-Type".to_string()).is_empty() {
            if let Some(s) = request.content_type.clone() {
                list.append(format!("Content-Type: {}", s).as_str())
                    .unwrap();
            } else {
                list.append("Content-Type:").unwrap(); // remove header Content-Type
            }
        }

        if get_header_values(request.headers.clone(), "Expect".to_string()).is_empty() {
            list.append("Expect:").unwrap(); // remove header Expect
        }

        if get_header_values(request.headers.clone(), "User-Agent".to_string()).is_empty() {
            list.append(format!("User-Agent: hurl/{}", clap::crate_version!()).as_str())
                .unwrap();
        }

        if let Some(user) = self.options.user.clone() {
            let authorization = base64::encode(user.as_bytes());
            if get_header_values(request.headers.clone(), "Authorization".to_string()).is_empty() {
                list.append(format!("Authorization: Basic {}", authorization).as_str())
                    .unwrap();
            }
        }
        if self.options.compressed
            && get_header_values(request.headers.clone(), "Accept-Encoding".to_string()).is_empty()
        {
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
    fn get_follow_location(&mut self, headers: Vec<Header>) -> Option<String> {
        if !self.options.follow_location {
            return None;
        }

        let response_code = self.handle.response_code().unwrap();
        if !(300..400).contains(&response_code) {
            return None;
        }

        let location = match get_header_values(headers, "Location".to_string()).get(0) {
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
            let line = str::from_utf8(cookie).unwrap().to_string();
            let fields: Vec<&str> = line.split('\t').collect();

            let domain = fields.get(0).unwrap().to_string();
            let include_subdomain = fields.get(1).unwrap().to_string();
            let path = fields.get(2).unwrap().to_string();
            let https = fields.get(3).unwrap().to_string();
            let expires = fields.get(4).unwrap().to_string();
            let name = fields.get(5).unwrap().to_string();
            let value = fields.get(6).unwrap().to_string();
            cookies.push(Cookie {
                domain,
                include_subdomain,
                path,
                https,
                expires,
                name,
                value,
            });
        }
        cookies
    }

    ///
    /// Add cookie to Cookiejar
    ///
    pub fn add_cookie(&mut self, cookie: Cookie) {
        if self.options.verbose {
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
        if self.options.verbose {
            eprintln!("* clear cookie storage");
        }
        self.handle.cookie_list("ALL").unwrap();
    }
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
}
