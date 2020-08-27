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
use std::fmt;

use percent_encoding::{AsciiSet, CONTROLS, percent_decode, utf8_percent_encode};
use serde::{Deserialize, Serialize};

use super::cookie::*;
use super::core::*;

const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b':')
    .add(b'/')
    .add(b'<')
    .add(b'>')
    .add(b'+')
    .add(b'=')
    .add(b'?')
    .add(b'%')
    .add(b'`');


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub url: Url,
    pub querystring: Vec<Param>,
    pub headers: Vec<Header>,
    pub cookies: Vec<ResponseCookie>,
    pub body: Vec<u8>,
    pub multipart: Vec<MultipartParam>,

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultipartParam {
    TextParam { name: String, value: String },
    FileParam { name: String, filename: String, content_type: Option<String> },
}


fn has_header(headers: &[Header], name: String) -> bool {
    for header in headers {
        if header.name.as_str() == name {
            return true;
        }
    }
    false
}

pub enum RequestEncoding {
    Utf8,
    Latin1,
}

impl fmt::Display for RequestEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            RequestEncoding::Utf8 => "utf8",
            RequestEncoding::Latin1 => "iso8859-1"
        })
    }
}

impl Request {
    pub fn host(self) -> String {
        self.url.host
    }

    pub fn url(self) -> String {
        let port = match self.url.port {
            None => String::from(""),
            Some(p) => format!(":{}", p)
        };
        let querystring = self.url.query_string.clone();

        // add params
        let querystring = if self.querystring.is_empty() {
            querystring
        } else {
            let mut buf = querystring.clone();
            if !querystring.is_empty() {
                buf.push('&');
            }
            for param in self.querystring {
                if !buf.is_empty() {
                    buf.push('&');
                }
                let encoded = utf8_percent_encode(param.value.as_str(), FRAGMENT).to_string();
                buf.push_str(format!("{}={}", param.name, encoded).as_str());
            }
            buf
        };
        let querystring = if querystring.is_empty() {
            "".to_string()
        } else {
            format!("?{}", querystring)
        };
        return format!("{}://{}{}{}{}",
                       self.url.scheme,
                       self.url.host,
                       port,
                       self.url.path,
                       querystring
        );
    }

    pub fn headers(self) -> Vec<Header> {
        let mut headers: Vec<Header> = self.headers.clone();
        let user_agent = format!("hurl/{}", clap::crate_version!());
        let default_headers = vec![
            (String::from("User-Agent"), user_agent),
            (String::from("Host"), self.url.clone().host)
        ];

        for (name, value) in default_headers {
            if !has_header(&self.headers, name.clone()) {
                headers.push(Header { name, value });
            }
        }

        if !self.cookies.is_empty() {
            headers.push(Header {
                name: String::from("Cookie"),
                value: self.cookies
                    .iter()
                    .map(|c| format!("{}={}", c.name, c.value))
                    .collect::<Vec<String>>()
                    .join("; "),
            });
        }
        headers
    }

    pub fn content_type(self) -> Option<String> {
        for Header { name, value } in self.headers {
            if name.as_str() == "Content-Type" {
                return Some(value);
            }
        }
        None
    }

    pub fn add_session_cookies(&mut self, cookies: Vec<ResponseCookie>) {
        //eprintln!("add session cookies {:?}", cookies);

        for cookie in cookies {

            // TBC: both request and session cookies should have a domain => should not be an Option
            let session_domain = cookie.clone().domain.unwrap();
            match self.clone().get_cookie(cookie.clone().name) {
                Some(ResponseCookie { domain: Some(domain), .. }) => {
                    if session_domain != domain {
                        self.cookies.push(cookie.clone());
                    }
                }
                _ => {
                    self.cookies.push(cookie.clone());
                }
            }
        }
    }


    pub fn get_cookie(self, name: String) -> Option<ResponseCookie> {
        for cookie in self.cookies {
            if cookie.name == name {
                return Some(cookie);
            }
        }
        None
    }


    pub fn form_params(self) -> Option<Vec<Param>> {
        if self.clone().content_type() != Some(String::from("application/x-www-form-urlencoded")) {
            return None;
        }
        let decoded = percent_decode(&self.body);
        let params = match decoded.decode_utf8() {
            Ok(v) => {
                let params: Vec<&str> = v.split('&').collect();
                params.iter().map(|s| Param::parse(s)).collect()
            }
            _ => vec![]
        };
        Some(params)
    }


    pub fn encoding(&self) -> Encoding {
        if let Some(v) = self.get_header("content-type", true) {
            if v.contains("charset=ISO-8859-1") {
                return Encoding::Latin1;
            }
        }
        Encoding::Utf8
    }

