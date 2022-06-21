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
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub domain: String,
    pub include_subdomain: String,
    pub path: String,
    pub https: String,
    pub expires: String,
    pub name: String,
    pub value: String,
    pub http_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestCookie {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub value: String,
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}\t{}\t{}\t{}\t{}\t{}\t{}",
            if self.http_only { "#HttpOnly_" } else { "" },
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

impl fmt::Display for RequestCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseCookieError {}

impl FromStr for Cookie {
    type Err = ParseCookieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let (http_only, domain) = if let Some(&v) = tokens.get(0) {
            if let Some(domain) = v.strip_prefix("#HttpOnly_") {
                (true, domain.to_string())
            } else {
                (false, v.to_string())
            }
        } else {
            return Err(ParseCookieError {});
        };
        let include_subdomain = if let Some(&v) = tokens.get(1) {
            v.to_string()
        } else {
            return Err(ParseCookieError {});
        };
        let path = if let Some(&v) = tokens.get(2) {
            v.to_string()
        } else {
            return Err(ParseCookieError {});
        };
        let https = if let Some(&v) = tokens.get(3) {
            v.to_string()
        } else {
            return Err(ParseCookieError {});
        };
        let expires = if let Some(&v) = tokens.get(4) {
            v.to_string()
        } else {
            return Err(ParseCookieError {});
        };
        let name = if let Some(&v) = tokens.get(5) {
            v.to_string()
        } else {
            return Err(ParseCookieError {});
        };
        let value = if let Some(&v) = tokens.get(6) {
            v.to_string()
        } else {
            "".to_string()
        };
        Ok(Cookie {
            domain,
            include_subdomain,
            path,
            https,
            expires,
            name,
            value,
            http_only,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_cookie_from_str() {
        assert_eq!(
            Cookie::from_str("httpbin.org\tFALSE\t/\tFALSE\t0\tcookie1\tvalueA").unwrap(),
            Cookie {
                domain: "httpbin.org".to_string(),
                include_subdomain: "FALSE".to_string(),
                path: "/".to_string(),
                https: "FALSE".to_string(),
                expires: "0".to_string(),
                name: "cookie1".to_string(),
                value: "valueA".to_string(),
                http_only: false,
            }
        );
        assert_eq!(
            Cookie::from_str("localhost\tFALSE\t/\tFALSE\t1\tcookie2\t").unwrap(),
            Cookie {
                domain: "localhost".to_string(),
                include_subdomain: "FALSE".to_string(),
                path: "/".to_string(),
                https: "FALSE".to_string(),
                expires: "1".to_string(),
                name: "cookie2".to_string(),
                value: "".to_string(),
                http_only: false,
            }
        );

        assert_eq!(Cookie::from_str("xxx").err().unwrap(), ParseCookieError {});
    }
}
