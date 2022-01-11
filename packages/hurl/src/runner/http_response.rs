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

use crate::http::Response;

use super::cookie::ResponseCookie;
use super::core::RunnerError;

impl Response {
    pub fn cookies(&self) -> Vec<ResponseCookie> {
        self.headers
            .iter()
            .filter(|&h| h.name.to_lowercase().as_str() == "set-cookie")
            .filter_map(|h| ResponseCookie::parse(h.value.clone()))
            .collect()
    }

    ///
    /// Return encoding of the response
    ///
    fn encoding(&self) -> Result<EncodingRef, RunnerError> {
        match self.content_type() {
            Some(content_type) => match mime_charset(content_type) {
                Some(charset) => {
                    match encoding::label::encoding_from_whatwg_label(charset.as_str()) {
                        None => Err(RunnerError::InvalidCharset { charset }),
                        Some(enc) => Ok(enc),
                    }
                }
                None => Ok(encoding::all::UTF_8),
            },
            None => Ok(encoding::all::UTF_8),
        }
    }

    ///
    /// return response body as text
    ///
    pub fn text(&self) -> Result<String, RunnerError> {
        let encoding = self.encoding()?;
        let body = &self.uncompress_body()?;
        match encoding.decode(body, DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(_) => Err(RunnerError::InvalidDecoding {
                charset: encoding.name().to_string(),
            }),
        }
    }

    ///
    /// return true if response is an html response
    ///
    pub fn is_html(&self) -> bool {
        match self.content_type() {
            None => false,
            Some(s) => s.starts_with("text/html"),
        }
    }

    ///
    /// Return option cookie from response
    ///
    pub fn get_cookie(&self, name: String) -> Option<ResponseCookie> {
        for cookie in self.cookies() {
            if cookie.name == name {
                return Some(cookie);
            }
        }
        None
    }
}

///
/// Extract charset from mime-type String
///
fn mime_charset(mime_type: String) -> Option<String> {
    mime_type
        .find("charset=")
        .map(|index| mime_type[(index + 8)..].to_string())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::http::*;

    #[test]
    pub fn test_charset() {
        assert_eq!(
            mime_charset("text/plain; charset=utf-8".to_string()),
            Some("utf-8".to_string())
        );
        assert_eq!(
            mime_charset("text/plain; charset=ISO-8859-1".to_string()),
            Some("ISO-8859-1".to_string())
        );
        assert_eq!(mime_charset("text/plain;".to_string()), None);
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
    pub fn test_encoding() {
        assert_eq!(hello_response().encoding().unwrap().name(), "utf-8");
        assert_eq!(utf8_encoding_response().encoding().unwrap().name(), "utf-8");
        assert_eq!(
            latin1_encoding_response().encoding().unwrap().name(),
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
            .encoding()
            .err()
            .unwrap(),
            RunnerError::InvalidCharset {
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
            RunnerError::InvalidDecoding {
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
