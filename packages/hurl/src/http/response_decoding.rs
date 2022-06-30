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

use encoding::{DecoderTrap, EncodingRef};
///
/// Decompresses body response
/// using the Content-Encoding response header
///
/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding
use std::io::prelude::*;

use crate::http::{mimetype, HttpError, Response};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentEncoding {
    Brotli,
    Gzip,
    Deflate,
    Identity,
}

impl ContentEncoding {
    /// Returns an encoding from an HTTP header value.
    ///
    /// # Arguments
    ///
    /// * `s` - A Content-Encoding header value
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

    /// Decompresses bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - A compressed bytes array
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
    /// Returns character encoding of the HTTP response.
    fn character_encoding(&self) -> Result<EncodingRef, HttpError> {
        match self.content_type() {
            Some(content_type) => match mimetype::charset(&content_type) {
                Some(charset) => {
                    match encoding::label::encoding_from_whatwg_label(charset.as_str()) {
                        None => Err(HttpError::InvalidCharset { charset }),
                        Some(enc) => Ok(enc),
                    }
                }
                None => Ok(encoding::all::UTF_8),
            },
            None => Ok(encoding::all::UTF_8),
        }
    }

    /// Returns response body as text.
    pub fn text(&self) -> Result<String, HttpError> {
        let encoding = self.character_encoding()?;
        let body = &self.uncompress_body()?;
        match encoding.decode(body, DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(_) => Err(HttpError::InvalidDecoding {
                charset: encoding.name().to_string(),
            }),
        }
    }

    /// Returns true if response is an HTML response.
    pub fn is_html(&self) -> bool {
        match self.content_type() {
            None => false,
            Some(s) => mimetype::is_html(&s),
        }
    }

    /// Returns list of content encoding from HTTP response headers.
    ///
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding
    fn content_encoding(&self) -> Result<Vec<ContentEncoding>, HttpError> {
        for header in &self.headers {
            if header.name.as_str().to_ascii_lowercase() == "content-encoding" {
                let mut encodings = vec![];
                for value in header.value.as_str().split(',') {
                    let encoding = ContentEncoding::parse(value.trim())?;
                    encodings.push(encoding);
                }
                return Ok(encodings);
            }
        }
        Ok(vec![])
    }

    /// Decompresses HTTP body response.
    pub fn uncompress_body(&self) -> Result<Vec<u8>, HttpError> {
        let encodings = self.content_encoding()?;
        let mut data = self.body.clone();
        for encoding in encodings {
            data = encoding.decode(&data)?
        }
        Ok(data)
    }
}

/// Decompresses Brotli compressed data.
///
/// # Arguments
///
/// * data - Compressed bytes.
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

/// Decompresses GZip compressed data.
///
/// # Arguments
///
/// * data - Compressed bytes.
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

