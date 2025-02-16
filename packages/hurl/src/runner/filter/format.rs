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
use std::fmt::Write;

use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Formats a date `value` to a string given a specification `format`.
/// See <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>
pub fn eval_format(
    value: &Value,
    format: &Template,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    let format = eval_template(format, variables)?;

    match value {
        Value::Date(value) => {
            let mut formatted = String::new();
            match write!(formatted, "{}", value.format(format.as_str())) {
                Ok(_) => Ok(Some(Value::String(formatted))),
                Err(_) => {
                    let kind = RunnerErrorKind::FilterInvalidFormatSpecifier(format);
                    Err(RunnerError::new(source_info, kind, assert))
                }
            }
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::offset::Utc;
    use chrono::TimeZone;
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use super::*;
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::VariableSet;

    /// Helper function to return a new filter given a `fmt`
    fn new_format_filter(fmt: &str) -> Filter {
        // Example: format "%m/%d/%Y"
        Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Format {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(7, 1), Pos::new(8, 1)),
                },
                fmt: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: fmt.to_string(),
                        source: fmt.to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(8, 1), Pos::new(8 + fmt.len(), 1)),
                },
            },
        }
    }

    #[test]
    fn eval_filter_format_ok() {
        let variables = VariableSet::new();

        let date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let filter = new_format_filter("%m/%d/%Y");
        let ret = eval_filter(&filter, &Value::Date(date), &variables, false);
        assert_eq!(
            ret.unwrap().unwrap(),
            Value::String("01/01/2025".to_string())
        );
    }

    #[test]
    fn eval_filter_format_ko_bad_input_type() {
        let variables = VariableSet::new();

        let filter = new_format_filter("%m/%d/%Y");
        let ret = eval_filter(
            &filter,
            &Value::String("01/01/2025".to_string()),
            &variables,
            false,
        );
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("string".to_string())
        );
    }

    #[test]
    fn eval_filter_format_ko_invalid_format() {
        let variables = VariableSet::new();

        let date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let filter = new_format_filter("%%%");
        let ret = eval_filter(&filter, &Value::Date(date), &variables, false);
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidFormatSpecifier("%%%".to_string())
        );
    }
}
