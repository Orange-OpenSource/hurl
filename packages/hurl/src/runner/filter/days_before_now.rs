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

/// Returns the number of days between now and a date `value` in the past.
pub fn eval_days_before_now(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::Date(value) => {
            let diff = Utc::now().signed_duration_since(*value);
            Ok(Some(Value::Number(Number::Integer(diff.num_days()))))
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}
