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
use encoding::DecoderTrap;

use crate::http::{mimetype, HeaderVec};
use crate::util::logger::Logger;

/// Logs a buffer of bytes representing an HTTP request or response `body`.
/// If the body is kind of text, we log all the text lines. If we can't detect that this is a text
/// body (using Content-Type header in `headers`), we print the first 64 bytes.
/// TODO: this function does not manage any kind of compression so we can only use for an HTTP
/// request. For an HTTP response, see `[crate::http::Response::log_body]`.
/// If `debug` is true, logs are printed using debug (with * prefix), otherwise logs are printed
/// in info.
pub fn log_body(body: &[u8], headers: &HeaderVec, debug: bool, logger: &mut Logger) {
    if let Some(content_type) = headers.content_type() {
        if !mimetype::is_kind_of_text(content_type) {
            log_bytes(body, 64, debug, logger);
            return;
        }
    }
    // Decode body as text:
    let encoding = match headers.character_encoding() {
        Ok(encoding) => encoding,
        Err(_) => {
            log_bytes(body, 64, debug, logger);
            return;
        }
    };

    match encoding.decode(body, DecoderTrap::Strict) {
        Ok(text) => log_text(&text, debug, logger),
        Err(_) => log_bytes(body, 64, debug, logger),
    }
}

/// Debug log text.
pub fn log_text(text: &str, debug: bool, logger: &mut Logger) {
    if text.is_empty() {
        if debug {
            logger.debug("");
        } else {
            logger.info("");
        }
    } else {
        let lines = text.split('\n');
        if debug {
            lines.for_each(|l| logger.debug(l));
        } else {
            lines.for_each(|l| logger.info(l));
        }
    }
}

/// Debug log `bytes` with a maximum size of `max` bytes.
pub fn log_bytes(bytes: &[u8], max: usize, debug: bool, logger: &mut Logger) {
    let bytes = if bytes.len() > max {
        &bytes[..max]
    } else {
        bytes
    };

    let log = if bytes.is_empty() {
        String::new()
    } else {
        format!("Bytes <{}...>", hex::encode(bytes))
    };
    if debug {
        logger.debug(&log);
    } else {
        logger.info(&log);
    }
}
