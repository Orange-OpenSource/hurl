/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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

/// Evaluates the URL of an HTTP response, if `value` is of type `HttpResponse`
pub fn eval_location(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    if let Value::HttpResponse(resp) = value {
        Ok(resp.location().map(|loc| Value::String(loc.raw())))
    } else {
        let kind = RunnerErrorKind::FilterInvalidInput {
            actual: value.kind().to_string(),
            expected: "string".to_string(),
        };
        Err(RunnerError::new(source_info, kind, assert))
    }
}
