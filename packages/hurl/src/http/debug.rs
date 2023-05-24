/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use crate::util::logger::Logger;

/// Debug log text.
pub fn log_text(text: &str, debug: bool, logger: &Logger) {
    if text.is_empty() {
        if debug {
            logger.debug("");
        } else {
            logger.info("");
        }
    } else {
        let lines = text.split('\n');
        if debug {
            lines.for_each(|l| logger.debug(l))
        } else {
            lines.for_each(|l| logger.info(l))
        }
    }
}

/// Debug log `bytes` with a maximum size of `max` bytes.
pub fn log_bytes(bytes: &[u8], max: usize, debug: bool, logger: &Logger) {
    let bytes = if bytes.len() > max {
        &bytes[..max]
    } else {
        bytes
    };

    let log = if bytes.is_empty() {
        "".to_string()
    } else {
        format!("Bytes <{}...>", hex::encode(bytes))
    };
    if debug {
        logger.debug(&log);
    } else {
        logger.info(&log)
    }
}
