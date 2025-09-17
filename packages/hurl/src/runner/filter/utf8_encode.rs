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

pub fn eval_utf8_encode(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(value) => {
            let encoded = value.bytes().collect::<Vec<_>>();
            Ok(Some(Value::Bytes(encoded)))
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

    use hurl_core::ast::SourceInfo;
    use hurl_core::reader::Pos;

    #[test]
    fn eval_filter_utf8_decode_ok() {
        let datas = [
            ("Hello World", b"Hello World".to_vec()),
            // With emojis
            (
                "🐶🐱🐭🐹",
                b"\xF0\x9F\x90\xB6\xF0\x9F\x90\xB1\xF0\x9F\x90\xAD\xF0\x9F\x90\xB9".to_vec(),
            ),
        ];
        let source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10));

        for (str, expected) in datas {
            let ret = eval_utf8_encode(&Value::String(str.to_string()), source_info, false);
            assert_eq!(ret.unwrap().unwrap(), Value::Bytes(expected));
        }
    }
}
