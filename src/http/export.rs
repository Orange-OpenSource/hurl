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

use super::cookie::*;
use super::request::*;
use super::response::*;

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("method", &self.clone().method.to_text())?;
        state.serialize_field("url", &self.clone().url())?;
        state.serialize_field("queryString", &self.clone().querystring)?;
        state.serialize_field("headers", &self.clone().headers())?;
        state.serialize_field("cookies", &self.clone().cookies)?;

        if let Some(params) = self.clone().form_params() {
            state.serialize_field("format_params", &params)?;
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

        // WIP - Serialize response body only for json for the timebeing
        let content_type = self.get_header("content_type", true);
        if let Some(value) = content_type.first() {
            if value.as_str() == "application/json; charset=UTF-8" {
                let s = String::from_utf8(self.body.clone()).expect("Found invalid UTF-8");
                let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(s.as_str());
                if let Ok(v) = result {
                    state.serialize_field("json", &v)?;
                }
            }
        }

        state.end()
    }
}

impl Serialize for ResponseCookie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("??", 3)?;
        state.serialize_field("name", &self.clone().name)?;
        state.serialize_field("value", &self.clone().value)?;
        if let Some(value) = self.clone().domain {
            state.serialize_field("domain", &value)?;
        }
        state.end()
    }
}

impl Serialize for Cookie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("InternalCookie", 3)?;
        state.serialize_field("name", &self.clone().name)?;
        state.serialize_field("value", &self.clone().value)?;
        state.serialize_field("domain", &self.clone().domain)?;
        state.serialize_field("path", &self.clone().path)?;
        state.serialize_field("include_subdomain", &self.clone().subdomains)?;
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

