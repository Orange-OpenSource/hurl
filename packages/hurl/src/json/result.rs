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
use chrono::{DateTime, SecondsFormat, Utc};
use hurl_core::ast::SourceInfo;
use serde_json::Number;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::http::{
    Call, Certificate, Cookie, Header, HttpVersion, Param, Request, RequestCookie, Response,
    ResponseCookie, Timings,
};
use crate::runner::{AssertResult, CaptureResult, EntryResult, HurlResult, Input};
use crate::util::logger;

impl HurlResult {
    /// Serializes an [`HurlResult`] to a JSON representation.
    ///
    /// Note: `content` is passed to this method to save asserts and errors messages (with lines
    /// and columns). This parameter will be removed soon and the original content will be
    /// accessible through the [`HurlResult`] instance.
    /// An optional directory `response_dir` can be used to save HTTP response.
    pub fn to_json(
        &self,
        content: &str,
        filename: &Input,
        response_dir: Option<&Path>,
    ) -> Result<serde_json::Value, io::Error> {
        let mut map = serde_json::Map::new();
        map.insert(
            "filename".to_string(),
            serde_json::Value::String(filename.to_string()),
        );
        let entries = self
            .entries
            .iter()
            .map(|e| e.to_json(filename, content, response_dir))
            .collect::<Result<_, _>>()?;
        map.insert("entries".to_string(), serde_json::Value::Array(entries));
        map.insert("success".to_string(), serde_json::Value::Bool(self.success));
        map.insert(
            "time".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.time_in_ms as u64)),
        );
        let cookies = self.cookies.iter().map(|e| e.to_json()).collect();
        map.insert("cookies".to_string(), serde_json::Value::Array(cookies));
        Ok(serde_json::Value::Object(map))
    }
}

impl EntryResult {
    /// Serializes an [`EntryResult`] to a JSON representation, optionally saving HTTP response in
    /// `response_dir` directory.
    fn to_json(
        &self,
        filename: &Input,
        content: &str,
        response_dir: Option<&Path>,
    ) -> Result<serde_json::Value, io::Error> {
        let mut map = serde_json::Map::new();

        map.insert(
            "index".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.entry_index)),
        );
        map.insert(
            "line".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.source_info.start.line)),
        );
        let calls = self
            .calls
            .iter()
            .map(|c| c.to_json(response_dir))
            .collect::<Result<_, _>>()?;
        map.insert("calls".to_string(), calls);
        let captures = self.captures.iter().map(|c| c.to_json()).collect();
        map.insert("captures".to_string(), captures);
        let asserts = self
            .asserts
            .iter()
            .map(|a| a.to_json(filename, content, self.source_info))
            .collect();
        map.insert("asserts".to_string(), asserts);
        map.insert(
            "time".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.time_in_ms as u64)),
        );
        Ok(serde_json::Value::Object(map))
    }
}

impl Call {
    /// Serializes a [`Call`] to a JSON representation, optionally saving HTTP response in
    /// `response_dir` directory.
    fn to_json(&self, response_dir: Option<&Path>) -> Result<serde_json::Value, io::Error> {
        let mut map = serde_json::Map::new();
        map.insert("request".to_string(), self.request.to_json());
        map.insert("response".to_string(), self.response.to_json(response_dir)?);
        map.insert("timings".to_string(), self.timings.to_json());
        Ok(serde_json::Value::Object(map))
    }
}

impl Request {
    /// Serializes a [`Request`] to a JSON representation.
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "method".to_string(),
            serde_json::Value::String(self.method.clone()),
        );
        map.insert(
            "url".to_string(),
            serde_json::Value::String(self.url.to_string()),
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
    /// Serializes a [`Response`] to a JSON representation, optionally saving HTTP response in
    /// `response_dir` directory.
    fn to_json(&self, response_dir: Option<&Path>) -> Result<serde_json::Value, io::Error> {
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
        if let Some(certificate) = &self.certificate {
            map.insert("certificate".to_string(), certificate.to_json());
        }
        if let Some(response_dir) = response_dir {
            let path = write_response(self, response_dir)?;
            map.insert(
                "body".to_string(),
                serde_json::Value::String(path.display().to_string()),
            );
        }
        Ok(serde_json::Value::Object(map))
    }
}

