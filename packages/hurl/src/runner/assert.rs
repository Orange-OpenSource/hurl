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

use std::collections::HashMap;

use crate::http;
use crate::runner::filter::eval_filters;
use hurl_core::ast::*;

use super::core::*;
use super::core::{Error, RunnerError};
use super::predicate::eval_predicate;
use super::query::eval_query;
use super::value::Value;

impl AssertResult {
    /// Evaluates an assert and returns `None` if assert is succeeded or an `Error` if failed.
    pub fn error(&self) -> Option<Error> {
        match self {
            AssertResult::Version {
                actual,
                expected,
                source_info,
            } => {
                if expected.as_str() == "HTTP"
                    || expected.as_str() == "HTTP/*"
                    || actual == expected
                {
                    None
                } else {
                    Some(Error {
                        source_info: source_info.clone(),
                        inner: RunnerError::AssertVersion {
                            actual: actual.clone(),
                        },
                        assert: false,
                    })
                }
            }
            AssertResult::Status {
                actual,
                expected,
                source_info,
            } => {
                if actual == expected {
                    None
                } else {
                    Some(Error {
                        source_info: source_info.clone(),
                        inner: RunnerError::AssertStatus {
                            actual: actual.to_string(),
                        },
                        assert: false,
                    })
                }
            }
            AssertResult::Header {
                actual,
                expected,
                source_info,
            } => match actual {
                Err(e) => Some(e.clone()),
                Ok(s) => {
                    if s == expected {
                        None
                    } else {
                        Some(Error {
                            source_info: source_info.clone(),
                            inner: RunnerError::AssertHeaderValueError { actual: s.clone() },
                            assert: false,
                        })
                    }
                }
            },
            AssertResult::Body {
                actual,
                expected,
                source_info,
            } => match expected {
                Err(e) => Some(e.clone()),
                Ok(expected) => match actual {
                    Err(e) => Some(e.clone()),
                    Ok(actual) => {
                        if actual == expected {
                            None
                        } else {
                            let actual = actual.to_string();
                            let expected = expected.to_string();
                            Some(Error {
                                source_info: source_info.clone(),
                                inner: RunnerError::AssertBodyValueError { actual, expected },
                                assert: false,
                            })
                        }
                    }
                },
            },

            AssertResult::Explicit { actual: Err(e), .. } => Some(e.clone()),
            AssertResult::Explicit {
                predicate_result: Some(Err(e)),
                ..
            } => Some(e.clone()),
            _ => None,
        }
    }
    pub fn line(&self) -> usize {
        match self {
            AssertResult::Version { source_info, .. } => source_info.start.line,
            AssertResult::Status { source_info, .. } => source_info.start.line,
            AssertResult::Header { source_info, .. } => source_info.start.line,
            AssertResult::Body { source_info, .. } => source_info.start.line,
            AssertResult::Explicit { source_info, .. } => source_info.start.line,
        }
    }
}

pub fn eval_assert(
    assert: &Assert,
    variables: &HashMap<String, Value>,
    http_response: &http::Response,
) -> AssertResult {
    let query_result = eval_query(&assert.query, variables, http_response);

    let actual = if assert.filters.is_empty() {
        query_result
    } else if let Ok(optional_value) = query_result {
        match optional_value {
            None => Err(Error {
                source_info: assert
                    .filters
                    .first()
                    .expect("at least one filter")
                    .1
                    .source_info
                    .clone(),
                inner: RunnerError::FilterMissingInput {},
                assert: true,
            }),
            Some(value) => {
                let filters = assert.filters.iter().map(|(_, f)| f.clone()).collect();
                match eval_filters(&filters, &value, variables, true) {
                    Ok(value) => Ok(Some(value)),
                    Err(e) => Err(e),
                }
            }
        }
    } else {
        query_result
    };

    let source_info = &assert.predicate.predicate_func.source_info;
    let predicate_result = match &actual {
        Err(_) => None,
        Ok(actual) => Some(eval_predicate(&assert.predicate, variables, actual)),
    };

    AssertResult::Explicit {
        actual,
        source_info: source_info.clone(),
        predicate_result,
    }
}

#[cfg(test)]
pub mod tests {
    use hurl_core::ast::SourceInfo;

    use super::super::query;
    use super::*;

    // xpath //user countEquals 3
    pub fn assert_count_user() -> Assert {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(1, 1, 1, 1),
        };
        let predicate = Predicate {
            not: false,
            space0: whitespace.clone(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(1, 14, 1, 27),
                value: PredicateFuncValue::CountEqual {
                    space0: whitespace.clone(),
                    value: PredicateValue::Integer(3),
                },
            },
        };
        Assert {
            line_terminators: vec![],
            space0: whitespace.clone(),
            query: query::tests::xpath_users(),
            filters: vec![],
            space1: whitespace.clone(),
            predicate,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        }
    }

    #[test]
    fn test_invalid_xpath() {}

    #[test]
    fn test_eval() {
        let variables = HashMap::new();
        assert_eq!(
            eval_assert(
                &assert_count_user(),
                &variables,
                &http::xml_three_users_http_response()
            ),
            AssertResult::Explicit {
                actual: Ok(Some(Value::Nodeset(3))),
                source_info: SourceInfo::new(1, 14, 1, 27),
                predicate_result: Some(Ok(())),
            }
        );
    }
}
