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
    let json = match value {
        Value::String(text) => match serde_json::from_str(text) {
            Err(_) => {
                return Err(RunnerError::new(
                    source_info,
                    RunnerErrorKind::FilterInvalidInput("value is not a valid JSON".to_string()),
                    false,
                ));
            }
            Ok(v) => v,
        },

        // FIXME: for the moment `jsonpath` filter accepts only string. We need to decide if the filter
        // accept any value or if we need a `toJson` filter that takes any value and render to string.
        // See <https://github.com/Orange-OpenSource/hurl/issues/4249>
        // v => match v.try_to_json() {
        //     Some(v) => v,
        //     None => {
        //         let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
        //         return Err(RunnerError::new(source_info, kind, assert));
        //     }
        // },
        v => {
            let kind = RunnerErrorKind::FilterInvalidInput(v.kind().to_string());
            return Err(RunnerError::new(source_info, kind, assert));
        }
    };
    eval_jsonpath_json(&json, expr, variables)
}

impl Value {
    // /// Deserializes this [`Value`] to a JSON [`serde_json::Value`].
    // fn try_to_json(&self) -> Option<serde_json::Value> {
    //     match self {
    //         Value::Bool(value) => Some(serde_json::Value::Bool(*value)),
    //         Value::List(values) => values
    //             .iter()
    //             .map(|v| v.try_to_json())
    //             .collect::<Option<_>>(),
    //         Value::Null => Some(serde_json::Value::Null),
    //         Value::Number(Number::Float(value)) => {
    //             let number = serde_json::Number::from_f64(*value)?;
    //             Some(serde_json::Value::Number(number))
    //         }
    //         Value::Number(Number::Integer(value)) => {
    //             let number = serde_json::Number::from_i128(*value as i128)?;
    //             Some(serde_json::Value::Number(number))
    //         }
    //         Value::Number(Number::BigInteger(value)) => match serde_json::Number::from_str(value) {
    //             Ok(n) => Some(serde_json::Value::Number(n)),
    //             Err(_) => None,
    //         },
    //         Value::Object(values) => {
    //             let mut obj = serde_json::Map::new();
    //             for (key, value) in values {
    //                 let value = value.try_to_json()?;
    //                 obj.insert(key.clone(), value);
    //             }
    //             Some(serde_json::Value::Object(obj))
    //         }
    //         Value::String(value) => Some(serde_json::Value::String(value.clone())),
    //         Value::Regex(_)
    //         | Value::Nodeset(_)
    //         | Value::HttpResponse(_)
    //         | Value::Date(_)
    //         | Value::Bytes(_)
    //         | Value::Unit => None,
    //     }
    // }
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
    use crate::runner::filter::eval::eval_filter;
    use crate::runner::{Value, VariableSet};
    use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::types::ToSource;

    #[test]
    fn eval_filter_jsonpath() {
        let variables = VariableSet::new();

        let filter = Filter {
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            value: FilterValue::JsonPath {
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "$.message".to_string(),
                        source: "$.message".to_source(),
                    }],
                    SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                ),
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

    // #[test]
    // fn test_try_to_json_bool() {
    //     let input = Value::Bool(true);
    //     let expected = json!(true);
    //     let actual = input.try_to_json().unwrap();
    //     assert_eq!(actual, expected);
    //
    //     let input = Value::Bool(false);
    //     let expected = serde_json::Value::Bool(false);
    //     let actual = input.try_to_json().unwrap();
    //     assert_eq!(actual, expected);
    // }
    //
    // #[test]
    // fn test_try_to_json_number() {
    //     let input = Value::Number(Number::Integer(42));
    //     let expected = json!(42);
    //     let actual = input.try_to_json().unwrap();
    //     assert_eq!(actual, expected);
    //
    //     let input = Value::Number(Number::Float(3.33));
    //     let expected = json!(3.33);
    //     let actual = input.try_to_json().unwrap();
    //     assert_eq!(actual, expected);
    //
    //     let input = Value::Number(Number::BigInteger("10000000000000000365".to_string()));
    //     let expected: serde_json::Value = serde_json::from_str("10000000000000000365").unwrap();
    //     let actual = input.try_to_json().unwrap();
    //     assert_eq!(actual, expected);
    // }
    //
    // #[test]
    // fn test_try_to_json_list() {
    //     let input = Value::List(vec![
    //         Value::String("foo".to_string()),
    //         Value::String("bar".to_string()),
    //         Value::String("baz".to_string()),
    //     ]);
    //     let expected = json!(["foo", "bar", "baz"]);
    //     let actual = input.try_to_json().unwrap();
    //     assert_eq!(actual, expected);
    // }
    //
    // #[test]
    // fn test_try_to_json_object() {
    //     let input = Value::Object(vec![
    //         ("name".to_string(), Value::String("bob".to_string())),
    //         ("age".to_string(), Value::Number(Number::Integer(33))),
    //     ]);
    //     let expected = json!({
    //         "name": "bob",
    //         "age": 33
    //     });
    //     let actual = input.try_to_json().unwrap();
    //     assert_eq!(actual, expected);
    // }
}
