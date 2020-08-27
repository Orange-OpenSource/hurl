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

use crate::core::common::Value;
use crate::http;

use super::core::{Error, RunnerError};
use super::core::*;
use super::super::core::ast::*;

impl AssertResult {
    pub fn fail(self) -> bool {
        match self {
            AssertResult::Version { actual, expected, .. } => actual != expected,
            AssertResult::Status { actual, expected, .. } => actual != expected,
            AssertResult::Header { .. } => false,
            AssertResult::Explicit { .. } => true,
            AssertResult::Body { .. } => true
        }
    }

    pub fn error(self) -> Option<Error> {
        match self {
            AssertResult::Version { actual, expected, source_info } => {
                if expected.as_str() == "*" || actual == expected {
                    None
                } else {
                    Some(Error {
                        source_info,
                        inner: RunnerError::AssertVersion { actual }
                        ,
                        assert: false,
                    })
                }
            }
            AssertResult::Status { actual, expected, source_info } => {
                if actual == expected {
                    None
                } else {
                    Some(Error {
                        source_info,
                        inner: RunnerError::AssertStatus { actual: actual.to_string() },
                        assert: false,
                    })
                }
            }
            AssertResult::Header { actual, expected, source_info } => {
                match actual {
                    Err(e) => Some(e),
                    Ok(s) => {
                        if s == expected {
                            None
                        } else {
                            Some(Error {
                                source_info,
                                inner: RunnerError::AssertHeaderValueError { actual: s },
                                assert: false,
                            })
                        }
                    }
                }
            }
            AssertResult::Body { actual, expected, source_info } => {
                match expected {
                    Err(e) => Some(e),
                    Ok(expected) => {
                        match actual {
                            Err(e) => Some(e),
                            Ok(actual) => if actual == expected {
                                None
                            } else {
                                let actual = actual.to_string();
                                let expected = expected.to_string();
                                Some(Error {
                                    source_info,
                                    inner: RunnerError::AssertBodyValueError { actual, expected },
                                    assert: false,
                                })
                            }
                        }
                    }
                }
            }

            AssertResult::Explicit { actual: Err(e), .. } => { Some(e) }
            AssertResult::Explicit { predicate_result: Some(Err(e)), .. } => { Some(e) }
            _ => None,
        }
    }
}

impl Assert {
    pub fn eval(self, http_response: http::response::Response, variables: &HashMap<String, Value>) -> AssertResult {
        let actual = self.query.eval(variables, http_response);
        let source_info = self.predicate.clone().predicate_func.source_info;
        let predicate_result = match actual.clone() {
            Err(_) => None,
            Ok(actual) => Some(self.predicate.eval(variables, actual))
        };

        AssertResult::Explicit { actual, source_info, predicate_result }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::core::common::SourceInfo;

    use super::*;
    use super::super::query;

    // xpath //user countEquals 3
    pub fn assert_count_user() -> Assert {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(1, 1, 1, 1),
        };
        let predicate = Predicate {
            not: false,
            space0: whitespace.clone(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::init(1, 14, 1, 27),
                value: PredicateFuncValue::CountEqual { space0: whitespace.clone(), value: 3 },
            },
        };
        Assert {
            line_terminators: vec![],
            space0: whitespace.clone(),
            query: query::tests::xpath_users(),
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
            assert_count_user().eval(http::response::tests::xml_three_users_http_response(), &variables),
            AssertResult::Explicit {
                actual: Ok(Some(Value::Nodeset(3))),
                source_info: SourceInfo::init(1, 14, 1, 27),
                predicate_result: Some(Ok(())),
            }
        );
    }
}
