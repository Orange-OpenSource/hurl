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
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use hurl_core::ast::SourceInfo;

use crate::runner::{RunnerError, RunnerErrorKind, Value};

/// Encode bytes 'value' into base 64 URL safe encoded string.
pub fn eval_base64_url_safe_encode(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::Bytes(value) => Ok(Some(Value::String(BASE64_URL_SAFE_NO_PAD.encode(value)))),
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
    fn eval_filter_base64_url_safe_encode_ok() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Base64UrlSafeEncode,
        };
        let bytes = vec![0xd1, 0x88, 0xd0, 0xb5, 0xd0, 0xbb, 0xd0, 0xbb, 0xd1, 0x8b];

        let ret = eval_filter(&filter, &Value::Bytes(bytes), &variables, false);
        assert_eq!(
            ret.unwrap().unwrap(),
            Value::String("0YjQtdC70LvRiw".to_string())
        );
    }

    #[test]
    fn eval_filter_base64_url_safe_encode_ko_invalid_input() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Base64UrlSafeEncode,
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
