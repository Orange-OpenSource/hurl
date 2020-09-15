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

use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: Vec<Header>,
    pub querystring: Vec<Param>,
    pub form: Vec<Param>,
    pub multipart: Vec<MultipartParam>,
    pub cookies: Vec<RequestCookie>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub version: Version,
    pub status: u32,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}


#[derive(Clone, Debug, PartialEq, Eq)]
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

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH"
        };
        write!(f, "{}", value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Version {
    Http10,
    Http11,
    Http2,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultipartParam {
    Param(Param),
    FileParam(FileParam),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileParam {
    pub name: String,
    pub filename: String,
    pub data: Vec<u8>,
    pub content_type: String,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestCookie {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub domain: String,
    pub include_subdomain: String,
    pub path: String,
    pub https: String,
    pub expires: String,
    pub name: String,
    pub value: String,
}


impl fmt::Display for RequestCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpError {
    CouldNotResolveProxyName,
    CouldNotResolveHost,
    FailToConnect,
    TooManyRedirect,
    CouldNotParseResponse,
}


impl Response {
    ///
    /// return a list of headers values for the given header name
    ///
    pub fn get_header_values(&self, expected_name: String) -> Vec<String> {
        get_header_values(self.headers.clone(), expected_name)
    }
}


///
/// return a list of headers values for the given header name
///
pub fn get_header_values(headers: Vec<Header>, expected_name: String) -> Vec<String> {
    headers
        .iter()
        .filter_map(|Header { name, value }| if name.clone() == expected_name { Some(value.to_string()) } else { None })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn get_header_values() {
        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header { name: "Content-Length".to_string(), value: "12".to_string() }
            ],
            body: vec![],
        };
        assert_eq!(response.get_header_values("Content-Length".to_string()), vec!["12".to_string()]);
        assert!(response.get_header_values("Unknown".to_string()).is_empty());
    }

    pub fn hello_http_request() -> Request {
        Request {
            method: Method::Get,
            url: "http://localhost:8000/hello".to_string(),
            querystring: vec![],
            headers: vec![],
            cookies: vec![],
            body: vec![],
            multipart: vec![],
            form: vec![],
        }
    }

    pub fn custom_http_request() -> Request {
        Request {
            method: Method::Get,
            url: "http://localhost/custom".to_string(),
            querystring: vec![],
            headers: vec![
                Header { name: String::from("User-Agent"), value: String::from("iPhone") },
                Header { name: String::from("Foo"), value: String::from("Bar") },
            ],
            cookies: vec![
                RequestCookie {
                    name: String::from("theme"),
                    value: String::from("light"),
                },
                RequestCookie {
                    name: String::from("sessionToken"),
                    value: String::from("abc123"),
                }
            ],
            body: vec![],
            multipart: vec![],
            form: vec![],
        }
    }


    pub fn form_http_request() -> Request {
        Request {
            method: Method::Post,
            url: "http://localhost/form-params".to_string(),
            querystring: vec![],
            headers: vec![
                Header { name: String::from("Content-Type"), value: String::from("application/x-www-form-urlencoded") },
            ],
            cookies: vec![],
            body: "param1=value1&param2=&param3=a%3db&param4=a%253db".to_string().into_bytes(),
            multipart: vec![],
            form: vec![]
        }
    }
}