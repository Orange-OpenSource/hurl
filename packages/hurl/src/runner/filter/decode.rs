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
use std::collections::HashMap;

use encoding::DecoderTrap;
use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value};

pub fn eval_decode(
    value: &Value,
    encoding_value: &Template,
    variables: &HashMap<String, Value>,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    let encoding_value = eval_template(encoding_value, variables)?;
    match value {
        Value::Bytes(value) => {
            match encoding::label::encoding_from_whatwg_label(encoding_value.as_str()) {
                None => {
                    let kind = RunnerErrorKind::FilterInvalidEncoding(encoding_value);
                    Err(RunnerError::new(source_info, kind, assert))
                }
                Some(enc) => match enc.decode(value, DecoderTrap::Strict) {
                    Ok(decoded) => Ok(Some(Value::String(decoded))),
                    Err(_) => {
                        let kind = RunnerErrorKind::FilterDecode(encoding_value);
                        Err(RunnerError::new(source_info, kind, assert))
                    }
                },
            }
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v._type());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo};
    use hurl_core::reader::Pos;

    use super::*;
    use crate::runner::filter::eval::eval_filter;

    #[test]
    pub fn eval_filter_url_decode() {
        let variables = HashMap::new();
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
