/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

use serde::ser::{Serializer, SerializeStruct};
use serde::Serialize;

use crate::http::*;


use super::cookie::*;
use super::core::*;

impl Serialize for HurlResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("filename", &self.clone().filename)?;
        state.serialize_field("entries", &self.clone().entries)?;
        state.serialize_field("success", &self.clone().success)?;
        state.serialize_field("time", &self.time_in_ms)?;
        state.serialize_field("cookies", &self.cookies)?;
        state.end()
    }
}

impl Serialize for EntryResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("EntryResult", 3)?;
        if let Some(request) = &self.request {
            state.serialize_field("request", request)?;
        }
        if let Some(response) = &self.response {
            state.serialize_field("response", response)?;
        }
        state.serialize_field("captures", &self.captures)?;
        state.serialize_field("asserts", &self.asserts)?;
        state.serialize_field("time", &self.time_in_ms)?;
        state.end()
    }
}

impl Serialize for AssertResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("??", 3)?;
        if let AssertResult::Version {  actual, expected,.. } = self {
            //state.serialize_field("source_info", source_info)?;
            state.serialize_field("actual", actual)?;
            state.serialize_field("expected", expected)?;
        };
        state.end()
    }
}


impl Serialize for CaptureResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("CaptureResult", 3)?;
        state.serialize_field("name", self.name.as_str())?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}


impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("method", &self.clone().method.to_string())?;
        state.serialize_field("url", &self.clone().url)?;
        state.serialize_field("queryString", &self.clone().querystring)?;
        state.serialize_field("headers", &self.clone().headers)?;
        state.serialize_field("cookies", &self.clone().cookies)?;

        if !self.clone().form.is_empty() {
            state.serialize_field("form", &self.clone().form)?;
        }
        state.serialize_field("body", &base64::encode(&self.body))?;

        state.end()
    }
}


impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("httpVersion", &self.clone().version)?;
        state.serialize_field("status", &self.clone().status)?;
        state.serialize_field("cookies", &self.clone().cookies())?;
        state.serialize_field("headers", &self.clone().headers)?;

        // TODO  Serialize body
        state.end()
    }
}

impl Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Serialize for Param {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Serialize for RequestCookie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("??", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            Version::Http10 => serializer.serialize_str("HTTP/1.0"),
            Version::Http11 => serializer.serialize_str("HTTP/1.1"),
            Version::Http2 => serializer.serialize_str("HTTP/2"),
        }
    }
}

impl Serialize for ResponseCookie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResponseCookie", 3)?;
        state.serialize_field("name", &self.clone().name)?;
        state.serialize_field("value", &self.clone().value)?;
        if let Some(expires) = &self.clone().expires() {
            state.serialize_field("expires", expires)?;
        }
        if let Some(max_age) = &self.clone().max_age() {
            state.serialize_field("max_age", max_age)?;
        }
        if let Some(domain) = &self.clone().domain() {
            state.serialize_field("domain", domain)?;
        }
        if let Some(path) = &self.clone().path() {
            state.serialize_field("path", path)?;
        }
        if self.clone().has_secure() {
            state.serialize_field("secure", &true)?;
        }
        if self.clone().has_httponly() {
            state.serialize_field("httponly", &true)?;
        }
        if let Some(samesite) = &self.clone().samesite() {
            state.serialize_field("samesite", samesite)?;
        }
        state.end()
    }
}


impl Serialize for Cookie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Cookie", 3)?;
        state.serialize_field("domain", &self.clone().domain)?;
        state.serialize_field("include_subdomain", &self.clone().include_subdomain)?;
        state.serialize_field("path", &self.clone().path)?;
        state.serialize_field("https", &self.clone().https)?;
        state.serialize_field("expires", &self.clone().expires)?;
        state.serialize_field("name", &self.clone().name)?;
        state.serialize_field("value", &self.clone().value)?;
        state.end()
    }
}