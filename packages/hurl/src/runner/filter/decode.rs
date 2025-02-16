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

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use super::*;
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::VariableSet;

    /// Helper function to return a new filter given an `encoding`
    fn new_decode_filter(encoding: &str) -> Filter {
        // Example: decode "gb2312"
        Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Decode {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(7, 1), Pos::new(8, 1)),
                },
                encoding: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: encoding.to_string(),
                        source: encoding.to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(8, 1), Pos::new(8 + encoding.len(), 1)),
                },
            },
        }
    }

    #[test]
    fn eval_filter_decode_ok() {
        let variables = VariableSet::new();

        let filter = new_decode_filter("utf-8");
        let bytes = vec![
            0xe4, 0xbd, 0xa0, 0xe5, 0xa5, 0xbd, 0xe4, 0xb8, 0x96, 0xe7, 0x95, 0x8c,
        ];
        let ret = eval_filter(&filter, &Value::Bytes(bytes), &variables, false);
        assert_eq!(ret.unwrap().unwrap(), Value::String("你好世界".to_string()));

        let filter = new_decode_filter("gb2312");
        let bytes = vec![0xc4, 0xe3, 0xba, 0xc3, 0xca, 0xc0, 0xbd, 0xe7];
        let ret = eval_filter(&filter, &Value::Bytes(bytes), &variables, false);
        assert_eq!(ret.unwrap().unwrap(), Value::String("你好世界".to_string()));
    }

    #[test]
    fn eval_filter_decode_ko_unknown_encoding() {
        let variables = VariableSet::new();

        let filter = new_decode_filter("xxx");
        let bytes = vec![];

        let ret = eval_filter(&filter, &Value::Bytes(bytes), &variables, false);

        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidEncoding("xxx".to_string()),
        );
    }

    #[test]
    fn eval_filter_decode_ko_bad_bytes_input() {
        let variables = VariableSet::new();

        let filter = new_decode_filter("gb2312");
        let bytes = vec![0xc4, 0x00];
        let ret = eval_filter(&filter, &Value::Bytes(bytes), &variables, false);
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterDecode("gb2312".to_string()),
        );
    }

    #[test]
    fn eval_filter_decode_ko_bad_input_type() {
        let variables = VariableSet::new();

        let filter = new_decode_filter("utf-8");
        let ret = eval_filter(
            &filter,
            &Value::String("café".to_string()),
            &variables,
            false,
        );
        assert_eq!(
            ret.unwrap_err().kind,
            RunnerErrorKind::FilterInvalidInput("string".to_string()),
        );
    }
}
