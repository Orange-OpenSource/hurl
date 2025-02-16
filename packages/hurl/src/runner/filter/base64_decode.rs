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
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use hurl_core::ast::SourceInfo;

use crate::runner::{RunnerError, RunnerErrorKind, Value};

/// Decode base 64 encoded string 'value' into bytes.
pub fn eval_base64_decode(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(value) => match BASE64_STANDARD.decode(value) {
            Ok(decoded) => Ok(Some(Value::Bytes(decoded))),
            Err(_) => {
                let kind = RunnerErrorKind::FilterInvalidInput("Invalid base64 string".to_string());
                Err(RunnerError::new(source_info, kind, assert))
            }
        },
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
    fn eval_filter_base64_decode_ok() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Base64Decode,
        };
        let bytes = vec![
            0xe4, 0xbd, 0xa0, 0xe5, 0xa5, 0xbd, 0xe4, 0xb8, 0x96, 0xe7, 0x95, 0x8c,
        ];

        let ret = eval_filter(
            &filter,
            &Value::String("5L2g5aW95LiW55WM".to_string()),
            &variables,
            false,
        );
        assert_eq!(ret.unwrap().unwrap(), Value::Bytes(bytes));
    }
    #[test]
    fn eval_filter_base64_decode_ok_invalid_characters() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Base64Decode,
        };

        let ret = eval_filter(
            &filter,
            &Value::String("!@#".to_string()),
            &variables,
            false,
        );
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("Invalid base64 string".to_string())
        );
    }

    #[test]
    fn eval_filter_base64_decode_ok_invalid_input() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Base64Decode,
        };

        let ret = eval_filter(
            &filter,
            &Value::Bytes([0xc4, 0xe3, 0xba].to_vec()),
            &variables,
            false,
        );
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("bytes".to_string())
        );
    }
}
