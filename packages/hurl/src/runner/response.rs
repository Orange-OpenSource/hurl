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

use crate::http;
use hurl_core::ast::*;

use super::assert::eval_assert;
use super::body::eval_body;
use super::capture::eval_capture;
use super::core::*;
use super::json::eval_json_value;
use super::template::eval_template;
use super::value::Value;

pub fn eval_asserts(
    response: Response,
    variables: &HashMap<String, Value>,
    http_response: http::Response,
    context_dir: String,
) -> Vec<AssertResult> {
    let mut asserts = vec![];

    let version = response.clone().version;
    asserts.push(AssertResult::Version {
        actual: http_response.version.to_string(),
        expected: version.value.as_str().to_string(),
        source_info: version.source_info,
    });

    let status = response.clone().status;
    if let StatusValue::Specific(v) = status.value {
        asserts.push(AssertResult::Status {
            actual: http_response.status as u64,
            expected: v as u64,
            source_info: status.source_info,
        });
    }

    for header in response.clone().headers {
        match eval_template(header.value.clone(), variables) {
            Err(e) => {
                asserts.push(AssertResult::Header {
                    actual: Err(e),
                    expected: String::from(""),
                    source_info: header.key.clone().source_info,
                });
            }
            Ok(expected) => {
                let header_name = header.key.value.clone();
                let actuals = http_response.get_header(header_name);
                if actuals.is_empty() {
                    asserts.push(AssertResult::Header {
                        actual: Err(Error {
                            source_info: header.key.clone().source_info,
                            inner: RunnerError::QueryHeaderNotFound {},
                            assert: false,
                        }),
                        expected,
                        source_info: header.key.clone().source_info,
                    });
                } else if actuals.len() == 1 {
                    let actual = actuals.first().unwrap().to_string();
                    asserts.push(AssertResult::Header {
                        actual: Ok(actual),
                        expected,
                        source_info: header.value.clone().source_info,
                    });
                } else {
                    // failure by default
                    // expected value not found in the list
                    // actual is therefore the full list
                    let mut actual = format!(
                        "[{}]",
                        actuals
                            .iter()
                            .map(|v| format!("\"{}\"", v))
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                    for value in actuals {
                        if value == expected {
                            actual = value;
                            break;
                        }
                    }
                    asserts.push(AssertResult::Header {
                        actual: Ok(actual),
                        expected,
                        source_info: header.value.clone().source_info,
                    });
                }
            }
        }
    }

    // implicit assert on body
    if let Some(body) = response.clone().body {
        match body.value {
            Bytes::Json { value } => {
                let expected = match eval_json_value(value, variables) {
                    Ok(s) => Ok(Value::String(s)),
                    Err(e) => Err(e),
                };
                let actual = match http_response.text() {
                    Ok(s) => Ok(Value::String(s)),
                    Err(e) => Err(Error {
                        source_info: SourceInfo {
                            start: body.space0.source_info.end.clone(),
                            end: body.space0.source_info.end.clone(),
                        },
                        inner: e,
                        assert: true,
                    }),
                };
                asserts.push(AssertResult::Body {
                    actual,
                    expected,
                    source_info: body.space0.source_info.clone(),
                })
            }
            Bytes::Xml { value } => {
                let expected = Ok(Value::String(value));
                let actual = match http_response.text() {
                    Ok(s) => Ok(Value::String(s)),
                    Err(e) => Err(Error {
                        source_info: SourceInfo {
                            start: body.space0.source_info.end.clone(),
                            end: body.space0.source_info.end.clone(),
                        },
                        inner: e,
                        assert: true,
                    }),
                };
                asserts.push(AssertResult::Body {
                    actual,
                    expected,
                    source_info: body.space0.source_info.clone(),
                })
            }
            Bytes::RawString(RawString { value, .. }) => {
                let expected = match eval_template(value.clone(), variables) {
                    Ok(s) => Ok(Value::String(s)),
                    Err(e) => Err(e),
                };
                let actual = match http_response.text() {
                    Ok(s) => Ok(Value::String(s)),
                    Err(e) => Err(Error {
                        source_info: SourceInfo {
                            start: body.space0.source_info.end.clone(),
                            end: body.space0.source_info.end.clone(),
                        },
                        inner: e,
                        assert: true,
                    }),
                };
                asserts.push(AssertResult::Body {
                    actual,
                    expected,
                    source_info: value.source_info,
                })
            }
            Bytes::Base64(Base64 {
                value,
                space0,
                space1,
                ..
            }) => asserts.push(AssertResult::Body {
                actual: Ok(Value::Bytes(http_response.body.clone())),
                expected: Ok(Value::Bytes(value)),
                source_info: SourceInfo {
                    start: space0.source_info.end,
                    end: space1.source_info.start,
                },
            }),
            Bytes::Hex(Hex {
                value,
                space0,
                space1,
                ..
            }) => asserts.push(AssertResult::Body {
                actual: Ok(Value::Bytes(http_response.body.clone())),
                expected: Ok(Value::Bytes(value)),
                source_info: SourceInfo {
                    start: space0.source_info.end,
                    end: space1.source_info.start,
                },
            }),
            Bytes::File { .. } => {
                let expected = match eval_body(body.clone(), variables, context_dir) {
                    Ok(body) => Ok(Value::Bytes(body.bytes())),
                    Err(e) => Err(e),
                };
                let actual = Ok(Value::Bytes(http_response.body.clone()));
                asserts.push(AssertResult::Body {
                    actual,
                    expected,
                    source_info: body.space0.source_info.clone(),
                })
            }
        };
    }

    for assert in response.asserts() {
        let assert_result = eval_assert(assert, variables, http_response.clone());
        asserts.push(assert_result);
    }
    asserts
}

pub fn eval_captures(
    response: Response,
    http_response: &http::Response,
    variables: &HashMap<String, Value>,
) -> Result<Vec<CaptureResult>, Error> {
    let mut captures = vec![];
    for capture in response.captures() {
        let capture_result = eval_capture(capture, variables, http_response.clone())?;
        captures.push(capture_result);
    }
    Ok(captures)
}

#[cfg(test)]
mod tests {
    use super::*;

    use self::super::super::assert;
    use self::super::super::capture;

    pub fn user_response() -> Response {
        let whitespace = Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::init(1, 1, 1, 1),
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
                source_info: SourceInfo::init(2, 6, 2, 9),
            },
            space0: whitespace.clone(),
            status: Status {
                value: StatusValue::Specific(200),
                source_info: SourceInfo::init(2, 10, 2, 13),
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
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                Section {
                    line_terminators: vec![],
                    space0: whitespace,
                    line_terminator0: line_terminator,
                    value: SectionValue::Captures(vec![capture::tests::user_count_capture()]),
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
            ],
            body: None,
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }

    #[test]
    pub fn test_eval_asserts() {
        let variables = HashMap::new();
        let context_dir = "undefined".to_string();
        assert_eq!(
            eval_asserts(
                user_response(),
                &variables,
                http::xml_two_users_http_response(),
                context_dir,
            ),
            vec![
                AssertResult::Version {
                    actual: String::from("1.0"),
                    expected: String::from("1.0"),
                    source_info: SourceInfo::init(2, 6, 2, 9),
                },
                AssertResult::Status {
                    actual: 200,
                    expected: 200,
                    source_info: SourceInfo::init(2, 10, 2, 13),
                },
                AssertResult::Explicit {
                    actual: Ok(Some(Value::Nodeset(2))),
                    source_info: SourceInfo::init(1, 14, 1, 27),
                    predicate_result: Some(Err(Error {
                        source_info: SourceInfo::init(1, 0, 1, 0),
                        inner: RunnerError::AssertFailure {
                            actual: "2".to_string(),
                            expected: "3".to_string(),
                            type_mismatch: false,
                        },
                        assert: true,
                    })),
                }
            ]
        );
    }

    #[test]
    pub fn test_eval_captures() {
        let variables = HashMap::new();
        assert_eq!(
            eval_captures(
                user_response(),
                &http::xml_two_users_http_response(),
                &variables,
            )
            .unwrap(),
            vec![CaptureResult {
                name: "UserCount".to_string(),
                value: Value::Float(2.0),
            }]
        );
    }
}
