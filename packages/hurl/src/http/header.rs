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

use core::fmt;

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

/// Returns all header values for given name
///
/// # Arguments
///
/// * `headers` - A list of HTTP headers
/// * `name` - A name to filter header (case insensitively)
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
