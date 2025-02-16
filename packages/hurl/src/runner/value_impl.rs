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

use super::value::ValueKind;
use super::{EvalError, Value};

impl Value {
    /// Compare with another value.
    ///
    /// Returns a [`EvalError::Type`] if the given value types are not supported.
    pub fn compare(&self, other: &Value) -> Result<Ordering, EvalError> {
        match (self, other) {
            (Value::String(s1), Value::String(s2)) => Ok(s1.cmp(s2)),
            (Value::Number(n1), Value::Number(n2)) => Ok(n1.cmp_value(n2)),
            _ => Err(EvalError::Type),
        }
    }

    /// Returns `true` if the value starts with the given prefix.
    ///
    /// Returns `false` if it does not.
    ///
    /// Returns a [`EvalError::Type`] if the given value types are not supported.
    pub fn starts_with(&self, other: &Value) -> Result<bool, EvalError> {
        match (self, other) {
            (Value::String(value), Value::String(prefix)) => Ok(value.starts_with(prefix)),
            (Value::Bytes(value), Value::Bytes(prefix)) => Ok(value.starts_with(prefix)),
            _ => Err(EvalError::Type),
        }
    }

    /// Returns `true` if the value ends with the given suffix.
    ///
    /// Returns `false` if it does not.
    ///
    /// Returns a [`EvalError::Type`] if the given value types are not supported.
    pub fn ends_with(&self, other: &Value) -> Result<bool, EvalError> {
        match (self, other) {
            (Value::String(value), Value::String(suffix)) => Ok(value.ends_with(suffix)),
            (Value::Bytes(value), Value::Bytes(suffix)) => Ok(value.ends_with(suffix)),
            _ => Err(EvalError::Type),
        }
    }

    /// Returns `true` if the value contains another value.
    ///
    /// Returns `false` if it does not.
    ///     
    /// Returns a [`EvalError::Type`] if the given value types are not supported.
    pub fn contains(&self, other: &Value) -> Result<bool, EvalError> {
        match (self, other) {
            (Value::String(s), Value::String(substr)) => Ok(s.as_str().contains(substr.as_str())),

            (Value::Bytes(s), Value::Bytes(substr)) => {
                Ok(contains(s.as_slice(), substr.as_slice()))
            }
            (Value::List(values), _) => {
                let mut included = false;
                for v in values {
                    if v == other {
                        included = true;
                        break;
                    }
                }
                Ok(included)
            }
            _ => Err(EvalError::Type),
        }
    }

    /// Returns `true` if the list value includes another value.
    ///
    /// Returns `false` if it does not.
    ///     
    /// Returns a [`EvalError::Type`] if the given value types are not supported.
    ///
    /// TODO: deprecate method in favor of contains.
    pub fn includes(&self, other: &Value) -> Result<bool, EvalError> {
        match self {
            Value::List(values) => {
                let mut included = false;
                for v in values {
                    if v == other {
                        included = true;
                        break;
                    }
                }
                Ok(included)
            }
            _ => Err(EvalError::Type),
        }
    }

    /// Returns `true` the value is a boolean.
    ///
    /// Returns `false` if it is not.
    pub fn is_boolean(&self) -> bool {
        self.kind() == ValueKind::Bool
    }

    /// Returns `true` the value is a collection.
    ///
    /// Returns `false` if it is not.
    pub fn is_collection(&self) -> bool {
        self.kind() == ValueKind::Bytes
            || self.kind() == ValueKind::List
            || self.kind() == ValueKind::Nodeset
            || self.kind() == ValueKind::Object
    }

    /// Returns `true` the value is a date.
    ///
    /// Returns `false` if it is not.
    pub fn is_date(&self) -> bool {
        self.kind() == ValueKind::Date
    }

    /// Returns `true` the value is a float.
    ///
    /// Returns `false` if it is not.
    pub fn is_float(&self) -> bool {
        self.kind() == ValueKind::Float
    }

    /// Returns `true` the value is an integer.
    ///
    /// Returns `false` if it is not.
    pub fn is_integer(&self) -> bool {
        self.kind() == ValueKind::Integer
    }

    /// Returns `true` the value is a number.
    ///
    /// Returns `false` if it is not.
    pub fn is_number(&self) -> bool {
        self.kind() == ValueKind::Integer || self.kind() == ValueKind::Float
    }

    /// Returns `true` the value is a String.
    ///
    /// Returns `false` if it is not.
    pub fn is_string(&self) -> bool {
        self.kind() == ValueKind::String || self.kind() == ValueKind::Secret
    }

    /// Returns `true` the string value represents a RFC339 date (format YYYY-MM-DDTHH:mm:ss.sssZ).
    ///
    /// Returns `false` if it does not.
    ///
    /// Returns a [`EvalError::Type`] if the given value is not a String.
    pub fn is_iso_date(&self) -> Result<bool, EvalError> {
        match self {
            Value::String(value) => Ok(chrono::DateTime::parse_from_rfc3339(value).is_ok()),
            _ => Err(EvalError::Type),
        }
    }

    /// Returns count of the value.
    ///
    /// Returns a [`EvalError::Type`] if the type of the value is not supported.
    pub fn count(&self) -> Result<usize, EvalError> {
        match self {
            Value::List(values) => Ok(values.len()),
            Value::String(data) => Ok(data.len()),
            Value::Nodeset(count) => Ok(*count),
            Value::Object(props) => Ok(props.len()),
            Value::Bytes(data) => Ok(data.len()),
            _ => Err(EvalError::Type),
        }
    }

