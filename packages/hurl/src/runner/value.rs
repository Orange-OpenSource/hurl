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
use crate::runner::Number;
use std::fmt;

/// System types used in Hurl.
///
/// Values are used by queries, captures, asserts and predicates.
#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Bytes(Vec<u8>),
    Date(chrono::DateTime<chrono::Utc>),
    List(Vec<Value>),
    Nodeset(usize),
    Null,
    Number(Number),
    Object(Vec<(String, Value)>),
    Regex(regex::Regex),
    String(String),
    Unit,
}

// You must implement it yourself because of the Regex Value
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(v1), Value::Bool(v2)) => v1 == v2,
            (Value::Bytes(v1), Value::Bytes(v2)) => v1 == v2,
            (Value::Date(v1), Value::Date(v2)) => v1 == v2,
            (Value::Number(v1), Value::Number(v2)) => v1 == v2,
            (Value::List(v1), Value::List(v2)) => v1 == v2,
            (Value::Nodeset(v1), Value::Nodeset(v2)) => v1 == v2,
            (Value::Null, Value::Null) => true,
            (Value::Object(v1), Value::Object(v2)) => v1 == v2,
            (Value::String(v1), Value::String(v2)) => v1 == v2,
            (Value::Unit, Value::Unit) => true,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Value::Bool(x) => x.to_string(),
            Value::Bytes(v) => format!("hex, {};", hex::encode(v)),
            Value::Date(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            Value::List(values) => {
                let values: Vec<String> = values.iter().map(|e| e.to_string()).collect();
                format!("[{}]", values.join(","))
            }
            Value::Nodeset(x) => format!("Nodeset{x:?}"),
            Value::Null => "null".to_string(),
            Value::Object(_) => "Object()".to_string(),
            Value::Regex(x) => {
                let s = str::replace(x.as_str(), "/", "\\/");
                format!("/{s}/")
            }
            Value::String(x) => x.clone(),
            Value::Unit => "Unit".to_string(),
        };
        write!(f, "{value}")
    }
}

impl Value {
    pub fn _type(&self) -> String {
        match self {
            Value::Bool(_) => "boolean".to_string(),
            Value::Bytes(_) => "bytes".to_string(),
            Value::Date(_) => "date".to_string(),
            Value::Number(v) => v._type(),
            Value::List(_) => "list".to_string(),
            Value::Nodeset(_) => "nodeset".to_string(),
            Value::Null => "null".to_string(),
            Value::Object(_) => "object".to_string(),
            Value::Regex(_) => "regex".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Unit => "unit".to_string(),
        }
    }

    pub fn is_scalar(&self) -> bool {
        !matches!(self, Value::Nodeset(_) | Value::List(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_scalar() {
        assert!(Value::Number(Number::Integer(1)).is_scalar());
        assert!(!Value::List(vec![]).is_scalar());
    }
}
