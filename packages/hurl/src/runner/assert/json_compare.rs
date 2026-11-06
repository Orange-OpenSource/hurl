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
use crate::runner::expr;
use crate::runner::template::eval_template;
use crate::runner::{Number, RunnerError, Value, VariableSet};
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
        serde_json::Value::String(value) => Value::String(value.clone()),
        _ => todo!(),
    }
}

fn value_from_expected(
    expected: &JsonValue,
    variables: &VariableSet,
) -> Result<Value, RunnerError> {
    let value = match expected.kind() {
        JsonValueKind::Placeholder(placeholder) => expr::eval(&placeholder.expr, variables)?,
        JsonValueKind::String(template) => Value::String(eval_template(template, variables)?),
        JsonValueKind::Number(number) => Value::Number(number_from_str(number)),
        JsonValueKind::Boolean(value) => Value::Bool(*value),
        JsonValueKind::Null => Value::Null,
        _ => todo!(),
    };
    Ok(value)
}

fn mismatch(
    path: &str,
    expected: &JsonValue,
    actual: &serde_json::Value,
    variables: &VariableSet,
) -> Result<Mismatch, RunnerError> {
    Ok(Mismatch::new(
        path.to_string(),
        *expected.source_info(),
        MismatchKind::Value {
            actual: value_from_json(actual),
            expected: value_from_expected(expected, variables)?,
        },
    ))
}

/// Compares a JSON value against an expected JSON value and returns a list of mismatches.
///
/// # Arguments
/// * `path` - The JSON path to the value being compared.
/// * `expected` - The expected JSON value.
/// * `actual` - The actual JSON value.
/// * `variables` - The variables used to evaluate placeholders and string templates.
///
/// # Returns
/// A vector of mismatches, or an error if evaluating the expected value fails.
pub fn compare(
    path: &str,
    expected: &JsonValue,
    actual: &serde_json::Value,
    variables: &VariableSet,
) -> Result<Vec<Mismatch>, RunnerError> {
    let mismatches = match (expected.kind(), actual) {
        (JsonValueKind::Number(expected_number), serde_json::Value::Number(actual_number)) => {
            // TODO: avoid f64 equality
            let expected_number_value = expected_number
                .parse::<f64>()
                .expect("expected number is valid JSON");
            let actual_number_value = actual_number
                .as_f64()
                .expect("actual JSON number is representable as f64");

            if expected_number_value == actual_number_value {
                vec![]
            } else {
                vec![mismatch(path, expected, actual, variables)?]
            }
        }
        (JsonValueKind::Boolean(expected_boolean), serde_json::Value::Bool(actual_boolean)) => {
            if expected_boolean == actual_boolean {
                vec![]
            } else {
                vec![mismatch(path, expected, actual, variables)?]
            }
        }
        (JsonValueKind::Null, serde_json::Value::Null) => {
            vec![]
        }
        (JsonValueKind::String(expected_string), serde_json::Value::String(actual_string)) => {
            let expected_string = eval_template(expected_string, variables)?;
            if expected_string == *actual_string {
                vec![]
            } else {
                vec![Mismatch::new(
                    path.to_string(),
                    *expected.source_info(),
                    MismatchKind::Value {
                        actual: Value::String(actual_string.clone()),
                        expected: Value::String(expected_string),
                    },
                )]
            }
        }
        (JsonValueKind::Placeholder(placeholder), _) => {
            let expected_value = expr::eval(&placeholder.expr, variables)?;
            let actual_value = value_from_json(actual);
            if expected_value == actual_value {
                vec![]
            } else {
                vec![Mismatch::new(
                    path.to_string(),
                    *expected.source_info(),
                    MismatchKind::Value {
                        actual: actual_value,
                        expected: expected_value,
                    },
                )]
            }
        }
        _ => vec![mismatch(path, expected, actual, variables)?],
    };
    Ok(mismatches)
}

#[cfg(test)]
mod tests {

    use super::*;
    use hurl_core::ast::{
        Expr, ExprKind, Placeholder, Template, TemplateElement, Variable, Whitespace,
    };
    use hurl_core::reader::Pos;
    use hurl_core::types::ToSource;

