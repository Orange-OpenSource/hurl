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
use chrono::NaiveDateTime;
use std::collections::HashMap;

use percent_encoding::AsciiSet;

use hurl_core::ast::{Filter, FilterValue, RegexValue, SourceInfo, Template};

use crate::html;
use crate::runner::regex::eval_regex_value;
use crate::runner::template::eval_template;
use crate::runner::{Error, RunnerError, Value};

/// Apply successive `filters` to an input `value`.
/// Specify whether they are executed  `in_assert` or not.
pub fn eval_filters(
    filters: &Vec<Filter>,
    value: &Value,
    variables: &HashMap<String, Value>,
    in_assert: bool,
) -> Result<Value, Error> {
    let mut value = value.clone();
    for filter in filters {
        value = eval_filter(filter, &value, variables, in_assert)?;
    }
    Ok(value)
}

fn eval_filter(
    filter: &Filter,
    value: &Value,
    variables: &HashMap<String, Value>,
    in_assert: bool,
) -> Result<Value, Error> {
    match &filter.value {
        FilterValue::Count => eval_count(value, &filter.source_info, in_assert),
        FilterValue::Format { fmt, .. } => {
            eval_format(value, fmt, variables, &filter.source_info, in_assert)
        }
        FilterValue::HtmlEscape => eval_html_escape(value, &filter.source_info, in_assert),
        FilterValue::HtmlUnescape => eval_html_unescape(value, &filter.source_info, in_assert),
        FilterValue::Regex {
            value: regex_value, ..
        } => eval_regex(
            value,
            regex_value,
            variables,
            &filter.source_info,
            in_assert,
        ),
        FilterValue::Nth { n, .. } => eval_nth(value, &filter.source_info, in_assert, *n),
        FilterValue::Replace {
            old_value,
            new_value,
            ..
        } => eval_replace(
            value,
            variables,
            &filter.source_info,
            in_assert,
            old_value,
            new_value,
        ),
        FilterValue::Split { sep, .. } => {
            eval_split(value, variables, &filter.source_info, in_assert, sep)
        }
        FilterValue::ToDate { fmt, .. } => {
            eval_to_date(value, fmt, variables, &filter.source_info, in_assert)
        }
        FilterValue::ToInt => eval_to_int(value, &filter.source_info, in_assert),
        FilterValue::UrlDecode => eval_url_decode(value, &filter.source_info, in_assert),
        FilterValue::UrlEncode => eval_url_encode(value, &filter.source_info, in_assert),
    }
}

fn eval_regex(
    value: &Value,
    regex_value: &RegexValue,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
    assert: bool,
) -> Result<Value, Error> {
    let re = eval_regex_value(regex_value, variables)?;
    match value {
        Value::String(s) => match re.captures(s.as_str()) {
            Some(captures) => match captures.get(1) {
                Some(v) => Ok(Value::String(v.as_str().to_string())),
                None => Err(Error {
                    source_info: source_info.clone(),
                    inner: RunnerError::FilterRegexNoCapture {},
                    assert,
                }),
            },
            None => Err(Error {
                source_info: source_info.clone(),
                inner: RunnerError::FilterRegexNoCapture {},
                assert,
            }),
        },
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert,
        }),
    }
}

fn eval_count(value: &Value, source_info: &SourceInfo, assert: bool) -> Result<Value, Error> {
    match value {
        Value::List(values) => Ok(Value::Integer(values.len() as i64)),
        Value::Bytes(values) => Ok(Value::Integer(values.len() as i64)),
        Value::Nodeset(size) => Ok(Value::Integer(*size as i64)),
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert,
        }),
    }
}

fn eval_format(
    value: &Value,
    fmt: &Template,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
    assert: bool,
) -> Result<Value, Error> {
    let fmt = eval_template(fmt, variables)?;

    match value {
        Value::Date(value) => {
            let formatted = format!("{}", value.format(fmt.as_str()));
            Ok(Value::String(formatted))
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert,
        }),
    }
}

// does not encode "/"
// like Jinja template (https://jinja.palletsprojects.com/en/3.1.x/templates/#jinja-filters.urlencode)
fn eval_url_encode(value: &Value, source_info: &SourceInfo, assert: bool) -> Result<Value, Error> {
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
            assert,
        }),
    }
}

fn eval_url_decode(value: &Value, source_info: &SourceInfo, assert: bool) -> Result<Value, Error> {
    match value {
        Value::String(value) => {
            match percent_encoding::percent_decode(value.as_bytes()).decode_utf8() {
                Ok(decoded) => Ok(Value::String(decoded.to_string())),
                Err(_) => Err(Error {
                    source_info: source_info.clone(),
                    inner: RunnerError::FilterInvalidInput("Invalid UTF8 stream".to_string()),
                    assert,
                }),
            }
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert,
        }),
    }
}

fn eval_nth(value: &Value, source_info: &SourceInfo, assert: bool, n: u64) -> Result<Value, Error> {
    match value {
        Value::List(values) => match values.get(n as usize) {
            None => Err(Error {
                source_info: source_info.clone(),
                inner: RunnerError::FilterInvalidInput(format!(
                    "Out of bound - size is {}",
                    values.len()
                )),
                assert,
            }),
            Some(value) => Ok(value.clone()),
        },
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v.display()),
            assert,
        }),
    }
}

