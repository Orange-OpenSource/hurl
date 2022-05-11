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

///
/// Uncompress body response
/// using the Content-Encoding response header
///
use std::io::prelude::*;

use crate::http;

use crate::runner::RunnerError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Encoding {
    Brotli,
    Gzip,
    Deflate,
    Identity,
}

impl Encoding {
    /// Returns an encoding from an HTTP header value.
    ///
    /// # Arguments
    ///
    /// * `s` - A Content-Encoding header value
    pub fn parse(s: &str) -> Result<Encoding, RunnerError> {
        match s {
            "br" => Ok(Encoding::Brotli),
            "gzip" => Ok(Encoding::Gzip),
            "deflate" => Ok(Encoding::Deflate),
            "identity" => Ok(Encoding::Identity),
            v => Err(RunnerError::UnsupportedContentEncoding(v.to_string())),
        }
    }

    /// Decompress bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - A compressed bytes array
    pub fn decode(&self, data: &[u8]) -> Result<Vec<u8>, RunnerError> {
        match self {
            Encoding::Identity => Ok(data.to_vec()),
            Encoding::Gzip => uncompress_gzip(data),
            Encoding::Deflate => uncompress_zlib(data),
            Encoding::Brotli => uncompress_brotli(data),
        }
    }
}

impl http::Response {
    /// Returns list of encoding from HTTP response headers.
    fn content_encoding(&self) -> Result<Vec<Encoding>, RunnerError> {
        for header in &self.headers {
            if header.name.as_str().to_ascii_lowercase() == "content-encoding" {
                let mut encodings = vec![];
                for value in header.value.as_str().split(',') {
                    let encoding = Encoding::parse(value.trim())?;
                    encodings.push(encoding);
                }
                return Ok(encodings);
            }
        }
        Ok(vec![])
    }

    /// Decompress HTTP body response.
    pub fn uncompress_body(&self) -> Result<Vec<u8>, RunnerError> {
        let encodings = self.content_encoding()?;
        let mut data = self.body.clone();
        for encoding in encodings {
            data = encoding.decode(&data)?
        }
        Ok(data)
    }
}

/// Decompress Brotli compressed data.
///
/// # Arguments
///
/// * data - Compressed bytes.
fn uncompress_brotli(data: &[u8]) -> Result<Vec<u8>, RunnerError> {
    let buffer_size = 4096;
    let mut reader = brotli::Decompressor::new(data, buffer_size);
    let mut buf = Vec::new();
    match reader.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err(RunnerError::CouldNotUncompressResponse(
            "brotli".to_string(),
        )),
    }
}

/// Decompress GZip compressed data.
///
/// # Arguments
///
/// * data - Compressed bytes.
fn uncompress_gzip(data: &[u8]) -> Result<Vec<u8>, RunnerError> {
    let mut decoder = match libflate::gzip::Decoder::new(data) {
        Ok(v) => v,
        Err(_) => return Err(RunnerError::CouldNotUncompressResponse("gzip".to_string())),
    };
    let mut buf = Vec::new();
    match decoder.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err(RunnerError::CouldNotUncompressResponse("gzip".to_string())),
    }
}

/// Decompress Zlib compressed data.
///
/// # Arguments
///
/// * data - Compressed bytes.
fn uncompress_zlib(data: &[u8]) -> Result<Vec<u8>, RunnerError> {
    let mut decoder = match libflate::zlib::Decoder::new(data) {
        Ok(v) => v,
        Err(_) => return Err(RunnerError::CouldNotUncompressResponse("zlib".to_string())),
    };
    let mut buf = Vec::new();
    match decoder.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err(RunnerError::CouldNotUncompressResponse("zlib".to_string())),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse_encoding() {
        assert_eq!(Encoding::parse("br").unwrap(), Encoding::Brotli);
        assert_eq!(
            Encoding::parse("xx").err().unwrap(),
            RunnerError::UnsupportedContentEncoding("xx".to_string())
        );
    }

    #[test]
    fn test_content_encoding() {
        let response = http::Response {
            version: http::Version::Http10,
            status: 200,
            headers: vec![],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(response.content_encoding().unwrap(), vec![]);

        let response = http::Response {
            version: http::Version::Http10,
            status: 200,
            headers: vec![http::Header {
                name: "Content-Encoding".to_string(),
                value: "xx".to_string(),
            }],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(
            response.content_encoding().err().unwrap(),
            RunnerError::UnsupportedContentEncoding("xx".to_string())
        );

        let response = http::Response {
            version: http::Version::Http10,
            status: 200,
            headers: vec![http::Header {
                name: "Content-Encoding".to_string(),
                value: "br".to_string(),
            }],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(response.content_encoding().unwrap(), vec![Encoding::Brotli]);
    }

    #[test]
    fn test_multiple_content_encoding() {
        let response = http::Response {
            version: http::Version::Http10,
            status: 200,
            headers: vec![http::Header {
                name: "Content-Encoding".to_string(),
                value: "br, identity".to_string(),
            }],
            body: vec![],
            duration: Default::default(),
        };
        assert_eq!(
            response.content_encoding().unwrap(),
            vec![Encoding::Brotli, Encoding::Identity]
        );
    }

    #[test]
    fn test_uncompress_body() {
        let response = http::Response {
            version: http::Version::Http10,
            status: 200,
            headers: vec![http::Header {
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

        let response = http::Response {
            version: http::Version::Http10,
            status: 200,
            headers: vec![http::Header {
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

        let response = http::Response {
            version: http::Version::Http10,
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
            RunnerError::CouldNotUncompressResponse("brotli".to_string())
        );
        assert_eq!(
            uncompress_gzip(&data[..]).err().unwrap(),
            RunnerError::CouldNotUncompressResponse("gzip".to_string())
        );
    }
}
