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
use crate::http::HttpError;
use std::fmt;

/// A parsed URL.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Url {
    /// A structured URL (implementation).
    inner: url::Url,
}

impl Url {
    /// Parses an absolute URL from a string.
    pub fn new(url: &str) -> Result<Self, HttpError> {
        let inner = match url::Url::parse(url) {
            Ok(url) => url,
            Err(e) => return Err(HttpError::InvalidUrl(url.to_string(), e.to_string())),
        };
        let scheme = inner.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(HttpError::InvalidUrl(
                url.to_string(),
                "Missing protocol http or https".to_string(),
            ));
        }
        Ok(Url { inner })
    }

    /// TODO: Temporary method, will be deleted soon
    pub fn base_url(&self) -> Result<String, HttpError> {
        let scheme = self.inner.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(HttpError::InvalidUrl(
                self.inner.to_string(),
                "Missing protocol http or https".to_string(),
            ));
        }
        let host = match self.inner.host() {
            Some(host) => host,
            None => {
                return Err(HttpError::InvalidUrl(
                    self.inner.to_string(),
                    "Can not extract host".to_string(),
                ))
            }
        };
        let port = match self.inner.port() {
            Some(port) => format!(":{port}"),
            None => String::new(),
        };
        Ok(format!("{scheme}://{host}{port}"))
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
            assert!(Url::new(url).is_ok());
        }
    }
}
