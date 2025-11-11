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
use encoding_rs::Encoding;

use super::error::HttpError;
use super::header::{Header, HeaderVec, CONTENT_ENCODING, CONTENT_TYPE};
use super::mimetype;
use super::response_decoding::ContentEncoding;

impl HeaderVec {
    /// Returns optional Content-type header value.
    pub fn content_type(&self) -> Option<&str> {
        self.get(CONTENT_TYPE).map(|h| h.value.as_str())
    }

    /// Returns character encoding from this list of headers.
    ///
    /// If no character encoding can be found, returns UTF-8.
    pub fn character_encoding(&self) -> Result<&'static Encoding, HttpError> {
        match self.content_type() {
            Some(content_type) => match mimetype::charset(content_type) {
                Some(charset) => match Encoding::for_label(charset.as_bytes()) {
                    None => Err(HttpError::InvalidCharset { charset }),
                    Some(enc) => Ok(enc),
                },
                None => Ok(encoding_rs::UTF_8),
            },
            None => Ok(encoding_rs::UTF_8),
        }
    }

    /// Returns list of content encoding from HTTP response headers.
    ///
    /// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding>
    pub fn content_encoding(&self) -> Result<Vec<ContentEncoding>, HttpError> {
        for header in self {
            if header.name_eq(CONTENT_ENCODING) {
                let mut encodings = vec![];
                for value in header.value.split(',') {
                    let encoding = ContentEncoding::parse(value.trim())?;
                    encodings.push(encoding);
                }
                return Ok(encodings);
            }
        }
        Ok(vec![])
    }

    /// Returns a new list of headers with the headers from `self` and the raw headers `raw_headers`.
    pub fn with_raw_headers(&self, raw_headers: &[&str]) -> HeaderVec {
        let mut headers = self.clone();
        // TODO: use another function that [`Header::parse`] because [`Header::parse`] is for
        // parsing headers line coming from a server (and not from options header)
        let raw_headers = raw_headers.iter().filter_map(|h| Header::parse(h));
        for header in raw_headers {
            headers.push(header);
        }
        headers
    }
}

#[cfg(test)]
mod tests {
    use crate::http::response_decoding::ContentEncoding;
    use crate::http::{Header, HeaderVec};

    #[test]
    fn content_type_basic() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Host", "localhost:8000"));
        headers.push(Header::new("Accept", "*/*"));
        headers.push(Header::new("User-Agent", "hurl/1.0"));
        headers.push(Header::new("content-type", "application/json"));
        assert_eq!(headers.content_type(), Some("application/json"));

        let mut headers = HeaderVec::new();
        headers.push(Header::new("foo", "bar"));
        assert_eq!(headers.content_type(), None);
    }

    #[test]
    fn content_encoding() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Encoding", "deflate, gzip"));
        assert_eq!(
            headers.content_encoding(),
            Ok(vec![ContentEncoding::Deflate, ContentEncoding::Gzip])
        );
    }

    #[test]
    fn character_encoding() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Type", "text/html; charset=utf-8"));
        assert_eq!(headers.character_encoding().unwrap(), encoding_rs::UTF_8);

        let mut headers = HeaderVec::new();
        headers.push(Header::new("content-type", "text/plain; charset=us-ascii"));
        assert_eq!(
            headers.character_encoding().unwrap(),
            encoding_rs::WINDOWS_1252
        );

        let mut headers = HeaderVec::new();
        headers.push(Header::new("content-type", "text/plain"));
        assert_eq!(headers.character_encoding().unwrap(), encoding_rs::UTF_8);
    }

    #[test]
    fn test_with_raw_headers() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Host", "localhost:8000"));
        headers.push(Header::new("Repeated-Header", "original"));

        let raw_headers = &[
            "User-Agent: hurl/6.1.0",
            "Invalid-Header",
            "Repeated-Header: aggregated-1",
            "Repeated-Header: aggregated-2",
        ];
        let headers = headers.with_raw_headers(raw_headers);

        assert_eq!(
            headers.get("Host"),
            Some(&Header::new("Host", "localhost:8000"))
        );
        assert_eq!(
            headers.get("User-Agent"),
            Some(&Header::new("User-Agent", "hurl/6.1.0"))
        );
        assert_eq!(headers.get("Invalid-Header"), None);
        assert_eq!(
            headers.get_all("Repeated-Header"),
            vec![
                &Header::new("Repeated-Header", "original"),
                &Header::new("Repeated-Header", "aggregated-1"),
                &Header::new("Repeated-Header", "aggregated-2")
            ]
        );
    }
}
