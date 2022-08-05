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
use crate::cli::Logger;

/// Debug log text.
pub fn log_text(text: &str, logger: &Logger) {
    if text.is_empty() {
        logger.debug("");
    } else {
        text.split('\n').for_each(|l| logger.debug(l))
    }
}

/// Debug log bytes with a maximum size.
///
/// # Arguments
///
/// * `bytes`- the bytes to log
/// * `max` - The maximum number if bytes to log
pub fn log_bytes(bytes: &[u8], max: usize, logger: &Logger) {
    let bytes = if bytes.len() > max {
        &bytes[..max]
    } else {
        bytes
    };

    if bytes.is_empty() {
        logger.debug("");
    } else {
        logger.debug(format!("Bytes <{}...>", hex::encode(bytes)).as_str());
    }
}
