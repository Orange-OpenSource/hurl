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
use crate::http::Url;
use core::fmt;
use std::fmt::Formatter;

/// Represents the storage of cookies for an HTTP client.
#[derive(Default)]
pub struct CookieStore {
    cookies: Vec<Cookie>,
}

impl CookieStore {
    /// Create a new instance.
    pub fn new() -> Self {
        CookieStore { cookies: vec![] }
    }

    /// Add a new cookie from a Netscape formatted string <http://www.cookiecentral.com/faq/#3.5>.
    pub fn add_cookie(&mut self, netscape_str: &str) -> Result<(), ParseCookieError> {
        let cookie = Cookie::from_netscape_str(netscape_str)?;
        self.cookies.push(cookie);
        Ok(())
    }

    /// Returns an iterator over [`Cookie`].
    pub fn cookies(&self) -> impl Iterator<Item = &Cookie> {
        self.cookies.iter()
    }

    /// Consumes the store and transform it into a vec of [`Cookie`]
    pub fn into_vec(self) -> Vec<Cookie> {
        self.cookies
    }
}

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

impl Cookie {
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
    pub fn to_netscape_str(&self) -> String {
        format!(
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

    /// Creates a [`Cookie`] from a Netscape cookie formatted string.
    pub fn from_netscape_str(s: &str) -> Result<Self, ParseCookieError> {
        let mut tokens = CookieAttributes::new(s);
        let (http_only, domain) = if let Some(v) = tokens.next() {
            if let Some(domain) = v.strip_prefix("#HttpOnly_") {
                (true, domain.to_string())
            } else {
                (false, v.to_string())
            }
        } else {
            return Err(ParseCookieError);
        };
        let include_subdomain = if let Some(v) = tokens.next() {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let path = if let Some(v) = tokens.next() {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let https = if let Some(v) = tokens.next() {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let expires = if let Some(v) = tokens.next() {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let name = if let Some(v) = tokens.next() {
            v.to_string()
        } else {
            return Err(ParseCookieError);
        };
        let value = if let Some(v) = tokens.next() {
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

    pub fn is_expired(&self) -> bool {
        // cookie expired when libcurl set value to 1?
        self.expires == "1"
    }

    pub fn include_subdomain(&self) -> bool {
        self.include_subdomain == "TRUE"
    }

    pub fn match_domain(&self, url: &Url) -> bool {
        // We remove the legacy optional dot in cookie domain.
        let cookie_domain = self.domain.strip_prefix(".").unwrap_or(&self.domain);
        if let Some(url_domain) = url.domain() {
            if !self.include_subdomain() {
                if url_domain != cookie_domain {
                    return false;
                }
            } else if !url_domain.ends_with(&cookie_domain) {
                return false;
            }
        }
        url.path().starts_with(&self.path)
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let repr = self.to_netscape_str();
        write!(f, "{repr}")
    }
}

/// Represents an iterator over cookie attributes parsed from a Netscape formatted string
/// (see <http://www.cookiecentral.com/faq/#3.5>).
/// The Netscape format uses tab as separator, we want also to import cookie with a space
/// separator (for inline use in Hurl files with `@cookie_storage` command for instance).
/// The format has only 7 values, and the last token can include whitespaces.
struct CookieAttributes<'line> {
    line: &'line str,
    /// Current index of the char
    pos: BytePos,
    /// Number of values parsed
    parts: usize,
}

#[derive(Copy, Clone)]
struct BytePos(usize);

impl<'line> CookieAttributes<'line> {
    fn new(line: &'line str) -> Self {
        CookieAttributes {
            line,
            pos: BytePos(0),
            parts: 0,
        }
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        let bytes = self.line.as_bytes();
        while self.pos.0 < bytes.len() && is_whitespace(bytes[self.pos.0]) {
            self.pos.0 += 1;
        }
    }
}

#[inline]
fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n')
}

impl<'line> Iterator for CookieAttributes<'line> {
    type Item = &'line str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.parts == 7 {
            return None;
        }

        // Skip leading whitespace
        self.skip_whitespace();
        if self.pos.0 >= self.line.len() {
            return None;
        }

        // 7th logical field = remainder (value may contain spaces)
        if self.parts == 6 {
            self.parts += 1;
            return Some(&self.line[self.pos.0..]);
        }

        let start = self.pos;
        let bytes = self.line.as_bytes();
        while self.pos.0 < bytes.len() && !is_whitespace(bytes[self.pos.0]) {
            self.pos.0 += 1;
        }
        let end = self.pos;
        self.parts += 1;
        Some(&self.line[start.0..end.0])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseCookieError;

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    pub fn parse_cookie_from_str() {
        assert_eq!(
            Cookie::from_netscape_str("httpbin.org\tFALSE\t/\tFALSE\t0\tcookie1\tvalueA").unwrap(),
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
            Cookie::from_netscape_str("localhost\tFALSE\t/\tFALSE\t1\tcookie2\t").unwrap(),
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

        assert_eq!(
            Cookie::from_netscape_str("localhost FALSE / FALSE 1 cookie3 value3").unwrap(),
            Cookie {
                domain: "localhost".to_string(),
                include_subdomain: "FALSE".to_string(),
                path: "/".to_string(),
                https: "FALSE".to_string(),
                expires: "1".to_string(),
                name: "cookie3".to_string(),
                value: "value3".to_string(),
                http_only: false,
            }
        );

        assert_eq!(
            Cookie::from_netscape_str("#HttpOnly_localhost FALSE / FALSE 1 cookie3 a b c").unwrap(),
            Cookie {
                domain: "localhost".to_string(),
                include_subdomain: "FALSE".to_string(),
                path: "/".to_string(),
                https: "FALSE".to_string(),
                expires: "1".to_string(),
                name: "cookie3".to_string(),
                value: "a b c".to_string(),
                http_only: true,
            }
        );

        assert_eq!(
            Cookie::from_netscape_str("xxx").err().unwrap(),
            ParseCookieError
        );
    }

    #[test]
    fn test_match_cookie() {
        let cookie = Cookie {
            domain: "example.com".to_string(),
            include_subdomain: "FALSE".to_string(),
            path: "/".to_string(),
            https: String::new(),
            expires: String::new(),
            name: String::new(),
            value: String::new(),
            http_only: false,
        };
        assert!(cookie.match_domain(&Url::from_str("http://example.com/toto").unwrap()));
        assert!(!cookie.match_domain(&Url::from_str("http://sub.example.com/tata").unwrap()));
        assert!(!cookie.match_domain(&Url::from_str("http://toto/tata").unwrap()));

        let cookie = Cookie {
            domain: "example.com".to_string(),
            include_subdomain: "TRUE".to_string(),
            path: "/toto".to_string(),
            https: String::new(),
            expires: String::new(),
            name: String::new(),
            value: String::new(),
            http_only: false,
        };
        assert!(cookie.match_domain(&Url::from_str("http://example.com/toto").unwrap()));
        assert!(cookie.match_domain(&Url::from_str("http://sub.example.com/toto").unwrap()));
        assert!(!cookie.match_domain(&Url::from_str("http://example.com/tata").unwrap()));

        // Legacy cookie domain with dot prefix
        let cookie = Cookie {
            domain: ".example.com".to_string(),
            include_subdomain: "TRUE".to_string(),
            path: "/foo".to_string(),
            https: String::new(),
            expires: String::new(),
            name: String::new(),
            value: String::new(),
            http_only: false,
        };
        assert!(cookie.match_domain(&Url::from_str("http://example.com/foo").unwrap()));
        assert!(cookie.match_domain(&Url::from_str("http://sub.example.com/foo").unwrap()));
        assert!(!cookie.match_domain(&Url::from_str("http://example.com/tata").unwrap()));
        assert!(!cookie.match_domain(&Url::from_str("http://sub.example.com/tata").unwrap()));
    }

    #[test]
    fn test_add_cookie() {
        let mut cookie_store = CookieStore::new();
        cookie_store
            .add_cookie("localhost  TRUE    /   FALSE   0   cookie1 valueA")
            .unwrap();
        cookie_store
            .add_cookie(
                "#HttpOnly_example.com\t\t   FALSE\t\t  \t/\tFALSE\t1\tcookie2\tfoo bar baz",
            )
            .unwrap();
        let cookies = cookie_store.into_vec();

        assert_eq!(cookies.len(), 2);
        assert_eq!(
            cookies[0],
            Cookie {
                domain: "localhost".to_string(),
                include_subdomain: "TRUE".to_string(),
                path: "/".to_string(),
                https: "FALSE".to_string(),
                expires: "0".to_string(),
                name: "cookie1".to_string(),
                value: "valueA".to_string(),
                http_only: false,
            }
        );
        assert_eq!(
            cookies[1],
            Cookie {
                domain: "example.com".to_string(),
                include_subdomain: "FALSE".to_string(),
                path: "/".to_string(),
                https: "FALSE".to_string(),
                expires: "1".to_string(),
                name: "cookie2".to_string(),
                value: "foo bar baz".to_string(),
                http_only: true,
            }
        );
    }
}
