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

use hurl_core::ast::{SourceInfo, Template};

use crate::jsonpath;
use crate::runner::template::eval_template;
use crate::runner::{Error, RunnerError, Value};

pub fn eval_jsonpath(
    value: &Value,
    expr: &Template,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
    assert: bool,
) -> Result<Option<Value>, Error> {
    match value {
        Value::String(json) => eval_jsonpath_string(json, expr, variables, source_info),
        v => Err(Error {
            source_info: source_info.clone(),
            inner: RunnerError::FilterInvalidInput(v._type()),
            assert,
        }),
    }
}

pub fn eval_jsonpath_string(
    json: &str,
    expr: &Template,
    variables: &HashMap<String, Value>,
    source_info: &SourceInfo,
) -> Result<Option<Value>, Error> {
    let value = eval_template(expr, variables)?;
    let expr_source_info = &expr.source_info;
    let jsonpath_query = match jsonpath::parse(value.as_str()) {
        Ok(q) => q,
        Err(_) => {
            return Err(Error {
                source_info: expr_source_info.clone(),
                inner: RunnerError::QueryInvalidJsonpathExpression { value },
                assert: false,
            });
        }
    };
    let value = match serde_json::from_str(json) {
        Err(_) => {
            return Err(Error {
                source_info: source_info.clone(),
                inner: RunnerError::QueryInvalidJson,
                assert: false,
            });
        }
        Ok(v) => v,
    };

    let results = jsonpath_query.eval(&value);
    match results {
        None => Ok(None),
        Some(jsonpath::JsonpathResult::SingleEntry(value)) => Ok(Some(Value::from_json(&value))),
        Some(jsonpath::JsonpathResult::Collection(values)) => {
            Ok(Some(Value::from_json(&serde_json::Value::Array(values))))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use std::collections::HashMap;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::Value;

    #[test]
    pub fn eval_filter_jsonpath() {
        let variables = HashMap::new();

        let filter = Filter {
            source_info: SourceInfo::new(1, 1, 1, 1),
            value: FilterValue::JsonPath {
                expr: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "$.message".to_string(),
                        encoded: "$.message".to_string(),
                    }],
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(0, 0, 0, 0),
                },
            },
        };
        assert_eq!(
            eval_filter(
                &filter,
                &Value::String(r#"{"message":"Hello"}"#.to_string()),
                &variables,
                false
            )
            .unwrap()
            .unwrap(),
            Value::String("Hello".to_string())
        );
    }
}
