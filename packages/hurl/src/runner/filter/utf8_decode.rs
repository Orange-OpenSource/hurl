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
use crate::runner::{RunnerError, RunnerErrorKind, Value};
use hurl_core::ast::SourceInfo;

pub fn eval_utf8_decode(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::Bytes(value) => {
            let decoded = String::from_utf8_lossy(value);
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
    use super::*;

    use crate::runner::Number;
    use hurl_core::ast::SourceInfo;
    use hurl_core::reader::Pos;

    #[test]
    fn eval_filter_utf8_decode_ok() {
        let datas = [
            (b"Hello World".to_vec(), "Hello World"),
            // With some replacements for invalid UTF-8 bytes
            (b"Hello \xF0\x90\x80World".to_vec(), "Hello �World"),
            // With emojis
            (
                b"\xF0\x9F\x8D\x8E\xF0\x9F\x8D\x8C\xF0\x9F\x8D\x90\xF0\x9F\x8D\x8A".to_vec(),
                "🍎🍌🍐🍊",
            ),
        ];
        let source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10));

        for (bytes, expected) in datas {
            let ret = eval_utf8_decode(&Value::Bytes(bytes), source_info, false);
            assert_eq!(ret.unwrap().unwrap(), Value::String(expected.to_string()));
        }
    }

    #[test]
    fn eval_filter_utf8_decode_failed() {
        let source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10));
        let ret = eval_utf8_decode(&Value::Number(Number::Integer(42)), source_info, false);
        assert!(ret.is_err());
    }
}
