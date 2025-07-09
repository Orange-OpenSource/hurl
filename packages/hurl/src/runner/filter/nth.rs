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
use hurl_core::ast::{IntegerValue, Placeholder, SourceInfo};

use crate::runner::{expr, Number, RunnerError, RunnerErrorKind, Value, VariableSet};

/// Returns the element from a collection `value` at a zero-based index.
pub fn eval_nth(
    value: &Value,
    n: &IntegerValue,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    let n = eval_integer_value(n, variables)?;

    match value {
        Value::List(values) => match try_nth(values, n) {
            Ok(value) => Ok(Some(value.clone())),
            Err(err) => {
                let kind = RunnerErrorKind::FilterInvalidInput(err);
                Err(RunnerError::new(source_info, kind, assert))
            }
        },
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.repr());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

/// Returns the element in the `items` collection at `index`.
/// Ths function accepts negative indices for indexing from the end of the collection.
fn try_nth<U>(items: &[U], index: i64) -> Result<&U, String> {
    let len = items.len() as i64;
    let value = if index >= 0 && index < len {
        &items[index as usize]
    } else if index < 0 && len - index.abs() >= 0 {
        &items[(len - index.abs()) as usize]
    } else {
        let error = format!("out of bound - size is {len}");
        return Err(error);
    };
    Ok(value)
}

/// Evaluates an [`IntegerValue`] against a variable set.
fn eval_integer_value(n: &IntegerValue, variables: &VariableSet) -> Result<i64, RunnerError> {
    match n {
        IntegerValue::Literal(value) => Ok(value.as_i64()),
        IntegerValue::Placeholder(Placeholder { expr, .. }) => match expr::eval(expr, variables)? {
            Value::Number(Number::Integer(value)) => Ok(value),
            v => {
                let kind = RunnerErrorKind::ExpressionInvalidType {
                    value: v.repr(),
                    expecting: "integer".to_string(),
                };
                Err(RunnerError::new(expr.source_info, kind, false))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, IntegerValue, SourceInfo, Whitespace, I64};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::filter::nth::try_nth;
    use crate::runner::{Number, RunnerError, RunnerErrorKind, Value, VariableSet};

    #[test]
    fn eval_filter_nth_positive() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Nth {
                n: IntegerValue::Literal(I64::new(2, "2".to_source())),
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
                RunnerErrorKind::FilterInvalidInput("out of bound - size is 2".to_string()),
                false
            )
        );
    }

    #[test]
    fn eval_filter_nth_negative() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Nth {
                n: IntegerValue::Literal(I64::new(-4, "-4".to_source())),
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
            Value::Number(Number::Integer(0))
        );
        assert_eq!(
            eval_filter(
                &filter,
                &Value::List(vec![Value::Number(Number::Integer(0))]),
                &variables,
                false
            )
            .err()
            .unwrap(),
            RunnerError::new(
                SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                RunnerErrorKind::FilterInvalidInput("out of bound - size is 1".to_string()),
                false
            )
        );
    }

    #[test]
    fn test_try_nth() {
        let values = [12, 4, 5];
        assert_eq!(try_nth(&values, 0).unwrap(), &12);
        assert_eq!(try_nth(&values, 2).unwrap(), &5);
        assert_eq!(try_nth(&values, 3).unwrap_err(), "out of bound - size is 3");
        assert_eq!(try_nth(&values, -1).unwrap(), &5);
        assert_eq!(try_nth(&values, -3).unwrap(), &12);
        assert_eq!(try_nth(&values, 4).unwrap_err(), "out of bound - size is 3");
    }
}