    fn json_number(number: &str) -> JsonValue {
        JsonValue::new(
            SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)),
            JsonValueKind::Number(number.to_string()),
        )
    }

    fn json_value(kind: JsonValueKind) -> JsonValue {
        JsonValue::new(SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)), kind)
    }

    // return {{name}} placeholder
    fn name_placeholder() -> Placeholder {
        let source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1));
        Placeholder {
            space0: Whitespace {
                value: String::new(),
                source_info,
            },
            expr: Expr {
                kind: ExprKind::Variable(Variable {
                    name: "name".to_string(),
                    source_info,
                }),
                source_info,
            },
            space1: Whitespace {
                value: String::new(),
                source_info,
            },
        }
    }

    // return {{value}} placeholder
    fn value_placeholder() -> Placeholder {
        let source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 5));
        Placeholder {
            space0: Whitespace {
                value: String::new(),
                source_info,
            },
            expr: Expr {
                kind: ExprKind::Variable(Variable {
                    name: "value".to_string(),
                    source_info,
                }),
                source_info,
            },
            space1: Whitespace {
                value: String::new(),
                source_info,
            },
        }
    }

    fn json_string(elements: Vec<TemplateElement>) -> JsonValue {
        json_value(JsonValueKind::String(Template::new(
            Some('"'),
            elements,
            SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)),
        )))
    }

    fn variables(value: Value) -> VariableSet {
        let mut variables = VariableSet::new();
        variables.insert("value".to_string(), value);
        variables
    }

    #[test]
    fn test_compare_placeholder_with_string() {
        let expected = json_value(JsonValueKind::Placeholder(value_placeholder()));
        assert!(
            compare(
                "$.value",
                &expected,
                &serde_json::json!("Alice"),
                &variables(Value::String("Alice".to_string()))
            )
            .unwrap()
            .is_empty()
        );
        assert_eq!(
            compare(
                "$.value",
                &expected,
                &serde_json::json!("Alice"),
                &variables(Value::String("Bob".to_string()))
            )
            .unwrap(),
            vec![Mismatch::new(
                "$.value".to_string(),
                *expected.source_info(),
                MismatchKind::Value {
                    actual: Value::String("Alice".to_string()),
                    expected: Value::String("Bob".to_string()),
                },
            )]
        );
        assert_eq!(
            compare(
                "$.value",
                &expected,
                &serde_json::json!("Alice"),
                &variables(Value::Bool(true))
            )
            .unwrap(),
            vec![Mismatch::new(
                "$.value".to_string(),
                *expected.source_info(),
                MismatchKind::Value {
                    actual: Value::String("Alice".to_string()),
                    expected: Value::Bool(true),
                },
            )]
        );
    }
    #[test]
    fn test_compare_placeholder_with_bool() {
        let expected = json_value(JsonValueKind::Placeholder(value_placeholder()));
        assert!(
            compare(
                "$.value",
                &expected,
                &serde_json::json!(true),
                &variables(Value::Bool(true))
            )
            .unwrap()
            .is_empty()
        );
        assert_eq!(
            compare(
                "$.value",
                &expected,
                &serde_json::json!(true),
                &variables(Value::Bool(false))
            )
            .unwrap(),
            vec![Mismatch::new(
                "$.value".to_string(),
                *expected.source_info(),
                MismatchKind::Value {
                    actual: Value::Bool(true),
                    expected: Value::Bool(false),
                },
            )]
        );
        assert_eq!(
            compare(
                "$.value",
                &expected,
                &serde_json::json!(true),
                &variables(Value::Number(Number::Integer(1)))
            )
            .unwrap(),
            vec![Mismatch::new(
                "$.value".to_string(),
                *expected.source_info(),
                MismatchKind::Value {
                    actual: Value::Bool(true),
                    expected: Value::Number(Number::Integer(1)),
                },
            )]
        );
    }

    #[test]
    fn test_compare_string_template() {
        // Hello {{name}}
        let expected = json_string(vec![
            TemplateElement::String {
                value: "Hello ".to_string(),
                source: "Hello ".to_source(),
            },
            TemplateElement::Placeholder(name_placeholder()),
        ]);
        let mut variables = VariableSet::new();
        variables.insert("name".to_string(), Value::String("Alice".to_string()));
        assert!(
            compare(
                "$.name",
                &expected,
                &serde_json::json!("Hello Alice"),
                &variables
            )
            .unwrap()
            .is_empty()
        );
        assert_eq!(
            compare(
                "$.name",
                &expected,
                &serde_json::json!("Hello Bob"),
                &variables
            )
            .unwrap(),
            vec![Mismatch::new(
                "$.name".to_string(),
                *expected.source_info(),
                MismatchKind::Value {
                    actual: Value::String("Hello Bob".to_string()),
                    expected: Value::String("Hello Alice".to_string()),
                },
            )]
        );
    }

    #[test]
    fn test_compare_number_as_equal() {
        let variables = VariableSet::new();
        for (expected, actual) in [
            ("22", serde_json::json!(22)),
            ("1.0", serde_json::json!(1)),
            ("1", serde_json::json!(1.0)),
        ] {
            assert!(
                compare("$.age", &json_number(expected), &actual, &variables)
                    .unwrap()
                    .is_empty()
            );
        }
    }

    #[test]
    fn test_compare_number_as_not_equal() {
        assert_eq!(
            compare(
                "$.age",
                &json_number("20"),
                &serde_json::json!(22),
                &VariableSet::new()
            )
            .unwrap(),
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
            compare(
                "$.age",
                &json_number("20.0"),
                &serde_json::json!(22.0),
                &VariableSet::new()
            )
            .unwrap(),
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
                &serde_json::json!(true),
                &VariableSet::new()
            )
            .unwrap()
            .is_empty()
        );

        assert_eq!(
            compare(
                "$.enabled",
                &json_value(JsonValueKind::Boolean(true)),
                &serde_json::json!(false),
                &VariableSet::new()
            )
            .unwrap(),
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
                &serde_json::Value::Null,
                &VariableSet::new()
            )
            .unwrap()
            .is_empty()
        );
    }
}
