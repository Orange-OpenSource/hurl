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

/// Converts `value` to an integer.
pub fn eval_to_int(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::Number(Number::Integer(v)) => Ok(Some(Value::Number(Number::Integer(*v)))),
        Value::Number(Number::Float(v)) => Ok(Some(Value::Number(Number::Integer(*v as i64)))),
        Value::String(v) => match v.parse::<i64>() {
            Ok(i) => Ok(Some(Value::Number(Number::Integer(i)))),
            _ => {
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
    use hurl_core::ast::{Filter, FilterValue, SourceInfo};
    use hurl_core::reader::Pos;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Number, RunnerErrorKind, Value, VariableSet};

    #[test]
    fn eval_filter_to_int() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToInt,
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("123".to_string()),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(123))
        );
        assert_eq!(
            eval_filter(
                &filter,
                &Value::Number(Number::Integer(123)),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(123))
        );
        assert_eq!(
            eval_filter(
                &filter,
                &Value::Number(Number::Float(1.6)),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(1))
        );
    }

    #[test]
    fn eval_filter_to_int_error() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
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
            err.kind,
            RunnerErrorKind::FilterInvalidInput("string <123x>".to_string())
        );
        let err = eval_filter(&filter, &Value::Bool(true), &variables, false)
            .err()
            .unwrap();
        assert_eq!(
            err.kind,
            RunnerErrorKind::FilterInvalidInput("boolean <true>".to_string())
        );
    }
}
