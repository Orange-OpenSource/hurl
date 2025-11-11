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
use core::str;
use std::fmt;
use std::str::FromStr;

use regex::Regex;

use super::error::HttpError;
use super::param::Param;

/// Represents errors for the URL module.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UrlError {
    pub url: String,
    pub reason: String,
}

impl UrlError {
    /// Creates a new error.
    fn new(url: &str, reason: &str) -> Self {
        UrlError {
            url: url.to_string(),
            reason: reason.to_string(),
        }
    }
}

/// A parsed URL.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Url {
    /// The input url from the user
    raw: String,

    /// A structured URL (implementation).
    inner: url::Url,
}

impl Default for Url {
    fn default() -> Self {
        Url::from_str("https://localhost").unwrap()
    }
}

impl Url {
    pub fn raw(&self) -> String {
        self.raw.clone()
    }

    /// Returns a list of query parameters (values are URL decoded).
    pub fn query_params(&self) -> Vec<Param> {
        self.inner
            .query_pairs()
            .map(|(k, v)| Param::new(&k, &v))
            .collect()
    }

    /// Returns the parsed representation of the host for this URL.
    /// See also the `host_str` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use hurl::http::Url;
    ///
    /// let url = Url::from_str("https://127.0.0.1/index.html").unwrap();
    /// assert_eq!(url.host(), "127.0.0.1".to_string());
    ///
    /// let url = Url::from_str("http://foo.com/index.html").unwrap();
    /// assert_eq!(url.host(), "foo.com".to_string());
    ///
    /// ```
    pub fn host(&self) -> String {
        self.inner
            .host()
            .expect("HTTP and HTTPS URL must have a domain")
            .to_string()
    }

    /// Returns the scheme of this URL, lower-cased, as an ASCII string without the ':' delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use hurl::http::Url;
    ///
    /// let url: Url = "http://toto.com/foo".parse().unwrap();
    /// assert_eq!(url.scheme(), "http");
    /// ```
    pub fn scheme(&self) -> &str {
        self.inner.scheme()
    }

    /// Returns the port of this URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use hurl::http::Url;
    ///
    /// let url = Url::from_str("https://bar.com:8081/foo").unwrap();
    /// assert_eq!(url.port(), Some(8081));
    ///
    /// let url = Url::from_str("https://baz.com").unwrap();
    /// assert_eq!(url.port(), Some(443));
    /// ```
    pub fn port(&self) -> Option<u16> {
        self.inner.port().or_else(|| match self.scheme() {
            "http" | "ws" => Some(80),
            "https" | "wss" => Some(443),
            "ftp" => Some(21),
            _ => None,
        })
    }

    pub fn domain(&self) -> Option<&str> {
        self.inner.domain()
    }

    pub fn path(&self) -> &str {
        self.inner.path()
    }

    /// Parse a string `input` as an URL, with this URL as the base URL.
    pub fn join(&self, input: &str) -> Result<Url, UrlError> {
        let new_inner = self.inner.join(input);
        let new_inner = match new_inner {
            Ok(u) => u,
            Err(_) => {
                let error = UrlError::new(
                    self.inner.as_str(),
                    &format!("Can not use relative path '{input}'"),
                );
                return Err(error);
            }
        };
        new_inner.as_str().parse()
    }
}

impl FromStr for Url {
    type Err = UrlError;

    /// Parses an absolute URL from a string.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // We try the happy path first: for the moment we're only supporting HTTP/HTTPS scheme.
        // Other scheme will go into `try_scheme` which uses regex and can be less performant in
        // stress tests usages than this simple `starts_with`.
        if value.starts_with("https://") || value.starts_with("http://") {
            let raw = value.to_string();
            let inner = url::Url::parse(&raw).map_err(|e| UrlError::new(value, &e.to_string()))?;
            Ok(Url { raw, inner })
        } else {
            match try_scheme(value) {
                Some(_) => Err(UrlError::new(
                    value,
                    "Only <http://> and <https://> schemes are supported",
                )),
                None => Err(UrlError::new(
                    value,
                    "Missing scheme <http://> or <https://>",
                )),
            }
        }
    }
}

/// Extracting scheme from `url`
///
/// The parse method from the url crate does not seem to parse url without scheme
/// For example, "localhost:8000" is parsed with its scheme set to "localhost"
///
fn try_scheme(url: &str) -> Option<String> {
    let re = Regex::new("^([a-z]+://).*").unwrap();
    if let Some(caps) = re.captures(url) {
        let scheme = &caps[1];
        Some(scheme.to_string())
    } else {
        None
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<UrlError> for HttpError {
    fn from(error: UrlError) -> Self {
        HttpError::InvalidUrl(error.url, error.reason)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{try_scheme, Url, UrlError};
    use crate::http::Param;

    #[test]
    fn parse_url_ok() {
        let urls = [
            "http://localhost:8000/hello",
            "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3",
            "http://localhost:8000/cookies",
            "http://localhost",
            "https://localhost:8000",
            "http://localhost:8000/path-as-is/../resource"
        ];
        for url in urls {
            assert!(Url::from_str(url).is_ok());
        }
    }

    #[test]
    fn query_params() {
        let url: Url = "http://localhost:8000/hello".parse().unwrap();
        assert_eq!(url.query_params(), vec![]);

        let url: Url = "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3".parse().unwrap();
        assert_eq!(
            url.query_params(),
            vec![
                Param::new("param1", "value1"),
                Param::new("param2", ""),
                Param::new("param3", "a=b"),
                Param::new("param4", "1,2,3"),
            ]
        );
    }

    #[test]
    fn test_join() {
        let base: Url = "http://example.net/foo/index.html".parse().unwrap();

        // Test join with absolute
        assert_eq!(
            base.join("http://bar.com/redirected").unwrap(),
            "http://bar.com/redirected".parse().unwrap()
        );

        // Test join with relative
        assert_eq!(
            base.join("/redirected").unwrap(),
            "http://example.net/redirected".parse().unwrap()
        );

        assert_eq!(
            base.join("../bar/index.html").unwrap(),
            "http://example.net/bar/index.html".parse().unwrap()
        );

        // Scheme relative URL
        assert_eq!(
            base.join("//example.org/baz/index.html").unwrap(),
            "http://example.org/baz/index.html".parse().unwrap()
        );
    }

    #[test]
    fn test_parsing_error() {
        assert_eq!(
            Url::from_str("localhost:8000").err().unwrap(),
            UrlError::new("localhost:8000", "Missing scheme <http://> or <https://>")
        );
        assert_eq!(
            Url::from_str("file://localhost:8000").err().unwrap(),
            UrlError::new(
                "file://localhost:8000",
                "Only <http://> and <https://> schemes are supported"
            )
        );
    }

    #[test]
    fn test_extract_scheme() {
        assert!(try_scheme("localhost:8000").is_none());
        assert!(try_scheme("http1://localhost:8000").is_none());
        assert!(try_scheme("://localhost:8000").is_none());
        assert_eq!(try_scheme("file://data").unwrap(), "file://".to_string());
        assert_eq!(try_scheme("http://data").unwrap(), "http://".to_string());
    }
}
