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

use crate::html;
use crate::runner::{Error, RunnerError, Value};

pub fn eval_html_unescape(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, Error> {
    match value {
        Value::String(value) => {
            let decoded = html::html_unescape(value);
            Ok(Some(Value::String(decoded)))
        }
        v => {
            let inner = RunnerError::FilterInvalidInput(v._type());
            Err(Error::new(source_info, inner, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::Value;
    use hurl_core::ast::{Filter, FilterValue, Pos, SourceInfo};
    use std::collections::HashMap;

    #[test]
    pub fn eval_filter_html_unescape() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::HtmlUnescape,
        };

        let tests = [
            ("foo", "foo"),
            ("&lt;tag&gt;", "<tag>"),
            ("foo &amp; bar", "foo & bar"),
            (
                "string with double quote: &quot;baz&quot;",
                "string with double quote: \"baz\"",
            ),
        ];
        for (input, output) in tests.iter() {
            assert_eq!(
                eval_filter(
                    &filter,
                    &Value::String(input.to_string()),
                    &variables,
                    false
                )
                .unwrap()
                .unwrap(),
                Value::String(output.to_string())
            );
        }
    }
}
