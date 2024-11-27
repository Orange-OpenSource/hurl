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
use std::fmt;

use crate::runner::Number;

/// System types used in Hurl.
///
/// Values are used by queries, captures, asserts and predicates.
#[derive(Clone, Debug)]
pub enum Value {
    /// A boolean value.
    Bool(bool),
    /// A buffer of bytes.
    Bytes(Vec<u8>),
    /// A date.
    Date(chrono::DateTime<chrono::Utc>),
    /// A list of [`Value`].
    List(Vec<Value>),
    /// A structure to represents node of object (returned from XPath queries).
    Nodeset(usize),
    /// The null type.
    Null,
    /// A number, can be a float, a 64-bit integer or any precision integer.
    Number(Number),
    /// A structure to represents objects (returned from JSONPath queries).
    Object(Vec<(String, Value)>),
    /// A regular expression.
    Regex(regex::Regex),
    /// A string.
    String(String),
    /// The unit type.
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
            Value::Nodeset(x) => format!("Nodeset(size={x})"),
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

const FORMAT_ISO: &str = "%Y-%m-%dT%H:%M:%S.%6fZ";

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

    pub fn render(&self) -> Option<String> {
        match self {
            Value::Bool(v) => Some(v.to_string()),
            Value::Date(d) => Some(d.format(FORMAT_ISO).to_string()),
            Value::Null => Some("null".to_string()),
            Value::Number(v) => Some(v.to_string()),
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::{DateTime, NaiveDate, Utc};

    use super::*;

    #[test]
    fn test_is_scalar() {
        assert!(Value::Number(Number::Integer(1)).is_scalar());
        assert!(!Value::List(vec![]).is_scalar());
    }

    #[test]
    fn test_format_iso() {
        let datetime_naive = NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_micro_opt(12, 0, 0, 123000)
            .unwrap();
        let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(datetime_naive, Utc);
        assert_eq!(
            datetime_utc.format(FORMAT_ISO).to_string(),
            "2000-01-01T12:00:00.123000Z"
        );

        let naive_datetime = NaiveDate::from_ymd_opt(2000, 2, 1)
            .unwrap()
            .and_hms_micro_opt(12, 0, 0, 123456)
            .unwrap();
        let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);
        assert_eq!(
            datetime_utc.format(FORMAT_ISO).to_string(),
            "2000-02-01T12:00:00.123456Z"
        );

        let naive_datetime = NaiveDate::from_ymd_opt(2000, 2, 1)
            .unwrap()
            .and_hms_nano_opt(12, 0, 0, 123456789)
            .unwrap();
        let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);
        assert_eq!(
            datetime_utc.format(FORMAT_ISO).to_string(),
            "2000-02-01T12:00:00.123456Z"
        );
    }
}
