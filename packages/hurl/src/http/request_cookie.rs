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

    /// Parses a list of cookies from a `Cookie` header value, with format `name1=value1; name2=value2`.
    ///
    /// A cookie without `=` is parsed as a cookie with an empty value. Empty pieces (for instance,
    /// a trailing `;`) are ignored.
    pub fn from_header(s: &str) -> Vec<RequestCookie> {
        s.split(';')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(|t| match t.split_once('=') {
                Some((name, value)) => RequestCookie::new(name, value),
                None => RequestCookie::new(t, ""),
            })
            .collect()
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
    fn test_from_header() {
        assert_eq!(
            RequestCookie::from_header("cookie1=value1"),
            vec![RequestCookie::new("cookie1", "value1")]
        );
        assert_eq!(
            RequestCookie::from_header("cookie1=value1; cookie2=value2"),
            vec![
                RequestCookie::new("cookie1", "value1"),
                RequestCookie::new("cookie2", "value2"),
            ]
        );
        assert_eq!(
            RequestCookie::from_header("cookie1=value1;cookie2=value2"),
            vec![
                RequestCookie::new("cookie1", "value1"),
                RequestCookie::new("cookie2", "value2"),
            ]
        );
        assert_eq!(
            RequestCookie::from_header("token=a=b; cookie2"),
            vec![
                RequestCookie::new("token", "a=b"),
                RequestCookie::new("cookie2", ""),
            ]
        );
        assert_eq!(
            RequestCookie::from_header("cookie1=value1;"),
            vec![RequestCookie::new("cookie1", "value1")]
        );
        assert_eq!(
            RequestCookie::from_header(" ; cookie1=value1;; cookie2=value2 ; "),
            vec![
                RequestCookie::new("cookie1", "value1"),
                RequestCookie::new("cookie2", "value2"),
            ]
        );
        assert_eq!(RequestCookie::from_header(""), vec![]);
        assert_eq!(RequestCookie::from_header(" ; "), vec![]);
    }
}
