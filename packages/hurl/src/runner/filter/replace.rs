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
use hurl_core::ast::{RegexValue, SourceInfo, Template};

use crate::runner::regex::eval_regex_value;
use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Replaces all occurrences of `old_value` with `new_value` in `value`.
pub fn eval_replace(
    value: &Value,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
    old_value: &RegexValue,
    new_value: &Template,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(v) => {
            let re = eval_regex_value(old_value, variables)?;
            let new_value = eval_template(new_value, variables)?;
            let s = re.replace_all(v, new_value).to_string();
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
    use hurl_core::ast::{
        Filter, FilterValue, RegexValue, SourceInfo, Template, TemplateElement, Whitespace,
    };
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Value, VariableSet};

    #[test]
    fn eval_filter_replace() {
        let variables = VariableSet::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Replace {
                old_value: RegexValue::Template(Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\\s+".to_string(),
                        source: ",".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 20)),
                }),
                new_value: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: ",".to_string(),
                        source: ",".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
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
                &Value::String("1 2\t3  4".to_string()),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::String("1,2,3,4".to_string())
        );
    }
}
