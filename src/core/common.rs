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
use std::fmt;

use serde::Serialize;
use serde::ser::Serializer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeprecatedValue {
    Int(i32),
    String(String),
    List(usize),
    Bool(bool),
    Number(i32, u32),
    // 9 decimal digits
    ListInt(Vec<i32>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
//#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Integer(i64),

    // can use simply Float(f64)
    // the trait `std::cmp::Eq` is not implemented for `f64`
    // integer/ decimals with 18 digits
    Float(i64, u64),
    // integer part, decimal part (9 digits) TODO Clarify your custom type
    String(String),
    List(Vec<Value>),
    Object(Vec<(String, Value)>),
    Nodeset(usize),
    Bytes(Vec<u8>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Value::Integer(x) => x.to_string(),
            Value::Bool(x) => x.to_string(),
            Value::Float(int, dec) => format!("{}.{}", int, dec),
            Value::String(x) => x.clone(),
            Value::List(values) => {
                let values: Vec<String> = values.iter().map(|e| e.to_string()).collect();
                format!("[{}]", values.join(","))
            }
            Value::Object(_) => "Object()".to_string(),
            Value::Nodeset(x) => format!("Nodeset{:?}", x),
            Value::Bytes(x) => format!("Bytes({:x?})", x),
            Value::Null => "Null".to_string(),
        };
        write!(f, "{}", value)
    }
}

impl Value {
    pub fn _type(&self) -> String {
        match self {
            Value::Integer(_) => "integer".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Float(_, _) => "float".to_string(),
            Value::String(_) => "string".to_string(),
            Value::List(_) => "list".to_string(),
            Value::Object(_) => "object".to_string(),
            Value::Nodeset(_) => "nodeset".to_string(),
            Value::Bytes(_) => "bytes".to_string(),
            Value::Null => "unit".to_string(),
        }
    }


    pub fn from_f64(value: f64) -> Value {
        let integer = if value < 0.0 { value.ceil() as i64 } else { value.floor() as i64 };
        let decimal = (value.abs().fract() * 1_000_000_000_000_000_000.0).round() as u64;
        Value::Float(integer, decimal)
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            Value::Nodeset(_) | Value::List(_) => false,
            _ => true,
        }
    }

    pub fn from_json(value: &serde_json::Value) -> Value {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(bool) => Value::Bool(*bool),
            serde_json::Value::Number(n) =>
                if n.is_f64() {
                    Value::from_f64(n.as_f64().unwrap())
                } else {
                    Value::Integer(n.as_i64().unwrap())
                }
            ,
            serde_json::Value::String(s) => Value::String(s.to_string()),
            serde_json::Value::Array(elements) =>
                Value::List(elements
                    .iter()
                    .map(|e| Value::from_json(e))
                    .collect())
            ,
            serde_json::Value::Object(map) => {
                let mut elements = vec![];
                for (key, value) in map {
                    elements.push((key.to_string(), Value::from_json(value)));
                    //
                }
                Value::Object(elements)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceInfo {
    pub start: Pos,
    pub end: Pos,
}

impl SourceInfo {
    pub fn init(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_column: usize,
    ) -> SourceInfo {
        SourceInfo {
            start: Pos {
                line: start_line,
                column: start_col,
            },
            end: Pos {
                line: end_line,
                column: end_column,
            },
        }
    }
}

pub trait FormatError {
    fn source_info(&self) -> SourceInfo;
    fn description(&self) -> String;
    fn fixme(&self) -> String;
}


impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::Integer(v) => serializer.serialize_i64(*v),
            Value::Float(i, d) => {
                let value = *i as f64 + (*d as f64) / 1_000_000_000_000_000_000.0;
                serializer.serialize_f64(value)
            }
            Value::String(s) => serializer.serialize_str(s),
            Value::List(values) => serializer.collect_seq(values),
            Value::Object(values) => serializer.collect_map(values.iter().map(|(k, v)| (k, v))),
            Value::Nodeset(size) => {
                let size = *size as i64;
                serializer.collect_map(vec![
                    ("type", serde_json::Value::String("nodeset".to_string())),
                    ("size", serde_json::Value::from(size))
                ])
            }
            Value::Bytes(v) => {
                let encoded = base64::encode(v);
                serializer.serialize_str(&encoded)
            }
            Value::Null => serializer.serialize_none(),
        }
    }
}


