/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

use hurl_core::ast::{RegexValue, SourceInfo, Template};

use crate::runner::regex::eval_regex_value;
use crate::runner::template::eval_template;
use crate::runner::{Error, RunnerError, Value};

pub fn eval_replace(
    value: &Value,
    variables: &HashMap<String, Value>,
    source_info: SourceInfo,
    assert: bool,
    old_value: &RegexValue,
    new_value: &Template,
) -> Result<Option<Value>, Error> {
    match value {
        Value::String(v) => {
            let re = eval_regex_value(old_value, variables)?;
            let new_value = eval_template(new_value, variables)?;
            let s = re.replace_all(v, new_value).to_string();
            Ok(Some(Value::String(s)))
        }
        v => {
            let inner = RunnerError::FilterInvalidInput(v.display());
            Err(Error::new(source_info, inner, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::Value;
    use hurl_core::ast::{
        Filter, FilterValue, Pos, RegexValue, SourceInfo, Template, TemplateElement, Whitespace,
    };
    use std::collections::HashMap;

    #[test]
    pub fn eval_filter_replace() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Replace {
                old_value: RegexValue::Template(Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\\s+".to_string(),
                        encoded: ",".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 20)),
                }),
                new_value: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: ",".to_string(),
                        encoded: ",".to_string(),
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
