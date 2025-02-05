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

///
/// Decompresses body response
/// using the Content-Encoding response header
///
/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding
use std::io::prelude::*;

use encoding::DecoderTrap;

use crate::http::{mimetype, HttpError, Response};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContentEncoding {
    /// A format using the Brotli algorithm structure (defined in RFC 7932).
    Brotli,
    /// A format using the Lempel-Ziv coding (LZ77), with a 32-bit CRC.
    Gzip,
    /// Using the zlib structure (defined in RFC 1950) with the deflate compression algorithm.
    Deflate,
    /// No encoding.
    Identity,
}

impl ContentEncoding {
    /// Returns an encoding from an HTTP header value `s`.
    pub fn parse(s: &str) -> Result<ContentEncoding, HttpError> {
        match s {
            "br" => Ok(ContentEncoding::Brotli),
            "gzip" => Ok(ContentEncoding::Gzip),
            "deflate" => Ok(ContentEncoding::Deflate),
            "identity" => Ok(ContentEncoding::Identity),
            v => Err(HttpError::UnsupportedContentEncoding {
                description: v.to_string(),
            }),
        }
    }

    /// Decompresses `data` bytes.
    pub fn decode(&self, data: &[u8]) -> Result<Vec<u8>, HttpError> {
        match self {
            ContentEncoding::Identity => Ok(data.to_vec()),
            ContentEncoding::Gzip => uncompress_gzip(data),
            ContentEncoding::Deflate => uncompress_zlib(data),
            ContentEncoding::Brotli => uncompress_brotli(data),
        }
    }
}

impl Response {
    /// Returns response body as text.
    pub fn text(&self) -> Result<String, HttpError> {
        let content_encodings = self.headers.content_encoding()?;
        let body = if content_encodings.is_empty() {
            &self.body
        } else {
            &self.uncompress_body()?
        };
        let character_encoding = self.headers.character_encoding()?;
        match character_encoding.decode(body, DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(_) => Err(HttpError::InvalidDecoding {
                charset: character_encoding.name().to_string(),
            }),
        }
    }

    /// Returns true if response is an HTML response.
    pub fn is_html(&self) -> bool {
        self.headers.content_type().is_some_and(mimetype::is_html)
    }

    /// Returns true if response is a JSON response.
    pub fn is_json(&self) -> bool {
        self.headers.content_type().is_some_and(mimetype::is_json)
    }

    /// Returns true if response is a XML response.
    pub fn is_xml(&self) -> bool {
        self.headers.content_type().is_some_and(mimetype::is_xml)
    }

    /// Decompresses HTTP body response.
    pub fn uncompress_body(&self) -> Result<Vec<u8>, HttpError> {
        let encodings = self.headers.content_encoding()?;
        let mut data = self.body.clone();
        for encoding in &encodings {
            data = encoding.decode(&data)?;
        }
        Ok(data)
    }
}

/// Decompresses Brotli compressed `data`.
fn uncompress_brotli(data: &[u8]) -> Result<Vec<u8>, HttpError> {
    let buffer_size = 4096;
    let mut reader = brotli::Decompressor::new(data, buffer_size);
    let mut buf = Vec::new();
    match reader.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err(HttpError::CouldNotUncompressResponse {
            description: "brotli".to_string(),
        }),
    }
}

/// Decompresses GZip compressed `data`.
fn uncompress_gzip(data: &[u8]) -> Result<Vec<u8>, HttpError> {
    let mut decoder = match libflate::gzip::Decoder::new(data) {
        Ok(v) => v,
        Err(_) => {
            return Err(HttpError::CouldNotUncompressResponse {
                description: "gzip".to_string(),
            })
        }
    };
    let mut buf = Vec::new();
    match decoder.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err(HttpError::CouldNotUncompressResponse {
            description: "gzip".to_string(),
        }),
    }
}

