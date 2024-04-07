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

use colored::Colorize;

use crate::http::{debug, mimetype, Response};
use crate::util::logger::Logger;
use serde_json::Value;

impl Response {
    /// Log a response body as text if possible, or a slice of body bytes.
    pub fn log_body(&self, debug: bool, logger: &mut Logger) {
        // We try to decode the HTTP body as text if the response has a text kind content type.
        // If it ok, we print each line of the body in debug format. Otherwise, we
        // print the body first 64 bytes.
        if let Some(content_type) = self.headers.content_type() {
            if !mimetype::is_kind_of_text(content_type) {
                debug::log_bytes(&self.body, 64, debug, logger);
                return;
            }
        }
        match self.text() {
            Ok(text) => match serde_json::from_str::<Value>(&text) {
                Ok(json_value) => {
                    if logger.pretty_print {
                        debug::log_text(
                            &serde_json::to_string_pretty(&json_value).unwrap(),
                            debug,
                            logger,
                        );
                    } else {
                        debug::log_text(&text, debug, logger);
                    }
                }
                Err(_) => {
                    debug::log_text(&text, debug, logger);
                }
            },
            Err(_) => debug::log_bytes(&self.body, 64, debug, logger),
        }
    }

    pub fn log_info_all(&self, logger: &mut Logger) {
        let status_line = self.get_status_line_headers(logger.color);
        logger.info(&status_line);
        self.log_body(false, logger);
        logger.info("");
    }

    /// Returns status, version and HTTP headers from this HTTP response.
    pub fn get_status_line_headers(&self, color: bool) -> String {
        let mut str = String::new();
        let status_line = format!("{} {}\n", self.version, self.status);
        let status_line = if color {
            format!("{}", status_line.green().bold())
        } else {
            status_line
        };
        str.push_str(&status_line);
        for header in self.headers.iter() {
            let header_line = if color {
                format!("{}: {}\n", header.name.cyan().bold(), header.value)
            } else {
                format!("{}: {}\n", header.name, header.value)
            };
            str.push_str(&header_line);
        }
        str
    }
}
