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

use crate::http::{mimetype, HttpError, Request};
use encoding::{DecoderTrap, EncodingRef};

impl Request {
    /// Returns character encoding of the HTTP request.
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
        match encoding.decode(&self.body, DecoderTrap::Strict) {
            Ok(s) => Ok(s),
            Err(_) => Err(HttpError::InvalidDecoding {
                charset: encoding.name().to_string(),
            }),
        }
    }
}
