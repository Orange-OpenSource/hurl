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
use percent_encoding::AsciiSet;
use regex::Regex;
use std::collections::HashMap;

// TODO: indicated whether you running the filter in an assert / this produce an "assert" error
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
        FilterValue::UrlEncode { .. } => eval_url_encode(value, &filter.source_info),
        FilterValue::UrlDecode { .. } => eval_url_decode(value, &filter.source_info),
        FilterValue::HtmlEscape { .. } => eval_html_encode(value, &filter.source_info),
        FilterValue::HtmlUnescape { .. } => eval_html_decode(value, &filter.source_info),
        FilterValue::ToInt { .. } => eval_to_int(value, &filter.source_info),
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

// does not encopde "/"
// like Jinja template (https://jinja.palletsprojects.com/en/3.1.x/templates/#jinja-filters.urlencode)
fn eval_url_encode(value: &Value, source_info: &SourceInfo) -> Result<Value, Error> {
    match value {
        Value::String(value) => {
            const FRAGMENT: &AsciiSet = &percent_encoding::NON_ALPHANUMERIC
                .remove(b'-')
                .remove(b'.')
                .remove(b'_')
                .remove(b'~')
                .remove(b'/');
            let encoded = percent_encoding::percent_encode(value.as_bytes(), FRAGMENT).to_string();
            Ok(Value::String(encoded))
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert: false,
        }),
    }
}

fn eval_url_decode(value: &Value, source_info: &SourceInfo) -> Result<Value, Error> {
    match value {
        Value::String(value) => {
            match percent_encoding::percent_decode(value.as_bytes()).decode_utf8() {
                Ok(decoded) => Ok(Value::String(decoded.to_string())),
                Err(_) => Err(Error {
                    source_info: source_info.clone(),
                    inner: RunnerError::FilterInvalidInput("Invalid UTF8 stream".to_string()),
                    assert: false,
                }),
            }
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert: false,
        }),
    }
}

fn eval_html_encode(value: &Value, source_info: &SourceInfo) -> Result<Value, Error> {
    match value {
        Value::String(value) => {
            let mut enco = String::from(value);
            let encoded = html_escape::encode_text_to_string(value, &mut enco);
            Ok(Value::String(encoded.to_string()))
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert: false,
        }),
    }
}

fn eval_html_decode(value: &Value, source_info: &SourceInfo) -> Result<Value, Error> {
    match value {
        Value::String(value) => {
            let decoded = html_escape::decode_html_entities(value).to_string();
            Ok(Value::String(decoded))
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert: false,
        }),
    }
}

fn eval_to_int(value: &Value, source_info: &SourceInfo) -> Result<Value, Error> {
    match value {
        Value::Integer(v) => Ok(Value::Integer(*v)),
        Value::Float(v) => Ok(Value::Integer(*v as i64)),
        Value::String(v) => match v.parse::<i64>() {
            Ok(i) => Ok(Value::Integer(i)),
            Err(_) => Err(Error {
                source_info: source_info.clone(),
                inner: RunnerError::FilterInvalidInput(value.display()),
                assert: false,
            }),
        },
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v.display()),
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
                    delimiter: None,
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
                    delimiter: None,
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

    #[test]
    pub fn eval_filter_url_encode() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(0, 0, 0, 0),
            value: FilterValue::UrlEncode {},
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("https://mozilla.org/?x=шеллы".to_string()),
                &variables,
            )
            .unwrap(),
            Value::String(
                "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B".to_string()
            )
        );
    }

    #[test]
    pub fn eval_filter_url_decode() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(0, 0, 0, 0),
            value: FilterValue::UrlDecode {},
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B".to_string()),
                &variables,
            )
            .unwrap(),
            Value::String("https://mozilla.org/?x=шеллы".to_string())
        );
    }

    #[test]
    pub fn eval_filter_to_int() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::ToInt {},
        };
        assert_eq!(
            eval_filter(&filter, &Value::String("123".to_string()), &variables).unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            eval_filter(&filter, &Value::Integer(123), &variables).unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            eval_filter(&filter, &Value::Float(1.6), &variables).unwrap(),
            Value::Integer(1)
        );
    }

    #[test]
    pub fn eval_filter_to_int_error() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::ToInt {},
        };
        let err = eval_filter(&filter, &Value::String("123x".to_string()), &variables)
            .err()
            .unwrap();
        assert_eq!(
            err.inner,
            RunnerError::FilterInvalidInput("string <123x>".to_string())
        );
        let err = eval_filter(&filter, &Value::Bool(true), &variables)
            .err()
            .unwrap();
        assert_eq!(
            err.inner,
            RunnerError::FilterInvalidInput("bool <true>".to_string())
        );
    }
}
