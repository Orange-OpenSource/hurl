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

use crate::runner::Value;

impl Value {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Value::Bool(v) => serde_json::Value::Bool(*v),
            Value::Integer(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            Value::Float(f) => serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap()),
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::List(values) => {
                let values = values.iter().map(|v| v.to_json()).collect();
                serde_json::Value::Array(values)
            }
            Value::Object(key_values) => {
                let mut map = serde_json::Map::new();
                for (key, value) in key_values {
                    map.insert(key.to_string(), value.to_json());
                }
                serde_json::Value::Object(map)
            }
            Value::Nodeset(size) => {
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
                let encoded = base64::encode(v);
                serde_json::Value::String(encoded)
            }
            Value::Null => serde_json::Value::Null,
            Value::Regex(value) => serde_json::Value::String(value.to_string()),
            Value::Unit => todo!("how to serialize that in json?"),
        }
    }
}
