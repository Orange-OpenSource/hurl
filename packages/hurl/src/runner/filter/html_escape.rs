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

use crate::html;
use crate::runner::{RunnerError, RunnerErrorKind, Value};

/// Converts the characters `&`, `<` and `>` in `value` to HTML-safe sequence.
pub fn eval_html_escape(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(value) => {
            let encoded = html::html_escape(value);
            Ok(Some(Value::String(encoded)))
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo};
    use hurl_core::reader::Pos;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Value, VariableSet};

    #[test]
    pub fn eval_filter_html_escape() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::HtmlEscape,
        };

        let tests = [
            ("foo", "foo"),
            ("<tag>", "&lt;tag&gt;"),
            ("foo & bar", "foo &amp; bar"),
            (
                "string with double quote: \"baz\"",
                "string with double quote: &quot;baz&quot;",
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
