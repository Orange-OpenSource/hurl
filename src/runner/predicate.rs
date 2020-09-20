/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

use regex::Regex;

use crate::core::common::Value;
use crate::core::common::{Pos, SourceInfo};

use super::super::core::ast::*;
use super::core::*;
use super::core::{Error, RunnerError};

// equals 10         function  return ()
// not equals 10
// countEquals 3               return () => ok        PredicateExpectedError
// not countEquals                           nok

// PredicateValue   => Recoverable with a not
// PredicateType

// xpath boolean(//user) equals 10
//                       ^^^^^^^^^^   Type does not matched with value return by query (generic message for the time-being
// xpath boolean(//user) not equals 10
//                       ^^^^^^^^^^^^^   Type does not matched with value return by query
// xpath cont(//user)  equals 10
//                     ^^^^^^^^^^^^^   actual value is 9
// xpath cont(//user)  greaterThan 10
//                     ^^^^^^^^^^^^^^   actual value is 9

// Predicate
// 2 evals

// 1) eval template
// 2) eval predicate

// equals template  becomes and equals string

impl Predicate {
    pub fn eval(self, variables: &HashMap<String, Value>, value: Option<Value>) -> PredicateResult {
        let assert_result = self.predicate_func.clone().eval(variables, value)?;
        let source_info = SourceInfo {
            start: Pos {
                line: self.space0.source_info.start.line,
                column: 0,
            },
            end: Pos {
                line: self.space0.source_info.start.line,
                column: 0,
            },
        };
        if assert_result.type_mismatch {
            Err(Error {
                source_info,
                inner: RunnerError::AssertFailure {
                    actual: assert_result.actual,
                    expected: assert_result.expected,
                    type_mismatch: true,
                },
                assert: true,
            })
        } else if self.not && assert_result.success {
            Err(Error {
                source_info,
                inner: RunnerError::AssertFailure {
                    actual: assert_result.actual,
                    expected: format!("not {}", assert_result.expected),
                    type_mismatch: false,
                },
                assert: true,
            })
        } else if !self.not && !assert_result.success {
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
}

struct AssertResult {
    pub success: bool,
    pub type_mismatch: bool,
    pub actual: String,
    pub expected: String,
}

impl Value {
    pub fn display(self) -> String {
        match self {
            Value::Bool(v) => format!("bool <{}>", v.to_string()),
            Value::Integer(v) => format!("int <{}>", v.to_string()),
            Value::String(v) => format!("string <{}>", v),
            Value::Float(i, d) => format!("float <{}.{}>", i, d),
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
            Value::Bytes(values) => format!("byte array of size <{}>", values.len()),
            Value::Null => "null".to_string(),
        }
    }
}

impl PredicateFunc {
    fn eval(
        self,
        variables: &HashMap<String, Value>,
        optional_value: Option<Value>,
    ) -> Result<AssertResult, Error> {
        match optional_value {
            None => {
                let type_mismatch = if let PredicateFuncValue::Exist {} = self.value {
                    false
                } else {
                    true
                };
                Ok(AssertResult {
                    success: false,
                    actual: "none".to_string(),
                    expected: self.expected(variables)?,
                    type_mismatch,
                })
            }
            Some(value) => self.eval_something(variables, value),
        }
    }

    fn expected(self, variables: &HashMap<String, Value>) -> Result<String, Error> {
        match self.value {
            PredicateFuncValue::EqualInt {
                value: expected, ..
            } => {
                let expected = expected.to_string();
                Ok(format!("int <{}>", expected))
            }
            PredicateFuncValue::EqualFloat {
                value:
                    Float {
                        int: expected_int,
                        decimal: expected_dec,
                        ..
                    },
                ..
            } => Ok(format!("float <{}.{}>", expected_int, expected_dec)),
            PredicateFuncValue::EqualNull { .. } => Ok("null".to_string()),
            PredicateFuncValue::EqualBool {
                value: expected, ..
            } => {
                let expected = expected.to_string();
                Ok(format!("bool <{}>", expected))
            }
            PredicateFuncValue::EqualString {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                Ok(format!("string <{}>", expected))
            }
            PredicateFuncValue::EqualExpression {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                todo!(">> {:?}", expected)
            }
            PredicateFuncValue::CountEqual {
                value: expected, ..
            } => {
                let expected = expected.to_string();
                Ok(format!("count equals to <{}>", expected))
            }
            PredicateFuncValue::StartWith {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                Ok(format!("starts with string <{}>", expected))
            }
            PredicateFuncValue::Contain {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                Ok(format!("contains string <{}>", expected))
            }
            PredicateFuncValue::IncludeString {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                Ok(format!("includes string <{}>", expected))
            }
            PredicateFuncValue::IncludeInt {
                value: expected, ..
            } => Ok(format!("includes int <{}>", expected)),
            PredicateFuncValue::IncludeFloat {
                value: expected, ..
            } => Ok(format!("includes float <{}>", expected)),
            PredicateFuncValue::IncludeNull { .. } => Ok("includes null".to_string()),
            PredicateFuncValue::IncludeBool {
                value: expected, ..
            } => Ok(format!("includes bool <{}>", expected)),
            PredicateFuncValue::IncludeExpression {
                value: _expected, ..
            } => todo!(),
            PredicateFuncValue::Match {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                Ok(format!("matches regex <{}>", expected))
            }
            PredicateFuncValue::Exist {} => Ok("something".to_string()),
        }
    }

    fn eval_something(
        self,
        variables: &HashMap<String, Value>,
        value: Value,
    ) -> Result<AssertResult, Error> {
        match self.value {
            // equals int
            PredicateFuncValue::EqualInt {
                value: expected, ..
            } => assert_values_equal(value, Value::Integer(expected)),

            // equals null
            PredicateFuncValue::EqualNull { .. } => assert_values_equal(value, Value::Null),

            // equals bool
            PredicateFuncValue::EqualBool {
                value: expected, ..
            } => assert_values_equal(value, Value::Bool(expected)),

            // equals float
            PredicateFuncValue::EqualFloat {
                value: Float { int, decimal, .. },
                ..
            } => assert_values_equal(value, Value::Float(int, decimal)),

            // equals string
            PredicateFuncValue::EqualString {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                assert_values_equal(value, Value::String(expected))
            }

            // equals expression
            PredicateFuncValue::EqualExpression {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                assert_values_equal(value, expected)
            }

            // countEquals
            PredicateFuncValue::CountEqual {
                value: expected_value,
                ..
            } => {
                let actual = value.clone().display();
                let expected = format!("count equals to <{}>", expected_value);
                match value {
                    Value::List(values) => Ok(AssertResult {
                        success: values.len() as u64 == expected_value,
                        actual,
                        expected,
                        type_mismatch: false,
                    }),
                    Value::Nodeset(n) => Ok(AssertResult {
                        success: n as u64 == expected_value,
                        actual,
                        expected,
                        type_mismatch: false,
                    }),
                    _ => Ok(AssertResult {
                        success: false,
                        actual,
                        expected,
                        type_mismatch: true,
                    }),
                }
            }

            // starts with string
            PredicateFuncValue::StartWith {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                match value.clone() {
                    Value::String(actual) => Ok(AssertResult {
                        success: actual.as_str().starts_with(expected.as_str()),
                        actual: value.display(),
                        expected: format!("starts with string <{}>", expected),
                        type_mismatch: false,
                    }),
                    _ => Ok(AssertResult {
                        success: false,
                        actual: value.display(),
                        expected: format!("starts with string <{}>", expected),
                        type_mismatch: true,
                    }),
                }
            }

            // contains
            PredicateFuncValue::Contain {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                match value.clone() {
                    Value::String(actual) => Ok(AssertResult {
                        success: actual.as_str().contains(expected.as_str()),
                        actual: value.display(),
                        expected: format!("contains string <{}>", expected),
                        type_mismatch: false,
                    }),
                    _ => Ok(AssertResult {
                        success: false,
                        actual: value.display(),
                        expected: format!("contains string <{}>", expected),
                        type_mismatch: true,
                    }),
                }
            }

            // includes String
            PredicateFuncValue::IncludeString {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                assert_include(value, Value::String(expected))
            }

            // includes int
            PredicateFuncValue::IncludeInt {
                value: expected, ..
            } => assert_include(value, Value::Integer(expected)),

            // includes float
            PredicateFuncValue::IncludeFloat {
                value: Float { int, decimal, .. },
                ..
            } => assert_include(value, Value::Float(int, decimal)),

            // includes bool
            PredicateFuncValue::IncludeBool {
                value: expected, ..
            } => assert_include(value, Value::Bool(expected)),

            // includes null
            PredicateFuncValue::IncludeNull { .. } => assert_include(value, Value::Null),

            // includes expression
            PredicateFuncValue::IncludeExpression {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                assert_include(value, expected)
            }

            // match string
            PredicateFuncValue::Match {
                value: expected, ..
            } => {
                let expected = expected.eval(variables)?;
                let regex = match Regex::new(expected.as_str()) {
                    Ok(re) => re,
                    Err(_) => {
                        return Err(Error {
                            source_info: self.source_info.clone(),
                            inner: RunnerError::InvalidRegex(),
                            assert: false,
                        })
                    }
                };
                match value.clone() {
                    Value::String(actual) => Ok(AssertResult {
                        success: regex.is_match(actual.as_str()),
                        actual: value.display(),
                        expected: format!("matches regex <{}>", expected),
                        type_mismatch: false,
                    }),
                    _ => Ok(AssertResult {
                        success: false,
                        actual: value.display(),
                        expected: format!("matches regex <{}>", expected),
                        type_mismatch: true,
                    }),
                }
            }

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
        }
    }
}

fn assert_values_equal(actual: Value, expected: Value) -> Result<AssertResult, Error> {
    match (actual.clone(), expected.clone()) {
        (Value::Null {}, Value::Null {}) => Ok(AssertResult {
            success: true,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        }),
        (Value::Bool(value1), Value::Bool(value2)) => Ok(AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        }),
        (Value::Integer(value1), Value::Integer(value2)) => Ok(AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        }),
        (Value::Float(int1, decimal), Value::Integer(int2)) => Ok(AssertResult {
            success: int1 == int2 && decimal == 0,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        }),
        (Value::Float(i1, d1), Value::Float(i2, d2)) => Ok(AssertResult {
            success: i1 == i2 && d1 == d2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        }),
        (Value::String(value1), Value::String(value2)) => Ok(AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        }),
        (Value::List(value1), Value::List(value2)) => Ok(AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        }),
        (Value::Bytes(value1), Value::Bytes(value2)) => Ok(AssertResult {
            success: value1 == value2,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: false,
        }),
        _ => Ok(AssertResult {
            success: false,
            actual: actual.display(),
            expected: expected.display(),
            type_mismatch: true,
        }),
    }
}

