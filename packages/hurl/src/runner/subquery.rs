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

use super::core::{Error, RunnerError};
use super::template::eval_template;
use super::value::Value;
use hurl_core::ast::*;
use regex::Regex;
use std::collections::HashMap;

pub fn eval_subquery(
    subquery: Subquery,
    value: Value,
    variables: &HashMap<String, Value>,
) -> Result<Option<Value>, Error> {
    match subquery.value {
        SubqueryValue::Regex {
            value: regex_value, ..
        } => eval_regex(value, regex_value, variables, subquery.source_info),
        SubqueryValue::Count {} => eval_count(value, subquery.source_info),
    }
}

fn eval_regex(
    value: Value,
    regex_value: RegexValue,
    variables: &HashMap<String, Value>,
    source_info: SourceInfo,
) -> Result<Option<Value>, Error> {
    let re = match regex_value {
        RegexValue::Template(t) => {
            let value = eval_template(&t, variables)?;
            match Regex::new(value.as_str()) {
                Ok(re) => re,
                Err(_) => {
                    return Err(Error {
                        source_info: t.source_info,
                        inner: RunnerError::InvalidRegex(),
                        assert: false,
                    })
                }
            }
        }
        RegexValue::Regex(re) => re.inner,
    };

    match value {
        Value::String(s) => match re.captures(s.as_str()) {
            Some(captures) => match captures.get(1) {
                Some(v) => Ok(Some(Value::String(v.as_str().to_string()))),
                None => Ok(None),
            },
            None => Ok(None),
        },
        v => Err(Error {
            source_info,
            inner: RunnerError::SubqueryInvalidInput(v._type()),
            assert: false,
        }),
    }
}

fn eval_count(value: Value, source_info: SourceInfo) -> Result<Option<Value>, Error> {
    match value {
        Value::List(values) => Ok(Some(Value::Integer(values.len() as i64))),
        Value::Bytes(values) => Ok(Some(Value::Integer(values.len() as i64))),
        Value::Nodeset(size) => Ok(Some(Value::Integer(size as i64))),
        v => Err(Error {
            source_info,
            inner: RunnerError::SubqueryInvalidInput(v._type()),
            assert: false,
        }),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use hurl_core::ast::SourceInfo;

    #[test]
    fn test_subquery_regex() {
        // regex "Hello (.*)!"
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let subquery = Subquery {
            source_info: SourceInfo::init(1, 1, 1, 20),
            value: SubqueryValue::Regex {
                space0: whitespace,
                value: RegexValue::Template(Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "Hello (.*)!".to_string(),
                        encoded: "Hello (.*)!".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 7, 1, 20),
                }),
            },
        };
        assert_eq!(
            eval_subquery(
                subquery.clone(),
                Value::String("Hello Bob!".to_string()),
                &variables,
            )
            .unwrap()
            .unwrap(),
            Value::String("Bob".to_string())
        );

        let error = eval_subquery(subquery, Value::Bool(true), &variables)
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 1, 1, 20));
        assert_eq!(
            error.inner,
            RunnerError::SubqueryInvalidInput("boolean".to_string())
        );
    }

    #[test]
    fn test_subquery_invalid_regex() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(""),
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        let subquery = Subquery {
            source_info: SourceInfo::init(1, 1, 1, 20),
            value: SubqueryValue::Regex {
                space0: whitespace,
                value: RegexValue::Template(Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "???".to_string(),
                        encoded: "???".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 7, 1, 20),
                }),
            },
        };
        let error = eval_subquery(
            subquery,
            Value::String("Hello Bob!".to_string()),
            &variables,
        )
        .err()
        .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 7, 1, 20));
        assert_eq!(error.inner, RunnerError::InvalidRegex {});
    }

    #[test]
    fn test_subquery_count() {
        let variables = HashMap::new();
        let subquery = Subquery {
            source_info: SourceInfo::init(1, 1, 1, 20),
            value: SubqueryValue::Count {},
        };
        assert_eq!(
            eval_subquery(
                subquery.clone(),
                Value::List(vec![
                    Value::Integer(1),
                    Value::Integer(2),
                    Value::Integer(3),
                ]),
                &variables,
            )
            .unwrap()
            .unwrap(),
            Value::Integer(3)
        );

        let error = eval_subquery(subquery, Value::Bool(true), &variables)
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 1, 1, 20));
        assert_eq!(
            error.inner,
            RunnerError::SubqueryInvalidInput("boolean".to_string())
        );
    }
}
