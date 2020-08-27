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

use super::cookie::*;
use super::core::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub version: Version,
    pub status: u16,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Version {
    Http10,
    Http11,
    Http2,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Version::Http10 => "1.0",
            Version::Http11 => "1.1",
            Version::Http2 => "2",
        };
        write!(f, "{}", s)
    }
}

impl Response {
    pub fn get_header(&self, name: &str, case_sensitive: bool) -> Vec<String> {
        let mut values = vec![];
        for header in self.headers.clone() {
            if header.name == name
                || !case_sensitive && header.name.to_lowercase() == name.to_lowercase()
            {
                values.push(header.value);
            }
        }
        values
    }

    pub fn get_cookie(&self, name: &str) -> Option<ResponseCookie> {
        for cookie in self.cookies() {
            if cookie.name.as_str() == name
            {
                return Some(cookie);
            }
        }
        None
    }

    pub fn cookies(&self) -> Vec<ResponseCookie> {
        let mut cookies = vec![];
        for Header { name, value } in self.clone().headers {
            if name.to_lowercase() == "set-cookie" {
                let c = cookie::Cookie::parse(value.as_str()).unwrap();
//                eprintln!(">>> parse set-cookie header");
//                eprintln!(">>> c = {:?}", c);
//
//                let fields = value.split(";").collect::<Vec<&str>>();
//                let name_value = fields.get(0).unwrap().split("=").collect::<Vec<&str>>();
//                let name = name_value.get(0).unwrap().to_string();
//                let value = name_value.get(1).unwrap().to_string();
                let name = c.name().to_string();
                let value = c.value().to_string();
                let max_age = match c.max_age() {
                    None => None,
                    Some(d) => Some(d.num_seconds())
                };
                let expires = match c.expires() {
                    None => None,
                    Some(time) => Some(time.rfc822().to_string())
                };
                let domain = match c.domain() {
                    None => None,
                    Some(v) => Some(v.to_string())
                };
                let path = match c.path() {
                    None => None,
                    Some(v) => Some(v.to_string())
                };
                let secure = if let Some(value) = c.secure() {
                    Some(value)
                } else {
                    None
                };
                let http_only = if let Some(value) = c.http_only() {
                    Some(value)
                } else {
                    None
                };
                let same_site = match c.same_site() {
                    None => None,
                    Some(v) => Some(v.to_string())
                };
                cookies.push(ResponseCookie { name, value, max_age, expires, domain, path, secure, http_only, same_site });
            }
        }
        cookies
    }
}

impl Response {
    pub fn content_type(&self) -> Option<String> {
        let values = self.get_header("content-type", true);
        if let Some(value) = values.first() {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn encoding(&self) -> Encoding {
        if let Some(value) = self.content_type() {
            if value.contains("charset=ISO-8859-1") {
                return Encoding::Latin1;
            }
        }
        Encoding::Utf8
    }

    pub fn has_utf8_body(&self) -> bool {
        if let Some(value) = self.content_type() {
            value.contains("charset=utf-8")
        } else {
            false
        }
    }


    pub fn is_html(&self) -> bool {
        if let Some(value) = self.content_type() {
            value.contains("html")
        } else {
            false
        }
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
                Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
                Header { name: String::from("Content-Length"), value: String::from("12") },
            ],
            body: String::into_bytes(String::from("Hello World!")),
        }
    }

    pub fn html_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
            ],
            body: String::into_bytes(String::from("<html><head><meta charset=\"UTF-8\"></head><body><br></body></html>")),
        }
    }

    pub fn xml_invalid_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
                Header { name: String::from("Content-Length"), value: String::from("12") },
            ],
            body: String::into_bytes(r#"
xxx
"#.to_string()),
        }
    }

    pub fn xml_two_users_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
                Header { name: String::from("Content-Length"), value: String::from("12") },
            ],
            body: String::into_bytes(r#"
<?xml version="1.0"?>
<users>
  <user id="1">Bob</user>
  <user id="2">Bill</user>
</users>
"#.to_string()),
        }
    }

    pub fn xml_three_users_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header { name: String::from("Content-Type"), value: String::from("text/html; charset=utf-8") },
                Header { name: String::from("Content-Length"), value: String::from("12") },
            ],
            body: String::into_bytes(r#"
<?xml version="1.0"?>
<users>
  <user id="1">Bob</user>
  <user id="2">Bill</user>
  <user id="3">Bruce</user>
</users>
"#.to_string()),
        }
    }

    pub fn json_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 0,
            headers: vec![],
            body: String::into_bytes(r#"
{
  "success":false,
  "errors": [
    { "id": "error1"},
    {"id": "error2"}
  ],
  "duration": 1.5
}
"#.to_string()),
        }
    }

    pub fn bytes_http_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![
                Header { name: String::from("Content-Type"), value: String::from("application/octet-stream") },
                Header { name: String::from("Content-Length"), value: String::from("1") },
            ],
            body: vec![255],
        }
    }
}
