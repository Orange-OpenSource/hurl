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
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
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
        Value::String(v) => {
            // Chrono’s parser enforces strictly parsing on `DateTime`, `NaiveDateTime` and `NaiveDate`.
            // If we try to parse a string "2024-12-31" into a `NaiveDateTime`, Chrono
            // considers that there are missing information (time) and can't parse this value. As we
            // can't enforce the user input date format, we heuristically try to parse it from the richer
            // format to the information-less format: date + time + timezone, date + time and finally
            // date.
            if let Ok(dt) = DateTime::parse_from_str(v, format.as_str()) {
                return Ok(Some(Value::Date(dt.with_timezone(&Utc))));
            }

            if let Ok(dt) = NaiveDateTime::parse_from_str(v, format.as_str()) {
                return Ok(Some(Value::Date(
                    DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc),
                )));
            }

            if let Ok(date) = NaiveDate::parse_from_str(v, format.as_str()) {
                let dt = date.and_hms_opt(0, 0, 0).unwrap();
                return Ok(Some(Value::Date(
                    DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc),
                )));
            }

            let kind = RunnerErrorKind::FilterDateParsingError {
                date: v.to_string(),
                format,
            };
            Err(RunnerError::new(source_info, kind, assert))
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.repr());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::types::ToSource;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{RunnerErrorKind, Value, VariableSet};

    #[test]
    fn eval_filter_to_date_ok_with_timezone() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToDate {
                fmt: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "%Y-%m-%d %H:%M:%S %:z".to_string(),
                        source: "%Y-%m-%d %H:%M:%S %:z".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        };

        let datetime_utc =
            DateTime::parse_from_str("Thu Aug 27 09:07:46 2020 +0200", "%a %b %d %H:%M:%S %Y %z")
                .unwrap()
                .with_timezone(&Utc);

        let ret = eval_filter(
            &filter,
            &Value::String("2020-08-27 09:07:46 +02:00".to_string()),
            &variables,
            false,
        );
        assert_eq!(ret.unwrap().unwrap(), Value::Date(datetime_utc));
    }

    #[test]
    fn eval_filter_to_date_ok_without_timezone() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToDate {
                fmt: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "%Y-%m-%d %H:%M:%S".to_string(),
                        source: "%Y-%m-%d %H:%M:%S".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        };

        let datetime_utc =
            NaiveDateTime::parse_from_str("2020-08-27 09:07:46", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc();

        let ret = eval_filter(
            &filter,
            &Value::String("2020-08-27 09:07:46".to_string()),
            &variables,
            false,
        );
        assert_eq!(ret.unwrap().unwrap(), Value::Date(datetime_utc));
    }

    #[test]
    fn eval_filter_to_date_ok_date_only() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToDate {
                fmt: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "%Y-%m-%d".to_string(),
                        source: "%Y-%m-%d".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        };

        let datetime_utc: DateTime<Utc> = NaiveDate::parse_from_str("2020-08-27", "%Y-%m-%d")
            .unwrap()
            .and_time(NaiveTime::MIN)
            .and_utc();

        let ret = eval_filter(
            &filter,
            &Value::String("2020-08-27".to_string()),
            &variables,
            false,
        );
        assert_eq!(ret.unwrap().unwrap(), Value::Date(datetime_utc));
    }

    #[test]
    fn eval_filter_to_date_ko_invalid_format() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToDate {
                fmt: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "ymd".to_string(),
                        source: "ymd".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        };

        let ret = eval_filter(
            &filter,
            &Value::String("2020-08-27".to_string()),
            &variables,
            false,
        );
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterDateParsingError {
                date: "2020-08-27".to_string(),
                format: "ymd".to_string(),
            }
        );
    }

    #[test]
    fn eval_filter_to_date_ko_invalid_input() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToDate {
                fmt: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "%Y-%m-%d".to_string(),
                        source: "%Y-%m-%d".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        };

        let ret = eval_filter(
            &filter,
            &Value::Bytes([0xc4, 0xe3, 0xba].to_vec()),
            &variables,
            false,
        );
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("bytes <c4e3ba>".to_string())
        );
    }
}
