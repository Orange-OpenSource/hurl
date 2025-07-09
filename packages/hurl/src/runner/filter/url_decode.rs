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

use hurl_core::ast::SourceInfo;

use crate::runner::{RunnerError, RunnerErrorKind, Value};

/// Replaces `%xx` URL escapes in `value` with their single-character equivalent.
pub fn eval_url_decode(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(value) => {
            // We can use `decode_utf8_lossy` here instead of `decode_utf8` because our input
            // is an UTF-8 string going to another UTF-8 string (and not any bytes as input that
            // may be decoded as an invalid UTF-8 string.)
            let decoded = percent_encoding::percent_decode_str(value).decode_utf8_lossy();
            Ok(Some(Value::String(decoded.to_string())))
        }
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

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Value, VariableSet};

    #[test]
    fn eval_filter_url_decode() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: FilterValue::UrlDecode,
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B".to_string()),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::String("https://mozilla.org/?x=шеллы".to_string())
        );
    }
}
