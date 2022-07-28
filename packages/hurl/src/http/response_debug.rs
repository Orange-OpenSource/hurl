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

use crate::http::{debug, mimetype, Response};

impl Response {
    /// Log response headers.
    pub fn log_headers(&self) {
        debug::log_headers_in(&self.headers)
    }

    /// Log a response body as text if possible, or a slice of body bytes.
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response
    pub fn log_body(&self) {
        debug::log_text("Response body:");

        // We try to decode the HTTP body as text if the request has a text kind content type.
        // If it ok, we print each line of the body in debug format. Otherwise, we
        // print the body first 64 bytes.
        if let Some(content_type) = self.content_type() {
            if !mimetype::is_kind_of_text(&content_type) {
                debug::log_bytes(&self.body, 64);
                return;
            }
        }
        match self.text() {
            Ok(text) => debug::log_text(&text),
            Err(_) => debug::log_bytes(&self.body, 64),
        }
    }
}
