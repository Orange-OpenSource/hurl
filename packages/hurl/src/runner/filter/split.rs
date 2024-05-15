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

use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value};

pub fn eval_split(
    value: &Value,
    variables: &HashMap<String, Value>,
    source_info: SourceInfo,
    assert: bool,
    sep: &Template,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(s) => {
            let sep = eval_template(sep, variables)?;
            let values = s
                .split(&sep)
                .map(|v| Value::String(v.to_string()))
                .collect();
            Ok(Some(Value::List(values)))
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.display());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::Value;
    use hurl_core::ast::{
        Filter, FilterValue, Pos, SourceInfo, Template, TemplateElement, Whitespace,
    };
    use std::collections::HashMap;

    #[test]
    pub fn eval_filter_split() {
        let variables = HashMap::new();
        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::Split {
                sep: Template {
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
            },
        };

        assert_eq!(
            eval_filter(
                &filter,
                &Value::String("1,2,3".to_string()),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::List(vec![
                Value::String("1".to_string()),
                Value::String("2".to_string()),
                Value::String("3".to_string()),
            ])
        );
    }
}
