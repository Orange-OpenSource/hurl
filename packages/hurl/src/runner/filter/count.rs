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

use hurl_core::ast::SourceInfo;

use crate::runner::{Error, Number, RunnerError, Value};

pub fn eval_count(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, Error> {
    match value {
        Value::List(values) => Ok(Some(Value::Number(Number::Integer(values.len() as i64)))),
        Value::Bytes(values) => Ok(Some(Value::Number(Number::Integer(values.len() as i64)))),
        Value::Nodeset(size) => Ok(Some(Value::Number(Number::Integer(*size as i64)))),
        v => {
            let inner = RunnerError::FilterInvalidInput(v._type());
            Err(Error::new(source_info, inner, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::runner::filter::eval::eval_filter;
    use hurl_core::ast::{Filter, FilterValue, Pos, SourceInfo};
    use std::collections::HashMap;

    use super::*;

    #[test]
    pub fn eval_filter_count() {
        let variables = HashMap::new();

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
            error.inner,
            RunnerError::FilterInvalidInput("boolean".to_string())
        );
    }
}
