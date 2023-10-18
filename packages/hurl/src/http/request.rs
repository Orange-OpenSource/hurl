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
use std::fmt;
use url::Url;

use crate::http::core::*;
use crate::http::{header, Header, HttpError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<Header>,
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
            .iter()
            .filter(|h| h.name.as_str() == "Cookie")
            .flat_map(|h| parse_cookies(h.value.as_str().trim()))
            .collect()
    }

    /// Returns optional Content-type header value.
    pub fn content_type(&self) -> Option<String> {
        header::get_values(&self.headers, "Content-Type")
            .get(0)
            .cloned()
    }

    /// Returns the base url http(s)://host(:port)
    pub fn base_url(&self) -> Result<String, HttpError> {
        // FIXME: is it possible to do it with libcurl?
        let url = match Url::parse(&self.url) {
            Ok(url) => url,
            Err(_) => return Err(HttpError::InvalidUrl(self.url.clone())),
        };
        let scheme = url.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(HttpError::InvalidUrlPrefix(self.url.clone()));
        }
        let host = match url.host() {
            Some(host) => host,
            None => return Err(HttpError::InvalidUrl(self.url.clone())),
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
    use crate::http::RequestCookie;

    fn hello_request() -> Request {
        Request {
            method: "GET".to_string(),
            url: "http://localhost:8000/hello".to_string(),
            headers: vec![
                Header::new("Host", "localhost:8000"),
                Header::new("Accept", "*/*"),
                Header::new("User-Agent", "hurl/1.0"),
            ],
            body: vec![],
        }
    }

    fn query_string_request() -> Request {
        Request {
            method: "GET".to_string(),
            url: "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3".to_string(),
            headers: vec![],
            body: vec![],
        }
    }

    fn cookies_request() -> Request {
        Request {
            method: "GET".to_string(),
            url: "http://localhost:8000/cookies".to_string(),
            headers: vec![Header::new("Cookie", "cookie1=value1; cookie2=value2")],
            body: vec![],
        }
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
        )
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
        )
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
        )
    }

    #[test]
    fn test_parse_cookie() {
        assert_eq!(
            parse_cookie("cookie1=value1"),
            RequestCookie {
                name: "cookie1".to_string(),
                value: "value1".to_string(),
            },
        )
    }

    #[test]
    fn test_base_url() {
        assert_eq!(
            Request {
                url: "http://localhost".to_string(),
                method: String::new(),
                headers: vec![],
                body: vec![],
            }
            .base_url()
            .unwrap(),
            "http://localhost".to_string()
        );
        assert_eq!(
            Request {
                url: "http://localhost:8000/redirect-relative".to_string(),
                method: String::new(),
                headers: vec![],
                body: vec![],
            }
            .base_url()
            .unwrap(),
            "http://localhost:8000".to_string()
        );
        assert_eq!(
            Request {
                url: "https://localhost:8000".to_string(),
                method: String::new(),
                headers: vec![],
                body: vec![],
            }
            .base_url()
            .unwrap(),
            "https://localhost:8000".to_string()
        );
    }
}
