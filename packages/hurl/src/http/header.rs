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
use core::fmt;

/// Represents an HTTP header
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl Header {
    pub const ACCEPT_ENCODING: &'static str = "Accept-Encoding";
    pub const AUTHORIZATION: &'static str = "Authorization";
    pub const CONTENT_TYPE: &'static str = "Content-Type";
    pub const EXPECT: &'static str = "Expect";
    pub const USER_AGENT: &'static str = "User-Agent";

    pub fn new(name: &str, value: &str) -> Self {
        Header {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

/// Returns all `headers` values for given `name`.
pub fn get_values(headers: &[Header], name: &str) -> Vec<String> {
    headers
        .iter()
        .filter_map(|Header { name: key, value }| {
            if key.to_lowercase() == name.to_lowercase() {
                Some(value.to_string())
            } else {
                None
            }
        })
        .collect()
}
