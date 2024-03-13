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

use super::CliOptionsError;
use crate::runner::{Number, Value};

pub fn parse(s: &str) -> Result<(String, Value), CliOptionsError> {
    match s.find('=') {
        None => Err(CliOptionsError::Error(format!(
            "Missing value for variable {s}!"
        ))),
        Some(index) => {
            let (name, value) = s.split_at(index);
            let value = parse_value(&value[1..])?;
            Ok((name.to_string(), value))
        }
    }
}

pub fn parse_value(s: &str) -> Result<Value, CliOptionsError> {
    if s == "true" {
        Ok(Value::Bool(true))
    } else if s == "false" {
        Ok(Value::Bool(false))
    } else if s == "null" {
        Ok(Value::Null)
    } else if let Ok(v) = s.parse::<i64>() {
        Ok(Value::Number(Number::Integer(v)))
    } else if let Ok(v) = s.parse::<f64>() {
        Ok(Value::Number(Number::Float(v)))
    } else if let Some(s) = s.strip_prefix('"') {
        if let Some(s) = s.strip_suffix('"') {
            Ok(Value::String(s.to_string()))
        } else {
            Err(CliOptionsError::Error(
                "Value should end with a double quote".to_string(),
            ))
        }
    } else {
        Ok(Value::String(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::{CliOptionsError, *};

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("name=Jennifer").unwrap(),
            ("name".to_string(), Value::String("Jennifer".to_string()))
        );
        assert_eq!(
            parse("female=true").unwrap(),
            ("female".to_string(), Value::Bool(true))
        );
        assert_eq!(
            parse("age=30").unwrap(),
            ("age".to_string(), Value::Number(Number::Integer(30)))
        );
        assert_eq!(
            parse("height=1.7").unwrap(),
            ("height".to_string(), Value::Number(Number::Float(1.7)))
        );
        assert_eq!(
            parse("id=\"123\"").unwrap(),
            ("id".to_string(), Value::String("123".to_string()))
        );
        assert_eq!(
            parse("a_null=null").unwrap(),
            ("a_null".to_string(), Value::Null)
        );
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(
            parse("name").err().unwrap(),
            CliOptionsError::Error("Missing value for variable name!".to_string())
        );
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(
            parse_value("Jennifer").unwrap(),
            Value::String("Jennifer".to_string())
        );
        assert_eq!(parse_value("true").unwrap(), Value::Bool(true));
        assert_eq!(
            parse_value("30").unwrap(),
            Value::Number(Number::Integer(30))
        );
        assert_eq!(
            parse_value("1.7").unwrap(),
            Value::Number(Number::Float(1.7))
        );
        assert_eq!(
            parse_value("1.0").unwrap(),
            Value::Number(Number::Float(1.0))
        );
        assert_eq!(
            parse_value("-1.0").unwrap(),
            Value::Number(Number::Float(-1.0))
        );
        assert_eq!(
            parse_value("\"123\"").unwrap(),
            Value::String("123".to_string())
        );
        assert_eq!(parse_value("null").unwrap(), Value::Null);
    }

    #[test]
    fn test_parse_value_error() {
        assert_eq!(
            parse_value("\"123").err().unwrap(),
            CliOptionsError::Error("Value should end with a double quote".to_string())
        );
    }
}
