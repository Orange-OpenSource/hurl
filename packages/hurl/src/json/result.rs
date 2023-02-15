/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

use crate::cli;
use crate::http::{
    Cookie, Header, Param, Request, RequestCookie, Response, ResponseCookie, Version,
};
use crate::runner::{AssertResult, Call, CaptureResult, EntryResult, HurlResult};

impl HurlResult {
    /// Serializes an [`HurlResult`] to a JSON representation.
    ///
    /// Note: `content` is passed to this method to save asserts and
    /// errors messages (with lines and columns). This parameter will be removed
    /// soon and the original content will be accessible through the [`HurlResult`] instance.
    pub fn to_json(&self, content: &str) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "filename".to_string(),
            serde_json::Value::String(self.filename.clone()),
        );
        let entries = self
            .entries
            .iter()
            .map(|e| e.to_json(&self.filename, content))
            .collect();
        map.insert("entries".to_string(), serde_json::Value::Array(entries));
        map.insert("success".to_string(), serde_json::Value::Bool(self.success));
        map.insert(
            "time".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.time_in_ms as u64)),
        );
        let cookies = self.cookies.iter().map(|e| e.to_json()).collect();
        map.insert("cookies".to_string(), serde_json::Value::Array(cookies));
        serde_json::Value::Object(map)
    }
}

impl EntryResult {
    fn to_json(&self, filename: &str, content: &str) -> serde_json::Value {
        let mut map = serde_json::Map::new();

        map.insert(
            "index".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.entry_index)),
        );
        let calls = self.calls.iter().map(|c| c.to_json()).collect();
        map.insert("calls".to_string(), calls);
        let captures = self.captures.iter().map(|c| c.to_json()).collect();
        map.insert("captures".to_string(), captures);
        let asserts = self
            .asserts
            .iter()
            .map(|a| a.to_json(filename, content))
            .collect();
        map.insert("asserts".to_string(), asserts);
        map.insert(
            "time".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.time_in_ms as u64)),
        );
        serde_json::Value::Object(map)
    }
}

impl Call {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("request".to_string(), self.request.to_json());
        map.insert("response".to_string(), self.response.to_json());
        serde_json::Value::Object(map)
    }
}

impl Request {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "method".to_string(),
            serde_json::Value::String(self.method.clone()),
        );
        map.insert(
            "url".to_string(),
            serde_json::Value::String(self.url.clone()),
        );
        let headers = self.headers.iter().map(|h| h.to_json()).collect();
        map.insert("headers".to_string(), headers);
        let cookies = self.cookies().iter().map(|e| e.to_json()).collect();
        map.insert("cookies".to_string(), serde_json::Value::Array(cookies));
        let query_string = self
            .query_string_params()
            .iter()
            .map(|e| e.to_json())
            .collect();
        map.insert(
            "queryString".to_string(),
            serde_json::Value::Array(query_string),
        );
        serde_json::Value::Object(map)
    }
}

impl Response {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("httpVersion".to_string(), self.version.to_json());
        map.insert(
            "status".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.status)),
        );
        let headers = self.headers.iter().map(|h| h.to_json()).collect();
        map.insert("headers".to_string(), headers);
        let cookies = self.cookies().iter().map(|e| e.to_json()).collect();
        map.insert("cookies".to_string(), serde_json::Value::Array(cookies));
        serde_json::Value::Object(map)
    }
}

impl Header {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "name".to_string(),
            serde_json::Value::String(self.name.clone()),
        );
        map.insert(
            "value".to_string(),
            serde_json::Value::String(self.value.clone()),
        );
        serde_json::Value::Object(map)
    }
}

impl Version {
    fn to_json(&self) -> serde_json::Value {
        let value = match self {
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
            Version::Http2 => "HTTP/2",
        };
        serde_json::Value::String(value.to_string())
    }
}

impl Param {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "name".to_string(),
            serde_json::Value::String(self.name.clone()),
        );
        map.insert(
            "value".to_string(),
            serde_json::Value::String(self.value.clone()),
        );
        serde_json::Value::Object(map)
    }
}

impl RequestCookie {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "name".to_string(),
            serde_json::Value::String(self.name.clone()),
        );
        map.insert(
            "value".to_string(),
            serde_json::Value::String(self.value.clone()),
        );
        serde_json::Value::Object(map)
    }
}

impl ResponseCookie {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "name".to_string(),
            serde_json::Value::String(self.name.clone()),
        );
        map.insert(
            "value".to_string(),
            serde_json::Value::String(self.value.clone()),
        );

        if let Some(expires) = &self.expires() {
            map.insert(
                "expires".to_string(),
                serde_json::Value::String(expires.to_string()),
            );
        }
        if let Some(max_age) = &self.max_age() {
            map.insert(
                "max_age".to_string(),
                serde_json::Value::String(max_age.to_string()),
            );
        }
        if let Some(domain) = &self.domain() {
            map.insert(
                "domain".to_string(),
                serde_json::Value::String(domain.to_string()),
            );
        }
        if let Some(path) = &self.path() {
            map.insert(
                "path".to_string(),
                serde_json::Value::String(path.to_string()),
            );
        }
        if self.has_secure() {
            map.insert("secure".to_string(), serde_json::Value::Bool(true));
        }
        if self.has_httponly() {
            map.insert("httponly".to_string(), serde_json::Value::Bool(true));
        }
        if let Some(samesite) = &self.samesite() {
            map.insert(
                "samesite".to_string(),
                serde_json::Value::String(samesite.to_string()),
            );
        }
        serde_json::Value::Object(map)
    }
}

impl CaptureResult {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "name".to_string(),
            serde_json::Value::String(self.name.clone()),
        );
        map.insert("value".to_string(), self.value.to_json());
        serde_json::Value::Object(map)
    }
}

impl AssertResult {
    fn to_json(&self, filename: &str, content: &str) -> serde_json::Value {
        let mut map = serde_json::Map::new();

        let success = self.error().is_none();
        map.insert("success".to_string(), serde_json::Value::Bool(success));

        if let Some(err) = self.error() {
            let message = cli::error_string_no_color(filename, content, &err);
            map.insert("message".to_string(), serde_json::Value::String(message));
        }
        map.insert(
            "line".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.line())),
        );

        serde_json::Value::Object(map)
    }
}

impl Cookie {
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "domain".to_string(),
            serde_json::Value::String(self.domain.clone()),
        );
        map.insert(
            "include_subdomain".to_string(),
            serde_json::Value::String(self.include_subdomain.clone()),
        );
        map.insert(
            "path".to_string(),
            serde_json::Value::String(self.path.clone()),
        );
        map.insert(
            "https".to_string(),
            serde_json::Value::String(self.https.clone()),
        );
        map.insert(
            "expires".to_string(),
            serde_json::Value::String(self.expires.clone()),
        );
        map.insert(
            "name".to_string(),
            serde_json::Value::String(self.name.clone()),
        );
        map.insert(
            "value".to_string(),
            serde_json::Value::String(self.value.clone()),
        );
        serde_json::Value::Object(map)
    }
}