    pub fn get_header(&self, name: &str, case_sensitive: bool) -> Option<String> {
        for header in self.headers.clone() {
            if header.name == name
                || !case_sensitive && header.name.to_lowercase() == name.to_lowercase()
            {
                return Some(header.value);
            }
        }
        None
    }
}

impl Param {
    fn parse(s: &str) -> Param {
        match s.find('=') {
            None => Param { name: s.to_string(), value: String::from("") },
            Some(i) => {
                let (name, value) = s.split_at(i);
                Param { name: name.to_string(), value: value[1..].to_string() }
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Method {
    pub fn to_reqwest(&self) -> reqwest::Method {
        match self {
            Method::Get => reqwest::Method::GET,
            Method::Head => reqwest::Method::HEAD,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Connect => reqwest::Method::CONNECT,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Trace => reqwest::Method::TRACE,
            Method::Patch => reqwest::Method::PATCH,
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn hello_http_request() -> Request {
        Request {
            method: Method::Get,
            url: Url {
                scheme: "http".to_string(),
                host: "localhost".to_string(),
                port: Some(8000),
                path: "/hello".to_string(),
                query_string: "".to_string(),
            },
            querystring: vec![],
            headers: vec![],
            cookies: vec![],
            body: vec![],
            multipart: vec![],
        }
    }

    // GET http://localhost:8000/querystring-params?param1=value1&param2
    pub fn query_http_request() -> Request {
        Request {
            method: Method::Get,
            url: Url {
                scheme: "http".to_string(),
                host: "localhost".to_string(),
                port: Some(8000),
                path: "/querystring-params".to_string(),
                query_string: "".to_string(),
            },
            querystring: vec![
                Param { name: String::from("param1"), value: String::from("value1") },
                Param { name: String::from("param2"), value: String::from("a b") },
            ],
            headers: vec![],
            cookies: vec![],
            body: vec![],
            multipart: vec![],
        }
    }


    pub fn custom_http_request() -> Request {
        Request {
            method: Method::Get,
            url: Url {
                scheme: "http".to_string(),
                host: "localhost".to_string(),
                port: None,
                path: "/custom".to_string(),
                query_string: "".to_string(),
            },
            querystring: vec![],
            headers: vec![
                Header { name: String::from("User-Agent"), value: String::from("iPhone") },
                Header { name: String::from("Foo"), value: String::from("Bar") },
            ],
            cookies: vec![
                ResponseCookie {
                    name: String::from("theme"),
                    value: String::from("light"),
                    max_age: None,
                    domain: None,
                    path: None,
                    secure: None,
                    http_only: None,
                    expires: None,
                    same_site: None,
                },
                ResponseCookie {
                    name: String::from("sessionToken"),
                    value: String::from("abc123"),
                    max_age: None,
                    domain: None,
                    path: None,
                    secure: None,
                    http_only: None,
                    expires: None,
                    same_site: None,
                }
            ],
            body: vec![],
            multipart: vec![],
        }
    }


    pub fn form_http_request() -> Request {
        Request {
            method: Method::Post,
            url: Url {
                scheme: "http".to_string(),
                host: "localhost".to_string(),
                port: None,
                path: "/form-params".to_string(),
                query_string: "".to_string(),
            },
            querystring: vec![],
            headers: vec![
                Header { name: String::from("Content-Type"), value: String::from("application/x-www-form-urlencoded") },
            ],
            cookies: vec![],
            body: "param1=value1&param2=&param3=a%3db&param4=a%253db".to_string().into_bytes(),
            multipart: vec![],
        }
    }

    #[test]
    pub fn test_headers() {
        assert_eq!(hello_http_request().headers(), vec![
            Header { name: String::from("User-Agent"), value: format!("hurl/{}", clap::crate_version!()) },
            Header { name: String::from("Host"), value: String::from("localhost") }
        ]);

        assert_eq!(custom_http_request().headers(), vec![
            Header { name: String::from("User-Agent"), value: String::from("iPhone") },
            Header { name: String::from("Foo"), value: String::from("Bar") },
            Header { name: String::from("Host"), value: String::from("localhost") },
            Header { name: String::from("Cookie"), value: String::from("theme=light; sessionToken=abc123") },
        ]);
    }

    #[test]
    pub fn test_url() {
        assert_eq!(hello_http_request().url(), String::from("http://localhost:8000/hello"));
        assert_eq!(query_http_request().url(), String::from("http://localhost:8000/querystring-params?param1=value1&param2=a%20b"));
    }

    #[test]
    pub fn test_form_params() {
        assert_eq!(hello_http_request().form_params(), None);
        assert_eq!(form_http_request().form_params().unwrap(), vec![
            Param { name: String::from("param1"), value: String::from("value1") },
            Param { name: String::from("param2"), value: String::from("") },
            Param { name: String::from("param3"), value: String::from("a=b") },
            Param { name: String::from("param4"), value: String::from("a%3db") },
        ]);
    }
}
