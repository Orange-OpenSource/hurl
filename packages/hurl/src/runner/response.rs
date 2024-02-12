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

use hurl_core::ast::*;

use crate::http;
use crate::runner::assert::eval_explicit_assert;
use crate::runner::body::eval_body;
use crate::runner::capture::eval_capture;
use crate::runner::error::{Error, RunnerError};
use crate::runner::json::eval_json_value;
use crate::runner::multiline::eval_multiline;
use crate::runner::result::{AssertResult, CaptureResult};
use crate::runner::template::eval_template;
use crate::runner::Value;
use crate::util::path::ContextDir;

/// Returns a list of assert results on the response status code and HTTP version,
/// given a set of `variables`, an actual `http_response` and a spec `response`.
pub fn eval_version_status_asserts(
    response: &Response,
    http_response: &http::Response,
) -> Vec<AssertResult> {
    let mut asserts = vec![];

    let version = &response.version;
    asserts.push(AssertResult::Version {
        actual: http_response.version.to_string(),
        expected: version.value.to_string(),
        source_info: version.source_info,
    });

    let status = &response.status;
    if let StatusValue::Specific(v) = status.value {
        asserts.push(AssertResult::Status {
            actual: http_response.status as u64,
            expected: v,
            source_info: status.source_info,
        });
    }
    asserts
}

/// Returns a list of assert results, given a set of `variables`, an actual `http_response` and a spec `response`.
///
/// Asserts on status and version and not run in this function, there are run with `eval_version_status_asserts`
/// as they're semantically stronger.
pub fn eval_asserts(
    response: &Response,
    variables: &HashMap<String, Value>,
    http_response: &http::Response,
    context_dir: &ContextDir,
) -> Vec<AssertResult> {
    let mut asserts = vec![];

    // First, evaluates implicit asserts on response headers.
    for header in response.headers.iter() {
        match eval_template(&header.value, variables) {
            Err(e) => {
                let result = AssertResult::Header {
                    actual: Err(e),
                    expected: String::new(),
                    source_info: header.key.source_info,
                };
                asserts.push(result);
            }
            Ok(expected) => {
                match eval_template(&header.key, variables) {
                    Ok(header_name) => {
                        let actuals = http_response.get_header_values(&header_name);
                        if actuals.is_empty() {
                            let result = AssertResult::Header {
                                actual: Err(Error::new(
                                    header.key.source_info,
                                    RunnerError::QueryHeaderNotFound,
                                    false,
                                )),
                                expected,
                                source_info: header.key.source_info,
                            };
                            asserts.push(result);
                        } else if actuals.len() == 1 {
                            let actual = actuals.first().unwrap().to_string();
                            let result = AssertResult::Header {
                                actual: Ok(actual),
                                expected,
                                source_info: header.value.source_info,
                            };
                            asserts.push(result);
                        } else {
                            // failure by default
                            // expected value not found in the list
                            // actual is therefore the full list
                            let mut actual = format!(
                                "[{}]",
                                actuals
                                    .iter()
                                    .map(|v| format!("\"{v}\""))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            );
                            for value in actuals {
                                if value == expected {
                                    actual = value.to_string();
                                    break;
                                }
                            }
                            let result = AssertResult::Header {
                                actual: Ok(actual),
                                expected,
                                source_info: header.value.source_info,
                            };
                            asserts.push(result);
                        }
                    }
                    Err(e) => {
                        let result = AssertResult::Header {
                            actual: Err(e),
                            expected,
                            source_info: header.value.source_info,
                        };
                        asserts.push(result);
                    }
                }
            }
        }
    }

    // Second, evaluates implicit asserts on response body.
    if let Some(body) = &response.body {
        let assert = eval_implicit_body_asserts(body, variables, http_response, context_dir);
        asserts.push(assert);
    }

    // Then, checks all the explicit asserts.
    for assert in response.asserts().iter() {
        let assert_result = eval_explicit_assert(assert, variables, http_response, context_dir);
        asserts.push(assert_result);
    }
    asserts
}

