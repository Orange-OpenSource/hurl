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

use hurl_core::ast::{Predicate, PredicateFunc, PredicateFuncValue, PredicateValue, SourceInfo};
use hurl_core::reader::Pos;

use crate::runner::error::RunnerError;
use crate::runner::predicate_value::{eval_predicate_value, eval_predicate_value_template};
use crate::runner::result::PredicateResult;
use crate::runner::value::{EvalError, Value};
use crate::runner::{Number, RunnerErrorKind, VariableSet};
use crate::util::path::ContextDir;

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
    variables: &VariableSet,
    value: &Option<Value>,
    context_dir: &ContextDir,
) -> PredicateResult {
    let assert_result = eval_predicate_func(
        &predicate.predicate_func,
        variables,
        value.as_ref(),
        context_dir,
    )?;

    // Column error is set to 0 to disable the error display of "^^^"
    let source_info = SourceInfo::new(
        Pos::new(predicate.space0.source_info.start.line, 0),
        Pos::new(predicate.space0.source_info.start.line, 0),
    );

    if assert_result.type_mismatch {
        let not = if predicate.not { "not " } else { "" };
        let expected = format!("{}{}", not, assert_result.expected);
        let kind = RunnerErrorKind::AssertFailure {
            actual: assert_result.actual,
            expected,
            type_mismatch: true,
        };
        Err(RunnerError::new(source_info, kind, true))
    } else if predicate.not && assert_result.success {
        let kind = RunnerErrorKind::AssertFailure {
            actual: assert_result.actual,
            expected: format!("not {}", assert_result.expected),
            type_mismatch: false,
        };
        Err(RunnerError::new(source_info, kind, true))
    } else if !predicate.not && !assert_result.success {
        let kind = RunnerErrorKind::AssertFailure {
            actual: assert_result.actual,
            expected: assert_result.expected,
            type_mismatch: false,
        };
        Err(RunnerError::new(source_info, kind, true))
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
    fn format(&self) -> String {
        match self {
            Value::Bool(value) => format!("boolean <{value}>"),
            Value::Bytes(values) => format!("list of size {}", values.len()),
            Value::Date(value) => format!("date <{value}>"),
            Value::List(value) => format!("list of size {}", value.len()),
            Value::Nodeset(size) => format!("list of size {size}"),
            Value::Null => "null".to_string(),
            Value::Number(number) => number.expected(),
            Value::Object(values) => format!("list of size {}", values.len()),
            Value::Regex(value) => format!("regex <{value}>"),
            Value::String(value) => format!("string <{value}>"),
            Value::Unit => "something".to_string(),
        }
    }
}

impl Number {
    fn expected(&self) -> String {
        match self {
            Number::Float(f) => format!("float <{}>", format_float(*f)),
            Number::Integer(value) => format!("integer <{value}>"),
            Number::BigInteger(s) => format!("number <{s}>"),
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
    variables: &VariableSet,
    context_dir: &ContextDir,
) -> Result<String, RunnerError> {
    match &predicate_func_value {
        PredicateFuncValue::Equal { value, .. } | PredicateFuncValue::NotEqual { value, .. } => {
            let value = eval_predicate_value(value, variables, context_dir)?;
            Ok(value.format())
        }
        PredicateFuncValue::GreaterThan { value, .. } => {
            let value = eval_predicate_value(value, variables, context_dir)?;
            Ok(format!("greater than <{}>", value.format()))
        }
        PredicateFuncValue::GreaterThanOrEqual { value, .. } => {
            let value = eval_predicate_value(value, variables, context_dir)?;
            Ok(format!("greater than or equals to <{}>", value.format()))
        }
        PredicateFuncValue::LessThan { value, .. } => {
            let value = eval_predicate_value(value, variables, context_dir)?;
            Ok(format!("less than <{}>", value.format()))
        }
        PredicateFuncValue::LessThanOrEqual { value, .. } => {
            let value = eval_predicate_value(value, variables, context_dir)?;
            Ok(format!("less than or equals to <{}>", value.format()))
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
            let value = eval_predicate_value(value, variables, context_dir)?;
            Ok(format!("include {}", value.format()))
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
        PredicateFuncValue::IsIsoDate => Ok("date".to_string()),
        PredicateFuncValue::Exist => Ok("something".to_string()),
        PredicateFuncValue::IsEmpty => Ok("empty".to_string()),
        PredicateFuncValue::IsNumber => Ok("number".to_string()),
    }
}

/// Evaluates a `predicate_func` against an actual `value`.
/// The `predicate_func` is a test with an expected value. The expected value can
/// use a set of `variables`.
fn eval_predicate_func(
    predicate_func: &PredicateFunc,
    variables: &VariableSet,
    value: Option<&Value>,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let value = match value {
        Some(value) => value,
        None => {
            let expected = expected_no_value(&predicate_func.value, variables, context_dir)?;
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
        } => eval_equal(expected, variables, value, context_dir),
        PredicateFuncValue::NotEqual {
            value: expected, ..
        } => eval_not_equal(expected, variables, value, context_dir),
        PredicateFuncValue::GreaterThan {
            value: expected, ..
        } => eval_greater_than(expected, variables, value, context_dir),
        PredicateFuncValue::GreaterThanOrEqual {
            value: expected, ..
        } => eval_greater_than_or_equal(expected, variables, value, context_dir),
        PredicateFuncValue::LessThan {
            value: expected, ..
        } => eval_less_than(expected, variables, value, context_dir),
        PredicateFuncValue::LessThanOrEqual {
            value: expected, ..
        } => eval_less_than_or_equal(expected, variables, value, context_dir),
        PredicateFuncValue::StartWith {
            value: expected, ..
        } => eval_start_with(expected, variables, value, context_dir),
        PredicateFuncValue::EndWith {
            value: expected, ..
        } => eval_end_with(expected, variables, value, context_dir),
        PredicateFuncValue::Contain {
            value: expected, ..
        } => eval_contain(expected, variables, value, context_dir),
        PredicateFuncValue::Include {
            value: expected, ..
        } => eval_include(expected, variables, value, context_dir),
        PredicateFuncValue::Match {
            value: expected, ..
        } => eval_match(
            expected,
            predicate_func.source_info,
            variables,
            value,
            context_dir,
        ),
        PredicateFuncValue::IsInteger => eval_is_integer(value),
        PredicateFuncValue::IsFloat => eval_is_float(value),
        PredicateFuncValue::IsBoolean => eval_is_boolean(value),
        PredicateFuncValue::IsString => eval_is_string(value),
        PredicateFuncValue::IsCollection => eval_is_collection(value),
        PredicateFuncValue::IsDate => eval_is_date(value),
        PredicateFuncValue::IsIsoDate => eval_is_iso_date(value),
        PredicateFuncValue::Exist => eval_exist(value),
        PredicateFuncValue::IsEmpty => eval_is_empty(value),
        PredicateFuncValue::IsNumber => eval_is_number(value),
    }
}

/// Evaluates if an `expected` value (using a `variables` set) is equal to an `actual` value.
fn eval_equal(
    expected: &PredicateValue,
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    Ok(assert_values_equal(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is not equal to an `actual` value.
fn eval_not_equal(
    expected: &PredicateValue,
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    Ok(assert_values_not_equal(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is greater than an `actual` value.
fn eval_greater_than(
    expected: &PredicateValue,
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    Ok(assert_values_greater(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is greater than or equal to an `actual` value.
fn eval_greater_than_or_equal(
    expected: &PredicateValue,
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    Ok(assert_values_greater_or_equal(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is less than an `actual` value.
fn eval_less_than(
    expected: &PredicateValue,
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    Ok(assert_values_less(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) is less than an `actual` value.
fn eval_less_than_or_equal(
    expected: &PredicateValue,
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    Ok(assert_values_less_or_equal(actual, &expected))
}

/// Evaluates if an `expected` value (using a `variables` set) starts with an `actual` value.
/// This predicate works with string and bytes.
fn eval_start_with(
    expected: &PredicateValue,
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    let expected_display = format!("starts with {}", expected.repr());
    let actual_display = actual.repr();
    match actual.starts_with(&expected) {
        Ok(success) => Ok(AssertResult {
            success,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        Err(_) => Ok(AssertResult {
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
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    let expected_display = format!("ends with {}", expected.repr());
    let actual_display = actual.repr();
    match actual.ends_with(&expected) {
        Ok(success) => Ok(AssertResult {
            success,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        Err(_) => Ok(AssertResult {
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
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    let expected_display = format!("contains {}", expected.repr());
    let actual_display = actual.repr();
    match actual.contains(&expected) {
        Ok(success) => Ok(AssertResult {
            success,
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
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    Ok(assert_include(actual, &expected))
}

/// Evaluates if an `expected` regex (using a `variables` set) matches an `actual` value.
fn eval_match(
    expected: &PredicateValue,
    source_info: SourceInfo,
    variables: &VariableSet,
    actual: &Value,
    context_dir: &ContextDir,
) -> Result<AssertResult, RunnerError> {
    let expected = eval_predicate_value(expected, variables, context_dir)?;
    let actual_display = actual.repr();
    let expected_display = format!("matches regex <{expected}>");
    match actual.is_match(&expected) {
        Ok(success) => Ok(AssertResult {
            success,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: false,
        }),
        Err(EvalError::Type) => Ok(AssertResult {
            success: false,
            actual: actual_display,
            expected: expected_display,
            type_mismatch: true,
        }),
        Err(EvalError::InvalidRegex) => Err(RunnerError::new(
            source_info,
            RunnerErrorKind::InvalidRegex,
            false,
        )),
    }
}

/// Evaluates if an `actual` value is an integer.
fn eval_is_integer(actual: &Value) -> Result<AssertResult, RunnerError> {
    Ok(AssertResult {
        success: actual.is_integer(),
        actual: actual.repr(),
        expected: "integer".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a float.
fn eval_is_float(actual: &Value) -> Result<AssertResult, RunnerError> {
    Ok(AssertResult {
        success: actual.is_float(),
        actual: actual.repr(),
        expected: "float".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a boolean.
fn eval_is_boolean(actual: &Value) -> Result<AssertResult, RunnerError> {
    Ok(AssertResult {
        success: actual.is_boolean(),
        actual: actual.repr(),
        expected: "boolean".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a string.
fn eval_is_string(actual: &Value) -> Result<AssertResult, RunnerError> {
    Ok(AssertResult {
        success: actual.is_string(),
        actual: actual.repr(),
        expected: "string".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a collection.
fn eval_is_collection(actual: &Value) -> Result<AssertResult, RunnerError> {
    Ok(AssertResult {
        success: actual.is_collection(),
        actual: actual.repr(),
        expected: "collection".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if an `actual` value is a date.
fn eval_is_date(actual: &Value) -> Result<AssertResult, RunnerError> {
    Ok(AssertResult {
        success: actual.is_date(),
        actual: actual.repr(),
        expected: "date".to_string(),
        type_mismatch: false,
    })
}

/// Evaluates if `actual` is a string representing a RFC339 date (format YYYY-MM-DDTHH:mm:ss.sssZ).
///
/// [`eval_is_date`] performs type check (is the input of [`Value::Date`]), whereas [`eval_is_iso_date`]
/// checks if a string conforms to a certain date-time format.
fn eval_is_iso_date(actual: &Value) -> Result<AssertResult, RunnerError> {
    match actual.is_iso_date() {
        Ok(success) => Ok(AssertResult {
            success,
            actual: actual.to_string(),
            expected: "string with format YYYY-MM-DDTHH:mm:ss.sssZ".to_string(),
            type_mismatch: false,
        }),
        _ => Ok(AssertResult {
            success: false,
            actual: actual.repr(),
            expected: "string".to_string(),
            type_mismatch: true,
        }),
    }
}

/// Evaluates if an `actual` value exists.
fn eval_exist(actual: &Value) -> Result<AssertResult, RunnerError> {
    let actual_display = actual.repr();
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
fn eval_is_empty(actual: &Value) -> Result<AssertResult, RunnerError> {
    let expected_display = "count equals to 0".to_string();
    match actual.count() {
        Ok(count) => {
            let actual_display = format!("count equals to {count}");
            Ok(AssertResult {
                success: count == 0,
                actual: actual_display,
                expected: expected_display,
                type_mismatch: false,
            })
        }
        _ => Ok(AssertResult {
            success: false,
            actual: actual.repr(),
            expected: expected_display,
            type_mismatch: true,
        }),
    }
}

/// Evaluates if an `actual` value is a number.
fn eval_is_number(actual: &Value) -> Result<AssertResult, RunnerError> {
    Ok(AssertResult {
        success: actual.is_number(),
        actual: actual.repr(),
        expected: "number".to_string(),
        type_mismatch: false,
    })
}

fn assert_values_equal(actual: &Value, expected: &Value) -> AssertResult {
    let success = actual == expected;
    let actual = actual.repr();
    let expected = expected.repr();
    let type_mismatch = false;
    AssertResult {
        success,
        actual,
        expected,
        type_mismatch,
    }
}

fn assert_values_not_equal(actual: &Value, expected: &Value) -> AssertResult {
    let success = actual != expected;
    let actual = actual.repr();
    let expected = expected.repr();
    let type_mismatch = false;
    AssertResult {
        success,
        actual,
        expected,
        type_mismatch,
    }
}

fn assert_values_greater(actual_value: &Value, expected_value: &Value) -> AssertResult {
    let actual = actual_value.repr();
    let expected = format!("greater than {}", expected_value.repr());

    match actual_value.compare(expected_value) {
        Ok(ordering) => AssertResult {
            success: ordering == Ordering::Greater,
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
    let actual = actual_value.repr();
    let expected = format!("greater or equal than {}", expected_value.repr());
    match actual_value.compare(expected_value) {
        Ok(ordering) => AssertResult {
            success: ordering == Ordering::Greater || ordering == Ordering::Equal,
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
    let actual = actual_value.repr();
    let expected = format!("less than {}", expected_value.repr());
    match actual_value.compare(expected_value) {
        Ok(ordering) => AssertResult {
            success: ordering == Ordering::Less,
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
    let actual = actual_value.repr();
    let expected = format!("less or equal than {}", expected_value.repr());
    match actual_value.compare(expected_value) {
        Ok(ordering) => AssertResult {
            success: ordering == Ordering::Less || ordering == Ordering::Equal,
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

fn assert_include(value: &Value, element: &Value) -> AssertResult {
    let actual = value.repr();
    let expected = format!("includes {}", element.repr());
    match value.includes(element) {
        Ok(success) => AssertResult {
            success,
            actual,
            expected,
            type_mismatch: false,
        },
        Err(_) => AssertResult {
            success: false,
            actual,
            expected,
            type_mismatch: true,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use hurl_core::ast::{
        Expr, ExprKind, Float, Placeholder, Regex, Template, TemplateElement, Variable, Whitespace,
        I64,
    };
    use hurl_core::typing::ToSource;

    use super::{AssertResult, *};

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    #[test]
    fn test_predicate() {
        // `not == 10` with value `1`     OK
        // `not == 10` with value `10`    ValueError
        // `not == 10` with value `true`  => this is now valid
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(0, 0)),
        };

        let predicate = Predicate {
            not: true,
            space0: whitespace.clone(),
            predicate_func: PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Number(hurl_core::ast::Number::Integer(I64::new(
                        10,
                        "10".to_source(),
                    ))),
                },
                source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 12)),
            },
        };

        assert!(eval_predicate(
            &predicate,
            &variables,
            &Some(Value::Bool(true)),
            &context_dir
        )
        .is_ok());

        let error = eval_predicate(
            &predicate,
            &variables,
            &Some(Value::Number(Number::Integer(10))),
            &context_dir,
        )
        .unwrap_err();
        assert_eq!(
            error.kind,
            RunnerErrorKind::AssertFailure {
                actual: "integer <10>".to_string(),
                expected: "not integer <10>".to_string(),
                type_mismatch: false,
            }
        );
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 0), Pos::new(1, 0))
        );

        assert!(eval_predicate(
            &predicate,
            &variables,
            &Some(Value::Number(Number::Integer(1))),
            &context_dir
        )
        .is_ok());
    }

    #[test]
    fn test_predicate_type_mismatch() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== 10`
        // value: true
        let expected = PredicateValue::Number(hurl_core::ast::Number::Integer(I64::new(
            10,
            "10".to_source(),
        )));
        let value = Value::Bool(true);
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(!assert_result.success);
        // FIXME: should be type_mismatch = true here
        // assert!(assert_result.type_mismatch);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "boolean <true>");
        assert_eq!(assert_result.expected, "integer <10>");
    }

    #[test]
    fn test_predicate_type_mismatch_with_unit() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== 10`
        // value: Unit
        let expected = PredicateValue::Number(hurl_core::ast::Number::Integer(I64::new(
            10,
            "10".to_source(),
        )));
        let value = Value::Unit;
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "unit");
        assert_eq!(assert_result.expected, "integer <10>");
    }

    #[test]
    fn test_predicate_value_error() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== 10`
        // value: 1
        let expected = PredicateValue::Number(hurl_core::ast::Number::Integer(I64::new(
            10,
            "10".to_source(),
        )));
        let value = Value::Number(Number::Integer(1));
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "integer <1>");
        assert_eq!(assert_result.expected, "integer <10>");

        // predicate: `== true`
        // value: false
        let expected = PredicateValue::Bool(true);
        let value = Value::Bool(false);
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "boolean <false>");
        assert_eq!(assert_result.expected, "boolean <true>");

        // predicate: `== 1.2`
        // value: 1.1
        let expected = PredicateValue::Number(hurl_core::ast::Number::Float(Float::new(
            1.2,
            "1.2".to_source(),
        )));
        let value = Value::Number(Number::Float(1.1));
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "float <1.1>");
        assert_eq!(assert_result.expected, "float <1.2>");
    }

    #[test]
    fn test_predicate_exist() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `exist`
        // value: Some(Unit) | None
        let pred_func = PredicateFunc {
            value: PredicateFuncValue::Exist,
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };

        let value = Some(&Value::Unit);
        let assert_result =
            eval_predicate_func(&pred_func, &variables, value, &context_dir).unwrap();
        assert!(assert_result.success);
        assert_eq!(assert_result.actual.as_str(), "unit");
        assert_eq!(assert_result.expected.as_str(), "something");

        let value = None;
        let assert_result =
            eval_predicate_func(&pred_func, &variables, value, &context_dir).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "none");
        assert_eq!(assert_result.expected, "something");
    }

    #[test]
    fn test_predicate_value_equals_integers() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== 1`
        // value: 1
        let expected = PredicateValue::Number(hurl_core::ast::Number::Integer(I64::new(
            1,
            "1".to_source(),
        )));
        let value = Value::Number(Number::Integer(1));
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "integer <1>");
        assert_eq!(assert_result.expected, "integer <1>");
    }

    #[test]
    fn test_predicate_value_equals_booleans() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== false`
        // value: false
        let expected = PredicateValue::Bool(false);
        let value = Value::Bool(false);
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "boolean <false>");
        assert_eq!(assert_result.expected, "boolean <false>");

        // predicate: `== true`
        // value: false
        let expected = PredicateValue::Bool(true);
        let value = Value::Bool(false);
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "boolean <false>");
        assert_eq!(assert_result.expected, "boolean <true>");

        // predicate: `== true`
        // value: true
        let expected = PredicateValue::Bool(true);
        let value = Value::Bool(true);
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "boolean <true>");
        assert_eq!(assert_result.expected, "boolean <true>");
    }

    #[test]
    fn test_predicate_value_equals_floats() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== 1.1`
        // value: 1.1
        let expected = PredicateValue::Number(hurl_core::ast::Number::Float(Float::new(
            1.1,
            "1.1".to_source(),
        )));
        let value = Value::Number(Number::Float(1.1));
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "float <1.1>");
        assert_eq!(assert_result.expected, "float <1.1>");
    }

    #[test]
    fn test_predicate_value_equals_float_integer() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== 1`
        // value: 1.0
        let expected = PredicateValue::Number(hurl_core::ast::Number::Integer(I64::new(
            1,
            "1".to_source(),
        )));
        let value = Value::Number(Number::Float(1.0));
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "float <1.0>");
        assert_eq!(assert_result.expected, "integer <1>");
    }

    #[test]
    fn test_predicate_value_not_equals() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== 1`
        // value: 2
        let expected = PredicateValue::Number(hurl_core::ast::Number::Integer(I64::new(
            1,
            "1".to_source(),
        )));
        let value = Value::Number(Number::Integer(2));
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "integer <2>");
        assert_eq!(assert_result.expected, "integer <1>");
    }

    #[test]
    fn test_predicate_value_equals_string() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // {{base_url}}
        let template = Template {
            delimiter: Some('"'),
            elements: vec![TemplateElement::Placeholder(Placeholder {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 11)),
                },
                expr: Expr {
                    kind: ExprKind::Variable(Variable {
                        name: "base_url".to_string(),
                        source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 19)),
                    }),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 19)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 19), Pos::new(1, 19)),
                },
            })],
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
        };

        // predicate: `== "{{base_url}}"`
        // value: "http://localhost:8000"
        // base_url is not defined
        let expected = PredicateValue::String(template.clone());
        let value = Value::String(String::from("http://localhost:8000"));
        let error = eval_equal(&expected, &variables, &value, &context_dir).unwrap_err();
        assert_eq!(
            error.kind,
            RunnerErrorKind::TemplateVariableNotDefined {
                name: String::from("base_url")
            }
        );
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 11), Pos::new(1, 19))
        );

        // predicate: `== "{{base_url}}"`
        // value: "http://localhost:8000"
        // variables: base_url=http://localhost:8080
        let mut variables = VariableSet::new();
        variables
            .insert(
                String::from("base_url"),
                Value::String(String::from("http://localhost:8000")),
            )
            .unwrap();
        let assert_result = eval_equal(&expected, &variables, &value, &context_dir).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "string <http://localhost:8000>");
        assert_eq!(assert_result.expected, "string <http://localhost:8000>");
    }

    #[test]
    fn test_assert_value_greater() {
        assert_eq!(
            assert_values_greater(
                &Value::Number(Number::Integer(2)),
                &Value::Number(Number::Integer(1))
            ),
            AssertResult {
                success: true,
                type_mismatch: false,
                actual: "integer <2>".to_string(),
                expected: "greater than integer <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(
                &Value::Number(Number::Integer(1)),
                &Value::Number(Number::Integer(1))
            ),
            AssertResult {
                success: false,
                type_mismatch: false,
                actual: "integer <1>".to_string(),
                expected: "greater than integer <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(
                &Value::Number(Number::Float(1.1)),
                &Value::Number(Number::Integer(1))
            ),
            AssertResult {
                success: true,
                type_mismatch: false,
                actual: "float <1.1>".to_string(),
                expected: "greater than integer <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(
                &Value::Number(Number::Float(1.1)),
                &Value::Number(Number::Integer(2))
            ),
            AssertResult {
                success: false,
                type_mismatch: false,
                actual: "float <1.1>".to_string(),
                expected: "greater than integer <2>".to_string(),
            }
        );
    }

    #[test]
    fn test_predicate_is_empty_are_false() {
        // predicate: `isEmpty`
        // value: [1]
        let value = Value::List(vec![Value::Number(Number::Integer(1))]);
        let assert_result = eval_is_empty(&value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "count equals to 1");
        assert_eq!(assert_result.expected, "count equals to 0");

        // predicate: `isEmpty`
        // value: Nodeset(12)
        let value = Value::Nodeset(12);
        let assert_result = eval_is_empty(&value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "count equals to 12");
        assert_eq!(assert_result.expected, "count equals to 0");
    }

    #[test]
    fn test_predicate_is_empty_are_true() {
        // predicate: `isEmpty`
        // value: [1]
        let value = Value::List(vec![]);
        let assert_result = eval_is_empty(&value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "count equals to 0");
        assert_eq!(assert_result.expected, "count equals to 0");

        // predicate: `isEmpty`
        // value: Nodeset(0)
        let value = Value::Nodeset(0);
        let assert_result = eval_is_empty(&value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "count equals to 0");
        assert_eq!(assert_result.expected, "count equals to 0");
    }

    #[test]
    fn test_predicate_type() {
        // predicate: `isInteger`
        // value: 1
        let value = Value::Number(Number::Integer(1));
        let assert_result = eval_is_integer(&value).unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "integer <1>");
        assert_eq!(assert_result.expected, "integer");

        // predicate: `isInteger`
        // value: 1
        let value = Value::Number(Number::Float(1.0));
        let assert_result = eval_is_integer(&value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "float <1.0>");
        assert_eq!(assert_result.expected, "integer");
    }

    #[test]
    fn test_predicate_not_with_different_types() {
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // equals predicate does not generate a type error with an integer value
        // predicate: `not == null`
        // value: 1
        let predicate = Predicate {
            not: true,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    value: PredicateValue::Null,
                },
            },
        };

        let variables = VariableSet::new();
        assert!(eval_predicate(
            &predicate,
            &variables,
            &Some(Value::Number(Number::Integer(1))),
            &context_dir
        )
        .is_ok());

        // startswith predicate generates a type error with an integer value
        // predicate: `not startWith "toto"`
        // value: 1
        let predicate = Predicate {
            not: true,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                value: PredicateFuncValue::StartWith {
                    space0: whitespace(),
                    value: PredicateValue::String(Template {
                        delimiter: None,
                        elements: vec![TemplateElement::String {
                            value: "toto".to_string(),
                            source: "toto".to_source(),
                        }],
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    }),
                },
            },
        };
        let error = eval_predicate(
            &predicate,
            &variables,
            &Some(Value::Number(Number::Integer(1))),
            &context_dir,
        )
        .unwrap_err();
        assert_eq!(
            error.kind,
            RunnerErrorKind::AssertFailure {
                actual: "integer <1>".to_string(),
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
        assert_eq!(assert_result.actual, "date <2002-06-16 10:10:10 UTC>");
        assert_eq!(assert_result.expected, "date");

        // predicate: `isDate`
        // value: "toto"
        let value = Value::String("toto".to_string());
        let assert_result = eval_is_date(&value).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "string <toto>");
        assert_eq!(assert_result.expected, "date");
    }

    #[test]
    fn test_no_type_mismatch_with_none_value() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `== null`
        let predicate = Predicate {
            not: false,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    value: PredicateValue::Null,
                },
            },
        };

        let error = eval_predicate(&predicate, &variables, &None, &context_dir)
            .err()
            .unwrap();
        assert_eq!(
            error.kind,
            RunnerErrorKind::AssertFailure {
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
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    value: PredicateValue::Null,
                },
            },
        };

        let variables = VariableSet::new();
        assert!(eval_predicate(&predicate, &variables, &None, &context_dir).is_ok());
    }

    #[test]
    fn test_predicate_match() {
        let variables = VariableSet::new();
        let current_dir = std::env::current_dir().unwrap();
        let file_root = Path::new("file_root");
        let context_dir = ContextDir::new(current_dir.as_path(), file_root);

        // predicate: `matches /a{3}/`
        // value: aa
        let expected = PredicateValue::Regex(Regex {
            inner: regex::Regex::new(r#"a{3}"#).unwrap(),
        });
        let value = Value::String("aa".to_string());
        let source_info = SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0));
        let assert_result =
            eval_match(&expected, source_info, &variables, &value, &context_dir).unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual, "string <aa>");
        assert_eq!(assert_result.expected, "matches regex </a{3}/>");
    }

    #[test]
    fn test_predicate_is_iso_date() {
        let value = Value::String("2020-03-09T22:18:26.625Z".to_string());
        let res = eval_is_iso_date(&value).unwrap();
        assert!(res.success);
        assert!(!res.type_mismatch);
        assert_eq!(res.actual, "2020-03-09T22:18:26.625Z");
        assert_eq!(res.expected, "string with format YYYY-MM-DDTHH:mm:ss.sssZ");
    }

    #[test]
    fn test_predicate_is_number() {
        let value = Value::Number(Number::Integer(1));
        let res = eval_is_number(&value).unwrap();
        assert!(res.success);
        assert!(!res.type_mismatch);
        assert_eq!(res.actual, "integer <1>");
        assert_eq!(res.expected, "number");

        let value = Value::Number(Number::Float(1.0));
        let res = eval_is_number(&value).unwrap();
        assert!(res.success);
        assert!(!res.type_mismatch);
        assert_eq!(res.actual, "float <1.0>");
        assert_eq!(res.expected, "number");
    }
}
