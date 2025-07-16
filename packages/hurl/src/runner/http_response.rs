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
use std::fmt::Display;

/// Represents an HTTP request for `Value::HttpResponse`
#[derive(Clone, Debug)]
pub struct HttpResponse {
    /// Absolute URL of redirection
    location: Option<Url>,
    /// Status code of the HTTP response
    status: u32,
}

impl HttpResponse {
    /// Returns a new HTTP response, given an optional `location` of redirection and a `status` code.
    pub fn new(location: Option<Url>, status: u32) -> Self {
        HttpResponse { location, status }
    }

    /// Returns the (optional) redirection URL suggested by this HTTP response.
    pub fn location(&self) -> Option<&Url> {
        self.location.as_ref()
    }

    /// Returns the HTTP status code of this response.
    pub fn status(&self) -> u32 {
        self.status
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.location {
            Some(url) => write!(f, "Response(location={}, status={})", url, self.status),
            None => write!(f, "Response(location=None, status={})", self.status),
        }
    }
}
