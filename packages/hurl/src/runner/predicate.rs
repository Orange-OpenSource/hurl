/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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
use std::collections::HashMap;

use regex;

use hurl_core::ast::*;

use super::core::Error;
use super::template::eval_template;
use super::value::Value;
use crate::runner::core::PredicateResult;
use crate::runner::predicate_value::eval_predicate_value;
use crate::runner::RunnerError;

pub fn eval_predicate(
    predicate: Predicate,
    variables: &HashMap<String, Value>,
    value: Option<Value>,
) -> PredicateResult {
    let assert_result = eval_predicate_func(predicate.predicate_func.clone(), variables, value)?;
    let source_info = SourceInfo {
        start: Pos {
            line: predicate.space0.source_info.start.line,
            column: 0,
        },
        end: Pos {
            line: predicate.space0.source_info.start.line,
            column: 0,
        },
    };
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
    pub fn display(self) -> String {
        match self {
            Value::Bool(v) => format!("bool <{}>", v),
            Value::Integer(v) => format!("int <{}>", v),
            Value::String(v) => format!("string <{}>", v),
            Value::Float(f) => format!("float <{}>", format_float(f)),
            Value::List(values) => format!(
                "[{}]",
                values
                    .iter()
                    .map(|v| v.clone().display())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Nodeset(n) => format!("nodeset of size <{}>", n),
            Value::Object(_) => "object".to_string(),
            Value::Bytes(value) => format!("byte array <{}>", hex::encode(value)),
            Value::Null => "null".to_string(),
            Value::Unit => "unit".to_string(),
            Value::Regex(value) => format!("regex <{}>", value.as_str()),
        }
    }
}

fn eval_predicate_func(
    predicate_func: PredicateFunc,
    variables: &HashMap<String, Value>,
    optional_value: Option<Value>,
) -> Result<AssertResult, Error> {
    match optional_value {
        None => {
            //let type_mismatch = !matches!(self.value, PredicateFuncValue::Exist {});
            Ok(AssertResult {
                success: false,
                actual: "none".to_string(),
                expected: expected(predicate_func, variables)?,
                type_mismatch: false,
            })
        }
        Some(value) => eval_something(predicate_func, variables, value),
    }
}

impl Value {
    pub fn expected(&self) -> String {
        match self {
            Value::Bool(value) => format!("bool <{}>", value),
            Value::Bytes(values) => format!("list of size {}", values.len()),
            Value::Float(f) => format!("float <{}>", format_float(*f)),
            Value::Integer(value) => format!("integer <{}>", value),
            Value::List(value) => format!("list of size {}", value.len()),
            Value::Nodeset(size) => format!("list of size {}", size),
            Value::Null => "null".to_string(),
            Value::Object(values) => format!("list of size {}", values.len()),
            Value::String(value) => format!("string <{}>", value),
            Value::Unit => "something".to_string(),
            Value::Regex(value) => format!("regex <{}>", value),
        }
    }
}

fn format_float(value: f64) -> String {
    if value.fract() < f64::EPSILON {
        format!("{}.0", value)
    } else {
        value.to_string()
    }
}

fn expected(
    predicate_func: PredicateFunc,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match predicate_func.value {
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

        PredicateFuncValue::CountEqual {
            value: expected, ..
        } => {
            let expected = if let PredicateValue::Integer(expected) = expected {
                expected
            } else {
                panic!();
            };
            Ok(format!("count equals to <{}>", expected))
        }
        PredicateFuncValue::StartWith {
            value: expected, ..
        } => {
            let expected = eval_predicate_value_template(expected, variables)?;
            Ok(format!("starts with string <{}>", expected))
        }
        PredicateFuncValue::EndWith {
            value: expected, ..
        } => {
            let expected = eval_predicate_value_template(expected, variables)?;
            Ok(format!("ends with string <{}>", expected))
        }
        PredicateFuncValue::Contain {
            value: expected, ..
        } => {
            let expected = eval_predicate_value_template(expected, variables)?;
            Ok(format!("contains string <{}>", expected))
        }
        PredicateFuncValue::Include { value, .. } => {
            let value = eval_predicate_value(value, variables)?;
            Ok(format!("include {}", value.expected()))
        }
        PredicateFuncValue::Match {
            value: expected, ..
        } => {
            let expected = eval_predicate_value_template(expected, variables)?;
            Ok(format!("matches regex <{}>", expected))
        }
        PredicateFuncValue::IsInteger {} => Ok("integer".to_string()),
        PredicateFuncValue::IsFloat {} => Ok("float".to_string()),
        PredicateFuncValue::IsBoolean {} => Ok("boolean".to_string()),
        PredicateFuncValue::IsString {} => Ok("string".to_string()),
        PredicateFuncValue::IsCollection {} => Ok("collection".to_string()),
        PredicateFuncValue::Exist {} => Ok("something".to_string()),
    }
}

pub fn eval_predicate_value_template(
    predicate_value: PredicateValue,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match predicate_value {
        PredicateValue::String(template) => eval_template(&template, variables),
        PredicateValue::Regex(regex) => Ok(regex.inner.to_string()),
        // All others value should have failed in parsing:
        _ => panic!("expect a string or a regex predicate value"),
    }
}

fn eval_something(
    predicate_func: PredicateFunc,
    variables: &HashMap<String, Value>,
    value: Value,
) -> Result<AssertResult, Error> {
    match predicate_func.value {
        PredicateFuncValue::Equal {
            value: expected, ..
        } => {
            let expected = eval_predicate_value(expected, variables)?;
            Ok(assert_values_equal(value, expected))
        }
        PredicateFuncValue::NotEqual {
            value: expected, ..
        } => {
            let expected = eval_predicate_value(expected, variables)?;
            Ok(assert_values_not_equal(value, expected))
        }
        PredicateFuncValue::GreaterThan {
            value: expected, ..
        } => {
            let expected = eval_predicate_value(expected, variables)?;
            Ok(assert_values_greater(value, expected))
        }
        PredicateFuncValue::GreaterThanOrEqual {
            value: expected, ..
        } => {
            let expected = eval_predicate_value(expected, variables)?;
            Ok(assert_values_greater_or_equal(value, expected))
        }
        PredicateFuncValue::LessThan {
            value: expected, ..
        } => {
            let expected = eval_predicate_value(expected, variables)?;
            Ok(assert_values_less(value, expected))
        }
        PredicateFuncValue::LessThanOrEqual {
            value: expected, ..
        } => {
            let expected = eval_predicate_value(expected, variables)?;
            Ok(assert_values_less_or_equal(value, expected))
        }

        // countEquals
        PredicateFuncValue::CountEqual {
            value: PredicateValue::Integer(expected_value),
            ..
        } => match value {
            Value::List(values) => Ok(AssertResult {
                success: values.len() as i64 == expected_value,
                actual: values.len().to_string(),
                expected: expected_value.to_string(),
                type_mismatch: false,
            }),
            Value::Nodeset(n) => Ok(AssertResult {
                success: n as i64 == expected_value,
                actual: n.to_string(),
                expected: expected_value.to_string(),
                type_mismatch: false,
            }),
            Value::Bytes(data) => Ok(AssertResult {
                success: data.len() as i64 == expected_value,
                actual: data.len().to_string(),
                expected: expected_value.to_string(),
                type_mismatch: false,
            }),
            _ => Ok(AssertResult {
                success: false,
                actual: value.clone().display(),
                expected: format!("count equals to <{}>", expected_value),
                type_mismatch: true,
            }),
        },

        // starts with string or bytes
        PredicateFuncValue::StartWith {
            value: expected, ..
        } => {
            let expected_value = eval_predicate_value(expected, variables)?;
            let expected = format!("starts with {}", expected_value.clone().display());
            match (expected_value, value.clone()) {
                (Value::String(s), Value::String(actual)) => Ok(AssertResult {
                    success: actual.as_str().starts_with(s.as_str()),
                    actual: value.display(),
                    expected,
                    type_mismatch: false,
                }),
                (Value::Bytes(bytes), Value::Bytes(actual)) => Ok(AssertResult {
                    success: actual.starts_with(&bytes),
                    actual: value.display(),
                    expected,
                    type_mismatch: false,
                }),
                _ => Ok(AssertResult {
                    success: false,
                    actual: value.display(),
                    expected,
                    type_mismatch: true,
                }),
            }
        }

        PredicateFuncValue::EndWith {
            value: expected, ..
        } => {
            let expected_value = eval_predicate_value(expected, variables)?;
            let expected = format!("ends with {}", expected_value.clone().display());
            match (expected_value, value.clone()) {
                (Value::String(s), Value::String(actual)) => Ok(AssertResult {
                    success: actual.as_str().ends_with(s.as_str()),
                    actual: value.display(),
                    expected,
                    type_mismatch: false,
                }),
                (Value::Bytes(bytes), Value::Bytes(actual)) => Ok(AssertResult {
                    success: actual.ends_with(&bytes),
                    actual: value.display(),
                    expected,
                    type_mismatch: false,
                }),
                _ => Ok(AssertResult {
                    success: false,
                    actual: value.display(),
                    expected,
                    type_mismatch: true,
                }),
            }
        }

        // contains
        PredicateFuncValue::Contain {
            value: expected, ..
        } => {
            let expected_value = eval_predicate_value(expected, variables)?;
            let expected = format!("contains {}", expected_value.clone().display());
            match (expected_value, value.clone()) {
                (Value::String(s), Value::String(actual)) => Ok(AssertResult {
                    success: actual.as_str().contains(s.as_str()),
                    actual: value.display(),
                    expected,
                    type_mismatch: false,
                }),
                (Value::Bytes(bytes), Value::Bytes(actual)) => Ok(AssertResult {
                    success: contains(actual.as_slice(), bytes.as_slice()),
                    actual: value.display(),
                    expected,
                    type_mismatch: false,
                }),
                _ => Ok(AssertResult {
                    success: false,
                    actual: value.display(),
                    expected,
                    type_mismatch: true,
                }),
            }
        }

        PredicateFuncValue::Include {
            value: expected, ..
        } => {
            let expected = eval_predicate_value(expected, variables)?;
            Ok(assert_include(value, expected))
        }

        PredicateFuncValue::Match {
            value: expected, ..
        } => {
            let regex = match expected {
                PredicateValue::String(template) => {
                    let expected = eval_template(&template, variables)?;
                    match regex::Regex::new(expected.as_str()) {
                        Ok(re) => re,
                        Err(_) => {
                            return Err(Error {
                                source_info: predicate_func.source_info.clone(),
                                inner: RunnerError::InvalidRegex(),
                                assert: false,
                            });
                        }
                    }
                }
                PredicateValue::Regex(regex) => regex.inner,
                _ => panic!("expect a string predicate value"), // should have failed in parsing
            };
            match value.clone() {
                Value::String(actual) => Ok(AssertResult {
                    success: regex.is_match(actual.as_str()),
                    actual: value.display(),
                    expected: format!("matches regex <{}>", regex),
                    type_mismatch: false,
                }),
                _ => Ok(AssertResult {
                    success: false,
                    actual: value.display(),
                    expected: format!("matches regex <{}>", regex),
                    type_mismatch: true,
                }),
            }
        }

        // types
        PredicateFuncValue::IsInteger {} => Ok(AssertResult {
            success: matches!(value, Value::Integer(_)),
            actual: value.display(),
            expected: "integer".to_string(),
            type_mismatch: false,
        }),
        PredicateFuncValue::IsFloat {} => Ok(AssertResult {
            success: matches!(value, Value::Float(_)),
            actual: value.display(),
            expected: "float".to_string(),
            type_mismatch: false,
        }),
        PredicateFuncValue::IsBoolean {} => Ok(AssertResult {
            success: matches!(value, Value::Bool(_)),
            actual: value.display(),
            expected: "boolean".to_string(),
            type_mismatch: false,
        }),
        PredicateFuncValue::IsString {} => Ok(AssertResult {
            success: matches!(value, Value::String(_)),
            actual: value.display(),
            expected: "string".to_string(),
            type_mismatch: false,
        }),
        PredicateFuncValue::IsCollection {} => Ok(AssertResult {
            success: matches!(value, Value::Bytes(_))
                || matches!(value, Value::List(_))
                || matches!(value, Value::Nodeset(_))
                || matches!(value, Value::Object(_)),
            actual: value.display(),
            expected: "collection".to_string(),
            type_mismatch: false,
        }),

        // exists
        PredicateFuncValue::Exist {} => match value {
            Value::Nodeset(0) => Ok(AssertResult {
                success: false,
                actual: value.display(),
                expected: "something".to_string(),
                type_mismatch: false,
            }),
            _ => Ok(AssertResult {
                success: true,
                actual: value.display(),
                expected: "something".to_string(),
                type_mismatch: false,
            }),
        },
        _ => panic!(),
    }
}

fn assert_values_equal(actual: Value, expected: Value) -> AssertResult {
    match (actual.clone(), expected.clone()) {
        (Value::Null {}, Value::Null {}) => AssertResult {
            success: true,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Bool(value1), Value::Bool(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Integer(value1), Value::Integer(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Float(f), Value::Integer(i)) => AssertResult {
            success: (f.trunc() - i as f64).abs() < f64::EPSILON && f.fract() == 0.0,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Integer(i), Value::Float(f)) => AssertResult {
            success: (f.trunc() - i as f64).abs() < f64::EPSILON && f.fract() == 0.0,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Float(f1), Value::Float(f2)) => AssertResult {
            success: (f1 - f2).abs() < f64::EPSILON,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::String(value1), Value::String(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::List(value1), Value::List(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Bytes(value1), Value::Bytes(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Unit, _) => AssertResult {
            success: false,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: true,
        },
        _ => AssertResult {
            success: false,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
    }
}

fn assert_values_not_equal(actual: Value, expected: Value) -> AssertResult {
    match (actual.clone(), expected.clone()) {
        (Value::Null {}, Value::Null {}) => AssertResult {
            success: false,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Bool(value1), Value::Bool(value2)) => AssertResult {
            success: value1 != value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Integer(value1), Value::Integer(value2)) => AssertResult {
            success: value1 != value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Float(f), Value::Integer(i)) => AssertResult {
            success: (f.trunc() - i as f64).abs() > f64::EPSILON || f.fract() != 0.0,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Integer(i), Value::Float(f)) => AssertResult {
            success: (f.trunc() - i as f64).abs() > f64::EPSILON || f.fract() != 0.0,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Float(f1), Value::Float(f2)) => AssertResult {
            success: (f1 - f2).abs() > f64::EPSILON,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::String(value1), Value::String(value2)) => AssertResult {
            success: value1 != value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::List(value1), Value::List(value2)) => AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Bytes(value1), Value::Bytes(value2)) => AssertResult {
            success: value1 != value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
        (Value::Unit, _) => AssertResult {
            success: false,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: true,
        },
        _ => AssertResult {
            success: true,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        },
    }
}

fn assert_values_greater(actual_value: Value, expected_value: Value) -> AssertResult {
    let actual = actual_value.clone().display();
    let expected = format!("greater than {}", expected_value.clone().display());
    match compare_numbers(actual_value, expected_value) {
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

fn assert_values_greater_or_equal(actual_value: Value, expected_value: Value) -> AssertResult {
    let actual = actual_value.clone().display();
    let expected = format!("greater or equal than {}", expected_value.clone().display());
    match compare_numbers(actual_value, expected_value) {
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

fn assert_values_less(actual_value: Value, expected_value: Value) -> AssertResult {
    let actual = actual_value.clone().display();
    let expected = format!("less than {}", expected_value.clone().display());
    match compare_numbers(actual_value, expected_value) {
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

fn assert_values_less_or_equal(actual_value: Value, expected_value: Value) -> AssertResult {
    let actual = actual_value.clone().display();
    let expected = format!("less or equal than {}", expected_value.clone().display());
    match compare_numbers(actual_value, expected_value) {
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

// return -1, 0 or 1
// none if one of the value is not a number
fn compare_numbers(actual: Value, expected: Value) -> Option<i32> {
    match (actual, expected) {
        (Value::Integer(i1), Value::Integer(i2)) => Some(compare_float(i1 as f64, i2 as f64)),
        (Value::Float(f1), Value::Float(f2)) => Some(compare_float(f1, f2)),
        (Value::Float(f1), Value::Integer(i2)) => Some(compare_float(f1, i2 as f64)),
        (Value::Integer(i1), Value::Float(f2)) => Some(compare_float(i1 as f64, f2)),
        _ => None,
    }
}

fn compare_float(f1: f64, f2: f64) -> i32 {
    if f1 > f2 {
        1
    } else if f1 < f2 {
        -1
    } else {
        0
    }
}

fn assert_include(value: Value, element: Value) -> AssertResult {
    let expected = format!("includes {}", element.clone().display());
    match value.clone() {
        Value::List(values) => {
            let mut success = false;
            for v in values {
                let result = assert_values_equal(v, element.clone());
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
    use super::AssertResult;
    use super::*;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
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
    fn test_invalid_xpath() {}

    #[test]
    fn test_predicate() {
        // not equals 10 with value 1     OK
        // not equals 10 with value 10    ValueError
        // not equals 10 with value true  => this is now valid
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(1, 1, 0, 0),
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
                source_info: SourceInfo::init(1, 11, 1, 12),
            },
        };

        assert!(eval_predicate(predicate.clone(), &variables, Some(Value::Bool(true))).is_ok());

        let error = eval_predicate(predicate.clone(), &variables, Some(Value::Integer(10)))
            .err()
            .unwrap();
        assert_eq!(
            error.inner,
            RunnerError::AssertFailure {
                actual: "int <10>".to_string(),
                expected: "not int <10>".to_string(),
                type_mismatch: false,
            }
        );
        assert_eq!(error.source_info, SourceInfo::init(1, 0, 1, 0));

        assert!(eval_predicate(predicate, &variables, Some(Value::Integer(1))).is_ok());
    }

    #[test]
    fn test_predicate_type_mismatch() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = eval_predicate_func(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Integer(10),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Some(Value::Bool(true)),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <true>");
        assert_eq!(assert_result.expected.as_str(), "int <10>");
    }

    #[test]
    fn test_predicate_type_mismatch_with_unit() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = eval_predicate_func(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Integer(10),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Some(Value::Unit),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "unit");
        assert_eq!(assert_result.expected.as_str(), "int <10>");
    }

    #[test]
    fn test_predicate_value_error() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace.clone(),
                    value: PredicateValue::Integer(10),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Integer(1),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "int <1>");
        assert_eq!(assert_result.expected.as_str(), "int <10>");

        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace.clone(),
                    value: PredicateValue::Bool(true),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Bool(false),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <false>");
        assert_eq!(assert_result.expected.as_str(), "bool <true>");

        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Float(Float {
                        value: 1.2,
                        encoded: "1.2".to_string(),
                    }),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Float(1.1),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "float <1.1>");
        assert_eq!(assert_result.expected.as_str(), "float <1.2>");
    }

    #[test]
    fn test_predicate_exist() {
        let variables = HashMap::new();
        let predicate_func = PredicateFunc {
            value: PredicateFuncValue::Exist {},
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let assert_result =
            eval_predicate_func(predicate_func.clone(), &variables, Some(Value::Unit)).unwrap();
        assert!(assert_result.success);
        assert_eq!(assert_result.actual.as_str(), "unit");
        assert_eq!(assert_result.expected.as_str(), "something");

        let assert_result = eval_predicate_func(predicate_func, &variables, None).unwrap();
        assert!(!assert_result.success);
        assert_eq!(assert_result.actual.as_str(), "none");
        assert_eq!(assert_result.expected.as_str(), "something");
    }

    #[test]
    fn test_predicate_value_equals_integers() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Integer(1),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Integer(1),
        )
        .unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "int <1>");
        assert_eq!(assert_result.expected.as_str(), "int <1>");
    }

    #[test]
    fn test_predicate_value_equals_booleans() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Bool(false),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Bool(false),
        )
        .unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <false>");
        assert_eq!(assert_result.expected.as_str(), "bool <false>");
    }

    #[test]
    fn test_predicate_value_equals_floats() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Float(Float {
                        value: 1.1,
                        encoded: "1.1".to_string(),
                    }),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Float(1.1),
        )
        .unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "float <1.1>");
        assert_eq!(assert_result.expected.as_str(), "float <1.1>");
    }

    #[test]
    fn test_predicate_value_equals_float_integer() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        // a float can be equals to an int (but the reverse)
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::Integer(1),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Float(1.0),
        )
        .unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "float <1.0>");
        assert_eq!(assert_result.expected.as_str(), "int <1>");
    }

    #[test]
    fn test_predicate_value_not_equals() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::NotEqual {
                    space0: whitespace,
                    value: PredicateValue::Integer(1),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Integer(2),
        )
        .unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "int <2>");
        assert_eq!(assert_result.expected.as_str(), "int <1>");
    }

    #[test]
    fn test_predicate_value_equals_string() {
        let mut variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let template = Template {
            quotes: true,
            elements: vec![TemplateElement::Expression(Expr {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 11, 1, 11),
                },
                variable: Variable {
                    name: String::from("base_url"),
                    source_info: SourceInfo::init(1, 11, 1, 19),
                },
                space1: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 19, 1, 19),
                },
            })],
            source_info: SourceInfo::init(1, 1, 1, 1),
        };

        let error = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace.clone(),
                    value: PredicateValue::String(template.clone()),
                    operator: false,
                },
                source_info: SourceInfo::init(1, 1, 1, 21),
            },
            &variables,
            Value::String(String::from("http://localhost:8000")),
        )
        .err()
        .unwrap();
        assert_eq!(
            error.inner,
            RunnerError::TemplateVariableNotDefined {
                name: String::from("base_url")
            }
        );
        assert_eq!(error.source_info, SourceInfo::init(1, 11, 1, 19));

        variables.insert(
            String::from("base_url"),
            Value::String(String::from("http://localhost:8000")),
        );
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Equal {
                    space0: whitespace,
                    value: PredicateValue::String(template),
                    operator: false,
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::String(String::from("http://localhost:8000")),
        )
        .unwrap();
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
            compare_numbers(Value::Integer(2), Value::Integer(1)).unwrap(),
            1
        );
        assert_eq!(
            compare_numbers(Value::Integer(1), Value::Integer(1)).unwrap(),
            0
        );
        assert_eq!(
            compare_numbers(Value::Integer(1), Value::Integer(2)).unwrap(),
            -1
        );
        assert_eq!(
            compare_numbers(Value::Integer(-1), Value::Integer(-2)).unwrap(),
            1
        );

        // 2 floats
        assert_eq!(
            compare_numbers(Value::Float(2.3), Value::Float(1.2)).unwrap(),
            1
        );
        assert_eq!(
            compare_numbers(Value::Float(2.3), Value::Float(2.2)).unwrap(),
            1
        );
        assert_eq!(
            compare_numbers(Value::Float(1.2), Value::Float(1.5)).unwrap(),
            -1
        );
        assert_eq!(
            compare_numbers(Value::Float(-2.1), Value::Float(-3.1)).unwrap(),
            1
        );
        assert_eq!(
            compare_numbers(Value::Float(1.1), Value::Float(-2.1)).unwrap(),
            1
        );
        assert_eq!(
            compare_numbers(Value::Float(1.1), Value::Float(1.1)).unwrap(),
            0
        );

        // 1 float and 1 integer
        assert_eq!(
            compare_numbers(Value::Float(2.3), Value::Integer(2)).unwrap(),
            1
        );
        assert_eq!(
            compare_numbers(Value::Float(2.3), Value::Integer(3)).unwrap(),
            -1
        );
        assert_eq!(
            compare_numbers(Value::Float(2.0), Value::Integer(2)).unwrap(),
            0
        );

        // 1 integer and 1 float
        assert_eq!(
            compare_numbers(Value::Integer(2), Value::Float(2.0)).unwrap(),
            0
        );

        // with a non number
        assert!(compare_numbers(Value::Integer(-1), Value::String("hello".to_string())).is_none());
    }

    #[test]
    fn test_assert_value_greater() {
        assert_eq!(
            assert_values_greater(Value::Integer(2), Value::Integer(1)),
            AssertResult {
                success: true,
                type_mismatch: false,
                actual: "int <2>".to_string(),
                expected: "greater than int <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(Value::Integer(1), Value::Integer(1)),
            AssertResult {
                success: false,
                type_mismatch: false,
                actual: "int <1>".to_string(),
                expected: "greater than int <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(Value::Float(1.1), Value::Integer(1)),
            AssertResult {
                success: true,
                type_mismatch: false,
                actual: "float <1.1>".to_string(),
                expected: "greater than int <1>".to_string(),
            }
        );
        assert_eq!(
            assert_values_greater(Value::Float(1.1), Value::Integer(2)),
            AssertResult {
                success: false,
                type_mismatch: false,
                actual: "float <1.1>".to_string(),
                expected: "greater than int <2>".to_string(),
            }
        );
    }

    #[test]
    fn test_predicate_count_equals_error() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::CountEqual {
                    space0: whitespace.clone(),
                    value: PredicateValue::Integer(10),
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Bool(true),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "bool <true>");
        assert_eq!(assert_result.expected.as_str(), "count equals to <10>");

        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::CountEqual {
                    space0: whitespace.clone(),
                    value: PredicateValue::Integer(1),
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::List(vec![]),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "0");
        assert_eq!(assert_result.expected.as_str(), "1");

        let assert_result = eval_something(
            PredicateFunc {
                source_info: SourceInfo::init(0, 0, 0, 0),
                value: PredicateFuncValue::CountEqual {
                    space0: whitespace,
                    value: PredicateValue::Integer(1),
                },
            },
            &variables,
            Value::Nodeset(3),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "3");
        assert_eq!(assert_result.expected.as_str(), "1");
    }

    #[test]
    fn test_predicate_count_equals() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::CountEqual {
                    space0: whitespace.clone(),
                    value: PredicateValue::Integer(1),
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::List(vec![Value::Integer(1)]),
        )
        .unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "1");
        assert_eq!(assert_result.expected.as_str(), "1");

        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::CountEqual {
                    space0: whitespace,
                    value: PredicateValue::Integer(1),
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Nodeset(1),
        )
        .unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "1");
        assert_eq!(assert_result.expected.as_str(), "1");
    }

    #[test]
    fn test_predicate_type() {
        let variables = HashMap::new();
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::IsInteger {},
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Integer(1),
        )
        .unwrap();
        assert!(assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "int <1>");
        assert_eq!(assert_result.expected.as_str(), "integer");

        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::IsInteger {},
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::Float(1.0),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "float <1.0>");
        assert_eq!(assert_result.expected.as_str(), "integer");
    }

    #[test]
    fn test_predicate_not_with_different_types() {
        // equals predicate does not generate a type error with an integer value
        let predicate = Predicate {
            not: true,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::init(0, 0, 0, 0),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    operator: false,
                    value: PredicateValue::Null {},
                },
            },
        };

        let variables = HashMap::new();
        assert!(eval_predicate(predicate, &variables, Some(Value::Integer(1))).is_ok());
        //assert!(predicate.eval(&variables, None).is_ok());

        // startswith predicate generates a type error with an integer value
        let predicate = Predicate {
            not: true,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::init(0, 0, 0, 0),
                value: PredicateFuncValue::StartWith {
                    space0: whitespace(),
                    value: PredicateValue::String(Template {
                        quotes: false,
                        elements: vec![TemplateElement::String {
                            value: "toto".to_string(),
                            encoded: "toto".to_string(),
                        }],
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    }),
                },
            },
        };
        let error = eval_predicate(predicate, &variables, Some(Value::Integer(1)))
            .err()
            .unwrap();
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
    fn test_no_type_mismatch_with_none_value() {
        let predicate = Predicate {
            not: false,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::init(0, 0, 0, 0),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    value: PredicateValue::Null {},
                    operator: false,
                },
            },
        };

        let variables = HashMap::new();
        let error = eval_predicate(predicate, &variables, None).err().unwrap();
        assert_eq!(
            error.inner,
            RunnerError::AssertFailure {
                actual: "none".to_string(),
                expected: "null".to_string(),
                type_mismatch: false,
            }
        );

        let predicate = Predicate {
            not: true,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::init(0, 0, 0, 0),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    operator: false,
                    value: PredicateValue::Null {},
                },
            },
        };

        let variables = HashMap::new();
        assert!(eval_predicate(predicate, &variables, None).is_ok());
    }

    #[test]
    fn test_predicate_match() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        // // a float can be equals to an int (but the reverse)
        let assert_result = eval_something(
            PredicateFunc {
                value: PredicateFuncValue::Match {
                    space0: whitespace,
                    value: PredicateValue::Regex(Regex {
                        inner: regex::Regex::new(r#"a{3}"#).unwrap(),
                    }),
                },
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            &variables,
            Value::String("aa".to_string()),
        )
        .unwrap();
        assert!(!assert_result.success);
        assert!(!assert_result.type_mismatch);
        assert_eq!(assert_result.actual.as_str(), "string <aa>");
        assert_eq!(assert_result.expected.as_str(), "matches regex <a{3}>");
    }
}
