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

use crate::runner::{Number, RunnerError, RunnerErrorKind, Value};

/// Counts the number of items in a collection `value`.
pub fn eval_count(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::List(values) => Ok(Some(Value::Number(Number::Integer(values.len() as i64)))),
        Value::Bytes(values) => Ok(Some(Value::Number(Number::Integer(values.len() as i64)))),
        Value::Nodeset(size) => Ok(Some(Value::Number(Number::Integer(*size as i64)))),
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
    use crate::runner::VariableSet;

    #[test]
    fn eval_filter_count() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6)),
            value: FilterValue::Count,
        };
        assert_eq!(
            eval_filter(
                &filter,
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

        let error = eval_filter(&filter, &Value::Bool(true), &variables, false)
            .err()
            .unwrap();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6))
        );
        assert_eq!(
            error.kind,
            RunnerErrorKind::FilterInvalidInput("boolean".to_string())
        );
    }
}
