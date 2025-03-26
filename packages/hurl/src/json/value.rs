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
use std::str::FromStr;

use base64::engine::general_purpose;
use base64::Engine;

use crate::runner::{Number, Value};
use crate::util::redacted::Redact;

/// Serializes a [`Value`] to JSON, used in captures serialization.
///
/// Natural JSON types are used to represent captures: if a [`Value::List`] is captured,
/// the serialized data will be a JSON list.
/// `secrets` are redacted from string values.
impl Value {
    pub fn to_json(&self, secrets: &[&str]) -> serde_json::Value {
        match self {
            Value::Bool(v) => serde_json::Value::Bool(*v),
            Value::Date(v) => serde_json::Value::String(v.to_string()),
            Value::Number(v) => v.to_json(),
            Value::String(s) => serde_json::Value::String(s.redact(secrets)),
            Value::List(values) => {
                let values = values.iter().map(|v| v.to_json(secrets)).collect();
                serde_json::Value::Array(values)
            }
            Value::Object(key_values) => {
                let mut map = serde_json::Map::new();
                for (key, value) in key_values {
                    map.insert(key.to_string(), value.to_json(secrets));
                }
                serde_json::Value::Object(map)
            }
            Value::Nodeset(size) => {
                // For nodeset, we don't have a "native" JSON representation to use as a serialized
                // format. As a fallback, we serialize with a `type` field:
                //
                // ```json
                // {
                //   "type": "nodeset",
                //   "size": 4,
                // }
                // ```
                let mut map = serde_json::Map::new();
                let size = *size as i64;
                map.insert(
                    "type".to_string(),
                    serde_json::Value::String("nodeset".to_string()),
                );
                map.insert("size".to_string(), serde_json::Value::from(size));
                serde_json::Value::Object(map)
            }
            Value::Bytes(v) => {
                let encoded = general_purpose::STANDARD.encode(v);
                serde_json::Value::String(encoded)
            }
            Value::Null => serde_json::Value::Null,
            Value::Regex(value) => serde_json::Value::String(value.to_string()),
            Value::Unit => {
                // Like nodeset, we don't have a "native" JSON representation for the unit type,
                // we use a general fallback with `type` field
                let mut map = serde_json::Map::new();
                map.insert(
                    "type".to_string(),
                    serde_json::Value::String("unit".to_string()),
                );
                serde_json::Value::Object(map)
            }
            Value::HttpResponse(v) => {
                let mut map = serde_json::Map::new();
                map.insert("url".to_string(), serde_json::Value::String(v.url().raw()));
                map.insert(
                    "status".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(v.status())),
                );
                serde_json::Value::Object(map)
            }
        }
    }
}

impl Number {
    /// Serializes a number to JSON.
    ///
    /// Numbers that are representable in JSON use the number JSON type, while big number
    /// will be serialized as string.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Number::Integer(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            Number::Float(f) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap())
            }
            Number::BigInteger(s) => {
                let number = serde_json::Number::from_str(s).unwrap();
                serde_json::Value::Number(number)
            }
        }
    }
}
