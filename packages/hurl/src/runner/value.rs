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
use std::fmt;

///
/// Type system used in hurl
/// Values are used by queries, captures, asserts and predicates
///
#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Bytes(Vec<u8>),
    Float(f64),
    Integer(i64),
    List(Vec<Value>),
    Nodeset(usize),
    Null,
    Object(Vec<(String, Value)>),
    String(String),
    Unit,
    Regex(regex::Regex),
}

// You must implement it yourself because of the Float
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(v1), Value::Bool(v2)) => v1 == v2,
            (Value::Bytes(v1), Value::Bytes(v2)) => v1 == v2,
            (Value::Float(v1), Value::Float(v2)) => (v1 - v2).abs() < f64::EPSILON,
            (Value::Integer(v1), Value::Integer(v2)) => v1 == v2,
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
            Value::Integer(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            Value::Float(f) => format_float(*f),
            Value::String(x) => x.clone(),
            Value::List(values) => {
                let values: Vec<String> = values.iter().map(|e| e.to_string()).collect();
                format!("[{}]", values.join(","))
            }
            Value::Object(_) => "Object()".to_string(),
            Value::Nodeset(x) => format!("Nodeset{:?}", x),
            Value::Bytes(v) => format!("hex, {};", hex::encode(v)),
            Value::Null => "null".to_string(),
            Value::Unit => "Unit".to_string(),
            Value::Regex(x) => {
                let s = str::replace(x.as_str(), "/", "\\/");
                format!("/{}/", s)
            }
        };
        write!(f, "{}", value)
    }
}

fn format_float(value: f64) -> String {
    if value.fract() < f64::EPSILON {
        format!("{}.0", value)
    } else {
        value.to_string()
    }
}

impl Value {
    pub fn _type(&self) -> String {
        match self {
            Value::Integer(_) => "integer".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::String(_) => "string".to_string(),
            Value::List(_) => "list".to_string(),
            Value::Object(_) => "object".to_string(),
            Value::Nodeset(_) => "nodeset".to_string(),
            Value::Bytes(_) => "bytes".to_string(),
            Value::Null => "null".to_string(),
            Value::Unit => "unit".to_string(),
            Value::Regex(_) => "regex".to_string(),
        }
    }

    pub fn from_f64(value: f64) -> Value {
        Value::Float(value)
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
        assert!(Value::Integer(1).is_scalar());
        assert!(!Value::List(vec![]).is_scalar());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Value::Float(1.0).to_string(), "1.0".to_string());
        assert_eq!(Value::Float(1.1).to_string(), "1.1".to_string());
    }
}
