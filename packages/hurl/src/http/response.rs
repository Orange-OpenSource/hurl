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
use std::time::Duration;

use crate::http::certificate::Certificate;
use crate::http::ip::IpAddr;
use crate::http::{HeaderVec, Url};

/// Represents a runtime HTTP response.
/// This is a real response, that has been executed by our HTTP client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub version: HttpVersion,
    pub status: u32,
    pub headers: HeaderVec,
    pub body: Vec<u8>,
    pub duration: Duration,
    pub url: Url,
    /// The end-user certificate, in the response certificate chain
    pub certificate: Option<Certificate>,
    pub ip_addr: IpAddr,
}

impl Response {
    /// Creates a new HTTP response
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        version: HttpVersion,
        status: u32,
        headers: HeaderVec,
        body: Vec<u8>,
        duration: Duration,
        url: Url,
        certificate: Option<Certificate>,
        ip_addr: IpAddr,
    ) -> Self {
        Response {
            version,
            status,
            headers,
            body,
            duration,
            url,
            certificate,
            ip_addr,
        }
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
    use crate::http::Header;

    #[test]
    fn get_header_values() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Length", "12"));
        let response = Response {
            version: HttpVersion::Http10,
            status: 200,
            headers,
            body: vec![],
            duration: Default::default(),
            url: "http://localhost".parse().unwrap(),
            certificate: None,
            ip_addr: Default::default(),
        };
        assert_eq!(response.headers.values("Content-Length"), vec!["12"]);
        assert!(response.headers.values("Unknown").is_empty());
    }
}
