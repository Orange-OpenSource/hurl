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

use crate::runner::{Error, RunnerError, Value};

pub fn eval_nth(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
    n: u64,
) -> Result<Option<Value>, Error> {
    match value {
        Value::List(values) => match values.get(n as usize) {
            None => {
                let inner = RunnerError::FilterInvalidInput(format!(
                    "Out of bound - size is {}",
                    values.len()
                ));
                Err(Error::new(source_info, inner, assert))
            }
            Some(value) => Ok(Some(value.clone())),
        },
        v => {
            let inner = RunnerError::FilterInvalidInput(v.display());
            Err(Error::new(source_info, inner, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Error, Number, RunnerError, Value};
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Whitespace};
    use std::collections::HashMap;

    #[test]
    pub fn eval_filter_nth() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::Nth {
                n: 2,
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(0, 0, 0, 0),
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
            Error::new(
                SourceInfo::new(1, 1, 1, 1),
                RunnerError::FilterInvalidInput("Out of bound - size is 2".to_string()),
                false
            )
        );
    }
}
