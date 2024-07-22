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

use hurl_core::ast::SourceInfo;
use percent_encoding::AsciiSet;

use crate::runner::{RunnerError, RunnerErrorKind, Value};

// does not encode "/"
// like Jinja template (https://jinja.palletsprojects.com/en/3.1.x/templates/#jinja-filters.urlencode)
pub fn eval_url_encode(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(value) => {
            const FRAGMENT: &AsciiSet = &percent_encoding::NON_ALPHANUMERIC
                .remove(b'-')
                .remove(b'.')
                .remove(b'_')
                .remove(b'~')
                .remove(b'/');
            let encoded = percent_encoding::percent_encode(value.as_bytes(), FRAGMENT).to_string();
            Ok(Some(Value::String(encoded)))
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v._type());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use hurl_core::ast::{Filter, FilterValue, SourceInfo};
    use hurl_core::reader::Pos;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::Value;

    #[test]
    pub fn eval_filter_url_encode() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: FilterValue::UrlEncode,
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("https://mozilla.org/?x=шеллы".to_string()),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::String(
                "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B".to_string()
            )
        );
    }
}
