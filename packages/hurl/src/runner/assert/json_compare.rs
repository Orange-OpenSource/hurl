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
use std::collections::HashSet;

use hurl_core::ast::{JsonListElement, JsonObjectElement, JsonValue, JsonValueKind, SourceInfo};

use crate::runner::expr;
use crate::runner::template::eval_template;
use crate::runner::{Number, RunnerError, Value, VariableSet};

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
#[allow(clippy::large_enum_variant)]
pub enum MismatchKind {
    /// The actual value differs from the expected value (including type mismatches).
    Value { actual: Value, expected: Value },
    /// A key of the expected object is absent from the actual object.
    MissingKey { expected: Value },
    /// A key of the actual object is absent from the expected object.
    UnexpectedKey { actual: Value },
    /// An element of the expected array is absent from the actual array.
    MissingElement { expected: Value },
    /// An element of the actual array is absent from the expected array.
    UnexpectedElement { actual: Value },
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
        serde_json::Value::Array(values) => {
            Value::List(values.iter().map(value_from_json).collect())
        }
        serde_json::Value::Object(values) => Value::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), value_from_json(value)))
                .collect(),
        ),
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
        JsonValueKind::List { elements, .. } => Value::List(
            elements
                .iter()
                .map(|element| value_from_expected(&element.value, variables))
                .collect::<Result<_, _>>()?,
        ),
        JsonValueKind::Object { elements, .. } => Value::Object(
            elements
                .iter()
                .map(|element| {
                    Ok((
                        eval_template(&element.name, variables)?,
                        value_from_expected(&element.value, variables)?,
                    ))
                })
                .collect::<Result<_, RunnerError>>()?,
        ),
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
        (JsonValueKind::List { elements, .. }, serde_json::Value::Array(actual_elements)) => {
            compare_lists(
                path,
                elements,
                actual_elements,
                *expected.source_info(),
                variables,
            )?
        }
        (JsonValueKind::Object { elements, .. }, serde_json::Value::Object(actual_elements)) => {
            compare_objects(
                path,
                elements,
                actual_elements,
                *expected.source_info(),
                variables,
            )?
        }
        _ => vec![mismatch(path, expected, actual, variables)?],
    };
    Ok(mismatches)
}

/// Compares two objects key by key
///
/// Missing keys and value mismatches are reported first,
/// then keys present only in the actual object.
fn compare_objects(
    path: &str,
    expected_elements: &[JsonObjectElement],
    actual_elements: &serde_json::Map<String, serde_json::Value>,
    source_info: SourceInfo,
    variables: &VariableSet,
) -> Result<Vec<Mismatch>, RunnerError> {
    let mut mismatches = vec![];
    let mut expected_keys = HashSet::new();
    for expected_element in expected_elements {
        let key = eval_template(&expected_element.name, variables)?;
        let key_path = format!("{path}.{key}");
        match actual_elements.get(&key) {
            Some(actual_value) => mismatches.extend(compare(
                &key_path,
                &expected_element.value,
                actual_value,
                variables,
            )?),
            None => mismatches.push(Mismatch::new(
                key_path,
                expected_element.name.source_info,
                MismatchKind::MissingKey {
                    expected: value_from_expected(&expected_element.value, variables)?,
                },
            )),
        }
        expected_keys.insert(key);
    }
    for (key, actual_value) in actual_elements {
        if !expected_keys.contains(key) {
            mismatches.push(Mismatch::new(
                format!("{path}.{key}"),
                source_info,
                MismatchKind::UnexpectedKey {
                    actual: value_from_json(actual_value),
                },
            ));
        }
    }
    Ok(mismatches)
}