impl Header {
    /// Serializes a [`Header`] to a JSON representation.
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

impl HttpVersion {
    /// Serializes a [`HttpVersion`] to a JSON representation.
    fn to_json(self) -> serde_json::Value {
        let value = match self {
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http2 => "HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
        };
        serde_json::Value::String(value.to_string())
    }
}

impl Param {
    /// Serializes a [`Param`] to a JSON representation.
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
    /// Serializes a [`RequestCookie`] to a JSON representation.
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
    /// Serializes a [`ResponseCookie`] to a JSON representation.
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

impl Certificate {
    /// Serializes a [`Certificate`] to a JSON representation.
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "subject".to_string(),
            serde_json::Value::String(self.subject.clone()),
        );
        map.insert(
            "issue".to_string(),
            serde_json::Value::String(self.issuer.clone()),
        );
        map.insert("start_date".to_string(), json_date(self.start_date));
        map.insert("expire_date".to_string(), json_date(self.expire_date));
        map.insert(
            "serial_number".to_string(),
            serde_json::Value::String(self.serial_number.clone()),
        );
        serde_json::Value::Object(map)
    }
}

impl Timings {
    /// Serializes a [`Timings`] to a JSON representation.
    fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "begin_call".to_string(),
            serde_json::Value::String(self.begin_call.to_rfc3339_opts(SecondsFormat::Micros, true)),
        );
        map.insert(
            "end_call".to_string(),
            serde_json::Value::String(self.end_call.to_rfc3339_opts(SecondsFormat::Micros, true)),
        );
        let value = self.name_lookup.as_micros() as u64;
        map.insert(
            "name_lookup".to_string(),
            serde_json::Value::Number(Number::from(value)),
        );
        let value = self.connect.as_micros() as u64;
        map.insert(
            "connect".to_string(),
            serde_json::Value::Number(Number::from(value)),
        );
        let value = self.app_connect.as_micros() as u64;
        map.insert(
            "app_connect".to_string(),
            serde_json::Value::Number(Number::from(value)),
        );
        let value = self.pre_transfer.as_micros() as u64;
        map.insert(
            "pre_transfer".to_string(),
            serde_json::Value::Number(Number::from(value)),
        );
        let value = self.start_transfer.as_micros() as u64;
        map.insert(
            "start_transfer".to_string(),
            serde_json::Value::Number(Number::from(value)),
        );
        let value = self.total.as_micros() as u64;
        map.insert(
            "total".to_string(),
            serde_json::Value::Number(Number::from(value)),
        );
        serde_json::Value::Object(map)
    }
}

impl CaptureResult {
    /// Serializes a [`CaptureResult`] to a JSON representation.
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
    /// Serializes an [`AssertResult`] to a JSON representation.
    fn to_json(
        &self,
        filename: &Input,
        content: &str,
        entry_src_info: SourceInfo,
    ) -> serde_json::Value {
        let mut map = serde_json::Map::new();

        let success = self.error().is_none();
        map.insert("success".to_string(), serde_json::Value::Bool(success));

        if let Some(err) = self.error() {
            let message = logger::error_string(
                &filename.to_string(),
                content,
                &err,
                Some(entry_src_info),
                false,
            );
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
    /// Serializes a [`Cookie`] to a JSON representation.
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

/// Serializes a [`DateTime<Utc>`] to a JSON representation.
fn json_date(value: DateTime<Utc>) -> serde_json::Value {
    serde_json::Value::String(value.to_string())
}

/// Write the HTTP `response` body to directory `dir`.
fn write_response(response: &Response, dir: &Path) -> Result<PathBuf, io::Error> {
    let extension = if response.is_json() {
        Some("json")
    } else if response.is_xml() {
        Some("xml")
    } else if response.is_html() {
        Some("html")
    } else {
        None
    };
    let id = Uuid::new_v4();
    let relative_path = format!("{id}_response");
    let relative_path = Path::new(&relative_path);
    let relative_path = match extension {
        Some(ext) => relative_path.with_extension(ext),
        None => relative_path.to_path_buf(),
    };
    let path = dir.join(relative_path.clone());
    let mut file = File::create(path)?;
    file.write_all(&response.body)?;
    Ok(relative_path)
}
