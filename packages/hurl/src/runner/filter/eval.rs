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
use hurl_core::ast::{Filter, FilterValue};

use crate::runner::filter::base64_decode::eval_base64_decode;
use crate::runner::filter::base64_encode::eval_base64_encode;
use crate::runner::filter::count::eval_count;
use crate::runner::filter::days_after_now::eval_days_after_now;
use crate::runner::filter::days_before_now::eval_days_before_now;
use crate::runner::filter::decode::eval_decode;
use crate::runner::filter::format::eval_format;
use crate::runner::filter::html_escape::eval_html_escape;
use crate::runner::filter::html_unescape::eval_html_unescape;
use crate::runner::filter::jsonpath::eval_jsonpath;
use crate::runner::filter::nth::eval_nth;
use crate::runner::filter::regex::eval_regex;
use crate::runner::filter::replace::eval_replace;
use crate::runner::filter::split::eval_split;
use crate::runner::filter::to_date::eval_to_date;
use crate::runner::filter::to_float::eval_to_float;
use crate::runner::filter::to_int::eval_to_int;
use crate::runner::filter::url_decode::eval_url_decode;
use crate::runner::filter::url_encode::eval_url_encode;
use crate::runner::filter::xpath::eval_xpath;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Apply successive `filter` to an input `value`.
/// Specify whether they are executed  `in_assert` or not.
pub fn eval_filters(
    filters: &[Filter],
    value: &Value,
    variables: &VariableSet,
    in_assert: bool,
) -> Result<Option<Value>, RunnerError> {
    let mut value = Some(value.clone());
    for filter in filters {
        value = if let Some(value) = value {
            eval_filter(filter, &value, variables, in_assert)?
        } else {
            return Err(RunnerError::new(
                filter.source_info,
                RunnerErrorKind::FilterMissingInput,
                in_assert,
            ));
        }
    }
    Ok(value)
}

/// Evaluates a `filter` with an input `value`, given a set of `variables`.
pub fn eval_filter(
    filter: &Filter,
    value: &Value,
    variables: &VariableSet,
    in_assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match &filter.value {
        FilterValue::Base64Decode => eval_base64_decode(value, filter.source_info, in_assert),
        FilterValue::Base64Encode => eval_base64_encode(value, filter.source_info, in_assert),
        FilterValue::Count => eval_count(value, filter.source_info, in_assert),
        FilterValue::DaysAfterNow => eval_days_after_now(value, filter.source_info, in_assert),
        FilterValue::DaysBeforeNow => eval_days_before_now(value, filter.source_info, in_assert),
        FilterValue::Decode { encoding, .. } => {
            eval_decode(value, encoding, variables, filter.source_info, in_assert)
        }
        FilterValue::Format { fmt, .. } => {
            eval_format(value, fmt, variables, filter.source_info, in_assert)
        }
        FilterValue::HtmlEscape => eval_html_escape(value, filter.source_info, in_assert),
        FilterValue::HtmlUnescape => eval_html_unescape(value, filter.source_info, in_assert),
        FilterValue::JsonPath { expr, .. } => {
            eval_jsonpath(value, expr, variables, filter.source_info, in_assert)
        }
        FilterValue::Regex {
            value: regex_value, ..
        } => eval_regex(value, regex_value, variables, filter.source_info, in_assert),
        FilterValue::Nth { n, .. } => eval_nth(value, filter.source_info, in_assert, n.as_u64()),
        FilterValue::Replace {
            old_value,
            new_value,
            ..
        } => eval_replace(
            value,
            variables,
            filter.source_info,
            in_assert,
            old_value,
            new_value,
        ),
        FilterValue::Split { sep, .. } => {
            eval_split(value, variables, filter.source_info, in_assert, sep)
        }
        FilterValue::ToDate { fmt, .. } => {
            eval_to_date(value, fmt, variables, filter.source_info, in_assert)
        }
        FilterValue::ToFloat => eval_to_float(value, filter.source_info, in_assert),
        FilterValue::ToInt => eval_to_int(value, filter.source_info, in_assert),
        FilterValue::UrlDecode => eval_url_decode(value, filter.source_info, in_assert),
        FilterValue::UrlEncode => eval_url_encode(value, filter.source_info, in_assert),
        FilterValue::XPath { expr, .. } => {
            eval_xpath(value, expr, variables, filter.source_info, in_assert)
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo};
    use hurl_core::reader::Pos;

    use crate::runner::filter::eval::eval_filters;
    use crate::runner::{Number, Value, VariableSet};

    #[test]
    fn test_filters() {
        let variables = VariableSet::new();

        assert_eq!(
            eval_filters(
                &[Filter {
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6)),
                    value: FilterValue::Count,
                }],
                &Value::List(vec![
                    Value::Number(Number::Integer(1)),
                    Value::Number(Number::Integer(2)),
                    Value::Number(Number::Integer(2)),
                ]),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(3))
        );
    }
}
