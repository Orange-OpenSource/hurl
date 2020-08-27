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

use crate::http;

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
        if let AssertResult::Version { source_info, actual, expected } = self {
            state.serialize_field("source_info", source_info)?;
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
        //let (_type, value) = self.value.to_json_value();
        //state.serialize_field("type", &_type)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}


// http-specific


type ParseError = String;


pub fn parse_results(value: serde_json::Value) -> Result<Vec<HurlResult>, ParseError> {
    if let serde_json::Value::Array(values) = value {
        let mut results = vec![];
        for value in values {
            let result = parse_result(value)?;
            results.push(result);
        }
        Ok(results)
    } else {
        Err("expecting an array of session".to_string())
    }
}


fn parse_result(value: serde_json::Value) -> Result<HurlResult, ParseError> {
    if let serde_json::Value::Object(map) = value.clone() {
        let filename = map.get("filename").unwrap().as_str().unwrap().to_string();
        let mut entries = vec![];
        let entries = if let Some(serde_json::Value::Array(values)) = map.get("entries") {
            for value in values {
                let entry = parse_entry_result(value.clone())?;
                entries.push(entry);
            }
            entries
        } else {
            return Err("expecting an array of entries".to_string());
        };
        let time_in_ms = match value.get("time") {
            Some(serde_json::Value::Number(n)) => {
                match n.as_u64() {
                    Some(x) => x as u128,
                    None => return Err("expecting an integer for the time".to_string()),
                }
            }
            _ => return Err("expecting an integer for the time".to_string()),
        };
        let success = match value.get("success") {
            Some(serde_json::Value::Bool(v)) => *v,
            _ => return Err("expecting a bool for the status".to_string()),
        };
        let cookies = match value.get("cookies") {
            None => vec![],
            Some(serde_json::Value::Array(values)) => {
                let mut cookies: Vec<http::cookie::Cookie> = vec![];
                for value in values {
                    let cookie = http::import::parse_cookie(value.clone())?;
                    cookies.push(cookie);
                }
                cookies
            }
            Some(_) => return Err("expecting list for cookies".to_string()),
        };

        Ok(HurlResult { filename, entries, time_in_ms, success, cookies })
    } else {
        Err("expecting an object for the result".to_string())
    }
}

fn parse_entry_result(value: serde_json::Value) -> Result<EntryResult, String> {
    let request = match value.get("request") {
        None => None,
        Some(v) => {
            let r = http::import::parse_request(v.clone())?;
            Some(r)
        }
    };
    let response = match value.get("response") {
        None => None,
        Some(v) => {
            let r = http::import::parse_response(v.clone())?;
            Some(r)
        }
    };
    Ok(EntryResult {
        request,
        response,
        captures: vec![],
        asserts: vec![],
        errors: vec![],
        time_in_ms: 0,
    })
}


