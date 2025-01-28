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

use super::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeError;

impl Value {
    /// Compare with another value
    ///
    /// Returns TypeError if the values are not comparable
    pub fn compare(&self, other: &Value) -> Result<Ordering, TypeError> {
        match (self, other) {
            (Value::String(s1), Value::String(s2)) => Ok(s1.cmp(s2)),
            (Value::Number(n1), Value::Number(n2)) => Ok(n1.cmp_value(n2)),
            _ => Err(TypeError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::Number;

    use super::*;

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
    }

    #[test]
    fn test_compare_error() {
        assert_eq!(
            Value::Number(Number::Integer(1))
                .compare(&Value::Bool(true))
                .unwrap_err(),
            TypeError
        );
    }
}
