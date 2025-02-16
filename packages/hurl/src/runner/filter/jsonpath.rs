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

use crate::jsonpath;
use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Evaluates a JSONPath expression `expr` against a `value`.
pub fn eval_jsonpath(
    value: &Value,
    expr: &Template,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(text) => {
            let json = match serde_json::from_str(text) {
                Err(_) => {
                    return Err(RunnerError::new(
                        source_info,
                        RunnerErrorKind::QueryInvalidJson,
                        false,
                    ));
                }
                Ok(v) => v,
            };
            eval_jsonpath_json(&json, expr, variables)
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

pub fn eval_jsonpath_json(
    json: &serde_json::Value,
    expr: &Template,
    variables: &VariableSet,
) -> Result<Option<Value>, RunnerError> {
    let expr_str = eval_template(expr, variables)?;
    let expr_source_info = expr.source_info;
    let jsonpath_query = match jsonpath::parse(&expr_str) {
        Ok(q) => q,
        Err(_) => {
            let kind = RunnerErrorKind::QueryInvalidJsonpathExpression { value: expr_str };
            return Err(RunnerError::new(expr_source_info, kind, false));
        }
    };

    let results = jsonpath_query.eval(json);
    match results {
        None => Ok(None),
        Some(jsonpath::JsonpathResult::SingleEntry(value)) => Ok(Some(Value::from_json(&value))),
        Some(jsonpath::JsonpathResult::Collection(values)) => {
            Ok(Some(Value::from_json(&serde_json::Value::Array(values))))
        }
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Value, VariableSet};

    #[test]
    fn eval_filter_jsonpath() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::JsonPath {
                expr: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "$.message".to_string(),
                        source: "$.message".to_source(),
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
