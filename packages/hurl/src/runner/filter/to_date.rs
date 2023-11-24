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

use chrono::NaiveDateTime;

use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{Error, RunnerError, Value};

pub fn eval_to_date(
    value: &Value,
    fmt: &Template,
    variables: &HashMap<String, Value>,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, Error> {
    let fmt = eval_template(fmt, variables)?;

    match value {
        Value::String(v) => match NaiveDateTime::parse_from_str(v, fmt.as_str()) {
            Ok(v) => Ok(Some(Value::Date(
                v.and_local_timezone(chrono::Utc).unwrap(),
            ))),
            Err(_) => Err(Error {
                source_info,
                inner: RunnerError::FilterInvalidInput(value.display()),
                assert,
            }),
        },
        v => Err(Error {
            source_info,
            inner: RunnerError::FilterInvalidInput(v.display()),
            assert,
        }),
    }
}

#[cfg(test)]
pub mod tests {

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::Value;
    use chrono::{DateTime, NaiveDate, Utc};
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use std::collections::HashMap;

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
                    value: String::new(),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
            },
        };

        let naive_datetime_utc = NaiveDate::from_ymd_opt(1983, 4, 13)
            .unwrap()
            .and_hms_micro_opt(12, 9, 14, 274000)
            .unwrap();
        let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime_utc, Utc);
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("1983 Apr 13 12:09:14.274 +0000".to_string()),
                &variables,
                false
            )
            .unwrap()
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
                    value: String::new(),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
            },
        };

        let naivedatetime_utc = NaiveDate::from_ymd_opt(2015, 10, 21)
            .unwrap()
            .and_hms_opt(7, 28, 0)
            .unwrap();
        let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(naivedatetime_utc, Utc);
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::Date(datetime_utc)
        );
    }
}
