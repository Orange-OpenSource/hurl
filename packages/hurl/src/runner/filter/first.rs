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

/// Returns the first item in a collection `value`.
pub fn eval_first(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::List(values) => match values.first().cloned() {
            Some(first_value) => Ok(Some(first_value)),
            None => {
                let kind = RunnerErrorKind::FilterInvalidInput("list is empty".to_string());
                Err(RunnerError::new(source_info, kind, assert))
            }
        },
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo};
    use hurl_core::reader::Pos;

    use super::*;
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Number, VariableSet};

    fn new_first_filter() -> Filter {
        Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 5)),
            value: FilterValue::First,
        }
    }

    #[test]
    fn eval_filter_first_ok() {
        let variables = VariableSet::new();

        let filter = new_first_filter();

        let ret = eval_filter(
            &filter,
            &Value::List(vec![
                Value::Number(Number::Integer(1)),
                Value::Number(Number::Integer(3)),
                Value::Number(Number::Integer(5)),
            ]),
            &variables,
            false,
        );

        assert_eq!(ret.unwrap().unwrap(), Value::Number(Number::Integer(1)));
    }

    #[test]
    fn eval_filter_first_ko_empty_list() {
        let variables = VariableSet::new();

        let filter = new_first_filter();

        let ret = eval_filter(&filter, &Value::List(vec![]), &variables, false);

        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("list is empty".to_string())
        );
    }

    #[test]
    fn eval_filter_first_ko_invalid_input() {
        let variables = VariableSet::new();

        let filter = new_first_filter();

        let ret = eval_filter(&filter, &Value::Bool(true), &variables, false);

        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("boolean".to_string())
        );
    }
}
