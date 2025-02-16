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
use std::cmp::Ordering;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueKind {
    Bool,
    Bytes,
    Date,
    Float,
    Integer,
    List,
    Nodeset,
    Null,
    Object,
    Regex,
    Secret,
    String,
    Unit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvalError {
    Type,
    InvalidRegex,
}

/// Equality of values
/// as used in the predicate ==
///
/// Any combination of value type can be used in the equality. There isn't any type/mismatch errors.
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(v1), Value::Bool(v2)) => v1 == v2,
            (Value::Bytes(v1), Value::Bytes(v2)) => v1 == v2,
            (Value::Date(v1), Value::Date(v2)) => v1 == v2,
            (Value::Number(v1), Value::Number(v2)) => v1.cmp_value(v2) == Ordering::Equal,
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
            Value::Bytes(v) => hex::encode(v).to_string(),
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
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Bool(_) => ValueKind::Bool,
            Value::Bytes(_) => ValueKind::Bytes,
            Value::Date(_) => ValueKind::Date,
            Value::Number(Number::Float(_)) => ValueKind::Float,
            Value::Number(Number::Integer(_)) | Value::Number(Number::BigInteger(_)) => {
                ValueKind::Integer
            }
            Value::List(_) => ValueKind::List,
            Value::Nodeset(_) => ValueKind::Nodeset,
            Value::Null => ValueKind::Null,
            Value::Object(_) => ValueKind::Object,
            Value::Regex(_) => ValueKind::Regex,
            Value::String(_) => ValueKind::String,
            Value::Unit => ValueKind::Unit,
        }
    }

    /// Returns a printable representation of the Value including its type.
    pub fn repr(&self) -> String {
        match self.kind() {
            ValueKind::Unit | ValueKind::Secret => self.kind().to_string(),
            _ => format!("{} <{}>", self.kind(), self),
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

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueKind::Bool => write!(f, "boolean"),
            ValueKind::Bytes => write!(f, "bytes"),
            ValueKind::Date => write!(f, "date"),
            ValueKind::Float => write!(f, "float"),
            ValueKind::Integer => write!(f, "integer"),
            ValueKind::List => write!(f, "list"),
            ValueKind::Nodeset => write!(f, "nodeset"),
            ValueKind::Null => write!(f, "null"),
            ValueKind::Object => write!(f, "object"),
            ValueKind::Regex => write!(f, "regex"),
            ValueKind::Secret => write!(f, "secret"),
            ValueKind::String => write!(f, "string"),
            ValueKind::Unit => write!(f, "unit"),
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::{DateTime, NaiveDate, Utc};
    use regex::Regex;

    use super::*;

    #[test]
    fn test_repr() {
        assert_eq!(Value::Bool(true).repr(), "boolean <true>".to_string());
        assert_eq!(
            Value::Bytes(vec![1, 2, 3]).repr(),
            "bytes <010203>".to_string()
        );
        let datetime_naive = NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_micro_opt(12, 0, 0, 123000)
            .unwrap();
        let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(datetime_naive, Utc);
        assert_eq!(
            Value::Date(datetime_utc).repr(),
            "date <2000-01-01 12:00:00.123 UTC>".to_string()
        );
        assert_eq!(Value::List(vec![]).repr(), "list <[]>".to_string());
        assert_eq!(
            Value::Nodeset(5).repr(),
            "nodeset <Nodeset(size=5)>".to_string()
        );
        assert_eq!(Value::Null.repr(), "null <null>".to_string());
        assert_eq!(
            Value::Number(Number::Integer(1)).repr(),
            "integer <1>".to_string()
        );
        assert_eq!(
            Value::Number(Number::Float(1.0)).repr(),
            "float <1.0>".to_string()
        );
        assert_eq!(
            Value::Object(vec![]).repr(),
            "object <Object()>".to_string()
        );
        assert_eq!(
            Value::Regex(Regex::new(r"[0-9]+").unwrap()).repr(),
            "regex </[0-9]+/>"
        );
        assert_eq!(
            Value::String("Hello".to_string()).repr(),
            "string <Hello>".to_string()
        );
        assert_eq!(Value::Unit.repr(), "unit".to_string());
    }

    #[test]
    fn test_is_scalar() {
        assert!(Value::Number(Number::Integer(1)).is_scalar());
        assert!(!Value::List(vec![]).is_scalar());
    }

    #[test]
    fn test_eq() {
        assert!(!(Value::Bool(true) == Value::Bool(false)));
        assert!(Value::Number(Number::Integer(1)) == Value::Number(Number::Float(1.0)));
        assert!(!(Value::Bool(true) == Value::String("Hello".to_string())));
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