/// Decompresses Zlib compressed `data`.
fn uncompress_zlib(data: &[u8]) -> Result<Vec<u8>, HttpError> {
    let mut decoder = match libflate::zlib::Decoder::new(data) {
        Ok(v) => v,
        Err(_) => {
            return Err(HttpError::CouldNotUncompressResponse {
                description: "zlib".to_string(),
            })
        }
    };
    let mut buf = Vec::new();
    match decoder.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err(HttpError::CouldNotUncompressResponse {
            description: "zlib".to_string(),
        }),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::http::{Header, HeaderVec, HttpVersion, Response};

    fn default_response() -> Response {
        Response {
            version: HttpVersion::Http10,
            status: 200,
            headers: HeaderVec::new(),
            body: vec![],
            duration: Default::default(),
            url: "http://localhost".parse().unwrap(),
            certificate: None,
            ip_addr: Default::default(),
        }
    }

    #[test]
    fn test_parse_content_encoding() {
        assert_eq!(
            ContentEncoding::parse("br").unwrap(),
            ContentEncoding::Brotli
        );
        assert_eq!(
            ContentEncoding::parse("xx").err().unwrap(),
            HttpError::UnsupportedContentEncoding {
                description: "xx".to_string()
            }
        );
    }

    #[test]
    fn test_content_encoding() {
        let response = default_response();
        assert_eq!(response.headers.content_encoding().unwrap(), vec![]);

        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Encoding", "xx"));

        let response = Response {
            headers,
            ..default_response()
        };
        assert_eq!(
            response.headers.content_encoding().err().unwrap(),
            HttpError::UnsupportedContentEncoding {
                description: "xx".to_string()
            }
        );

        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Encoding", "br"));

        let response = Response {
            headers,
            ..default_response()
        };
        assert_eq!(
            response.headers.content_encoding().unwrap(),
            vec![ContentEncoding::Brotli]
        );
    }

    #[test]
    fn test_multiple_content_encoding() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Encoding", "br, identity"));
        let response = Response {
            headers,
            ..default_response()
        };
        assert_eq!(
            response.headers.content_encoding().unwrap(),
            vec![ContentEncoding::Brotli, ContentEncoding::Identity]
        );
    }

    #[test]
    fn test_uncompress_body() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Encoding", "br"));

        let response = Response {
            headers,
            body: vec![
                0x21, 0x2c, 0x00, 0x04, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c,
                0x64, 0x21, 0x03,
            ],
            ..default_response()
        };
        assert_eq!(response.uncompress_body().unwrap(), b"Hello World!");

        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Encoding", "br, identity"));
        let response = Response {
            headers,
            body: vec![
                0x21, 0x2c, 0x00, 0x04, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c,
                0x64, 0x21, 0x03,
            ],
            ..default_response()
        };
        assert_eq!(response.uncompress_body().unwrap(), b"Hello World!");

        let response = Response {
            body: b"Hello World!".to_vec(),
            ..default_response()
        };
        assert_eq!(response.uncompress_body().unwrap(), b"Hello World!");
    }

    #[test]
    fn test_uncompress_brotli() {
        let data = [
            0x21, 0x2c, 0x00, 0x04, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c,
            0x64, 0x21, 0x03,
        ];
        assert_eq!(uncompress_brotli(&data[..]).unwrap(), b"Hello World!");
    }

    #[test]
    fn test_uncompress_gzip() {
        let data = [
            0x1f, 0x8b, 0x08, 0x08, 0xa7, 0x52, 0x85, 0x5f, 0x00, 0x03, 0x64, 0x61, 0x74, 0x61,
            0x2e, 0x74, 0x78, 0x74, 0x00, 0xf3, 0x48, 0xcd, 0xc9, 0xc9, 0x57, 0x08, 0xcf, 0x2f,
            0xca, 0x49, 0x51, 0x04, 0x00, 0xa3, 0x1c, 0x29, 0x1c, 0x0c, 0x00, 0x00, 0x00,
        ];
        assert_eq!(uncompress_gzip(&data[..]).unwrap(), b"Hello World!");
    }

    #[test]
    fn test_uncompress_zlib() {
        let data = [
            0x78, 0x9c, 0xf3, 0x48, 0xcd, 0xc9, 0xc9, 0x57, 0x08, 0xcf, 0x2f, 0xca, 0x49, 0x51,
            0x04, 0x00, 0x1c, 0x49, 0x04, 0x3e,
        ];
        assert_eq!(uncompress_zlib(&data[..]).unwrap(), b"Hello World!");
    }

    #[test]
    fn test_uncompress_error() {
        let data = [0x21];
        assert_eq!(
            uncompress_brotli(&data[..]).err().unwrap(),
            HttpError::CouldNotUncompressResponse {
                description: "brotli".to_string()
            }
        );
        assert_eq!(
            uncompress_gzip(&data[..]).err().unwrap(),
            HttpError::CouldNotUncompressResponse {
                description: "gzip".to_string()
            }
        );
    }

    fn hello_response() -> Response {
        Response {
            body: b"Hello World!".to_vec(),
            ..default_response()
        }
    }

    fn utf8_encoding_response() -> Response {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Type", "text/plain; charset=utf-8"));

        Response {
            headers,
            body: vec![0x63, 0x61, 0x66, 0xc3, 0xa9],
            ..default_response()
        }
    }

    fn latin1_encoding_response() -> Response {
        let mut headers = HeaderVec::new();
        headers.push(Header::new(
            "Content-Type",
            "text/plain; charset=ISO-8859-1",
        ));

        Response {
            headers,
            body: vec![0x63, 0x61, 0x66, 0xe9],
            ..default_response()
        }
    }

    #[test]
    pub fn test_content_type() {
        assert_eq!(hello_response().headers.content_type(), None);
        assert_eq!(
            utf8_encoding_response().headers.content_type(),
            Some("text/plain; charset=utf-8")
        );
        assert_eq!(
            latin1_encoding_response().headers.content_type(),
            Some("text/plain; charset=ISO-8859-1")
        );
    }

    #[test]
    pub fn test_character_encoding() {
        assert_eq!(
            hello_response()
                .headers
                .character_encoding()
                .unwrap()
                .name(),
            "utf-8"
        );
        assert_eq!(
            utf8_encoding_response()
                .headers
                .character_encoding()
                .unwrap()
                .name(),
            "utf-8"
        );
        assert_eq!(
            latin1_encoding_response()
                .headers
                .character_encoding()
                .unwrap()
                .name(),
            "windows-1252"
        );
    }

    #[test]
    pub fn test_text() {
        assert_eq!(hello_response().text().unwrap(), "Hello World!".to_string());
        assert_eq!(utf8_encoding_response().text().unwrap(), "café".to_string());
        assert_eq!(
            latin1_encoding_response().text().unwrap(),
            "café".to_string()
        );
    }

    #[test]
    pub fn test_invalid_charset() {
        let mut headers = HeaderVec::new();
        headers.push(Header::new("Content-Type", "test/plain; charset=xxx"));

        assert_eq!(
            Response {
                headers,
                body: b"Hello World!".to_vec(),
                ..default_response()
            }
            .headers
            .character_encoding()
            .err()
            .unwrap(),
            HttpError::InvalidCharset {
                charset: "xxx".to_string()
            }
        );
    }

    #[test]
    pub fn test_invalid_decoding() {
        assert_eq!(
            Response {
                body: vec![0x63, 0x61, 0x66, 0xe9],
                ..default_response()
            }
            .text()
            .err()
            .unwrap(),
            HttpError::InvalidDecoding {
                charset: "utf-8".to_string()
            }
        );

        let mut headers = HeaderVec::new();
        headers.push(Header::new(
            "Content-Type",
            "text/plain; charset=ISO-8859-1",
        ));

        assert_eq!(
            Response {
                headers,
                body: vec![0x63, 0x61, 0x66, 0xc3, 0xa9],
                ..default_response()
            }
            .text()
            .unwrap(),
            "cafÃ©".to_string()
        );
    }
}
