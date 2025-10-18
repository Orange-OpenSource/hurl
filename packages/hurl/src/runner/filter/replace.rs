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
use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Replaces all occurrences of the string `old_value` with `new_value` in `value`.
pub fn eval_replace(
    value: &Value,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
    old_value: &Template,
    new_value: &Template,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(v) => {
            let old_value = eval_template(old_value, variables)?;
            let new_value = eval_template(new_value, variables)?;
            let s = v.replace(&old_value, &new_value);
            Ok(Some(Value::String(s)))
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.repr());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::types::ToSource;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Value, VariableSet};

    #[test]
    fn eval_filter_replace() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Replace {
                old_value: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "bar".to_string(),
                        source: "bar".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                new_value: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "foo".to_string(),
                        source: "foo".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        };

        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("foo bar baz".to_string()),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::String("foo foo baz".to_string())
        );
    }

    #[test]
    fn test_first_arg_is_not_a_regex() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Replace {
                old_value: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "[0-9]".to_string(),
                        source: "[0-9]".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                new_value: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "x".to_string(),
                        source: "x".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        };

        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("1234 [0-9]".to_string()),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::String("1234 x".to_string())
        );
    }
}