/// Decompresses Zlib compressed data.
///
/// # Arguments
///
/// * data - Compressed bytes.
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
    use crate::http::{Header, Response, Version};

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
        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(response.content_encoding().unwrap(), vec![]);

        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: "Content-Encoding".to_string(),
                value: "xx".to_string(),
            }],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(
            response.content_encoding().err().unwrap(),
            HttpError::UnsupportedContentEncoding {
                description: "xx".to_string()
            }
        );

        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: "Content-Encoding".to_string(),
                value: "br".to_string(),
            }],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(
            response.content_encoding().unwrap(),
            vec![ContentEncoding::Brotli]
        );
    }

    #[test]
    fn test_multiple_content_encoding() {
        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: "Content-Encoding".to_string(),
                value: "br, identity".to_string(),
            }],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(
            response.content_encoding().unwrap(),
            vec![ContentEncoding::Brotli, ContentEncoding::Identity]
        );
    }

    #[test]
    fn test_uncompress_body() {
        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: "Content-Encoding".to_string(),
                value: "br".to_string(),
            }],
            body: vec![
                0x21, 0x2c, 0x00, 0x04, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c,
                0x64, 0x21, 0x03,
            ],
            duration: Default::default(),
        };
        assert_eq!(response.uncompress_body().unwrap(), b"Hello World!");

        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: "Content-Encoding".to_string(),
                value: "br, identity".to_string(),
            }],
            body: vec![
                0x21, 0x2c, 0x00, 0x04, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c,
                0x64, 0x21, 0x03,
            ],
            duration: Default::default(),
        };
        assert_eq!(response.uncompress_body().unwrap(), b"Hello World!");

        let response = Response {
            version: Version::Http10,
            status: 200,
            headers: vec![],
            body: b"Hello World!".to_vec(),
            duration: Default::default(),
        };
        assert_eq!(response.uncompress_body().unwrap(), b"Hello World!");
    }

    #[test]
    fn test_uncompress_brotli() {
        let data = vec![
            0x21, 0x2c, 0x00, 0x04, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c,
            0x64, 0x21, 0x03,
        ];
        assert_eq!(uncompress_brotli(&data[..]).unwrap(), b"Hello World!");
    }

    #[test]
    fn test_uncompress_gzip() {
        let data = vec![
            0x1f, 0x8b, 0x08, 0x08, 0xa7, 0x52, 0x85, 0x5f, 0x00, 0x03, 0x64, 0x61, 0x74, 0x61,
            0x2e, 0x74, 0x78, 0x74, 0x00, 0xf3, 0x48, 0xcd, 0xc9, 0xc9, 0x57, 0x08, 0xcf, 0x2f,
            0xca, 0x49, 0x51, 0x04, 0x00, 0xa3, 0x1c, 0x29, 0x1c, 0x0c, 0x00, 0x00, 0x00,
        ];
        assert_eq!(uncompress_gzip(&data[..]).unwrap(), b"Hello World!");
    }

    #[test]
    fn test_uncompress_zlib() {
        let data = vec![
            0x78, 0x9c, 0xf3, 0x48, 0xcd, 0xc9, 0xc9, 0x57, 0x08, 0xcf, 0x2f, 0xca, 0x49, 0x51,
            0x04, 0x00, 0x1c, 0x49, 0x04, 0x3e,
        ];
        assert_eq!(uncompress_zlib(&data[..]).unwrap(), b"Hello World!");
    }

    #[test]
    fn test_uncompress_error() {
        let data = vec![0x21];
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
            version: Version::Http10,
            status: 200,
            headers: vec![],
            body: b"Hello World!".to_vec(),
            duration: Default::default(),
        }
    }

    fn utf8_encoding_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: "Content-Type".to_string(),
                value: "text/plain; charset=utf-8".to_string(),
            }],
            body: vec![0x63, 0x61, 0x66, 0xc3, 0xa9],
            duration: Default::default(),
        }
    }

    fn latin1_encoding_response() -> Response {
        Response {
            version: Version::Http10,
            status: 200,
            headers: vec![Header {
                name: "Content-Type".to_string(),
                value: "text/plain; charset=ISO-8859-1".to_string(),
            }],
            body: vec![0x63, 0x61, 0x66, 0xe9],
            duration: Default::default(),
        }
    }

    #[test]
    pub fn test_content_type() {
        assert_eq!(hello_response().content_type(), None);
        assert_eq!(
            utf8_encoding_response().content_type(),
            Some("text/plain; charset=utf-8".to_string())
        );
        assert_eq!(
            latin1_encoding_response().content_type(),
            Some("text/plain; charset=ISO-8859-1".to_string())
        );
    }

    #[test]
    pub fn test_character_encoding() {
        assert_eq!(
            hello_response().character_encoding().unwrap().name(),
            "utf-8"
        );
        assert_eq!(
            utf8_encoding_response()
                .character_encoding()
                .unwrap()
                .name(),
            "utf-8"
        );
        assert_eq!(
            latin1_encoding_response()
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
        assert_eq!(
            Response {
                version: Version::Http10,
                status: 200,
                headers: vec![Header {
                    name: "Content-Type".to_string(),
                    value: "test/plain; charset=xxx".to_string()
                }],
                body: b"Hello World!".to_vec(),
                duration: Default::default()
            }
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
                version: Version::Http10,
                status: 200,
                headers: vec![],
                body: vec![0x63, 0x61, 0x66, 0xe9],
                duration: Default::default()
            }
            .text()
            .err()
            .unwrap(),
            HttpError::InvalidDecoding {
                charset: "utf-8".to_string()
            }
        );

        assert_eq!(
            Response {
                version: Version::Http10,
                status: 200,
                headers: vec![Header {
                    name: "Content-Type".to_string(),
                    value: "text/plain; charset=ISO-8859-1".to_string()
                }],
                body: vec![0x63, 0x61, 0x66, 0xc3, 0xa9],
                duration: Default::default()
            }
            .text()
            .unwrap(),
            "cafÃ©".to_string()
        );
    }
}
