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
use std::cmp::Ordering;
use std::collections::HashMap;

use hurl_core::ast::*;
use regex;

use crate::runner::core::{Error, PredicateResult};
use crate::runner::predicate_value::{eval_predicate_value, eval_predicate_value_template};
use crate::runner::template::eval_template;
use crate::runner::value::Value;
use crate::runner::RunnerError;

/// Evaluates a `predicate` against an actual `value`.
///
/// The predicate is a test with an expected value. The expected value (contained in the `predicate`
/// struct) can use a set of `variables`.
///
/// For instance, in the following Hurl assert:
///
/// ```hurl
///  jsonpath "$.books[0].name" startsWith "Dune"
/// ```
/// The predicate is `startsWith "Dune"`, a value could be "Foo".
/// With variables, we can have:
///
/// ```hurl
///  jsonpath "$.books[0].name" startsWith "{{name}}"
/// ```
///
/// In this case, the predicate is `startsWith "{{name}}"`.
pub fn eval_predicate(
    predicate: &Predicate,
    variables: &HashMap<String, Value>,
    value: &Option<Value>,
) -> PredicateResult {
    let assert_result = eval_predicate_func(&predicate.predicate_func, variables, value.as_ref())?;

    let source_info = SourceInfo::new(
        predicate.space0.source_info.start.line,
        0,
        predicate.space0.source_info.start.line,
        0,
    );

    if assert_result.type_mismatch {
        let not = if predicate.not { "not " } else { "" };
        let expected = format!("{}{}", not, assert_result.expected);
        Err(Error {
            source_info,
            inner: RunnerError::AssertFailure {
                actual: assert_result.actual,
                expected,
                type_mismatch: true,
            },
            assert: true,
        })
    } else if predicate.not && assert_result.success {
        Err(Error {
            source_info,
            inner: RunnerError::AssertFailure {
                actual: assert_result.actual,
                expected: format!("not {}", assert_result.expected),
                type_mismatch: false,
            },
            assert: true,
        })
    } else if !predicate.not && !assert_result.success {
        Err(Error {
            source_info,
            inner: RunnerError::AssertFailure {
                actual: assert_result.actual,
                expected: assert_result.expected,
                type_mismatch: false,
            },
            assert: true,
        })
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AssertResult {
    pub success: bool,
    pub type_mismatch: bool,
    pub actual: String,
    pub expected: String,
}

impl Value {
    pub fn display(&self) -> String {
        match self {
            Value::Bool(v) => format!("bool <{v}>"),
            Value::Date(v) => format!("date <{v}>"),
            Value::Integer(v) => format!("int <{v}>"),
            Value::String(v) => format!("string <{v}>"),
            Value::Float(f) => format!("float <{}>", format_float(*f)),
            Value::List(values) => format!(
                "[{}]",
                values
                    .iter()
                    .map(|v| v.display())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Nodeset(n) => format!("nodeset of size <{n}>"),
            Value::Object(_) => "object".to_string(),
            Value::Bytes(value) => format!("byte array <{}>", hex::encode(value)),
            Value::Null => "null".to_string(),
            Value::Unit => "unit".to_string(),
            Value::Regex(value) => format!("regex <{}>", value.as_str()),
        }
    }
}

impl Value {
    fn expected(&self) -> String {
        match self {
            Value::Bool(value) => format!("bool <{value}>"),
            Value::Bytes(values) => format!("list of size {}", values.len()),
            Value::Date(value) => format!("date <{value}>"),
            Value::Float(f) => format!("float <{}>", format_float(*f)),
            Value::Integer(value) => format!("integer <{value}>"),
            Value::List(value) => format!("list of size {}", value.len()),
            Value::Nodeset(size) => format!("list of size {size}"),
            Value::Null => "null".to_string(),
            Value::Object(values) => format!("list of size {}", values.len()),
            Value::String(value) => format!("string <{value}>"),
            Value::Unit => "something".to_string(),
            Value::Regex(value) => format!("regex <{value}>"),
        }
    }
}

fn format_float(value: f64) -> String {
    if value.fract() < f64::EPSILON {
        format!("{value}.0")
    } else {
        value.to_string()
    }
}

/// Returns a message formatting an expected value `predicate_func_value`, given
/// a set of `variables`, when there is no actual value.
fn expected_no_value(
    predicate_func_value: &PredicateFuncValue,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match &predicate_func_value {
        PredicateFuncValue::Equal { value, .. } | PredicateFuncValue::NotEqual { value, .. } => {
            let value = eval_predicate_value(value, variables)?;
            Ok(value.expected())
        }
        PredicateFuncValue::GreaterThan { value, .. } => {
            let value = eval_predicate_value(value, variables)?;
            Ok(format!("greater than <{}>", value.expected()))
        }
        PredicateFuncValue::GreaterThanOrEqual { value, .. } => {
            let value = eval_predicate_value(value, variables)?;
            Ok(format!("greater than or equals to <{}>", value.expected()))
        }
        PredicateFuncValue::LessThan { value, .. } => {
            let value = eval_predicate_value(value, variables)?;
            Ok(format!("less than <{}>", value.expected()))
        }
        PredicateFuncValue::LessThanOrEqual { value, .. } => {
            let value = eval_predicate_value(value, variables)?;
            Ok(format!("less than or equals to <{}>", value.expected()))
        }
        PredicateFuncValue::StartWith {
            value: expected, ..
        } => {
            let expected = eval_predicate_value_template(expected, variables)?;
            Ok(format!("starts with string <{expected}>"))
        }
        PredicateFuncValue::EndWith {
            value: expected, ..
        } => {
            let expected = eval_predicate_value_template(expected, variables)?;
            Ok(format!("ends with string <{expected}>"))
        }
        PredicateFuncValue::Contain {
            value: expected, ..
        } => {
            let expected = eval_predicate_value_template(expected, variables)?;
            Ok(format!("contains string <{expected}>"))
        }
        PredicateFuncValue::Include { value, .. } => {
            let value = eval_predicate_value(value, variables)?;
            Ok(format!("include {}", value.expected()))
        }
        PredicateFuncValue::Match {
            value: expected, ..
        } => {
            let expected = eval_predicate_value_template(expected, variables)?;
            Ok(format!("matches regex <{expected}>"))
        }
        PredicateFuncValue::IsInteger => Ok("integer".to_string()),
        PredicateFuncValue::IsFloat => Ok("float".to_string()),
        PredicateFuncValue::IsBoolean => Ok("boolean".to_string()),
        PredicateFuncValue::IsString => Ok("string".to_string()),
        PredicateFuncValue::IsCollection => Ok("collection".to_string()),
        PredicateFuncValue::IsDate => Ok("date".to_string()),
        PredicateFuncValue::Exist => Ok("something".to_string()),
        PredicateFuncValue::IsEmpty => Ok("empty".to_string()),
    }
}

/// Evaluates a `predicate_func` against an actual `value`.
/// The `predicate_func` is a test with an expected value. The expected value can
/// use a set of `variables`.
fn eval_predicate_func(
    predicate_func: &PredicateFunc,
    variables: &HashMap<String, Value>,
    value: Option<&Value>,
) -> Result<AssertResult, Error> {
    let value = match value {
        Some(value) => value,
        None => {
            let expected = expected_no_value(&predicate_func.value, variables)?;
            return Ok(AssertResult {
                success: false,
                actual: "none".to_string(),
                expected,
                type_mismatch: false,
            });
        }
    };

    match &predicate_func.value {
        PredicateFuncValue::Equal {
            value: expected, ..
        } => eval_equal(expected, variables, value),
        PredicateFuncValue::NotEqual {
            value: expected, ..
        } => eval_not_equal(expected, variables, value),
        PredicateFuncValue::GreaterThan {
            value: expected, ..
        } => eval_greater_than(expected, variables, value),
        PredicateFuncValue::GreaterThanOrEqual {
            value: expected, ..
        } => eval_greater_than_or_equal(expected, variables, value),
        PredicateFuncValue::LessThan {
            value: expected, ..
        } => eval_less_than(expected, variables, value),
        PredicateFuncValue::LessThanOrEqual {
            value: expected, ..
        } => eval_less_than_or_equal(expected, variables, value),
        PredicateFuncValue::StartWith {
            value: expected, ..
        } => eval_start_with(expected, variables, value),
        PredicateFuncValue::EndWith {
            value: expected, ..
        } => eval_end_with(expected, variables, value),
        PredicateFuncValue::Contain {
            value: expected, ..
        } => eval_contain(expected, variables, value),
        PredicateFuncValue::Include {
            value: expected, ..
        } => eval_include(expected, variables, value),
        PredicateFuncValue::Match {
            value: expected, ..
        } => eval_match(expected, &predicate_func.source_info, variables, value),
        PredicateFuncValue::IsInteger => eval_is_integer(value),
        PredicateFuncValue::IsFloat => eval_is_float(value),
        PredicateFuncValue::IsBoolean => eval_is_boolean(value),
        PredicateFuncValue::IsString => eval_is_string(value),
        PredicateFuncValue::IsCollection => eval_is_collection(value),
        PredicateFuncValue::IsDate => eval_is_date(value),
        PredicateFuncValue::Exist => eval_exist(value),
        PredicateFuncValue::IsEmpty => eval_is_empty(value),
    }
}

/// Evaluates if an `expected` value (using a `variables` set) is equal to an `actual` value.
fn eval_equal(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    Ok(assert_values_equal(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is not equal to an `actual` value.
fn eval_not_equal(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    Ok(assert_values_not_equal(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is greater than an `actual` value.
fn eval_greater_than(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    Ok(assert_values_greater(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is greater than or equal to an `actual` value.
fn eval_greater_than_or_equal(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    Ok(assert_values_greater_or_equal(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is less than an `actual` value.
fn eval_less_than(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    Ok(assert_values_less(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is less than an `actual` value.
fn eval_less_than_or_equal(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    Ok(assert_values_less_or_equal(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) starts with an `actual` value.
/// This predicate works with string and bytes.
fn eval_start_with(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    let expected_display = format!("starts with {}", expected.display());
    let actual_display = actual.display();
    match (expected, actual) {
        (Value::String(expected), Value::String(actual)) => Ok(AssertResult {
            success: actual.as_str().starts_with(expected.as_str()),
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        (Value::Bytes(expected), Value::Bytes(actual)) => Ok(AssertResult {
            success: actual.starts_with(&expected),
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        _ => Ok(AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: true,
        }),
    }
}

/// Evaluates if an `expected` value (using a `variables` set) ends with an `actual` value.
/// This predicate works with string and bytes.
fn eval_end_with(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    let expected_display = format!("ends with {}", expected.display());
    let actual_display = actual.display();
    match (expected, actual) {
        (Value::String(expected), Value::String(actual)) => Ok(AssertResult {
            success: actual.as_str().ends_with(expected.as_str()),
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        (Value::Bytes(expected), Value::Bytes(actual)) => Ok(AssertResult {
            success: actual.ends_with(&expected),
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        _ => Ok(AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: true,
        }),
    }
}

/// Evaluates if an `expected` value (using a `variables` set) contains an `actual` value.
/// This predicate works with string and bytes.
fn eval_contain(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    let expected_display = format!("contains {}", expected.display());
    let actual_display = actual.display();
    match (expected, actual) {
        (Value::String(expected), Value::String(actual)) => Ok(AssertResult {
            success: actual.as_str().contains(expected.as_str()),
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        (Value::Bytes(expected), Value::Bytes(actual)) => Ok(AssertResult {
            success: contains(actual.as_slice(), expected.as_slice()),
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        _ => Ok(AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: true,
        }),
    }
}

/// Evaluates if an `expected` value (using a `variables` set) includes an `actual` value.
/// This predicate works with list (maybe we should merge it with `eval_contains`?)
fn eval_include(
    expected: &PredicateValue,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let expected = eval_predicate_value(expected, variables)?;
    Ok(assert_include(actual, &expected))
}

/// Evaluates if an `expected` regex (using a `variables` set) matches an `actual` value.
fn eval_match(
    expected: &PredicateValue,
    source_info: &SourceInfo,
    variables: &HashMap<String, Value>,
    actual: &Value,
) -> Result<AssertResult, Error> {
    let regex = match expected {
        PredicateValue::String(template) => {
            let expected = eval_template(template, variables)?;
            match regex::Regex::new(expected.as_str()) {
                Ok(re) => re,
                Err(_) => {
                    return Err(Error {
                        source_info: source_info.clone(),
                        inner: RunnerError::InvalidRegex,
                        assert: false,
                    });
                }
            }
        }
        PredicateValue::Regex(regex) => regex.inner.clone(),
        _ => panic!("expect a string predicate value"), // should have failed in parsing
    };
    let actual_display = actual.display();
    let expected_display = format!("matches regex <{regex}>");
    match actual {
        Value::String(value) => Ok(AssertResult {
            success: regex.is_match(value.as_str()),
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        _ => Ok(AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: true,
        }),
    }
}

/// Evaluates if an `actual` value is an integer.
fn eval_is_integer(actual: &Value) -> Result<AssertResult, Error> {
    Ok(AssertResult {
        success: matches!(actual, Value::Integer(_)),
        actual: actual.display(),
        expected: "integer".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a float.
fn eval_is_float(actual: &Value) -> Result<AssertResult, Error> {
    Ok(AssertResult {
        success: matches!(actual, Value::Float(_)),
        actual: actual.display(),
        expected: "float".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a boolean.
fn eval_is_boolean(actual: &Value) -> Result<AssertResult, Error> {
    Ok(AssertResult {
        success: matches!(actual, Value::Bool(_)),
        actual: actual.display(),
        expected: "boolean".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a string.
fn eval_is_string(actual: &Value) -> Result<AssertResult, Error> {
    Ok(AssertResult {
        success: matches!(actual, Value::String(_)),
        actual: actual.display(),
        expected: "string".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a collection.
fn eval_is_collection(actual: &Value) -> Result<AssertResult, Error> {
    Ok(AssertResult {
        success: matches!(actual, Value::Bytes(_))
            || matches!(actual, Value::List(_))
            || matches!(actual, Value::Nodeset(_))
            || matches!(actual, Value::Object(_)),
        actual: actual.display(),
        expected: "collection".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a date.
fn eval_is_date(actual: &Value) -> Result<AssertResult, Error> {
    Ok(AssertResult {
        success: matches!(actual, Value::Date(_)),
        actual: actual.display(),
        expected: "date".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value exists.
fn eval_exist(actual: &Value) -> Result<AssertResult, Error> {
    let actual_display = actual.display();
    let expected_display = "something".to_string();
    match actual {
        Value::Nodeset(0) => Ok(AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        _ => Ok(AssertResult {
            success: true,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
    }
}

/// Evaluates if an `actual` is empty.
fn eval_is_empty(actual: &Value) -> Result<AssertResult, Error> {
    let expected_display = "count equals to 0".to_string();
    match actual {
        Value::List(values) => Ok(AssertResult {
            success: values.is_empty(),
            actual: format!("count equals to {}", values.len()),
            expected: expected_display,
            type_mismatch: false,
        }),
        Value::String(data) => Ok(AssertResult {
            success: data.is_empty(),
            actual: format!("count equals to {}", data.len()),
            expected: expected_display,
            type_mismatch: false,
        }),
        Value::Nodeset(count) => Ok(AssertResult {
            success: *count == 0,
            actual: format!("count equals to {count}"),
            expected: expected_display,
            type_mismatch: false,
        }),
        Value::Object(props) => Ok(AssertResult {
            success: props.is_empty(),
            actual: format!("count equals to {}", props.len()),
            expected: expected_display,
            type_mismatch: false,
        }),
        Value::Bytes(data) => Ok(AssertResult {
            success: data.is_empty(),
            actual: format!("count equals to {}", data.len()),
            expected: expected_display,
            type_mismatch: false,
        }),
        _ => Ok(AssertResult {
            success: false,
            actual: actual.display(),
            expected: expected_display,
            type_mismatch: true,
        }),
    }
}

fn assert_values_equal(actual: &Value, expected: &Value) -> AssertResult {
    let actual_display = actual.display();
    let expected_display = expected.display();
    match (actual, expected) {
        (Value::Null, Value::Null) => AssertResult {
            success: true,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Bool(value1), Value::Bool(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Integer(value1), Value::Integer(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Float(f), Value::Integer(i)) => AssertResult {
            success: (f.trunc() - *i as f64).abs() < f64::EPSILON && f.fract() == 0.0,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Integer(i), Value::Float(f)) => AssertResult {
            success: (f.trunc() - *i as f64).abs() < f64::EPSILON && f.fract() == 0.0,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Float(f1), Value::Float(f2)) => AssertResult {
            success: (f1 - f2).abs() < f64::EPSILON,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::String(value1), Value::String(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::List(value1), Value::List(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Bytes(value1), Value::Bytes(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        // FIXME: why case (UNIT UNIT) is not treated?
        (Value::Unit, _) => AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: true,
        },
        _ => AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            // FIXME: why type_mismatch is not true here?
            type_mismatch: false,
        },
    }
}

fn assert_values_not_equal(actual: &Value, expected: &Value) -> AssertResult {
    let actual_display = actual.display();
    let expected_display = expected.display();
    match (actual, expected) {
        (Value::Null, Value::Null) => AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Bool(value1), Value::Bool(value2)) => AssertResult {
            success: value1 != value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Integer(value1), Value::Integer(value2)) => AssertResult {
            success: value1 != value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Float(f), Value::Integer(i)) => AssertResult {
            success: (f.trunc() - *i as f64).abs() > f64::EPSILON || f.fract() != 0.0,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Integer(i), Value::Float(f)) => AssertResult {
            success: (f.trunc() - *i as f64).abs() > f64::EPSILON || f.fract() != 0.0,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Float(f1), Value::Float(f2)) => AssertResult {
            success: (f1 - f2).abs() > f64::EPSILON,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::String(value1), Value::String(value2)) => AssertResult {
            success: value1 != value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::List(value1), Value::List(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Bytes(value1), Value::Bytes(value2)) => AssertResult {
            success: value1 != value2,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
        (Value::Unit, _) => AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: true,
        },
        _ => AssertResult {
            success: true,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        },
    }
}

fn assert_values_greater(actual_value: &Value, expected_value: &Value) -> AssertResult {
    let actual = actual_value.display();
    let expected = format!("greater than {}", expected_value.display());

    match compare_values(actual_value, expected_value) {
        Some(1) => AssertResult {
            success: true,
            actual,
            expected,
            type_mismatch: false,
        },
        Some(0) | Some(-1) => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: false,
        },
        _ => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: true,
        },
    }
}

fn assert_values_greater_or_equal(actual_value: &Value, expected_value: &Value) -> AssertResult {
    let actual = actual_value.display();
    let expected = format!("greater or equal than {}", expected_value.display());
    match compare_values(actual_value, expected_value) {
        Some(1) | Some(0) => AssertResult {
            success: true,
            actual,
            expected,
            type_mismatch: false,
        },
        Some(-1) => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: false,
        },
        _ => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: true,
        },
    }
}

fn assert_values_less(actual_value: &Value, expected_value: &Value) -> AssertResult {
    let actual = actual_value.display();
    let expected = format!("less than {}", expected_value.display());
    match compare_values(actual_value, expected_value) {
        Some(-1) => AssertResult {
            success: true,
            actual,
            expected,
            type_mismatch: false,
        },
        Some(0) | Some(1) => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: false,
        },
        _ => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: true,
        },
    }
}

fn assert_values_less_or_equal(actual_value: &Value, expected_value: &Value) -> AssertResult {
    let actual = actual_value.display();
    let expected = format!("less or equal than {}", expected_value.display());
    match compare_values(actual_value, expected_value) {
        Some(-1) | Some(0) => AssertResult {
            success: true,
            actual,
            expected,
            type_mismatch: false,
        },
        Some(1) => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: false,
        },
        _ => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: true,
        },
    }
}

/// Compares `actual` and `expected`.
///
/// Returns
/// - `Some(-1)` if `actual` > `expected`,
/// - `Some(0)` if `actual` = `expected`
/// - `Some(-1)` if `actual` = `expected`
/// - `None` if `actual` and `expected` are not comparable.
fn compare_values(actual: &Value, expected: &Value) -> Option<i32> {
    match (actual, expected) {
        (Value::Integer(i1), Value::Integer(i2)) => Some(compare_float(*i1 as f64, *i2 as f64)),
        (Value::Float(f1), Value::Float(f2)) => Some(compare_float(*f1, *f2)),
        (Value::Float(f1), Value::Integer(i2)) => Some(compare_float(*f1, *i2 as f64)),
        (Value::Integer(i1), Value::Float(f2)) => Some(compare_float(*i1 as f64, *f2)),
        (Value::String(s1), Value::String(s2)) => Some(compare_string(s1, s2)),
        _ => None,
    }
}

/// Compares strings `s1` and `s2` lexicographically.
///
/// Returns
/// - `-1` if `s1` > `s2`,
/// - `0` if `s1` = `s2`
/// - `-1` if `s1` = `s2`
fn compare_string(s1: &str, s2: &str) -> i32 {
    let order = s1.cmp(s2);
    match order {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Compares floats `f1` and `f2`.
///
/// Returns
/// - `-1` if `f1` > `f2`,
/// - `0` if `f1` = `f2`
/// - `-1` if `f1` = `f2`
fn compare_float(f1: f64, f2: f64) -> i32 {
    if f1 > f2 {
        1
    } else if f1 < f2 {
        -1
    } else {
        0
    }
}

fn assert_include(value: &Value, element: &Value) -> AssertResult {
    let expected = format!("includes {}", element.display());
    match value {
        Value::List(values) => {
            let mut success = false;
            for v in values {
                let result = assert_values_equal(v, element);
                if result.success {
                    success = true;
                    break;
                }
            }
            AssertResult {
                success,
                actual: value.display(),
                expected,
                type_mismatch: false,
            }
        }
        _ => AssertResult {
            success: false,
            actual: value.display(),
            expected,
            type_mismatch: true,
        },
    }
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::{AssertResult, *};

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(0, 0, 0, 0),
        }
    }

    #[test]
    fn test_contains() {
        let haystack = [1, 2, 3];
        assert!(contains(&haystack, &[1]));
        assert!(contains(&haystack, &[1, 2]));
        assert!(!contains(&haystack, &[1, 3]));
    }

    #[test]
    fn test_predicate() {
        // `not == 10` with value `1`     OK
        // `not == 10` with value `10`    ValueError
        // `not == 10` with value `true`  => this is now valid
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(1, 1, 0, 0),
        };

        let predicate = Predicate {
            not: true,
            space0: whitespace.clone(),
            predicate_func: PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Integer(10),
                    operator: false,
                },
                source_info: SourceInfo::new(1, 11, 1, 12),
            },
        };

        assert!(eval_predicate(&predicate, &variables, &Some(Value::Bool(true))).is_ok());

        let error = eval_predicate(&predicate, &variables, &Some(Value::Integer(10))).unwrap_err();
        assert_eq!(
            error.inner,
            RunnerError::AssertFailure {
                actual: "int <10>".to_string(),
                expected: "not int <10>".to_string(),
                type_mismatch: false,
            }
        );
        assert_eq!(error.source_info, SourceInfo::new(1, 0, 1, 0));

        assert!(eval_predicate(&predicate, &variables, &Some(Value::Integer(1))).is_ok());
    }

    #[test]
    fn test_predicate_type_mismatch() {
        let variables = HashMap::new();

        // predicate: `== 10`
        // value: true
        let expected = PredicateValue::Integer(10);
        let value = Value::Bool(true);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(!assert_result.success);
        // FIXME: should be type_mismatch = true here
        // assert!(assert_result.type_mismatch);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <true>");
        assert_eq!(assert_result.expected.as_str(), "int <10>");
    }

    #[test]
    fn test_predicate_type_mismatch_with_unit() {
        let variables = HashMap::new();

        // predicate: `== 10`
        // value: Unit
        let expected = PredicateValue::Integer(10);
        let value = Value::Unit;
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(!assert_result.success);
        assert!(assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "unit");
        assert_eq!(assert_result.expected.as_str(), "int <10>");
    }

    #[test]
    fn test_predicate_value_error() {
        let variables = HashMap::new();

        // predicate: `== 10`
        // value: 1
        let expected = PredicateValue::Integer(10);
        let value = Value::Integer(1);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "int <1>");
        assert_eq!(assert_result.expected.as_str(), "int <10>");

        // predicate: `== true`
        // value: false
        let expected = PredicateValue::Bool(true);
        let value = Value::Bool(false);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <false>");
        assert_eq!(assert_result.expected.as_str(), "bool <true>");

        // predicate: `== 1.2`
        // value: 1.1
        let expected = PredicateValue::Float(Float {
            value: 1.2,
            encoded: "1.2".to_string(),
        });
        let value = Value::Float(1.1);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "float <1.1>");
        assert_eq!(assert_result.expected.as_str(), "float <1.2>");
    }

    #[test]
    fn test_predicate_exist() {
        let variables = HashMap::new();

        // predicate: `exist`
        // value: Some(Unit) | None
        let pred_func = PredicateFunc {
            value: PredicateFuncValue::Exist,
            source_info: SourceInfo::new(0, 0, 0, 0),
        };

        let value = Some(&Value::Unit);
        let assert_result = eval_predicate_func(&pred_func, &variables, value).unwrap();
        assert!(assert_result.success);
        assert_eq!(assert_result.actual.as_str(), "unit");
        assert_eq!(assert_result.expected.as_str(), "something");

        let value = None;
        let assert_result = eval_predicate_func(&pred_func, &variables, value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "none");
        assert_eq!(assert_result.expected.as_str(), "something");
    }

    #[test]
    fn test_predicate_value_equals_integers() {
        let variables = HashMap::new();

        // predicate: `== 1`
        // value: 1
        let expected = PredicateValue::Integer(1);
        let value = Value::Integer(1);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "int <1>");
        assert_eq!(assert_result.expected.as_str(), "int <1>");
    }

    #[test]
    fn test_predicate_value_equals_booleans() {
        let variables = HashMap::new();

        // predicate: `== false`
        // value: false
        let expected = PredicateValue::Bool(false);
        let value = Value::Bool(false);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <false>");
        assert_eq!(assert_result.expected.as_str(), "bool <false>");

        // predicate: `== true`
        // value: false
        let expected = PredicateValue::Bool(true);
        let value = Value::Bool(false);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <false>");
        assert_eq!(assert_result.expected.as_str(), "bool <true>");

        // predicate: `== true`
        // value: true
        let expected = PredicateValue::Bool(true);
        let value = Value::Bool(true);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <true>");
        assert_eq!(assert_result.expected.as_str(), "bool <true>");
    }

    #[test]
    fn test_predicate_value_equals_floats() {
        let variables = HashMap::new();

        // predicate: `== 1.1`
        // value: 1.1
        let expected = PredicateValue::Float(Float {
            value: 1.1,
            encoded: "1.1".to_string(),
        });
        let value = Value::Float(1.1);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "float <1.1>");
        assert_eq!(assert_result.expected.as_str(), "float <1.1>");
    }

    #[test]
    fn test_predicate_value_equals_float_integer() {
        let variables = HashMap::new();

        // predicate: `== 1`
        // value: 1.0
        let expected = PredicateValue::Integer(1);
        let value = Value::Float(1.0);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "float <1.0>");
        assert_eq!(assert_result.expected.as_str(), "int <1>");
    }

    #[test]
    fn test_predicate_value_not_equals() {
        let variables = HashMap::new();

        // predicate: `== 1`
        // value: 2
        let expected = PredicateValue::Integer(1);
        let value = Value::Integer(2);
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "int <2>");
        assert_eq!(assert_result.expected.as_str(), "int <1>");
    }

    #[test]
    fn test_predicate_value_equals_string() {
        let variables = HashMap::new();

        // {{base_url}}
        let template = Template {
            delimiter: Some('"'),
            elements: vec![TemplateElement::Expression(Expr {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(1, 11, 1, 11),
                },
                variable: Variable {
                    name: String::from("base_url"),
                    source_info: SourceInfo::new(1, 11, 1, 19),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(1, 19, 1, 19),
                },
            })],
            source_info: SourceInfo::new(1, 1, 1, 1),
        };

        // predicate: `== "{{base_url}}"`
        // value: "http://localhost:8000"
        // base_url is not defined
        let expected = PredicateValue::String(template.clone());
        let value = Value::String(String::from("http://localhost:8000"));
        let error = eval_equal(&expected, &variables, &value).unwrap_err();
        assert_eq!(
            error.inner,
            RunnerError::TemplateVariableNotDefined {
                name: String::from("base_url")
            }
        );
        assert_eq!(error.source_info, SourceInfo::new(1, 11, 1, 19));

        // predicate: `== "{{base_url}}"`
        // value: "http://localhost:8000"
        // variables: base_url=http://localhost:8080
        let mut variables = HashMap::new();
        variables.insert(
            String::from("base_url"),
            Value::String(String::from("http://localhost:8000")),
        );
        let assert_result = eval_equal(&expected, &variables, &value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(
            assert_result.actual.as_str(),
            "string <http://localhost:8000>"
        );
        assert_eq!(
            assert_result.expected.as_str(),
            "string <http://localhost:8000>"
        );
    }

    #[test]
    fn test_compare_float() {
        assert_eq!(compare_float(2.3, 1.2), 1);
        assert_eq!(compare_float(2.3, 2.2), 1);
        assert_eq!(compare_float(2.3, -4.2), 1);
        assert_eq!(compare_float(2.3, 2.3), 0);
        assert_eq!(compare_float(2.3, 3.2), -1);
        assert_eq!(compare_float(2.3, 2.4), -1);
    }

    #[test]
    fn test_compare_numbers() {
        // 2 integers
        assert_eq!(
            compare_values(&Value::Integer(2), &Value::Integer(1)).unwrap(),
            1
        );
        assert_eq!(
            compare_values(&Value::Integer(1), &Value::Integer(1)).unwrap(),
            0
        );
        assert_eq!(
            compare_values(&Value::Integer(1), &Value::Integer(2)).unwrap(),
            -1
        );
        assert_eq!(
            compare_values(&Value::Integer(-1), &Value::Integer(-2)).unwrap(),
            1
        );

        // 2 floats
        assert_eq!(
            compare_values(&Value::Float(2.3), &Value::Float(1.2)).unwrap(),
            1
        );
        assert_eq!(
            compare_values(&Value::Float(2.3), &Value::Float(2.2)).unwrap(),
            1
        );
        assert_eq!(
            compare_values(&Value::Float(1.2), &Value::Float(1.5)).unwrap(),
            -1
        );
        assert_eq!(
            compare_values(&Value::Float(-2.1), &Value::Float(-3.1)).unwrap(),
            1
        );
        assert_eq!(
            compare_values(&Value::Float(1.1), &Value::Float(-2.1)).unwrap(),
            1
        );
        assert_eq!(
            compare_values(&Value::Float(1.1), &Value::Float(1.1)).unwrap(),
            0
        );

        // 1 float and 1 integer
        assert_eq!(
            compare_values(&Value::Float(2.3), &Value::Integer(2)).unwrap(),
            1
        );
        assert_eq!(
            compare_values(&Value::Float(2.3), &Value::Integer(3)).unwrap(),
            -1
        );
        assert_eq!(
            compare_values(&Value::Float(2.0), &Value::Integer(2)).unwrap(),
            0
        );

        // 1 integer and 1 float
        assert_eq!(
            compare_values(&Value::Integer(2), &Value::Float(2.0)).unwrap(),
            0
        );

        // with a non number
        assert!(compare_values(&Value::Integer(-1), &Value::String("hello".to_string())).is_none());
    }

    #[test]
    fn test_compare_strings() {
        assert_eq!(
            compare_values(
                &Value::String("foo".to_string()),
                &Value::String("bar".to_string())
            )
            .unwrap(),
            1
        );
        assert_eq!(
            compare_values(
                &Value::String("bar".to_string()),
                &Value::String("foo".to_string())
            )
            .unwrap(),
            -1
        );
        assert_eq!(
            compare_values(
                &Value::String("foo".to_string()),
                &Value::String("foo".to_string())
            )
            .unwrap(),
            0
        );
        assert_eq!(
            compare_values(
                &Value::String("foo".to_string()),
                &Value::String("FOO".to_string())
            )
            .unwrap(),
            1
        );
        assert_eq!(
            compare_values(
                &Value::String("foobar".to_string()),
                &Value::String("foo".to_string())
            )
            .unwrap(),
            1
        );
    }

    #[test]
    fn test_assert_value_greater() {
        assert_eq!(
            assert_values_greater(&Value::Integer(2), &Value::Integer(1)),
            AssertResult {
                success: true,
                type_mismatch: false,
                actual: "int <2>".to_string(),
                expected: "greater than int <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(&Value::Integer(1), &Value::Integer(1)),
            AssertResult {
                success: false,
                type_mismatch: false,
                actual: "int <1>".to_string(),
                expected: "greater than int <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(&Value::Float(1.1), &Value::Integer(1)),
            AssertResult {
                success: true,
                type_mismatch: false,
                actual: "float <1.1>".to_string(),
                expected: "greater than int <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(&Value::Float(1.1), &Value::Integer(2)),
            AssertResult {
                success: false,
                type_mismatch: false,
                actual: "float <1.1>".to_string(),
                expected: "greater than int <2>".to_string(),
            }
        );
    }

    #[test]
    fn test_predicate_is_empty_are_false() {
        // predicate: `isEmpty`
        // value: [1]
        let value = Value::List(vec![Value::Integer(1)]);
        let assert_result = eval_is_empty(&value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "count equals to 1");
        assert_eq!(assert_result.expected.as_str(), "count equals to 0");

        // predicate: `isEmpty`
        // value: Nodeset(12)
        let value = Value::Nodeset(12);
        let assert_result = eval_is_empty(&value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "count equals to 12");
        assert_eq!(assert_result.expected.as_str(), "count equals to 0");
    }

    #[test]
    fn test_predicate_is_empty_are_true() {
        // predicate: `isEmpty`
        // value: [1]
        let value = Value::List(vec![]);
        let assert_result = eval_is_empty(&value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "count equals to 0");
        assert_eq!(assert_result.expected.as_str(), "count equals to 0");

        // predicate: `isEmpty`
        // value: Nodeset(0)
        let value = Value::Nodeset(0);
        let assert_result = eval_is_empty(&value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "count equals to 0");
        assert_eq!(assert_result.expected.as_str(), "count equals to 0");
    }

    #[test]
    fn test_predicate_type() {
        // predicate: `isInteger`
        // value: 1
        let value = Value::Integer(1);
        let assert_result = eval_is_integer(&value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "int <1>");
        assert_eq!(assert_result.expected.as_str(), "integer");

        // predicate: `isInteger`
        // value: 1
        let value = Value::Float(1.0);
        let assert_result = eval_is_integer(&value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "float <1.0>");
        assert_eq!(assert_result.expected.as_str(), "integer");
    }

    #[test]
    fn test_predicate_not_with_different_types() {
        // equals predicate does not generate a type error with an integer value
        // predicate: `not == null`
        // value: 1
        let predicate = Predicate {
            not: true,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(0, 0, 0, 0),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    operator: false,
                    value: PredicateValue::Null,
                },
            },
        };

        let variables = HashMap::new();
        assert!(eval_predicate(&predicate, &variables, &Some(Value::Integer(1))).is_ok());

        // startswith predicate generates a type error with an integer value
        // predicate: `not startWith "toto"`
        // value: 1
        let predicate = Predicate {
            not: true,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(0, 0, 0, 0),
                value: PredicateFuncValue::StartWith {
                    space0: whitespace(),
                    value: PredicateValue::String(Template {
                        delimiter: None,
                        elements: vec![TemplateElement::String {
                            value: "toto".to_string(),
                            encoded: "toto".to_string(),
                        }],
                        source_info: SourceInfo::new(0, 0, 0, 0),
                    }),
                },
            },
        };
        let error = eval_predicate(&predicate, &variables, &Some(Value::Integer(1))).unwrap_err();
        assert_eq!(
            error.inner,
            RunnerError::AssertFailure {
                actual: "int <1>".to_string(),
                expected: "not starts with string <toto>".to_string(),
                type_mismatch: true,
            }
        );
    }

    #[test]
    fn test_date_predicate() {
        // predicate: `isDate`
        // value: 2002-06-16T10:10:10
        let value = Value::Date(
            chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2002, 6, 16, 10, 10, 10).unwrap(),
        );
        let assert_result = eval_is_date(&value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(
            assert_result.actual.as_str(),
            "date <2002-06-16 10:10:10 UTC>"
        );
        assert_eq!(assert_result.expected.as_str(), "date");

        // predicate: `isDate`
        // value: "toto"
        let value = Value::String("toto".to_string());
        let assert_result = eval_is_date(&value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "string <toto>");
        assert_eq!(assert_result.expected.as_str(), "date");
    }

    #[test]
    fn test_no_type_mismatch_with_none_value() {
        let variables = HashMap::new();

        // predicate: `== null`
        let predicate = Predicate {
            not: false,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(0, 0, 0, 0),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    value: PredicateValue::Null,
                    operator: false,
                },
            },
        };

        let error = eval_predicate(&predicate, &variables, &None).err().unwrap();
        assert_eq!(
            error.inner,
            RunnerError::AssertFailure {
                actual: "none".to_string(),
                expected: "null".to_string(),
                type_mismatch: false,
            }
        );

        // predicate: `not == null`
        let predicate = Predicate {
            not: true,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(0, 0, 0, 0),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    operator: false,
                    value: PredicateValue::Null,
                },
            },
        };

        let variables = HashMap::new();
        assert!(eval_predicate(&predicate, &variables, &None).is_ok());
    }

    #[test]
    fn test_predicate_match() {
        let variables = HashMap::new();

        // predicate: `matches /a{3}/`
        // value: aa
        let expected = PredicateValue::Regex(Regex {
            inner: regex::Regex::new(r#"a{3}"#).unwrap(),
        });
        let value = Value::String("aa".to_string());
        let source_info = SourceInfo::new(0, 0, 0, 0);
        let assert_result = eval_match(&expected, &source_info, &variables, &value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "string <aa>");
        assert_eq!(assert_result.expected.as_str(), "matches regex <a{3}>");
    }
}