/// Check the body of an actual HTTP response against a spec body, given a set of variables.
fn eval_implicit_body_asserts(
    spec_body: &Body,
    variables: &HashMap<String, Value>,
    http_response: &http::Response,
    context_dir: &ContextDir,
) -> AssertResult {
    match &spec_body.value {
        Bytes::Json(value) => {
            let expected = match eval_json_value(value, variables, true) {
                Ok(s) => Ok(Value::String(s)),
                Err(e) => Err(e),
            };
            let actual = match http_response.text() {
                Ok(s) => Ok(Value::String(s)),
                Err(e) => {
                    let source_info = SourceInfo {
                        start: spec_body.space0.source_info.end,
                        end: spec_body.space0.source_info.end,
                    };
                    Err(Error::new(source_info, e.into(), true))
                }
            };
            AssertResult::Body {
                actual,
                expected,
                source_info: spec_body.space0.source_info,
            }
        }
        Bytes::Xml(value) => {
            let expected = Ok(Value::String(value.to_string()));
            let actual = match http_response.text() {
                Ok(s) => Ok(Value::String(s)),
                Err(e) => {
                    let source_info = SourceInfo {
                        start: spec_body.space0.source_info.end,
                        end: spec_body.space0.source_info.end,
                    };
                    Err(Error::new(source_info, e.into(), true))
                }
            };
            AssertResult::Body {
                actual,
                expected,
                source_info: spec_body.space0.source_info,
            }
        }
        Bytes::OnelineString(value) => {
            let expected = match eval_template(value, variables) {
                Ok(s) => Ok(Value::String(s)),
                Err(e) => Err(e),
            };
            let actual = match http_response.text() {
                Ok(s) => Ok(Value::String(s)),
                Err(e) => {
                    let source_info = SourceInfo {
                        start: spec_body.space0.source_info.end,
                        end: spec_body.space0.source_info.end,
                    };
                    Err(Error::new(source_info, e.into(), true))
                }
            };
            AssertResult::Body {
                actual,
                expected,
                source_info: value.source_info,
            }
        }
        Bytes::MultilineString(multi) => {
            let expected = match eval_multiline(multi, variables) {
                Ok(s) => Ok(Value::String(s)),
                Err(e) => Err(e),
            };
            let actual = match http_response.text() {
                Ok(s) => Ok(Value::String(s)),
                Err(e) => {
                    let source_info = SourceInfo {
                        start: spec_body.space0.source_info.end,
                        end: spec_body.space0.source_info.end,
                    };
                    Err(Error::new(source_info, e.into(), true))
                }
            };
            AssertResult::Body {
                actual,
                expected,
                source_info: multi.value().source_info,
            }
        }
        Bytes::Base64(Base64 {
            value,
            space0,
            space1,
            ..
        }) => {
            let expected = Ok(Value::Bytes(value.to_vec()));
            let actual = match http_response.uncompress_body() {
                Ok(b) => Ok(Value::Bytes(b)),
                Err(e) => {
                    let source_info = SourceInfo {
                        start: spec_body.space0.source_info.end,
                        end: spec_body.space0.source_info.end,
                    };
                    Err(Error::new(source_info, e.into(), true))
                }
            };
            AssertResult::Body {
                actual,
                expected,
                source_info: SourceInfo {
                    start: space0.source_info.end,
                    end: space1.source_info.start,
                },
            }
        }
        Bytes::Hex(Hex {
            value,
            space0,
            space1,
            ..
        }) => {
            let expected = Ok(Value::Bytes(value.to_vec()));
            let actual = match http_response.uncompress_body() {
                Ok(b) => Ok(Value::Bytes(b)),
                Err(e) => {
                    let source_info = SourceInfo {
                        start: spec_body.space0.source_info.end,
                        end: spec_body.space0.source_info.end,
                    };
                    Err(Error::new(source_info, e.into(), true))
                }
            };
            AssertResult::Body {
                actual,
                expected,
                source_info: SourceInfo {
                    start: space0.source_info.end,
                    end: space1.source_info.start,
                },
            }
        }
        Bytes::File { .. } => {
            let expected = match eval_body(spec_body, variables, context_dir) {
                Ok(body) => Ok(Value::Bytes(body.bytes())),
                Err(e) => Err(e),
            };
            let actual = match http_response.uncompress_body() {
                Ok(b) => Ok(Value::Bytes(b)),
                Err(e) => {
                    let source_info = SourceInfo {
                        start: spec_body.space0.source_info.end,
                        end: spec_body.space0.source_info.end,
                    };
                    Err(Error::new(source_info, e.into(), true))
                }
            };
            AssertResult::Body {
                actual,
                expected,
                source_info: spec_body.space0.source_info,
            }
        }
    }
}