fn eval_html_escape(value: &Value, source_info: &SourceInfo, assert: bool) -> Result<Value, Error> {
    match value {
        Value::String(value) => {
            let encoded = html::html_escape(value);
            Ok(Value::String(encoded))
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert,
        }),
    }
}

fn eval_html_unescape(
    value: &Value,
    source_info: &SourceInfo,
    assert: bool,
) -> Result<Value, Error> {
    match value {
        Value::String(value) => {
            let decoded = html::html_unescape(value);
            Ok(Value::String(decoded))
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert,
        }),
    }
}

fn eval_replace(
    value: &Value,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
    assert: bool,
    old_value: &RegexValue,
    new_value: &Template,
) -> Result<Value, Error> {
    match value {
        Value::String(v) => {
            let re = eval_regex_value(old_value, variables)?;
            let new_value = eval_template(new_value, variables)?;
            let s = re.replace_all(v, new_value).to_string();
            Ok(Value::String(s))
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v.display()),
            assert,
        }),
    }
}

fn eval_split(
    value: &Value,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
    assert: bool,
    sep: &Template,
) -> Result<Value, Error> {
    match value {
        Value::String(s) => {
            let sep = eval_template(sep, variables)?;
            let values = s
                .split(&sep)
                .map(|v| Value::String(v.to_string()))
                .collect();
            Ok(Value::List(values))
        }
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v.display()),
            assert,
        }),
    }
}

fn eval_to_date(
    value: &Value,
    fmt: &Template,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
    assert: bool,
) -> Result<Value, Error> {
    let fmt = eval_template(fmt, variables)?;

    match value {
        Value::String(v) => match NaiveDateTime::parse_from_str(v, fmt.as_str()) {
            Ok(v) => Ok(Value::Date(v.and_local_timezone(chrono::Utc).unwrap())),
            Err(_) => Err(Error {
                source_info: source_info.clone(),
                inner: RunnerError::FilterInvalidInput(value.display()),
                assert,
            }),
        },
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v.display()),
            assert,
        }),
    }
}

