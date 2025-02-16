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
use hurl_core::ast::Function;
use uuid::Uuid;

use crate::runner::error::RunnerError;
use crate::runner::value::Value;

/// Evaluates the function `function`, returns a [`Value`] on success or an [`RunnerError`] .
pub fn eval(function: &Function) -> Result<Value, RunnerError> {
    match &function {
        Function::NewDate => {
            let now = Utc::now();
            Ok(Value::Date(now))
        }
        Function::NewUuid => {
            let uuid = Uuid::new_v4();
            Ok(Value::String(uuid.to_string()))
        }
    }
}
