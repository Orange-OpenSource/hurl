/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use encoding::DecoderTrap;
use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Decode bytes `value` to string using an `encoding`.
pub fn eval_decode(
    value: &Value,
    encoding: &Template,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    let encoding = eval_template(encoding, variables)?;
    match value {
        Value::Bytes(value) => {
            match encoding::label::encoding_from_whatwg_label(encoding.as_str()) {
                None => {
                    let kind = RunnerErrorKind::FilterInvalidEncoding(encoding);
                    Err(RunnerError::new(source_info, kind, assert))
                }
                Some(enc) => match enc.decode(value, DecoderTrap::Strict) {
                    Ok(decoded) => Ok(Some(Value::String(decoded))),
                    Err(_) => {
                        let kind = RunnerErrorKind::FilterDecode(encoding);
                        Err(RunnerError::new(source_info, kind, assert))
                    }
                },
            }
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}
