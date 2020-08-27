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

use crate::http::core::*;
use crate::http::request::*;
use crate::http::response::*;

impl Request {
    pub fn to_text(&self) -> String {
        let mut s = format!("{} {}\n",
                            self.clone().method.to_text(),
                            self.clone().url()
        );
        for header in self.clone().headers() {
            s.push_str(header.to_text().as_str());
        }
        s.push_str("\n");

        let body = match self.clone().form_params() {
            None => body_text(self.clone().body, self.clone().content_type()),
            Some(params) => {
                let mut buf = String::from("[Form Params]");
                for param in params {
                    buf.push_str(format!("\n{}={}", param.name, param.value).as_str())
                }
                buf
            }
        };
        s.push_str(body.as_str());
        s.push_str("\n");
        s
    }
}

impl Response {
    pub fn to_text(&self, limit_body: usize) -> String {
        let mut s = format!("HTTP/{} {}\n", self.version.to_text(), self.status);
        for header in self.headers.clone() {
            s.push_str(header.to_text().as_str());
        }
        s.push_str("");

        // shoudl use number of char, not a number of bytes!!
        //let limit_body = 400; // TODO should be explicitly pass as a command-line argument
        if !self.body.is_empty() {
            let body = body_text(self.clone().body, get_header_value(self.clone().headers, "content-type"));
            s.push_str(substring(body.as_str(), 0, limit_body));
        }
        s
    }
}

impl Method {
    pub fn to_text(&self) -> String {
        match self {
            Method::Get => String::from("GET"),
            Method::Head => String::from("HEAD"),
            Method::Post => String::from("POST"),
            Method::Put => String::from("PUT"),
            Method::Delete => String::from("DELETE"),
            Method::Connect => String::from("CONNECT"),
            Method::Options => String::from("OPTIONS"),
            Method::Trace => String::from("TRACE"),
            Method::Patch => String::from("PATCH"),
        }
    }
}

impl Version {
    pub fn to_text(&self) -> String {
        match self {
            Version::Http10 => String::from("1.0"),
            Version::Http11 => String::from("1.1"),
            Version::Http2 => String::from("2"),
        }
    }
}

impl Header {
    fn to_text(&self) -> String {
        return format!("{}: {}\n", self.name, self.value);
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

fn body_text(bytes: Vec<u8>, content_type: Option<String>) -> String {
    match content_type {
        Some(content_type) =>
            if is_text(content_type.as_str()) {
                match String::from_utf8(bytes.clone()) {
                    Ok(v) => v,
                    Err(_) => format!("{:?}", bytes)
                }
            } else {
                format!("{:?}", bytes)
            }
        _ => {
            if bytes.is_empty() {
                String::from("")
            } else {
                format!("{:?}", bytes)
            }
        }
    }
}

fn is_text(content_type: &str) -> bool {
    for s in &[
        "application/json",
        "text/html",
        "charset=utf-8",
        "application/x-www-form-urlencoded"
    ] {
        if content_type.contains(s) {
            return true;
        }
    }
    false
}

fn substring(s: &str, start: usize, len: usize) -> &str {
    let mut char_pos = 0;
    let mut byte_start = 0;
    let mut it = s.chars();
    loop {
        if char_pos == start { break; }
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_start += c.len_utf8();
        } else { break; }
    }
    char_pos = 0;
    let mut byte_end = byte_start;
    loop {
        if char_pos == len { break; }
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_end += c.len_utf8();
        } else { break; }
    }
    &s[byte_start..byte_end]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_text() {
        assert_eq!(is_text("application/json"), true);
        assert_eq!(is_text("application/json;charset=utf-8"), true);
    }

    #[test]
    fn test_substring() {
        assert_eq!(substring("", 0, 0), "");
        assert_eq!(substring("hello world!", 0, 5), "hello");
        assert_eq!(substring("hello world!", 0, 15), "hello world!");
    }
}


