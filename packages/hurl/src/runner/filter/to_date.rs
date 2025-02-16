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
use chrono::NaiveDateTime;
use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Converts a string to a date given a specification `format`.
/// See <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>
pub fn eval_to_date(
    value: &Value,
    format: &Template,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    let format = eval_template(format, variables)?;

    match value {
        Value::String(v) => match NaiveDateTime::parse_from_str(v, format.as_str()) {
            Ok(v) => Ok(Some(Value::Date(
                v.and_local_timezone(chrono::Utc).unwrap(),
            ))),
            Err(_) => {
                let kind = RunnerErrorKind::FilterInvalidInput(value.repr());
                Err(RunnerError::new(source_info, kind, assert))
            }
        },
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.repr());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, Utc};
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Value, VariableSet};

    #[test]
    fn eval_filter_to_date() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToDate {
                fmt: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "%Y %b %d %H:%M:%S%.3f %z".to_string(),
                        source: "%Y %b %d %H:%M:%S%.3f %z".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
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
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToDate {
                fmt: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "%a, %d %b %Y %H:%M:%S GMT".to_string(),
                        source: "%a, %d %b %Y %H:%M:%S GMT".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
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