impl Value {
    pub fn to_json_value(&self) -> (String, serde_json::Value) {
        match self.clone() {
            Value::Bool(v) => ("bool".to_string(), serde_json::Value::Bool(v)),
            Value::Integer(v) => ("integer".to_string(), serde_json::Value::from(v)),
            Value::Float(i, d) => {
                let value = i as f64 + (d as f64) / 1_000_000_000_000_000_000.0;
                ("float".to_string(), serde_json::Value::from(value))
            }
            Value::String(v) => ("string".to_string(), serde_json::Value::String(v)),
            Value::List(_) => (
                "list".to_string(),
                serde_json::Value::Array(vec![])
            ),
            Value::Object(_) => todo!(),
            Value::Nodeset(_) => todo!(),
            Value::Bytes(_) => todo!(),
            Value::Null => todo!(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self.clone() {
            Value::Bool(v) => serde_json::Value::Bool(v),
            Value::Integer(v) => serde_json::Value::from(v),
            Value::Float(i, d) => {
                let value = i as f64 + (d as f64) / 1_000_000_000_000_000_000.0;
                serde_json::Value::from(value)
            }
            Value::String(v) => serde_json::Value::String(v),
            Value::List(_) => serde_json::Value::Array(vec![]),
            Value::Object(_) => todo!(),
            Value::Nodeset(_) => todo!(),
            Value::Bytes(_) => todo!(),
            Value::Null => todo!(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f64() {
        assert_eq!(Value::from_f64(1.0), Value::Float(1, 0));
        assert_eq!(Value::from_f64(-1.0), Value::Float(-1, 0));
        assert_eq!(Value::from_f64(1.1), Value::Float(1, 100_000_000_000_000_096)); //TBC!!
        assert_eq!(Value::from_f64(-1.1), Value::Float(-1, 100_000_000_000_000_096));
        assert_eq!(Value::from_f64(1.5), Value::Float(1, 500_000_000_000_000_000));
    }

    #[test]
    fn test_from_json() {
        assert_eq!(Value::from_json(&serde_json::Value::String("hello".to_string())), Value::String("hello".to_string()));
        assert_eq!(Value::from_json(&serde_json::Value::Bool(true)), Value::Bool(true));
        assert_eq!(Value::from_json(&serde_json::Value::from(1)), Value::Integer(1));
        assert_eq!(Value::from_json(&serde_json::Value::from(1.5)), Value::Float(1, 500_000_000_000_000_000));
    }

    #[test]
    fn test_is_scalar() {
        assert_eq!(Value::Integer(1).is_scalar(), true);
        assert_eq!(Value::List(vec![]).is_scalar(), false);
    }

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&Value::Bool(true)).unwrap(), "true");
        assert_eq!(serde_json::to_string(&Value::String("hello".to_string())).unwrap(), "\"hello\"");
        assert_eq!(serde_json::to_string(&Value::Integer(1)).unwrap(), "1");
        assert_eq!(serde_json::to_string(&Value::Float(1, 500_000_000_000_000_000)).unwrap(), "1.5");
        assert_eq!(serde_json::to_string(&Value::Float(1, 100_000_000_000_000_000)).unwrap(), "1.1");
        assert_eq!(serde_json::to_string(&Value::Float(1, 100_000_000_000_000_096)).unwrap(), "1.1");
        assert_eq!(serde_json::to_string(&Value::List(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)])).unwrap(), "[1,2,3]");
        assert_eq!(serde_json::to_string(&Value::Object(vec![
            ("name".to_string(), Value::String("Bob".to_string()))
        ])).unwrap(), r#"{"name":"Bob"}"#);
        assert_eq!(serde_json::to_string(&Value::Nodeset(4)).unwrap(), r#"{"type":"nodeset","size":4}"#);
        assert_eq!(serde_json::to_string(&Value::Bytes(vec![65])).unwrap(), r#""QQ==""#);
        assert_eq!(serde_json::to_string(&Value::Null {}).unwrap(), "null");
    }
}
