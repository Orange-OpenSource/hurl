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
use chrono::Utc;
use hurl_core::ast::SourceInfo;

use crate::runner::{Number, RunnerError, RunnerErrorKind, Value};

/// Returns the number of days between now and a date `value` in the future.
pub fn eval_days_after_now(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::Date(value) => {
            let diff = value.signed_duration_since(Utc::now());
            Ok(Some(Value::Number(Number::Integer(diff.num_days()))))
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
    use chrono::Duration;
    use hurl_core::ast::{Filter, FilterValue, SourceInfo};
    use hurl_core::reader::Pos;

    use super::*;
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::VariableSet;

    #[test]
    fn eval_filter_days_after_before_now() {
        let variables = VariableSet::new();

        let now = Utc::now();
        assert_eq!(
            eval_filter(
                &Filter {
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    value: FilterValue::DaysAfterNow,
                },
                &Value::Date(now),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(0))
        );

        let now_plus_30hours = now + Duration::try_hours(30).unwrap();
        assert_eq!(
            eval_filter(
                &Filter {
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    value: FilterValue::DaysAfterNow,
                },
                &Value::Date(now_plus_30hours),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(1))
        );
        assert_eq!(
            eval_filter(
                &Filter {
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    value: FilterValue::DaysBeforeNow,
                },
                &Value::Date(now_plus_30hours),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(-1))
        );
    }
}
