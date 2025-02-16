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
use hurl_core::ast::Capture;

use crate::http;
use crate::runner::cache::BodyCache;
use crate::runner::error::{RunnerError, RunnerErrorKind};
use crate::runner::filter::eval_filters;
use crate::runner::query::eval_query;
use crate::runner::result::CaptureResult;
use crate::runner::template::eval_template;
use crate::runner::VariableSet;

/// Evaluates a `capture` with `variables` map and `http_response`, returns a
/// [`CaptureResult`] on success or an [`RunnerError`].
///
/// The `cache` is used to store XML / JSON structured response data and avoid redundant parsing
/// operation on the response.
pub fn eval_capture(
    capture: &Capture,
    variables: &VariableSet,
    http_response: &http::Response,
    cache: &mut BodyCache,
) -> Result<CaptureResult, RunnerError> {
    let name = eval_template(&capture.name, variables)?;
    let value = eval_query(&capture.query, variables, http_response, cache)?;
    let value = match value {
        None => {
            return Err(RunnerError::new(
                capture.query.source_info,
                RunnerErrorKind::NoQueryResult,
                false,
            ));
        }
        Some(value) => {
            let filters = capture
                .filters
                .iter()
                .map(|(_, f)| f.clone())
                .collect::<Vec<_>>();
            match eval_filters(&filters, &value, variables, false)? {
                None => {
                    return Err(RunnerError::new(
                        capture.query.source_info,
                        RunnerErrorKind::NoQueryResult,
                        false,
                    ));
                }
                Some(v) => v,
            }
        }
    };

    Ok(CaptureResult {
        name: name.clone(),
        value,
    })
}

#[cfg(test)]
pub mod tests {
    use hurl_core::ast::{
        LineTerminator, Query, QueryValue, SourceInfo, Template, TemplateElement, Whitespace,
    };
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use self::super::super::query;
    use super::*;
    use crate::runner::{Number, Value};

    pub fn user_count_capture() -> Capture {
        // non scalar value
        let whitespace = Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        Capture {
            line_terminators: vec![],
            space0: whitespace.clone(),
            name: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "UserCount".to_string(),
                    source: "UserCount".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: whitespace.clone(),
            space2: whitespace.clone(),

            // xpath count(//user)
            query: query::tests::xpath_count_user_query(),
            filters: vec![],
            space3: whitespace.clone(),
            redact: false,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        }
    }

    pub fn duration_capture() -> Capture {
        // non scalar value
        let whitespace = Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        Capture {
            line_terminators: vec![],
            space0: whitespace.clone(),
            name: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "duration".to_string(),
                    source: "duration".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: whitespace.clone(),
            space2: whitespace.clone(),

            // xpath count(//user)
            query: query::tests::jsonpath_duration(),
            filters: vec![],
            space3: whitespace.clone(),
            redact: false,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        }
    }

    #[test]
    fn test_invalid_xpath() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();
        let whitespace = Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        let capture = Capture {
            line_terminators: vec![],
            space0: whitespace.clone(),
            name: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "count".to_string(),
                    source: "count".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            filters: vec![],
            space1: whitespace.clone(),
            space2: whitespace.clone(),

            query: query::tests::xpath_invalid_query(),
            space3: whitespace.clone(),
            redact: false,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        };

        let error = eval_capture(
            &capture,
            &variables,
            &http::xml_three_users_http_response(),
            &mut cache,
        )
        .err()
        .unwrap();
        assert_eq!(error.source_info.start, Pos { line: 1, column: 7 });
        assert_eq!(error.kind, RunnerErrorKind::QueryInvalidXpathEval);
    }

    #[test]
    fn test_capture_unsupported() {
        // non scalar value
        let whitespace = Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        let _capture = Capture {
            line_terminators: vec![],
            space0: whitespace.clone(),
            name: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "???".to_string(),
                    source: "???".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: whitespace.clone(),
            space2: whitespace.clone(),

            // xpath //user
            query: Query {
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 13)),
                value: QueryValue::Xpath {
                    space0: whitespace.clone(),
                    expr: Template {
                        delimiter: Some('"'),
                        elements: vec![TemplateElement::String {
                            value: "//user".to_string(),
                            source: "//user".to_source(),
                        }],
                        source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 13)),
                    },
                },
            },
            filters: vec![],
            space3: whitespace.clone(),
            redact: false,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        };
    }

    #[test]
    fn test_capture() {
        let variables = VariableSet::new();
        let mut cache = BodyCache::new();

        assert_eq!(
            eval_capture(
                &user_count_capture(),
                &variables,
                &http::xml_three_users_http_response(),
                &mut cache,
            )
            .unwrap(),
            CaptureResult {
                name: "UserCount".to_string(),
                value: Value::Number(Number::from(3.0)),
            }
        );

        assert_eq!(
            eval_capture(
                &duration_capture(),
                &variables,
                &http::json_http_response(),
                &mut cache
            )
            .unwrap(),
            CaptureResult {
                name: "duration".to_string(),
                value: Value::Number(Number::from(1.5)),
            }
        );
    }
}