    /// Returns `true` if and only if there is a match for the regex anywhere in the value.
    ///
    /// Returns `false` otherwise.
    ///
    /// Returns a [`EvalError::Type`] if the type of the value is not supported.
    ///
    /// Returns an [`EvalError::InvalidRegex`] if the String is not a valid Regex.
    pub fn is_match(&self, other: &Value) -> Result<bool, EvalError> {
        let regex = match other {
            Value::String(s) => match regex::Regex::new(s.as_str()) {
                Ok(re) => re,
                Err(_) => return Err(EvalError::InvalidRegex),
            },
            Value::Regex(re) => re.clone(),
            _ => {
                return Err(EvalError::Type);
            }
        };
        match self {
            Value::String(value) => Ok(regex.is_match(value.as_str())),
            _ => Err(EvalError::Type),
        }
    }
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::Number;

    #[test]
    fn test_compare() {
        assert_eq!(
            Value::Number(Number::Integer(1))
                .compare(&Value::Number(Number::Integer(1)))
                .unwrap(),
            Ordering::Equal
        );
        assert_eq!(
            Value::Number(Number::Integer(1))
                .compare(&Value::Number(Number::Integer(2)))
                .unwrap(),
            Ordering::Less
        );
        assert_eq!(
            Value::String("xyz".to_string())
                .compare(&Value::String("abc".to_string()))
                .unwrap(),
            Ordering::Greater
        );

        assert_eq!(
            Value::Number(Number::Integer(1))
                .compare(&Value::Bool(true))
                .unwrap_err(),
            EvalError::Type
        );
    }

    #[test]
    fn test_starts_with() {
        assert!(Value::String("Hello".to_string())
            .starts_with(&Value::String("H".to_string()))
            .unwrap());
        assert!(!Value::Bytes(vec![0, 1, 2])
            .starts_with(&Value::Bytes(vec![0, 2]))
            .unwrap());
    }

    #[test]
    fn test_ends_with() {
        assert!(Value::String("Hello".to_string())
            .ends_with(&Value::String("o".to_string()))
            .unwrap());
        assert!(!Value::Bytes(vec![0, 1, 2])
            .ends_with(&Value::Bytes(vec![0, 2]))
            .unwrap());
    }

    #[test]
    fn test_contains() {
        let haystack = [1, 2, 3];
        assert!(contains(&haystack, &[1]));
        assert!(contains(&haystack, &[1, 2]));
        assert!(!contains(&haystack, &[1, 3]));
        assert!(Value::String("abc".to_string())
            .contains(&Value::String("ab".to_string()))
            .unwrap());
        let values = Value::List(vec![
            Value::Number(Number::Integer(0)),
            Value::Number(Number::Integer(2)),
            Value::Number(Number::Integer(3)),
        ]);
        assert!(values.contains(&Value::Number(Number::Integer(0))).unwrap());
        assert!(!values.contains(&Value::Number(Number::Integer(4))).unwrap());
    }

    #[test]
    fn test_include() {
        let values = Value::List(vec![
            Value::Number(Number::Integer(0)),
            Value::Number(Number::Integer(2)),
            Value::Number(Number::Integer(3)),
        ]);
        assert!(values.includes(&Value::Number(Number::Integer(0))).unwrap());
        assert!(!values.includes(&Value::Number(Number::Integer(4))).unwrap());
    }

    #[test]
    fn test_type() {
        let value1 = Value::Bool(true);
        assert!(value1.is_boolean());
        assert!(!value1.is_collection());

        let value2 = Value::Number(Number::Integer(1));
        assert!(!value2.is_boolean());
        assert!(!value2.is_collection());
        assert!(value2.is_number());
        assert!(value2.is_integer());
    }

    #[test]
    fn test_iso_date() {
        // Some values from <https://datatracker.ietf.org/doc/html/rfc3339>
        assert!(Value::String("1985-04-12T23:20:50.52Z".to_string())
            .is_iso_date()
            .unwrap());
        assert!(Value::String("1996-12-19T16:39:57-08:00".to_string())
            .is_iso_date()
            .unwrap());
        assert!(Value::String("1990-12-31T23:59:60Z".to_string())
            .is_iso_date()
            .unwrap());
        assert!(Value::String("1990-12-31T15:59:60-08:00".to_string())
            .is_iso_date()
            .unwrap());
        assert!(Value::String("1937-01-01T12:00:27.87+00:20".to_string())
            .is_iso_date()
            .unwrap());
        assert!(!Value::String("1978-01-15".to_string())
            .is_iso_date()
            .unwrap());
        assert_eq!(
            Value::Bool(true).is_iso_date().unwrap_err(),
            EvalError::Type
        );
    }

    #[test]
    fn test_is_match() {
        let value = Value::String("hello".to_string());

        let regex1 = Value::String("he.*".to_string());
        assert!(value.is_match(&regex1).unwrap());

        let regex2 = Value::Regex(regex::Regex::new("he.*").unwrap());
        assert!(value.is_match(&regex2).unwrap());

        let regex3 = Value::String("HE.*".to_string());
        assert!(!value.is_match(&regex3).unwrap());

        let regex4 = Value::String("?HE.*".to_string());
        assert_eq!(
            value.is_match(&regex4).unwrap_err(),
            EvalError::InvalidRegex
        );

        let regex5 = Value::Bool(true);
        assert_eq!(value.is_match(&regex5).unwrap_err(), EvalError::Type);
    }
}
