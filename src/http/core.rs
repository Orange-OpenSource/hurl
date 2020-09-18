/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub domain: String,
    pub include_subdomain: String,
    pub path: String,
    pub https: String,
    pub expires: String,
    pub name: String,
    pub value: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.domain,
            self.include_subdomain,
            self.path,
            self.https,
            self.expires,
            self.name,
            self.value
        )
    }
}

///
/// return a list of headers values for the given header name
///
pub fn get_header_values(headers: Vec<Header>, expected_name: String) -> Vec<String> {
    headers
        .iter()
        .filter_map(|Header { name, value }| {
            if name.clone() == expected_name {
                Some(value.to_string())
            } else {
                None
            }
        })
        .collect()
}
