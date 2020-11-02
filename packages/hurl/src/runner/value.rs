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

///
/// Type system used in hurl
/// Values are used by queries, captures, asserts and predicates
///

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Unit,
    Bool(bool),
    Integer(i64),

    // can not use simply Float(f64)
    // the trait `std::cmp::Eq` is not implemented for `f64`
    // integer part, decimal part (9 digits) TODO Clarify your custom type
    Float(i64, u64),

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
            Value::Unit => "Unit".to_string(),
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
            Value::Null => "null".to_string(),
            Value::Unit => "unit".to_string(),
        }
    }

    pub fn from_f64(value: f64) -> Value {
        let integer = if value < 0.0 {
            value.ceil() as i64
        } else {
            value.floor() as i64
        };
        let decimal = (value.abs().fract() * 1_000_000_000_000_000_000.0).round() as u64;
        Value::Float(integer, decimal)
    }

    pub fn is_scalar(&self) -> bool {
        !matches!(self, Value::Nodeset(_) | Value::List(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f64() {
        assert_eq!(Value::from_f64(1.0), Value::Float(1, 0));
        assert_eq!(Value::from_f64(-1.0), Value::Float(-1, 0));
        assert_eq!(
            Value::from_f64(1.1),
            Value::Float(1, 100_000_000_000_000_096)
        ); //TBC!!
        assert_eq!(
            Value::from_f64(-1.1),
            Value::Float(-1, 100_000_000_000_000_096)
        );
        assert_eq!(
            Value::from_f64(1.5),
            Value::Float(1, 500_000_000_000_000_000)
        );
    }

    #[test]
    fn test_is_scalar() {
        assert_eq!(Value::Integer(1).is_scalar(), true);
        assert_eq!(Value::List(vec![]).is_scalar(), false);
    }
}
