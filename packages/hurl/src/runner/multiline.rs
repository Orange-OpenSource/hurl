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

use hurl_core::ast::{MultilineString, Text};
use serde_json::json;

use crate::runner::json::eval_json_value;
use crate::runner::template::eval_template;
use crate::runner::{Error, Value};

/// Renders to string a multiline body, given a set of variables.
pub fn eval_multiline(
    multiline: &MultilineString,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match multiline {
        MultilineString::OneLineText(value) => {
            let s = eval_template(value, variables)?;
            Ok(s)
        }
        MultilineString::Text(Text { value, .. })
        | MultilineString::Json(Text { value, .. })
        | MultilineString::Xml(Text { value, .. }) => {
            let s = eval_template(value, variables)?;
            Ok(s)
        }
        MultilineString::GraphQl(graphql) => {
            let query = eval_template(&graphql.value, variables)?;
            let body = match &graphql.variables {
                None => json!({ "query": query.trim()}).to_string(),
                Some(vars) => {
                    let s = eval_json_value(&vars.value, variables, false)?;
                    let query = json!(query.trim());
                    format!(r#"{{"query":{query},"variables":{s}}}"#)
                }
            };
            Ok(body)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use hurl_core::ast::{
        GraphQl, GraphQlVariables, JsonObjectElement, JsonValue, MultilineString, SourceInfo,
        Template, TemplateElement, Whitespace,
    };

    use crate::runner::multiline::eval_multiline;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(0, 0, 0, 0),
        }
    }

    fn newline() -> Whitespace {
        Whitespace {
            value: String::from("\n"),
            source_info: SourceInfo::new(0, 0, 0, 0),
        }
    }

    fn empty_source_info() -> SourceInfo {
        SourceInfo::new(0, 0, 0, 0)
    }

    #[test]
    fn eval_graphql_multiline_simple() {
        let query = r#"{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}"#;
        let variables = HashMap::new();
        let multiline = MultilineString::GraphQl(GraphQl {
            space: whitespace(),
            newline: newline(),
            value: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: query.to_string(),
                    encoded: query.to_string(),
                }],
                source_info: empty_source_info(),
            },
            variables: None,
        });
        let body = eval_multiline(&multiline, &variables).unwrap();
        assert_eq!(
            body,
            r#"{"query":"{\n  human(id: \"1000\") {\n    name\n    height(unit: FOOT)\n  }\n}"}"#
                .to_string()
        )
    }

    #[test]
    fn eval_graphql_multiline_with_graphql_variables() {
        let query = r#"{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}"#;
        let hurl_variables = HashMap::new();
        let graphql_variables = GraphQlVariables {
            space: whitespace(),
            value: JsonValue::Object {
                space0: "".to_string(),
                elements: vec![
                    JsonObjectElement {
                        space0: "".to_string(),
                        name: Template {
                            delimiter: Some('"'),
                            elements: vec![TemplateElement::String {
                                value: "episode".to_string(),
                                encoded: "episode".to_string(),
                            }],
                            source_info: empty_source_info(),
                        },
                        space1: "".to_string(),
                        space2: "".to_string(),
                        value: JsonValue::String(Template {
                            delimiter: Some('"'),
                            elements: vec![TemplateElement::String {
                                value: "JEDI".to_string(),
                                encoded: "JEDI".to_string(),
                            }],
                            source_info: empty_source_info(),
                        }),
                        space3: "".to_string(),
                    },
                    JsonObjectElement {
                        space0: "".to_string(),
                        name: Template {
                            delimiter: Some('"'),
                            elements: vec![TemplateElement::String {
                                value: "withFriends".to_string(),
                                encoded: "withFriends".to_string(),
                            }],
                            source_info: empty_source_info(),
                        },
                        space1: "".to_string(),
                        space2: "".to_string(),
                        value: JsonValue::Boolean(false),
                        space3: "".to_string(),
                    },
                ],
            },
            whitespace: whitespace(),
        };
        let multiline = MultilineString::GraphQl(GraphQl {
            space: whitespace(),
            newline: newline(),
            value: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: query.to_string(),
                    encoded: query.to_string(),
                }],
                source_info: empty_source_info(),
            },
            variables: Some(graphql_variables),
        });

        let body = eval_multiline(&multiline, &hurl_variables).unwrap();
        assert_eq!(body, r#"{"query":"{\n  human(id: \"1000\") {\n    name\n    height(unit: FOOT)\n  }\n}","variables":{"episode":"JEDI","withFriends":false}}"#.to_string())
    }
}
