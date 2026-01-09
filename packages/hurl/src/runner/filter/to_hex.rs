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

use crate::runner::{hex, RunnerError, RunnerErrorKind, Value};

/// Converts bytes `value` to hexadecimal string.
pub fn eval_to_hex(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::Bytes(value) => Ok(Some(Value::String(hex::encode(value)))),
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
    fn eval_filter_to_hex_ok() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToHex,
        };
        let bytes = vec![0x3c, 0x3c, 0x3f, 0x3f, 0x3f, 0x3e, 0x3e];

        let ret = eval_filter(&filter, &Value::Bytes(bytes), &variables, false);
        assert_eq!(
            ret.unwrap().unwrap(),
            Value::String("3c3c3f3f3f3e3e".to_string())
        );
    }

    #[test]
    fn eval_filter_to_hex_ko_invalid_input() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::ToHex,
        };

        let ret = eval_filter(
            &filter,
            &Value::String("你好世界".to_string()),
            &variables,
            false,
        );
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("string".to_string())
        );
    }
}
