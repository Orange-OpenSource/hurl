/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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

/// A cookie sent in a request, as a name/value pair. It is not scoped to a domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestCookie {
    pub name: String,
    pub value: String,
}

impl RequestCookie {
    /// Creates a new request cookie.
    pub fn new(name: &str, value: &str) -> Self {
        RequestCookie {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    /// Creates a new instance from a string with format `name=value`.
    pub fn from_str(s: &str) -> Self {
        match s.find('=') {
            Some(i) => {
                let name = s.split_at(i).0;
                let value = s.split_at(i + 1).1;
                RequestCookie::new(name, value)
            }
            None => RequestCookie::new(s, ""),
        }
    }
}

impl fmt::Display for RequestCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::http::RequestCookie;

    #[test]
    fn test_parse_cookie() {
        assert_eq!(
            RequestCookie::from_str("cookie1=value1"),
            RequestCookie::new("cookie1", "value1"),
        );
    }
}
