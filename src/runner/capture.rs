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
use crate::http;

use super::core::{CaptureResult, Error};
use super::core::RunnerError;
use super::super::core::ast::*;

impl Capture {
    pub fn eval(self, variables: &HashMap<String, Value>, http_response: http::response::Response) -> Result<CaptureResult, Error> {
        let name = self.name.value;
        let value = self.query.clone().eval(variables, http_response)?;
        let value = match value {
            None => return Err(Error {
                source_info: self.query.source_info,
                inner: RunnerError::NoQueryResult {},
                assert: false,
            }),
            Some(value) => match self.subquery {
                None => value,
                Some(subquery) => {
                    let value = subquery.clone().eval(variables, value)?;
                    match value {
                        None => return Err(Error {
                            source_info: subquery.source_info,
                            inner: RunnerError::NoQueryResult {},
                            assert: false,
                        }),
                        Some(value) => value
                    }
                }
            }
        };
        Ok(CaptureResult { name, value })
    }
}


impl Subquery {
    pub fn eval(self, variables: &HashMap<String, Value>, value: Value) -> Result<Option<Value>, Error> {
        match self.value {
            SubqueryValue::Regex { expr, .. } => {
                let source_info = expr.source_info.clone();
                let expr = expr.eval(variables)?;
                match value {
                    Value::String(s) => {
                        match Regex::new(expr.as_str()) {
                            Ok(re) => {
                                match re.captures(s.as_str()) {
                                    Some(captures) => match captures.get(1) {
                                        Some(v) => Ok(Some(Value::String(v.as_str().to_string()))),
                                        None => Ok(None),
                                    }
                                    None => Ok(None),
                                }
                            }
                            Err(_) => Err(Error {
                                source_info,
                                inner: RunnerError::InvalidRegex(),
                                assert: false,
                            })
                        }
                    }
                    _ => Err(Error {
                        source_info: self.source_info,
                        inner: RunnerError::SubqueryInvalidInput,
                        assert: false,
                    })
                }
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::core::common::{Pos, SourceInfo};

    use super::*;

    use self::super::super::query;

    pub fn user_count_capture() -> Capture {

        // non scalar value
        let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
        Capture {
            line_terminators: vec![],
            space0: whitespace.clone(),
            name: EncodedString {
                quotes: false,
                value: "UserCount".to_string(),
                encoded: "UserCount".to_string(),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            space1: whitespace.clone(),
            space2: whitespace.clone(),

            // xpath count(//user)
            query: query::tests::xpath_count_user_query(),
            space3: whitespace.clone(),
            subquery: None,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        }
    }

    pub fn duration_capture() -> Capture {

        // non scalar value
        let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
        Capture {
            line_terminators: vec![],
            space0: whitespace.clone(),
            name: EncodedString {
                quotes: false,
                value: "duration".to_string(),
                encoded: "duration".to_string(),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            space1: whitespace.clone(),
            space2: whitespace.clone(),

            // xpath count(//user)
            query: query::tests::jsonpath_duration(),
            space3: whitespace.clone(),
            subquery: None,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        }
    }

    #[test]
    fn test_invalid_xpath() {
        let variables = HashMap::new();
        let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
        let capture = Capture {
            line_terminators: vec![],
            space0: whitespace.clone(),
            name: EncodedString {
                quotes: false,
                value: "count".to_string(),
                encoded: "count".to_string(),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            space1: whitespace.clone(),
            space2: whitespace.clone(),

            query: query::tests::xpath_invalid_query(),
            space3: whitespace.clone(),
            subquery: None,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        };

        let error = capture.eval(&variables, http::response::tests::xml_three_users_http_response()).err().unwrap();
        assert_eq!(error.source_info.start, Pos { line: 1, column: 7 });
        assert_eq!(error.inner, RunnerError::QueryInvalidXpathEval)
    }

    #[test]
    fn test_capture_unsupported() {

        // non scalar value
        let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
        let _capture = Capture {
            line_terminators: vec![],
            space0: whitespace.clone(),
            name: EncodedString {
                quotes: false,
                value: "???".to_string(),
                encoded: "???".to_string(),
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            space1: whitespace.clone(),
            space2: whitespace.clone(),

            // xpath //user
            query: Query {
                source_info: SourceInfo::init(1, 1, 1, 13),
                value: QueryValue::Xpath {
                    space0: whitespace.clone(),
                    expr: Template {
                        quotes: true,
                        elements: vec![
                            TemplateElement::String {
                                value: "//user".to_string(),
                                encoded: "//user".to_string(),
                            }
                        ],
                        source_info: SourceInfo::init(1, 7, 1, 13),
                    },
                },
            },
            space3: whitespace.clone(),
            subquery: None,
            line_terminator0: LineTerminator {
                space0: whitespace.clone(),
                comment: None,
                newline: whitespace,
            },
        };



    }

    #[test]
    fn test_capture() {
        let variables = HashMap::new();
        assert_eq!(user_count_capture().eval(&variables, http::response::tests::xml_three_users_http_response()).unwrap(),
                   CaptureResult {
                       name: "UserCount".to_string(),
                       value: Value::from_f64(3.0),
                   });

        assert_eq!(duration_capture().eval(&variables, http::response::tests::json_http_response()).unwrap(),
                   CaptureResult {
                       name: "duration".to_string(),
                       value: Value::from_f64(1.5),
                   });
    }

    #[test]
    fn test_subquery_value_regex() {
        // regex "Hello (.*)!"
        let variables = HashMap::new();
        let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
        let subquery = Subquery {
            source_info: SourceInfo::init(1, 1, 1, 20),
            value: SubqueryValue::Regex {
                space0: whitespace,
                expr: Template {
                    quotes: false,
                    elements: vec![
                        TemplateElement::String { value: "Hello (.*)!".to_string(), encoded: "Hello (.*)!".to_string() }
                    ],
                    source_info: SourceInfo::init(1, 7, 1, 20),
                },
            },
        };
        assert_eq!(subquery.clone().eval(&variables, Value::String("Hello Bob!".to_string())).unwrap().unwrap(),
                   Value::String("Bob".to_string())
        );
        let error = subquery.eval(&variables, Value::Bool(true)).err().unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 1, 1, 20));
        assert_eq!(error.inner, RunnerError::SubqueryInvalidInput);
    }

    #[test]
    fn test_subquery_value_error() {
        let variables = HashMap::new();
        let whitespace = Whitespace { value: String::from(""), source_info: SourceInfo::init(0, 0, 0, 0) };
        let subquery = Subquery {
            source_info: SourceInfo::init(1, 1, 1, 20),
            value: SubqueryValue::Regex {
                space0: whitespace,
                expr: Template {
                    quotes: false,
                    elements: vec![
                        TemplateElement::String { value: "???".to_string(), encoded: "???".to_string() }
                    ],
                    source_info: SourceInfo::init(1, 7, 1, 20),
                },
            },
        };
        let error = subquery.eval(&variables, Value::String("Hello Bob!".to_string())).err().unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 20));
        assert_eq!(error.inner, RunnerError::InvalidRegex {});
    }
}
