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
use super::Response;
use super::ResponseCookie;

impl Response {
    pub fn cookies(&self) -> Vec<ResponseCookie> {
        self.headers
            .iter()
            .filter(|&h| h.name.to_lowercase().as_str() == "set-cookie")
            .filter_map(|h| ResponseCookie::parse(h.value.clone()))
            .collect()
    }

    /// Returns optional cookies from response.
    pub fn get_cookie(&self, name: String) -> Option<ResponseCookie> {
        for cookie in self.cookies() {
            if cookie.name == name {
                return Some(cookie);
            }
        }
        None
    }
}
