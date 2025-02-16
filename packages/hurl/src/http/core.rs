/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
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

use crate::util::redacted::Redact;

/// [Cookie](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie) returned by
/// the server with `Set-Cookie` header, and saved in the cookie storage of the internal HTTP
/// engine.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    /// Defines the host to which the cookie will be sent.
    pub domain: String,
    pub include_subdomain: String,
    /// Indicates the path that must exist in the requested URL for the browser to send the Cookie header.
    pub path: String,
    /// Indicates that the cookie is sent to the server only when a request is made with the https: scheme
    pub https: String,
    /// Indicates the maximum lifetime of the cookie as an HTTP-date timestamp.
    pub expires: String,
    pub name: String,
    pub value: String,
    /// Forbids JavaScript from accessing the cookie.
    pub http_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestCookie {
    pub name: String,
    pub value: String,
}

/// A key/value pair used for query params, form params and multipart-form params.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub value: String,
}

impl Param {
    /// Creates a new param pair.
    pub fn new(name: &str, value: &str) -> Param {
        Param {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl fmt::Display for Cookie {
    /// Formats this cookie using Netscape cookie format.
    ///
    /// <http://www.cookiecentral.com/faq/#3.5>
    ///
    /// > The layout of Netscape's cookies.txt file is such that each line contains one name-value
    /// > pair. An example cookies.txt file may have an entry that looks like this:
    /// >
    /// > `.netscape.com     TRUE   /  FALSE  946684799   NETSCAPE_ID  100103`
    /// >
    /// > Each line represents a single piece of stored information. A tab is inserted between each
    /// > of the fields.
    /// > From left-to-right, here is what each field represents:
    /// > - domain - The domain that created AND that can read the variable.
    /// > - flag - A TRUE/FALSE value indicating if all machines within a given domain can access
    /// >   the variable. This value is set automatically by the browser, depending on the value you
    /// >   set for domain.
    /// > - path - The path within the domain that the variable is valid for.
    /// > - secure - A TRUE/FALSE value indicating if a secure connection with the domain is
    /// >   needed to access the variable.
    /// > - expiration - The UNIX time that the variable will expire on. UNIX time is defined as the
    /// > - number of seconds since Jan 1, 1970 00:00:00 GMT.
    /// > - name - The name of the variable.
    /// > - value - The value of the variable.
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

impl Redact for Cookie {
    fn redact(&self, secrets: &[impl AsRef<str>]) -> String {
        format!(
            "{}{}\t{}\t{}\t{}\t{}\t{}\t{}",
            if self.http_only { "#HttpOnly_" } else { "" },
            self.domain,
            self.include_subdomain,
            self.path,
            self.https,
            self.expires,
            self.name,
            self.value.redact(secrets)
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
pub struct ParseCookieError;

impl FromStr for Cookie {
    type Err = ParseCookieError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let (http_only, domain) = if let Some(&v) = tokens.first() {
            if let Some(domain) = v.strip_prefix("#HttpOnly_") {
                (true, domain.to_string())
            } else {
                (false, v.to_string())
            }
        } else {
            return Err(ParseCookieError);
        };
        let include_subdomain = if let Some(&v) = tokens.get(1) {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let path = if let Some(&v) = tokens.get(2) {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let https = if let Some(&v) = tokens.get(3) {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let expires = if let Some(&v) = tokens.get(4) {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let name = if let Some(&v) = tokens.get(5) {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let value = if let Some(&v) = tokens.get(6) {
            v.to_string()
        } else {
            String::new()
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
                value: String::new(),
                http_only: false,
            }
        );

        assert_eq!(Cookie::from_str("xxx").err().unwrap(), ParseCookieError);
    }
}