fn assert_include(value: Value, element: Value) -> Result<AssertResult, Error> {
    let expected = format!("includes {}", element.clone().display());
    match value.clone() {
        Value::List(values) => {
            let mut success = false;
            for v in values {
                let result = assert_values_equal(v, element.clone())?;
                if result.success {
                    success = true;
                    break;
                }
            }
            Ok(AssertResult {
                success,
                actual: value.display(),
                expected,
                type_mismatch: false,
            })
        }
        _ => Ok(AssertResult {
            success: false,
            actual: value.display(),
            expected,
            type_mismatch: true,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_xpath() {}

    #[test]
    fn test_predicate() {
        // not equals 10 with value 1     OK
        // not equals 10 with value 10    ValueError
        // not equals 10 with value true  TypeError
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(1, 1, 0, 0),
        };

        let predicate = Predicate {
            not: true,
            space0: whitespace.clone(),
            predicate_func: PredicateFunc {
                value: PredicateFuncValue::EqualInt {
                    space0: whitespace,
                    value: 10,
                },
                source_info: SourceInfo::init(1, 11, 1, 12),
            },
        };

        let error = predicate
            .clone()
            .eval(&variables, Some(Value::Bool(true)))
            .err()
            .unwrap();
        assert_eq!(
            error.inner,
            RunnerError::AssertFailure {
                actual: "bool <true>".to_string(),
                expected: "int <10>".to_string(),
                type_mismatch: true,
            }
        );
        assert_eq!(error.source_info, SourceInfo::init(1, 0, 1, 0));

        let error = predicate
            .clone()
            .eval(&variables, Some(Value::Integer(10)))
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

        assert_eq!(
            predicate.eval(&variables, Some(Value::Integer(1))).unwrap(),
            ()
        );
    }

    #[test]
    fn test_predicate_type_mismatch() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualInt {
                space0: whitespace,
                value: 10,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval(&variables, Some(Value::Bool(true)))
        .unwrap();
        assert_eq!(assert_result.success, false);
        assert_eq!(assert_result.type_mismatch, true);
        assert_eq!(assert_result.actual.as_str(), "bool <true>");
        assert_eq!(assert_result.expected.as_str(), "int <10>");
    }

    #[test]
    fn test_predicate_value_error() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualInt {
                space0: whitespace.clone(),
                value: 10,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Integer(1))
        .unwrap();
        assert_eq!(assert_result.success, false);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "int <1>");
        assert_eq!(assert_result.expected.as_str(), "int <10>");

        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualBool {
                space0: whitespace.clone(),
                value: true,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Bool(false))
        .unwrap();
        assert_eq!(assert_result.success, false);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "bool <false>");
        assert_eq!(assert_result.expected.as_str(), "bool <true>");

        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualFloat {
                space0: whitespace,
                value: Float {
                    int: 1,
                    decimal: 200_000_000_000_000_000,
                    decimal_digits: 0,
                },
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Float(1, 1))
        .unwrap();
        assert_eq!(assert_result.success, false);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "float <1.1>");
        assert_eq!(
            assert_result.expected.as_str(),
            "float <1.200000000000000000>"
        );
    }

    #[test]
    fn test_predicate_value_equals() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualInt {
                space0: whitespace.clone(),
                value: 1,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Integer(1))
        .unwrap();
        assert_eq!(assert_result.success, true);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "int <1>");
        assert_eq!(assert_result.expected.as_str(), "int <1>");

        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualBool {
                space0: whitespace.clone(),
                value: false,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Bool(false))
        .unwrap();
        assert_eq!(assert_result.success, true);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "bool <false>");
        assert_eq!(assert_result.expected.as_str(), "bool <false>");

        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualFloat {
                space0: whitespace.clone(),
                value: Float {
                    int: 1,
                    decimal: 1,
                    decimal_digits: 1,
                },
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Float(1, 1))
        .unwrap();
        assert_eq!(assert_result.success, true);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "float <1.1>");
        assert_eq!(assert_result.expected.as_str(), "float <1.1>");

        // a float can be equals to an int (but the reverse)
        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualInt {
                space0: whitespace,
                value: 1,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Float(1, 0))
        .unwrap();
        assert_eq!(assert_result.success, true);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "float <1.0>");
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

        let error = PredicateFunc {
            value: PredicateFuncValue::EqualString {
                space0: whitespace.clone(),
                value: template.clone(),
            },
            source_info: SourceInfo::init(1, 1, 1, 21),
        }
        .eval_something(
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
        let assert_result = PredicateFunc {
            value: PredicateFuncValue::EqualString {
                space0: whitespace,
                value: template,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(
            &variables,
            Value::String(String::from("http://localhost:8000")),
        )
        .unwrap();
        assert_eq!(assert_result.success, true);
        assert_eq!(assert_result.type_mismatch, false);
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
    fn test_predicate_count_equals_error() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };

        let assert_result = PredicateFunc {
            value: PredicateFuncValue::CountEqual {
                space0: whitespace.clone(),
                value: 10,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Bool(true))
        .unwrap();
        assert_eq!(assert_result.success, false);
        assert_eq!(assert_result.type_mismatch, true);
        assert_eq!(assert_result.actual.as_str(), "bool <true>");
        assert_eq!(assert_result.expected.as_str(), "count equals to <10>");

        let assert_result = PredicateFunc {
            value: PredicateFuncValue::CountEqual {
                space0: whitespace.clone(),
                value: 1,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::List(vec![]))
        .unwrap();
        assert_eq!(assert_result.success, false);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "[]");
        assert_eq!(assert_result.expected.as_str(), "count equals to <1>");

        let assert_result = PredicateFunc {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: PredicateFuncValue::CountEqual {
                space0: whitespace,
                value: 1,
            },
        }
        .eval_something(&variables, Value::Nodeset(3))
        .unwrap();
        assert_eq!(assert_result.success, false);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "nodeset of size <3>");
        assert_eq!(assert_result.expected.as_str(), "count equals to <1>");
    }

    #[test]
    fn test_predicate_count_equals() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let assert_result = PredicateFunc {
            value: PredicateFuncValue::CountEqual {
                space0: whitespace.clone(),
                value: 1,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::List(vec![Value::Integer(1)]))
        .unwrap();
        assert_eq!(assert_result.success, true);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "[int <1>]");
        assert_eq!(assert_result.expected.as_str(), "count equals to <1>");

        let assert_result = PredicateFunc {
            value: PredicateFuncValue::CountEqual {
                space0: whitespace,
                value: 1,
            },
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
        .eval_something(&variables, Value::Nodeset(1))
        .unwrap();
        assert_eq!(assert_result.success, true);
        assert_eq!(assert_result.type_mismatch, false);
        assert_eq!(assert_result.actual.as_str(), "nodeset of size <1>");
        assert_eq!(assert_result.expected.as_str(), "count equals to <1>");
    }
}
