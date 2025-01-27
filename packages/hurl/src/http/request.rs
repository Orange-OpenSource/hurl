/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
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

use crate::http::header::{HeaderVec, COOKIE};
use crate::http::url::Url;
use crate::http::RequestCookie;

/// Represents a runtime HTTP request.
/// This is a real request, that has been executed by our HTTP client.
/// It's different from `crate::http::RequestSpec` which is the request asked to be executed by our
/// user. For instance, in the request spec, headers implicitly added by curl are not present, while
/// they will be present in the [`Request`] instances.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    /// Absolute URL.
    pub url: Url,
    /// Method.
    pub method: String,
    /// List of HTTP headers.
    pub headers: HeaderVec,
    /// Response body bytes.
    pub body: Vec<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum RequestedHttpVersion {
    /// The effective HTTP version will be chosen by libcurl
    #[default]
    Default,
    Http10,
    Http11,
    Http2,
    Http3,
}

impl fmt::Display for RequestedHttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            RequestedHttpVersion::Default => "HTTP (default)",
            RequestedHttpVersion::Http10 => "HTTP/1.0",
            RequestedHttpVersion::Http11 => "HTTP/1.1",
            RequestedHttpVersion::Http2 => "HTTP/2",
            RequestedHttpVersion::Http3 => "HTTP/3",
        };
        write!(f, "{value}")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum IpResolve {
    /// Default, can use addresses of all IP versions that your system allows.
    #[default]
    Default,
    IpV4,
    IpV6,
}

impl Request {
    /// Creates a new request.
    pub fn new(method: &str, url: Url, headers: HeaderVec, body: Vec<u8>) -> Self {
        Request {
            url,
            method: method.to_string(),
            headers,
            body,
        }
    }

    /// Returns a list of request headers cookie.
    ///
    /// see <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cookie>
    pub fn cookies(&self) -> Vec<RequestCookie> {
        self.headers
            .get_all(COOKIE)
            .iter()
            .flat_map(|h| parse_cookies(h.value.as_str().trim()))
            .collect()
    }
}

fn parse_cookies(s: &str) -> Vec<RequestCookie> {
    s.split(';').map(|t| parse_cookie(t.trim())).collect()
}

fn parse_cookie(s: &str) -> RequestCookie {
    match s.find('=') {
        Some(i) => RequestCookie {
            name: s.split_at(i).0.to_string(),
            value: s.split_at(i + 1).1.to_string(),
        },
        None => RequestCookie {
            name: s.to_string(),
            value: String::new(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{Header, RequestCookie};

    fn hello_request() -> Request {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Host", "localhost:8000"));
        headers.push(Header::new("Accept", "*/*"));
        headers.push(Header::new("User-Agent", "hurl/1.0"));
        headers.push(Header::new("content-type", "application/json"));
        let url = "http://localhost:8000/hello".parse().unwrap();

        Request::new("GET", url, headers, vec![])
    }

    fn query_string_request() -> Request {
        let url = "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3".parse().unwrap();

        Request::new("GET", url, HeaderVec::new(), vec![])
    }

    fn cookies_request() -> Request {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Cookie", "cookie1=value1; cookie2=value2"));
        let url = "http://localhost:8000/cookies".parse().unwrap();
        Request::new("GET", url, headers, vec![])
    }

    #[test]
    fn test_content_type() {
        assert_eq!(
            hello_request().headers.content_type(),
            Some("application/json")
        );
        assert_eq!(query_string_request().headers.content_type(), None);
        assert_eq!(cookies_request().headers.content_type(), None);
    }

    #[test]
    fn test_cookies() {
        assert!(hello_request().cookies().is_empty());
        assert_eq!(
            cookies_request().cookies(),
            vec![
                RequestCookie {
                    name: "cookie1".to_string(),
                    value: "value1".to_string(),
                },
                RequestCookie {
                    name: "cookie2".to_string(),
                    value: "value2".to_string(),
                },
            ]
        );
    }

    #[test]
    fn test_parse_cookies() {
        assert_eq!(
            parse_cookies("cookie1=value1; cookie2=value2"),
            vec![
                RequestCookie {
                    name: "cookie1".to_string(),
                    value: "value1".to_string(),
                },
                RequestCookie {
                    name: "cookie2".to_string(),
                    value: "value2".to_string(),
                },
            ]
        );
    }

    #[test]
    fn test_parse_cookie() {
        assert_eq!(
            parse_cookie("cookie1=value1"),
            RequestCookie {
                name: "cookie1".to_string(),
                value: "value1".to_string(),
            },
        );
    }
}
