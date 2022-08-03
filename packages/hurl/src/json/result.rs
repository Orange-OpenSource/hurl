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

use crate::cli;
use crate::http::{
    Cookie, Header, Param, Request, RequestCookie, Response, ResponseCookie, Version,
};
use crate::runner::{AssertResult, CaptureResult, EntryResult, HurlResult};

impl HurlResult {
    pub fn to_json(&self, content: &str) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "filename".to_string(),
            serde_json::Value::String(self.filename.clone()),
        );
        let entries = self
            .entries
            .iter()
            .map(|e| e.clone().to_json(&self.filename, content))
            .collect();
        map.insert("entries".to_string(), serde_json::Value::Array(entries));
        map.insert(
            "success".to_string(),
            serde_json::Value::Bool(self.clone().success),
        );
        map.insert(
            "time".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.time_in_ms as u64)),
        );
        let cookies = self.cookies.iter().map(|e| e.clone().to_json()).collect();
        map.insert("cookies".to_string(), serde_json::Value::Array(cookies));
        serde_json::Value::Object(map)
    }
}

impl EntryResult {
    fn to_json(&self, filename: &str, content: &str) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        if let Some(request) = &self.request {
            map.insert("request".to_string(), request.to_json());
        }
        if let Some(response) = &self.response {
            map.insert("response".to_string(), response.to_json());
        }
        let captures = self.captures.iter().map(|c| c.clone().to_json()).collect();
        map.insert("captures".to_string(), captures);
        let asserts = self
            .asserts
            .iter()
            .map(|a| a.clone().to_json(filename, content))
            .collect();
        map.insert("asserts".to_string(), asserts);
        map.insert(
            "time".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.time_in_ms as u64)),
        );
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
        let headers = self.headers.iter().map(|h| h.clone().to_json()).collect();
        map.insert("headers".to_string(), headers);
        let cookies = self
            .clone()
            .cookies()
            .iter()
            .map(|e| e.clone().to_json())
            .collect();
        map.insert("cookies".to_string(), serde_json::Value::Array(cookies));
        let query_string = self
            .clone()
            .query_string_params()
            .iter()
            .map(|e| e.clone().to_json())
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
        let headers = self.headers.iter().map(|h| h.clone().to_json()).collect();
        map.insert("headers".to_string(), headers);
        let cookies = self
            .clone()
            .cookies()
            .iter()
            .map(|e| e.clone().to_json())
            .collect();
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

        if let Some(expires) = &self.clone().expires() {
            map.insert(
                "expires".to_string(),
                serde_json::Value::String(expires.to_string()),
            );
        }
        if let Some(max_age) = &self.clone().max_age() {
            map.insert(
                "max_age".to_string(),
                serde_json::Value::String(max_age.to_string()),
            );
        }
        if let Some(domain) = &self.clone().domain() {
            map.insert(
                "domain".to_string(),
                serde_json::Value::String(domain.to_string()),
            );
        }
        if let Some(path) = &self.clone().path() {
            map.insert(
                "path".to_string(),
                serde_json::Value::String(path.to_string()),
            );
        }
        if self.clone().has_secure() {
            map.insert("secure".to_string(), serde_json::Value::Bool(true));
        }
        if self.clone().has_httponly() {
            map.insert("httponly".to_string(), serde_json::Value::Bool(true));
        }
        if let Some(samesite) = &self.clone().samesite() {
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

        let success = self.clone().error().is_none();
        map.insert("success".to_string(), serde_json::Value::Bool(success));

        if let Some(err) = self.clone().error() {
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
