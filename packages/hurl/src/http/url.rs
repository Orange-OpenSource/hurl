/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use crate::http::{HttpError, Param};
use std::fmt;

/// A parsed URL.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Url {
    /// A structured URL (implementation).
    inner: url::Url,
}

impl Url {
    /// Returns a list of query parameters (values are URL decoded).
    pub fn query_params(&self) -> Vec<Param> {
        self.inner
            .query_pairs()
            .map(|(k, v)| Param::new(&k, &v))
            .collect()
    }

    /// Parse a string `input` as an URL, with this URL as the base URL.
    pub fn join(&self, input: &str) -> Result<Url, HttpError> {
        let new_inner = self.inner.join(input);
        let new_inner = match new_inner {
            Ok(u) => u,
            Err(_) => {
                return Err(HttpError::InvalidUrl(
                    self.inner.to_string(),
                    format!("Can not use relative path '{input}'"),
                ))
            }
        };
        Url::try_from(new_inner.to_string().as_str())
    }
}

impl TryFrom<&str> for Url {
    type Error = HttpError;

    /// Parses an absolute URL from a string.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let inner = match url::Url::parse(value) {
            Ok(url) => url,
            Err(e) => return Err(HttpError::InvalidUrl(value.to_string(), e.to_string())),
        };
        let scheme = inner.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(HttpError::InvalidUrl(
                value.to_string(),
                "Missing protocol http or https".to_string(),
            ));
        }
        Ok(Url { inner })
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::Url;
    use crate::http::Param;

    #[test]
    fn parse_url_ok() {
        let urls = [
            "http://localhost:8000/hello",
            "http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3",
            "http://localhost:8000/cookies",
            "http://localhost",
            "https://localhost:8000",
        ];
        for url in urls {
            assert!(Url::try_from(url).is_ok());
        }
    }

    #[test]
    fn query_params() {
        let url = Url::try_from("http://localhost:8000/hello").unwrap();
        assert_eq!(url.query_params(), vec![]);

        let url = Url::try_from("http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3Db&param4=1%2C2%2C3").unwrap();
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
        let base = Url::try_from("http://example.net/foo/index.html").unwrap();

        // Test join with absolute
        assert_eq!(
            base.join("http://bar.com/redirected").unwrap(),
            Url::try_from("http://bar.com/redirected").unwrap()
        );

        // Test join with relative
        assert_eq!(
            base.join("/redirected").unwrap(),
            Url::try_from("http://example.net/redirected").unwrap()
        );

        assert_eq!(
            base.join("../bar/index.html").unwrap(),
            Url::try_from("http://example.net/bar/index.html").unwrap()
        );

        // Scheme relative URL
        assert_eq!(
            base.join("//example.org/baz/index.html").unwrap(),
            Url::try_from("http://example.org/baz/index.html").unwrap()
        )
    }
}
