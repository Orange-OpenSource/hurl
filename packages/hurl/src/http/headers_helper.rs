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

use encoding::EncodingRef;

use crate::http::header::CONTENT_ENCODING;
use crate::http::response_decoding::ContentEncoding;
use crate::http::{mimetype, HeaderVec, HttpError, CONTENT_TYPE};

impl HeaderVec {
    /// Returns optional Content-type header value.
    pub fn content_type(&self) -> Option<&str> {
        self.get(CONTENT_TYPE).map(|h| h.value.as_str())
    }

    /// Returns character encoding from this list of headers.
    ///
    /// If no character encoding can be found, returns UTF-8.
    pub fn character_encoding(&self) -> Result<EncodingRef, HttpError> {
        match self.content_type() {
            Some(content_type) => match mimetype::charset(content_type) {
                Some(charset) => match encoding::label::encoding_from_whatwg_label(&charset) {
                    None => Err(HttpError::InvalidCharset { charset }),
                    Some(enc) => Ok(enc),
                },
                None => Ok(encoding::all::UTF_8),
            },
            None => Ok(encoding::all::UTF_8),
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
        assert_eq!(headers.character_encoding().unwrap().name(), "utf-8");

        let mut headers = HeaderVec::new();
        headers.push(Header::new("content-type", "text/plain; charset=us-ascii"));
        assert_eq!(headers.character_encoding().unwrap().name(), "windows-1252");

        let mut headers = HeaderVec::new();
        headers.push(Header::new("content-type", "text/plain"));
        assert_eq!(headers.character_encoding().unwrap().name(), "utf-8");
    }
}
