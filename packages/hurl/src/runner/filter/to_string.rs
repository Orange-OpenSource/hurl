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
use hurl_core::ast::SourceInfo;

use crate::runner::{RunnerError, RunnerErrorKind, Value};

/// Converts `value` to an string.
///
/// return a RunnerError if the value is not renderable
pub fn eval_to_string(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value.render() {
        Some(value) => Ok(Some(Value::String(value))),
        None => {
            let kind = RunnerErrorKind::FilterInvalidInput(format!(
                "{} can not be converted to a string",
                value.repr()
            ));
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo};
    use hurl_core::reader::Pos;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Number, RunnerErrorKind, Value, VariableSet};

    #[test]
    fn eval_filter_to_string() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToString,
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::Number(Number::Integer(100)),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::String("100".to_string())
        );
    }

    #[test]
    fn eval_filter_to_string_error() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToString,
        };
        let err = eval_filter(&filter, &Value::List(vec![]), &variables, false)
            .err()
            .unwrap();
        assert_eq!(
            err.kind,
            RunnerErrorKind::FilterInvalidInput(
                "list <[]> can not be converted to a string".to_string()
            )
        );
    }
}