/// Evaluates captures from this HTTP `http_response`, given a set of `variables`.
pub fn eval_captures(
    response: &Response,
    http_response: &http::Response,
    variables: &mut HashMap<String, Value>,
) -> Result<Vec<CaptureResult>, Error> {
    let mut captures = vec![];
    for capture in response.captures().iter() {
        let capture_result = eval_capture(capture, variables, http_response)?;
        // Update variables now so the captures set is ready in case
        // the next captures reference this new variable.
        variables.insert(capture_result.name.clone(), capture_result.value.clone());
        captures.push(capture_result);
    }
    Ok(captures)
}

#[cfg(test)]
mod tests {
    use self::super::super::{assert, capture};
    use super::*;
    use crate::runner::Number;

    pub fn user_response() -> Response {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
        };
        let line_terminator = LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        };
        // HTTP/1.1 200
        Response {
            line_terminators: vec![],
            version: Version {
                value: VersionValue::Version1,
                source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 9)),
            },
            space0: whitespace.clone(),
            status: Status {
                value: StatusValue::Specific(200),
                source_info: SourceInfo::new(Pos::new(2, 10), Pos::new(2, 13)),
            },
            space1: whitespace.clone(),
            line_terminator0: line_terminator.clone(),
            headers: vec![],
            sections: vec![
                Section {
                    line_terminators: vec![],
                    space0: whitespace.clone(),
                    line_terminator0: line_terminator.clone(),
                    value: SectionValue::Asserts(vec![assert::tests::assert_count_user()]),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                Section {
                    line_terminators: vec![],
                    space0: whitespace,
                    line_terminator0: line_terminator,
                    value: SectionValue::Captures(vec![capture::tests::user_count_capture()]),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            ],
            body: None,
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    #[test]
    pub fn test_eval_asserts() {
        let variables = HashMap::new();
        let context_dir = ContextDir::default();
        assert_eq!(
            eval_asserts(
                &user_response(),
                &variables,
                &http::xml_two_users_http_response(),
                &context_dir,
            ),
            vec![AssertResult::Explicit {
                actual: Ok(Some(Value::Number(Number::Integer(2)))),
                source_info: SourceInfo::new(Pos::new(1, 22), Pos::new(1, 24)),
                predicate_result: Some(Err(Error::new(
                    SourceInfo::new(Pos::new(1, 0), Pos::new(1, 0)),
                    RunnerError::AssertFailure {
                        actual: "int <2>".to_string(),
                        expected: "int <3>".to_string(),
                        type_mismatch: false,
                    },
                    true
                ))),
            },]
        );
    }

    #[test]
    pub fn test_eval_version_status_asserts() {
        assert_eq!(
            eval_version_status_asserts(&user_response(), &http::xml_two_users_http_response(),),
            vec![
                AssertResult::Version {
                    actual: String::from("HTTP/1.0"),
                    expected: String::from("HTTP/1.0"),
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 9)),
                },
                AssertResult::Status {
                    actual: 200,
                    expected: 200,
                    source_info: SourceInfo::new(Pos::new(2, 10), Pos::new(2, 13)),
                },
            ]
        );
    }

    #[test]
    pub fn test_eval_captures() {
        let mut variables = HashMap::new();
        assert_eq!(
            eval_captures(
                &user_response(),
                &http::xml_two_users_http_response(),
                &mut variables,
            )
            .unwrap(),
            vec![CaptureResult {
                name: "UserCount".to_string(),
                value: Value::Number(Number::Float(2.0)),
            }]
        );
    }
}
