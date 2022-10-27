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
use crate::runner::template::eval_template;
use crate::runner::{Error, RunnerError, Value};
use hurl_core::ast::{Filter, FilterValue, RegexValue, SourceInfo};
use regex::Regex;
use std::collections::HashMap;

pub fn eval_filters(
    filters: &Vec<Filter>,
    value: &Value,
    variables: &HashMap<String, Value>,
) -> Result<Value, Error> {
    let mut value = value.clone();
    for filter in filters {
        value = eval_filter(filter, &value, variables)?;
    }
    Ok(value)
}

fn eval_filter(
    filter: &Filter,
    value: &Value,
    variables: &HashMap<String, Value>,
) -> Result<Value, Error> {
    match &filter.value {
        FilterValue::Regex {
            value: regex_value, ..
        } => eval_regex(value, regex_value, variables, &filter.source_info),
        FilterValue::Count {} => eval_count(value, &filter.source_info),
        FilterValue::EscapeUrl { .. } => todo!(),
        FilterValue::UnEscapeUrl { .. } => todo!(),
    }
}

fn eval_regex(
    value: &Value,
    regex_value: &RegexValue,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
) -> Result<Value, Error> {
    let re = match regex_value {
        RegexValue::Template(t) => {
            let value = eval_template(t, variables)?;
            match Regex::new(value.as_str()) {
                Ok(re) => re,
                Err(_) => {
                    return Err(Error {
                        source_info: t.source_info.clone(),
                        inner: RunnerError::InvalidRegex(),
                        assert: false,
                    });
                }
            }
        }
        RegexValue::Regex(re) => re.inner.clone(),
    };

    match value {
        Value::String(s) => match re.captures(s.as_str()) {
            Some(captures) => match captures.get(1) {
                Some(v) => Ok(Value::String(v.as_str().to_string())),
                None => Err(Error {
                    source_info: source_info.clone(),
                    inner: RunnerError::FilterRegexNoCapture {},
                    assert: false,
                }),
            },
            None => Err(Error {
                source_info: source_info.clone(),
                inner: RunnerError::FilterRegexNoCapture {},
                assert: false,
            }),
        },
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert: false,
        }),
    }
}

fn eval_count(value: &Value, source_info: &SourceInfo) -> Result<Value, Error> {
    match value {
        Value::List(values) => Ok(Value::Integer(values.len() as i64)),
        Value::Bytes(values) => Ok(Value::Integer(values.len() as i64)),
        Value::Nodeset(size) => Ok(Value::Integer(*size as i64)),
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert: false,
        }),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use hurl_core::ast::{FilterValue, SourceInfo, Template, TemplateElement, Whitespace};

    pub fn filter_count() -> Filter {
        Filter {
            source_info: SourceInfo::new(1, 1, 1, 6),
            value: FilterValue::Count {},
        }
    }

    #[test]
    pub fn test_filters() {
        let variables = HashMap::new();

        assert_eq!(
            eval_filters(
                &vec![filter_count()],
                &Value::List(vec![
                    Value::Integer(1),
                    Value::Integer(2),
                    Value::Integer(2),
                ]),
                &variables,
            )
            .unwrap(),
            Value::Integer(3)
        );
    }

    #[test]
    pub fn eval_filter_count() {
        let variables = HashMap::new();

        assert_eq!(
            eval_filter(
                &filter_count(),
                &Value::List(vec![
                    Value::Integer(1),
                    Value::Integer(2),
                    Value::Integer(2),
                ]),
                &variables,
            )
            .unwrap(),
            Value::Integer(3)
        );

        let error = eval_filter(&filter_count(), &Value::Bool(true), &variables)
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::new(1, 1, 1, 6));
        assert_eq!(
            error.inner,
            RunnerError::FilterInvalidInput("boolean".to_string())
        );
    }

    #[test]
    fn eval_filter_regex() {
        // regex "Hello (.*)!"
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(""),
            source_info: SourceInfo::new(0, 0, 0, 0),
        };
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 20),
            value: FilterValue::Regex {
                space0: whitespace,
                value: RegexValue::Template(Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "Hello (.*)!".to_string(),
                        encoded: "Hello (.*)!".to_string(),
                    }],
                    source_info: SourceInfo::new(1, 7, 1, 20),
                }),
            },
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("Hello Bob!".to_string()),
                &variables,
            )
            .unwrap(),
            Value::String("Bob".to_string())
        );

        let error = eval_filter(&filter, &Value::Bool(true), &variables)
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::new(1, 1, 1, 20));
        assert_eq!(
            error.inner,
            RunnerError::FilterInvalidInput("boolean".to_string())
        );
    }

    #[test]
    fn eval_filter_invalid_regex() {
        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(""),
            source_info: SourceInfo::new(0, 0, 0, 0),
        };
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 20),
            value: FilterValue::Regex {
                space0: whitespace,
                value: RegexValue::Template(Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "???".to_string(),
                        encoded: "???".to_string(),
                    }],
                    source_info: SourceInfo::new(1, 7, 1, 20),
                }),
            },
        };
        let error = eval_filter(
            &filter,
            &Value::String("Hello Bob!".to_string()),
            &variables,
        )
        .err()
        .unwrap();
        assert_eq!(error.source_info, SourceInfo::new(1, 7, 1, 20));
        assert_eq!(error.inner, RunnerError::InvalidRegex {});
    }
}