fn eval_to_int(value: &Value, source_info: &SourceInfo, assert: bool) -> Result<Value, Error> {
    match value {
        Value::Integer(v) => Ok(Value::Integer(*v)),
        Value::Float(v) => Ok(Value::Integer(*v as i64)),
        Value::String(v) => match v.parse::<i64>() {
            Ok(i) => Ok(Value::Integer(i)),
            Err(_) => Err(Error {
                source_info: source_info.clone(),
                inner: RunnerError::FilterInvalidInput(value.display()),
                assert,
            }),
        },
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v.display()),
            assert,
        }),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::offset::Utc;
    use chrono::prelude::*;
    use hurl_core::ast::{FilterValue, SourceInfo, Template, TemplateElement, Whitespace};

    pub fn filter_count() -> Filter {
        Filter {
            source_info: SourceInfo::new(1, 1, 1, 6),
            value: FilterValue::Count,
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
                false,
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
                false,
            )
            .unwrap(),
            Value::Integer(3)
        );

        let error = eval_filter(&filter_count(), &Value::Bool(true), &variables, false)
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::new(1, 1, 1, 6));
        assert_eq!(
            error.inner,
            RunnerError::FilterInvalidInput("boolean".to_string())
        );
    }

    #[test]
    pub fn eval_filter_format() {
        // let naivedatetime_utc = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(2, 0, 0).unwrap();
        //let datetime_utc = DateTime::<Utc>::from_utc(naivedatetime_utc, Utc);

        let variables = HashMap::new();
        let whitespace = Whitespace {
            value: String::from(""),
            source_info: SourceInfo::new(0, 0, 0, 0),
        };
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 20),
            value: FilterValue::Format {
                space0: whitespace,
                fmt: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "%d/%m/%Y %H:%M".to_string(),
                        encoded: "%d/%m/%Y %H:%M".to_string(),
                    }],
                    source_info: SourceInfo::new(1, 7, 1, 20),
                },
            },
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::Date(Utc.with_ymd_and_hms(2017, 04, 02, 12, 50, 32).unwrap()),
                &variables,
                false,
            )
            .unwrap(),
            Value::String("02/04/2017 12:50".to_string())
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
                false,
            )
            .unwrap(),
            Value::String("Bob".to_string())
        );

        let error = eval_filter(&filter, &Value::Bool(true), &variables, false)
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
            false,
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
            value: FilterValue::UrlEncode,
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("https://mozilla.org/?x=шеллы".to_string()),
                &variables,
                false,
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
            value: FilterValue::UrlDecode,
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B".to_string()),
                &variables,
                false,
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
            value: FilterValue::ToInt,
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("123".to_string()),
                &variables,
                false
            )
            .unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            eval_filter(&filter, &Value::Integer(123), &variables, false).unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            eval_filter(&filter, &Value::Float(1.6), &variables, false).unwrap(),
            Value::Integer(1)
        );
    }

    #[test]
    pub fn eval_filter_to_int_error() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::ToInt,
        };
        let err = eval_filter(
            &filter,
            &Value::String("123x".to_string()),
            &variables,
            false,
        )
        .err()
        .unwrap();
        assert_eq!(
            err.inner,
            RunnerError::FilterInvalidInput("string <123x>".to_string())
        );
        let err = eval_filter(&filter, &Value::Bool(true), &variables, false)
            .err()
            .unwrap();
        assert_eq!(
            err.inner,
            RunnerError::FilterInvalidInput("bool <true>".to_string())
        );
    }

    #[test]
    pub fn eval_filter_html_escape() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::HtmlEscape,
        };

        let tests = vec![
            ("foo", "foo"),
            ("<tag>", "&lt;tag&gt;"),
            ("foo & bar", "foo &amp; bar"),
            (
                "string with double quote: \"baz\"",
                "string with double quote: &quot;baz&quot;",
            ),
        ];
        for (input, output) in tests.iter() {
            assert_eq!(
                eval_filter(
                    &filter,
                    &Value::String(input.to_string()),
                    &variables,
                    false
                )
                .unwrap(),
                Value::String(output.to_string())
            );
        }
    }

    #[test]
    pub fn eval_filter_html_unescape() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::HtmlUnescape,
        };

        let tests = vec![
            ("foo", "foo"),
            ("&lt;tag&gt;", "<tag>"),
            ("foo &amp; bar", "foo & bar"),
            (
                "string with double quote: &quot;baz&quot;",
                "string with double quote: \"baz\"",
            ),
        ];
        for (input, output) in tests.iter() {
            assert_eq!(
                eval_filter(
                    &filter,
                    &Value::String(input.to_string()),
                    &variables,
                    false
                )
                .unwrap(),
                Value::String(output.to_string())
            );
        }
    }

    #[test]
    pub fn eval_filter_nth() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::Nth {
                n: 2,
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
            },
        };

        assert_eq!(
            eval_filter(
                &filter,
                &Value::List(vec![
                    Value::Integer(0),
                    Value::Integer(1),
                    Value::Integer(2),
                    Value::Integer(3)
                ]),
                &variables,
                false
            )
            .unwrap(),
            Value::Integer(2)
        );
        assert_eq!(
            eval_filter(
                &filter,
                &Value::List(vec![Value::Integer(0), Value::Integer(1)]),
                &variables,
                false
            )
            .err()
            .unwrap(),
            Error {
                source_info: SourceInfo::new(1, 1, 1, 1),
                inner: RunnerError::FilterInvalidInput("Out of bound - size is 2".to_string()),
                assert: false
            }
        );
    }

    #[test]
    pub fn eval_filter_replace() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::Replace {
                old_value: RegexValue::Template(Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\\s+".to_string(),
                        encoded: ",".to_string(),
                    }],
                    source_info: SourceInfo::new(1, 7, 1, 20),
                }),
                new_value: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: ",".to_string(),
                        encoded: ",".to_string(),
                    }],
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
                space1: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
            },
        };

        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("1 2\t3  4".to_string()),
                &variables,
                false
            )
            .unwrap(),
            Value::String("1,2,3,4".to_string())
        );
    }

    #[test]
    pub fn eval_filter_split() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::Split {
                sep: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: ",".to_string(),
                        encoded: ",".to_string(),
                    }],
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
            },
        };

        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("1,2,3".to_string()),
                &variables,
                false
            )
            .unwrap(),
            Value::List(vec![
                Value::String("1".to_string()),
                Value::String("2".to_string()),
                Value::String("3".to_string()),
            ])
        );
    }

    #[test]
    pub fn eval_filter_to_date() {
        let variables = HashMap::new();

        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::ToDate {
                fmt: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "%Y %b %d %H:%M:%S%.3f %z".to_string(),
                        encoded: "%Y %b %d %H:%M:%S%.3f %z".to_string(),
                    }],
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
            },
        };

        let naivedatetime_utc = NaiveDate::from_ymd_opt(1983, 4, 13)
            .unwrap()
            .and_hms_micro_opt(12, 9, 14, 274000)
            .unwrap();
        let datetime_utc = DateTime::<Utc>::from_utc(naivedatetime_utc, Utc);
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("1983 Apr 13 12:09:14.274 +0000".to_string()),
                &variables,
                false
            )
            .unwrap(),
            Value::Date(datetime_utc)
        );

        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::ToDate {
                fmt: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "%a, %d %b %Y %H:%M:%S GMT".to_string(),
                        encoded: "%a, %d %b %Y %H:%M:%S GMT".to_string(),
                    }],
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
            },
        };

        let naivedatetime_utc = NaiveDate::from_ymd_opt(2015, 10, 21)
            .unwrap()
            .and_hms_opt(7, 28, 0)
            .unwrap();
        let datetime_utc = DateTime::<Utc>::from_utc(naivedatetime_utc, Utc);
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
                &variables,
                false
            )
            .unwrap(),
            Value::Date(datetime_utc)
        );
    }
}
