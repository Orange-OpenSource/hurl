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
use hurl_core::ast::{MultilineString, MultilineStringKind, Text};
use serde_json::json;

use crate::runner::json::eval_json_value;
use crate::runner::template::eval_template;
use crate::runner::{RunnerError, VariableSet};

/// Renders to string a multiline body, given a set of variables.
pub fn eval_multiline(
    multiline: &MultilineString,
    variables: &VariableSet,
) -> Result<String, RunnerError> {
    match multiline {
        MultilineString {
            kind: MultilineStringKind::Text(Text { value, .. }),
            ..
        }
        | MultilineString {
            kind: MultilineStringKind::Json(Text { value, .. }),
            ..
        }
        | MultilineString {
            kind: MultilineStringKind::Xml(Text { value, .. }),
            ..
        } => {
            let s = eval_template(value, variables)?;
            Ok(s)
        }
        MultilineString {
            kind: MultilineStringKind::GraphQl(graphql),
            ..
        } => {
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
    use hurl_core::ast::{
        GraphQl, GraphQlVariables, JsonObjectElement, JsonValue, MultilineString,
        MultilineStringKind, SourceInfo, Template, TemplateElement, Whitespace,
    };
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use crate::runner::multiline::eval_multiline;
    use crate::runner::VariableSet;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::from(" "),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    fn newline() -> Whitespace {
        Whitespace {
            value: String::from("\n"),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    fn empty_source_info() -> SourceInfo {
        SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0))
    }

    #[test]
    fn eval_graphql_multiline_simple() {
        let query = r#"{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}"#;
        let variables = VariableSet::new();
        let multiline = MultilineString {
            kind: MultilineStringKind::GraphQl(GraphQl {
                space: whitespace(),
                newline: newline(),
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: query.to_string(),
                        source: query.to_source(),
                    }],
                    source_info: empty_source_info(),
                },
                variables: None,
            }),
            attributes: vec![],
        };
        let body = eval_multiline(&multiline, &variables).unwrap();
        assert_eq!(
            body,
            r#"{"query":"{\n  human(id: \"1000\") {\n    name\n    height(unit: FOOT)\n  }\n}"}"#
                .to_string()
        );
    }

    #[test]
    fn eval_graphql_multiline_with_graphql_variables() {
        let query = r#"{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}"#;
        let hurl_variables = VariableSet::new();
        let graphql_variables = GraphQlVariables {
            space: whitespace(),
            value: JsonValue::Object {
                space0: String::new(),
                elements: vec![
                    JsonObjectElement {
                        space0: String::new(),
                        name: Template {
                            delimiter: Some('"'),
                            elements: vec![TemplateElement::String {
                                value: "episode".to_string(),
                                source: "episode".to_source(),
                            }],
                            source_info: empty_source_info(),
                        },
                        space1: String::new(),
                        space2: String::new(),
                        value: JsonValue::String(Template {
                            delimiter: Some('"'),
                            elements: vec![TemplateElement::String {
                                value: "JEDI".to_string(),
                                source: "JEDI".to_source(),
                            }],
                            source_info: empty_source_info(),
                        }),
                        space3: String::new(),
                    },
                    JsonObjectElement {
                        space0: String::new(),
                        name: Template {
                            delimiter: Some('"'),
                            elements: vec![TemplateElement::String {
                                value: "withFriends".to_string(),
                                source: "withFriends".to_source(),
                            }],
                            source_info: empty_source_info(),
                        },
                        space1: String::new(),
                        space2: String::new(),
                        value: JsonValue::Boolean(false),
                        space3: String::new(),
                    },
                ],
            },
            whitespace: whitespace(),
        };
        let multiline = MultilineString {
            kind: MultilineStringKind::GraphQl(GraphQl {
                space: whitespace(),
                newline: newline(),
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: query.to_string(),
                        source: query.to_source(),
                    }],
                    source_info: empty_source_info(),
                },
                variables: Some(graphql_variables),
            }),
            attributes: vec![],
        };

        let body = eval_multiline(&multiline, &hurl_variables).unwrap();
        assert_eq!(body, r#"{"query":"{\n  human(id: \"1000\") {\n    name\n    height(unit: FOOT)\n  }\n}","variables":{"episode":"JEDI","withFriends":false}}"#.to_string());
    }
}
