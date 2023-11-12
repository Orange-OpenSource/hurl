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
use std::fmt;

/// System types used in Hurl.
///
/// Values are used by queries, captures, asserts and predicates.
#[derive(Clone, Debug)]
pub enum Number {
    Float(f64),
    Integer(i64),
}

// You must implement it yourself because of the Float
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Float(v1), Number::Float(v2)) => (v1 - v2).abs() < f64::EPSILON,
            (Number::Integer(v1), Number::Integer(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl Eq for Number {}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Number::Float(f) => format_float(*f),
            Number::Integer(x) => x.to_string(),
        };
        write!(f, "{value}")
    }
}

fn format_float(value: f64) -> String {
    if value.fract() < f64::EPSILON {
        format!("{value}.0")
    } else {
        value.to_string()
    }
}

impl Number {
    pub fn _type(&self) -> String {
        match self {
            Number::Float(_) => "float".to_string(),
            Number::Integer(_) => "integer".to_string(),
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::Integer(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(Number::Float(1.0).to_string(), "1.0".to_string());
        assert_eq!(Number::Float(1.1).to_string(), "1.1".to_string());
        assert_eq!(Number::from(1.0).to_string(), "1.0".to_string());
        assert_eq!(Number::from(1.1).to_string(), "1.1".to_string());
    }
}
