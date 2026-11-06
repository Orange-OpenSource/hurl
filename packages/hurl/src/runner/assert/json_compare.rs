/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
use crate::runner::Value;
use hurl_core::ast::{JsonValue, JsonValueKind, SourceInfo};
use serde_json;

/// Represents a mismatch between an expected and actual JSON value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mismatch {
    path: String,
    source_info: SourceInfo,
    kind: MismatchKind,
}

impl Mismatch {
    pub fn new(path: String, source_info: SourceInfo, kind: MismatchKind) -> Self {
        Self {
            path,
            source_info,
            kind,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MismatchKind {
    Value { actual: Value, expected: Value },
}

fn number_from_str(number: &str) -> Number {
    if number.contains(['.', 'e', 'E']) {
        return Number::Float(
            number
                .parse::<f64>()
                .expect("JSON number is representable as f64"),
        );
    }

    if let Ok(number) = number.parse::<i64>() {
        Number::Integer(number)
    } else {
        Number::BigInteger(number.to_string())
    }
}

fn value_from_json(value: &serde_json::Value) -> Value {
    match value {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(value) => Value::Bool(*value),
        serde_json::Value::Number(value) => Value::Number(number_from_str(&value.to_string())),
        _ => todo!(),
    }
}

fn value_from_expected(expected: &JsonValue) -> Value {
    match expected.kind() {
        JsonValueKind::Number(number) => Value::Number(number_from_str(number)),
        JsonValueKind::Boolean(value) => Value::Bool(*value),
        JsonValueKind::Null => Value::Null,
        _ => todo!(),
    }
}

fn mismatch(path: &str, expected: &JsonValue, actual: &serde_json::Value) -> Mismatch {
    Mismatch::new(
        path.to_string(),
        *expected.source_info(),
        MismatchKind::Value {
            actual: value_from_json(actual),
            expected: value_from_expected(expected),
        },
    )
}

/// Compares a JSON value against an expected JSON value and returns a list of mismatches.
///
/// # Arguments
/// * `path` - The JSON path to the value being compared.
/// * `expected` - The expected JSON value.
/// * `actual` - The actual JSON value.
///
/// # Returns
/// A vector of `Mismatch` instances representing the differences between the expected and actual JSON values.
pub fn compare(path: &str, expected: &JsonValue, actual: &serde_json::Value) -> Vec<Mismatch> {
    match (expected.kind(), actual) {
        (JsonValueKind::Number(expected_number), serde_json::Value::Number(actual_number)) => {
            // TODO: avoid f64 equality
            let expected_number_value = expected_number
                .parse::<f64>()
                .expect("expected number is valid JSON");
            let actual_number_value = actual_number
                .as_f64()
                .expect("actual JSON number is representable as f64");

            if expected_number_value != actual_number_value {
                return vec![mismatch(path, expected, actual)];
            }
            vec![]
        }
        (JsonValueKind::Boolean(expected_boolean), serde_json::Value::Bool(actual_boolean)) => {
            if expected_boolean == actual_boolean {
                vec![]
            } else {
                vec![mismatch(path, expected, actual)]
            }
        }
        (JsonValueKind::Null, serde_json::Value::Null) => {
            vec![]
        }
        _ => todo!("Mismatch at path {}", path),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use hurl_core::reader::Pos;

    fn json_number(number: &str) -> JsonValue {
        JsonValue::new(
            SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)),
            JsonValueKind::Number(number.to_string()),
        )
    }

    fn json_value(kind: JsonValueKind) -> JsonValue {
        JsonValue::new(SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)), kind)
    }

    #[test]
    fn test_compare_number_as_equal() {
        assert!(compare("$.age", &json_number("22"), &serde_json::json!(22)).is_empty());
        assert!(compare("$.age", &json_number("1.0"), &serde_json::json!(1)).is_empty());
        assert!(compare("$.age", &json_number("1"), &serde_json::json!(1.0)).is_empty());
    }

    #[test]
    fn test_compare_number_as_not_equal() {
        assert_eq!(
            compare("$.age", &json_number("20"), &serde_json::json!(22)),
            vec![Mismatch::new(
                "$.age".to_string(),
                SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)),
                MismatchKind::Value {
                    actual: Value::Number(Number::Integer(22)),
                    expected: Value::Number(Number::Integer(20)),
                }
            )]
        );
        assert_eq!(
            compare("$.age", &json_number("20.0"), &serde_json::json!(22.0)),
            vec![Mismatch::new(
                "$.age".to_string(),
                SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)),
                MismatchKind::Value {
                    actual: Value::Number(Number::Float(22.0)),
                    expected: Value::Number(Number::Float(20.0)),
                }
            )]
        );
    }

    #[test]
    fn test_compare_boolean() {
        assert!(
            compare(
                "$.enabled",
                &json_value(JsonValueKind::Boolean(true)),
                &serde_json::json!(true)
            )
            .is_empty()
        );

        assert_eq!(
            compare(
                "$.enabled",
                &json_value(JsonValueKind::Boolean(true)),
                &serde_json::json!(false)
            ),
            vec![Mismatch::new(
                "$.enabled".to_string(),
                SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)),
                MismatchKind::Value {
                    actual: Value::Bool(false),
                    expected: Value::Bool(true),
                }
            )]
        );
    }

    #[test]
    fn test_compare_null() {
        assert!(
            compare(
                "$.value",
                &json_value(JsonValueKind::Null),
                &serde_json::Value::Null
            )
            .is_empty()
        );
    }
}
