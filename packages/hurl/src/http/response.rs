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
use std::time::Duration;

use crate::http::certificate::Certificate;
use crate::http::{header, Header};

/// Represents an HTTP response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub version: HttpVersion,
    pub status: u32,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
    pub duration: Duration,
    pub url: String,
    /// The end-user certificate, in the response certificate chain
    pub certificate: Option<Certificate>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            version: HttpVersion::Http10,
            status: 200,
            headers: vec![],
            body: vec![],
            duration: Default::default(),
            url: String::new(),
            certificate: None,
        }
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

/// Represents the HTTP version of a HTTP transaction.
/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http2,
    Http3,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http2 => "HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
        };
        write!(f, "{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_header_values() {
        let response = Response {
            headers: vec![Header::new("Content-Length", "12")],
            ..Default::default()
        };
        assert_eq!(
            response.get_header_values("Content-Length"),
            vec!["12".to_string()]
        );
        assert!(response.get_header_values("Unknown").is_empty());
    }
}
