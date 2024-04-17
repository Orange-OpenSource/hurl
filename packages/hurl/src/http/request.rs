/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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

use url::Url;

use crate::http::core::*;
use crate::http::header::{HeaderVec, COOKIE};
use crate::http::HttpError;

/// Represents a runtime HTTP request.
/// This is a real request, that has been executed by our HTTP client.
/// It's different from `crate::http::RequestSpec` which is the request asked to be executed by our
/// user. For instance, in the request spec, headers implicitly added by curl are not present, while
/// they will be present in the [`Request`] instances.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: HeaderVec,
    pub body: Vec<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum RequestedHttpVersion {
    #[default]
    Default, // The effective HTTP version will be chosen by libcurl
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
    #[default]
    Default, // Default, can use addresses of all IP versions that your system allows.
    IpV4,
    IpV6,
}

impl Request {
    /// Creates a new request.
    pub fn new(method: &str, url: &str, headers: HeaderVec, body: Vec<u8>) -> Self {
        Request {
            url: url.to_string(),
            method: method.to_string(),
            headers,
            body,
        }
    }

    /// Extracts query string params from the url of the request.
    pub fn query_string_params(&self) -> Vec<Param> {
        let u = Url::parse(self.url.as_str()).expect("valid url");
        let mut params = vec![];
        for (name, value) in u.query_pairs() {
            let param = Param {
                name: name.to_string(),
                value: value.to_string(),
            };
            params.push(param);
        }
        params
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

    /// Returns the base url http(s)://host(:port)
    pub fn base_url(&self) -> Result<String, HttpError> {
        // FIXME: is it possible to do it with libcurl?
        let url = match Url::parse(&self.url) {
            Ok(url) => url,
            Err(e) => return Err(HttpError::InvalidUrl(self.url.clone(), e.to_string())),
        };
        let scheme = url.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(HttpError::InvalidUrl(
                self.url.clone(),
                "Missing protocol http or https".to_string(),
            ));
        }
        let host = match url.host() {
            Some(host) => host,
            None => {
                return Err(HttpError::InvalidUrl(
                    self.url.clone(),
                    "Can not extract host".to_string(),
                ))
            }
        };
        let port = match url.port() {
            Some(port) => format!(":{port}"),
            None => String::new(),
        };
        Ok(format!("{scheme}://{host}{port}"))
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

        Request::new("GET", "http://localhost:8000/hello", headers, vec![])
    }

    fn query_string_request() -> Request {
        Request::new("GET", "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3", HeaderVec::new(), vec![])
    }

    fn cookies_request() -> Request {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Cookie", "cookie1=value1; cookie2=value2"));
        Request::new("GET", "http://localhost:8000/cookies", headers, vec![])
    }

    #[test]
    fn test_query_string() {
        assert!(hello_request().query_string_params().is_empty());
        assert_eq!(
            query_string_request().query_string_params(),
            vec![
                Param {
                    name: "param1".to_string(),
                    value: "value1".to_string(),
                },
                Param {
                    name: "param2".to_string(),
                    value: String::new(),
                },
                Param {
                    name: "param3".to_string(),
                    value: "a=b".to_string(),
                },
                Param {
                    name: "param4".to_string(),
                    value: "1,2,3".to_string(),
                },
            ]
        );
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

    #[test]
    fn test_base_url() {
        assert_eq!(
            Request::new("", "http://localhost", HeaderVec::new(), vec![])
                .base_url()
                .unwrap(),
            "http://localhost".to_string()
        );
        assert_eq!(
            Request::new(
                "",
                "http://localhost:8000/redirect-relative",
                HeaderVec::new(),
                vec![]
            )
            .base_url()
            .unwrap(),
            "http://localhost:8000".to_string()
        );
        assert_eq!(
            Request::new("", "https://localhost:8000", HeaderVec::new(), vec![])
                .base_url()
                .unwrap(),
            "https://localhost:8000".to_string()
        );
    }
}
