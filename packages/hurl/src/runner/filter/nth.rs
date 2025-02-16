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

/// Returns the element from a collection `value` at a zero-based index.
pub fn eval_nth(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
    n: u64,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::List(values) => match values.get(n as usize) {
            None => {
                let kind = RunnerErrorKind::FilterInvalidInput(format!(
                    "Out of bound - size is {}",
                    values.len()
                ));
                Err(RunnerError::new(source_info, kind, assert))
            }
            Some(value) => Ok(Some(value.clone())),
        },
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.repr());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Whitespace, U64};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Number, RunnerError, RunnerErrorKind, Value, VariableSet};

    #[test]
    fn eval_filter_nth() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Nth {
                n: U64::new(2, "2".to_source()),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        };

        assert_eq!(
            eval_filter(
                &filter,
                &Value::List(vec![
                    Value::Number(Number::Integer(0)),
                    Value::Number(Number::Integer(1)),
                    Value::Number(Number::Integer(2)),
                    Value::Number(Number::Integer(3))
                ]),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(2))
        );
        assert_eq!(
            eval_filter(
                &filter,
                &Value::List(vec![
                    Value::Number(Number::Integer(0)),
                    Value::Number(Number::Integer(1))
                ]),
                &variables,
                false
            )
            .err()
            .unwrap(),
            RunnerError::new(
                SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                RunnerErrorKind::FilterInvalidInput("Out of bound - size is 2".to_string()),
                false
            )
        );
    }
}
