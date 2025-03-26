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
    /// URL of the request that triggers this response
    url: Url,
    /// Status code of the HTTP response
    status: u32,
}

impl HttpResponse {
    /// Returns a new HTTP response, given an `url` and a `status` code.
    pub fn new(url: Url, status: u32) -> Self {
        HttpResponse { url, status }
    }

    /// Returns the URL of the HTTP request associated to this response.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the HTTP status code of this response.
    pub fn status(&self) -> u32 {
        self.status
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "url={} status={}", self.url, self.status)
    }
}
