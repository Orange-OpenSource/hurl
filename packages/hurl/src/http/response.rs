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

use super::{header, Header};
use core::fmt;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub version: Version,
    pub status: u32,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
    pub duration: Duration,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Version {
    Http10,
    Http11,
    Http2,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Version::Http10 => "1.0",
            Version::Http11 => "1.1",
            Version::Http2 => "2",
        };
        write!(f, "{}", value)
    }
}

impl Response {
    /// Returns all header values.
    pub fn get_header_values(&self, name: &str) -> Vec<String> {
        header::get_values(&self.headers, name)
    }

    /// Returns optional Content-type header value.
    pub fn content_type(&self) -> Option<String> {
        header::get_values(&self.headers, "Content-Type")
            .get(0)
            .cloned()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn hello_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header {
                    name: String::from("Content-Type"),
                    value: String::from("text/html; charset=utf-8"),
                },
                Header {
                    name: String::from("Content-Length"),
                    value: String::from("12"),
                },
            ],
            body: String::into_bytes(String::from("Hello World!")),
            duration: Default::default(),
        }
    }

    pub fn html_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: String::from("Content-Type"),
                value: String::from("text/html; charset=utf-8"),
            }],
            body: String::into_bytes(String::from(
                "<html><head><meta charset=\"UTF-8\"></head><body><br></body></html>",
            )),
            duration: Default::default(),
        }
    }

    pub fn xml_invalid_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header {
                    name: String::from("Content-Type"),
                    value: String::from("text/html; charset=utf-8"),
                },
                Header {
                    name: String::from("Content-Length"),
                    value: String::from("12"),
                },
            ],
            body: String::into_bytes(
                r#"
xxx
"#
                .to_string(),
            ),
            duration: Default::default(),
        }
    }

    pub fn xml_two_users_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header {
                    name: String::from("Content-Type"),
                    value: String::from("text/html; charset=utf-8"),
                },
                Header {
                    name: String::from("Content-Length"),
                    value: String::from("12"),
                },
            ],
            body: String::into_bytes(
                r#"
<?xml version="1.0"?>
<users>
  <user id="1">Bob</user>
  <user id="2">Bill</user>
</users>
"#
                .to_string(),
            ),
            duration: Default::default(),
        }
    }

    pub fn xml_three_users_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header {
                    name: String::from("Content-Type"),
                    value: String::from("text/html; charset=utf-8"),
                },
                Header {
                    name: String::from("Content-Length"),
                    value: String::from("12"),
                },
            ],
            body: String::into_bytes(
                r#"
<?xml version="1.0"?>
<users>
  <user id="1">Bob</user>
  <user id="2">Bill</user>
  <user id="3">Bruce</user>
</users>
"#
                .to_string(),
            ),
            duration: Default::default(),
        }
    }

    pub fn json_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 0,
            headers: vec![],
            body: String::into_bytes(
                r#"
{
  "success":false,
  "errors": [
    { "id": "error1"},
    {"id": "error2"}
  ],
  "duration": 1.5
}
"#
                .to_string(),
            ),
            duration: Default::default(),
        }
    }

    pub fn bytes_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header {
                    name: String::from("Content-Type"),
                    value: String::from("application/octet-stream"),
                },
                Header {
                    name: String::from("Content-Length"),
                    value: String::from("1"),
                },
            ],
            body: vec![255],
            duration: Default::default(),
        }
    }

    #[test]
    fn get_header_values() {
        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: "Content-Length".to_string(),
                value: "12".to_string(),
            }],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(
            response.get_header_values("Content-Length"),
            vec!["12".to_string()]
        );
        assert!(response.get_header_values("Unknown").is_empty());
    }
}