/// Compares two arrays element by element
/// and stops at the first different item
///
/// A different item is:
/// - either a value mismatch
/// - an element missing from the actual array (located on the expected element)
/// - an unexpected actual element (located on the expected array)
fn compare_lists(
    path: &str,
    expected_elements: &[JsonListElement],
    actual_elements: &[serde_json::Value],
    source_info: SourceInfo,
    variables: &VariableSet,
) -> Result<Vec<Mismatch>, RunnerError> {
    let count = expected_elements.len().max(actual_elements.len());
    for index in 0..count {
        let element_path = format!("{path}[{index}]");
        let mismatches = match (expected_elements.get(index), actual_elements.get(index)) {
            (Some(expected_element), Some(actual_element)) => compare(
                &element_path,
                &expected_element.value,
                actual_element,
                variables,
            )?,
            (Some(expected_element), None) => vec![Mismatch::new(
                element_path,
                *expected_element.value.source_info(),
                MismatchKind::MissingElement {
                    expected: value_from_expected(&expected_element.value, variables)?,
                },
            )],
            (None, Some(actual_element)) => vec![Mismatch::new(
                element_path,
                source_info,
                MismatchKind::UnexpectedElement {
                    actual: value_from_json(actual_element),
                },
            )],
            (None, None) => unreachable!("index is below the longest array length"),
        };
        if !mismatches.is_empty() {
            return Ok(mismatches);
        }
    }
    Ok(vec![])
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

    fn source_info() -> SourceInfo {
        SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10))
    }

    fn json_list(values: Vec<JsonValue>) -> JsonValue {
        json_value(JsonValueKind::List {
            space0: String::new(),
            elements: values
                .into_iter()
                .map(|value| JsonListElement {
                    space0: String::new(),
                    value,
                    space1: String::new(),
                })
                .collect(),
        })
    }

    fn json_object(entries: Vec<(&str, JsonValue)>) -> JsonValue {
        json_value(JsonValueKind::Object {
            space0: String::new(),
            elements: entries
                .into_iter()
                .map(|(key, value)| JsonObjectElement {
                    space0: String::new(),
                    name: Template::new(
                        Some('"'),
                        vec![TemplateElement::String {
                            value: key.to_string(),
                            source: key.to_source(),
                        }],
                        source_info(),
                    ),
                    space1: String::new(),
                    space2: String::new(),
                    value,
                    space3: String::new(),
                })
                .collect(),
        })
    }

    #[test]
    fn test_compare_object() {
        let variables = VariableSet::new();
        let expected = json_object(vec![
            ("id", json_number("1")),
            ("enabled", json_value(JsonValueKind::Boolean(true))),
        ]);

        assert!(
            compare(
                "$",
                &expected,
                &serde_json::json!({"enabled": true, "id": 1}),
                &variables
            )
            .unwrap()
            .is_empty()
        );

        // Missing key, mismatched value and unexpected key
        assert_eq!(
            compare(
                "$",
                &expected,
                &serde_json::json!({"enabled": false, "name": "Alice"}),
                &variables
            )
            .unwrap(),
            vec![
                Mismatch::new(
                    "$.id".to_string(),
                    source_info(),
                    MismatchKind::MissingKey {
                        expected: Value::Number(Number::Integer(1)),
                    },
                ),
                Mismatch::new(
                    "$.enabled".to_string(),
                    source_info(),
                    MismatchKind::Value {
                        actual: Value::Bool(false),
                        expected: Value::Bool(true),
                    },
                ),
                Mismatch::new(
                    "$.name".to_string(),
                    source_info(),
                    MismatchKind::UnexpectedKey {
                        actual: Value::String("Alice".to_string()),
                    },
                ),
            ]
        );
    }

    #[test]
    fn test_compare_nested() {
        let variables = VariableSet::new();
        let expected = json_object(vec![(
            "user",
            json_object(vec![
                ("name", json_string_literal("Bob")),
                (
                    "address",
                    json_object(vec![("city", json_string_literal("Paris"))]),
                ),
            ]),
        )]);
        let actual = serde_json::json!({"user": {"name": "Bobby", "address": {"city": "Paris"}}});
        assert_eq!(
            compare("$", &expected, &actual, &variables).unwrap(),
            vec![Mismatch::new(
                "$.user.name".to_string(),
                source_info(),
                MismatchKind::Value {
                    actual: Value::String("Bobby".to_string()),
                    expected: Value::String("Bob".to_string()),
                },
            )]
        );
    }

    #[test]
    fn test_compare_list() {
        let variables = VariableSet::new();
        let expected = json_list(vec![json_number("1"), json_number("2")]);

        assert!(
            compare("$.ids", &expected, &serde_json::json!([1, 2]), &variables)
                .unwrap()
                .is_empty()
        );

        // Same length, one element differs
        assert_eq!(
            compare("$.ids", &expected, &serde_json::json!([1, 3]), &variables).unwrap(),
            vec![Mismatch::new(
                "$.ids[1]".to_string(),
                source_info(),
                MismatchKind::Value {
                    actual: Value::Number(Number::Integer(3)),
                    expected: Value::Number(Number::Integer(2)),
                },
            )]
        );

        // Different length, only the first different item is reported
        assert_eq!(
            compare(
                "$.ids",
                &expected,
                &serde_json::json!([1, 3, 4]),
                &variables
            )
            .unwrap(),
            vec![Mismatch::new(
                "$.ids[1]".to_string(),
                source_info(),
                MismatchKind::Value {
                    actual: Value::Number(Number::Integer(3)),
                    expected: Value::Number(Number::Integer(2)),
                },
            )]
        );

        // Missing expected array element
        assert_eq!(
            compare("$.ids", &expected, &serde_json::json!([1]), &variables).unwrap(),
            vec![Mismatch::new(
                "$.ids[1]".to_string(),
                source_info(),
                MismatchKind::MissingElement {
                    expected: Value::Number(Number::Integer(2)),
                },
            )]
        );
        assert_eq!(
            compare("$.ids", &expected, &serde_json::json!([]), &variables).unwrap(),
            vec![Mismatch::new(
                "$.ids[0]".to_string(),
                source_info(),
                MismatchKind::MissingElement {
                    expected: Value::Number(Number::Integer(1)),
                },
            )]
        );

        // Unexpected actual array element
        assert_eq!(
            compare(
                "$.ids",
                &expected,
                &serde_json::json!([1, 2, 3, 4]),
                &variables
            )
            .unwrap(),
            vec![Mismatch::new(
                "$.ids[2]".to_string(),
                source_info(),
                MismatchKind::UnexpectedElement {
                    actual: Value::Number(Number::Integer(3)),
                },
            )]
        );
    }

    #[test]
    fn test_compare_list_first_different_item() {
        let variables = VariableSet::new();
        let expected = json_list(vec![
            json_string_literal("Catherine"),
            json_string_literal("Thomas"),
            json_string_literal("Trevor"),
        ]);
        let actual = serde_json::json!(["Thomas", "Trevor"]);
        assert_eq!(
            compare("$.children", &expected, &actual, &variables).unwrap(),
            vec![Mismatch::new(
                "$.children[0]".to_string(),
                source_info(),
                MismatchKind::Value {
                    actual: Value::String("Thomas".to_string()),
                    expected: Value::String("Catherine".to_string()),
                },
            )]
        );
    }

    #[test]
    fn test_compare_list_of_objects() {
        let variables = VariableSet::new();
        let expected = json_object(vec![(
            "users",
            json_list(vec![
                json_object(vec![("name", json_string_literal("Alice"))]),
                json_object(vec![("name", json_string_literal("Bob"))]),
            ]),
        )]);
        let actual = serde_json::json!({"users": [{"name": "Alice"}, {"name": "Bobby"}]});
        assert_eq!(
            compare("$", &expected, &actual, &variables).unwrap(),
            vec![Mismatch::new(
                "$.users[1].name".to_string(),
                source_info(),
                MismatchKind::Value {
                    actual: Value::String("Bobby".to_string()),
                    expected: Value::String("Bob".to_string()),
                },
            )]
        );
    }

    #[test]
    fn test_compare_type_mismatch() {
        let variables = VariableSet::new();

        // Expected object, actual list
        let expected = json_object(vec![("id", json_number("1"))]);
        assert_eq!(
            compare("$", &expected, &serde_json::json!([1]), &variables).unwrap(),
            vec![Mismatch::new(
                "$".to_string(),
                source_info(),
                MismatchKind::Value {
                    actual: Value::List(vec![Value::Number(Number::Integer(1))]),
                    expected: Value::Object(vec![(
                        "id".to_string(),
                        Value::Number(Number::Integer(1))
                    )]),
                },
            )]
        );

        // Expected list, actual object
        let expected = json_list(vec![json_number("1")]);
        assert_eq!(
            compare("$", &expected, &serde_json::json!({"id": 1}), &variables).unwrap(),
            vec![Mismatch::new(
                "$".to_string(),
                source_info(),
                MismatchKind::Value {
                    actual: Value::Object(vec![(
                        "id".to_string(),
                        Value::Number(Number::Integer(1))
                    )]),
                    expected: Value::List(vec![Value::Number(Number::Integer(1))]),
                },
            )]
        );
    }

    fn json_string_literal(value: &str) -> JsonValue {
        json_string(vec![TemplateElement::String {
            value: value.to_string(),
            source: value.to_source(),
        }])
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
