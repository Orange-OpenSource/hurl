/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
use hurl_core::ast::{MultilineString, MultilineStringKind, Template, TemplateElement};
use hurl_core::types::ToSource;
use serde_json::json;

use super::error::RunnerError;
use super::json::eval_json_value;
use super::template::{eval_template, eval_template_element};
use super::variable::VariableSet;

/// Renders to string a multiline body, given a set of variables.
pub fn eval_multiline(
    multiline: &MultilineString,
    variables: &VariableSet,
) -> Result<String, RunnerError> {
    match &multiline.kind {
        MultilineStringKind::Text(value)
        | MultilineStringKind::Json(value)
        | MultilineStringKind::Xml(value) => {
            let s = eval_template(value, variables)?;
            Ok(s)
        }
        MultilineStringKind::Raw(value) => {
            let s = value.to_source().to_string();
            Ok(s)
        }
        MultilineStringKind::GraphQl(graphql) => {
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

/// Renders to string a multiline body and returns the rendered line-to-source-line mapping.
///
/// The returned vector is indexed by rendered 0-based line number and stores the corresponding
/// 0-based source line offset within the multiline template.
pub fn eval_multiline_with_source_line_map(
    multiline: &MultilineString,
    variables: &VariableSet,
) -> Result<(String, Vec<usize>), RunnerError> {
    match &multiline.kind {
        MultilineStringKind::Text(value)
        | MultilineStringKind::Json(value)
        | MultilineStringKind::Xml(value) => {
            eval_multiline_template_with_source_line_map(value, variables)
        }
        MultilineStringKind::Raw(value) => {
            let rendered = value.to_source().to_string();
            Ok((rendered.clone(), identity_source_line_map(&rendered)))
        }
        MultilineStringKind::GraphQl(graphql) => {
            let body = eval_multiline(multiline, variables)?;
            let source_line_map = identity_source_line_map(&graphql.value.to_source().to_string());
            Ok((body, source_line_map))
        }
    }
}

fn eval_multiline_template_with_source_line_map(
    template: &Template,
    variables: &VariableSet,
) -> Result<(String, Vec<usize>), RunnerError> {
    let mut rendered = String::new();
    let mut source_line_map = vec![0];
    let mut source_line = 0;

    for element in &template.elements {
        match element {
            TemplateElement::String { value, .. } => {
                rendered.push_str(value);
                for c in value.chars() {
                    if c == '\n' {
                        source_line += 1;
                        source_line_map.push(source_line);
                    }
                }
            }
            TemplateElement::Placeholder(_) => {
                let value = eval_template_element(element, variables)?;
                rendered.push_str(&value);
                for c in value.chars() {
                    if c == '\n' {
                        source_line_map.push(source_line);
                    }
                }
            }
        }
    }
    Ok((rendered, source_line_map))
}

fn identity_source_line_map(rendered: &str) -> Vec<usize> {
    let mut source_line_map = vec![0];
    let mut source_line = 0;
    for c in rendered.chars() {
        if c == '\n' {
            source_line += 1;
            source_line_map.push(source_line);
        }
    }
    source_line_map
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{
        Expr, ExprKind, GraphQl, GraphQlVariables, JsonObjectElement, JsonValue, MultilineString,
        MultilineStringKind, Placeholder, SourceInfo, Template, TemplateElement, Variable,
        Whitespace,
    };
    use hurl_core::reader::Pos;
    use hurl_core::types::ToSource;

    use crate::runner::multiline::{eval_multiline, eval_multiline_with_source_line_map};
    use crate::runner::{Value, VariableSet};

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
            space: whitespace(),
            newline: newline(),
            kind: MultilineStringKind::GraphQl(GraphQl {
                value: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: query.to_string(),
                        source: query.to_source(),
                    }],
                    empty_source_info(),
                ),
                variables: None,
            }),
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
                        name: Template::new(
                            Some('"'),
                            vec![TemplateElement::String {
                                value: "episode".to_string(),
                                source: "episode".to_source(),
                            }],
                            empty_source_info(),
                        ),
                        space1: String::new(),
                        space2: String::new(),
                        value: JsonValue::String(Template::new(
                            Some('"'),
                            vec![TemplateElement::String {
                                value: "JEDI".to_string(),
                                source: "JEDI".to_source(),
                            }],
                            empty_source_info(),
                        )),
                        space3: String::new(),
                    },
                    JsonObjectElement {
                        space0: String::new(),
                        name: Template::new(
                            Some('"'),
                            vec![TemplateElement::String {
                                value: "withFriends".to_string(),
                                source: "withFriends".to_source(),
                            }],
                            empty_source_info(),
                        ),
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
            space: whitespace(),
            newline: newline(),
            kind: MultilineStringKind::GraphQl(GraphQl {
                value: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: query.to_string(),
                        source: query.to_source(),
                    }],
                    empty_source_info(),
                ),
                variables: Some(graphql_variables),
            }),
        };

        let body = eval_multiline(&multiline, &hurl_variables).unwrap();
        assert_eq!(body, r#"{"query":"{\n  human(id: \"1000\") {\n    name\n    height(unit: FOOT)\n  }\n}","variables":{"episode":"JEDI","withFriends":false}}"#.to_string());
    }

    #[test]
    fn eval_multiline_with_source_line_map_preserves_newline_expanding_placeholders() {
        let multiline = MultilineString {
            space: whitespace(),
            newline: newline(),
            kind: MultilineStringKind::Text(Template::new(
                None,
                vec![
                    TemplateElement::String {
                        value: "line1\nline2".to_string(),
                        source: "line1\nline2".to_source(),
                    },
                    TemplateElement::Placeholder(Placeholder {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(3, 8), Pos::new(3, 8)),
                        },
                        expr: Expr {
                            source_info: SourceInfo::new(Pos::new(3, 8), Pos::new(3, 15)),
                            kind: ExprKind::Variable(Variable {
                                name: "newline".to_string(),
                                source_info: SourceInfo::new(Pos::new(3, 8), Pos::new(3, 15)),
                            }),
                        },
                        space1: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(3, 15), Pos::new(3, 15)),
                        },
                    }),
                    TemplateElement::String {
                        value: "line3\nline4\n".to_string(),
                        source: "line3\nline4\n".to_source(),
                    },
                ],
                SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
            )),
        };
        let mut variables = VariableSet::new();
        variables.insert("newline".to_string(), Value::String("\n".to_string()));

        let (body, source_line_map) =
            eval_multiline_with_source_line_map(&multiline, &variables).unwrap();

        assert_eq!(body, "line1\nline2\nline3\nline4\n".to_string());
        assert_eq!(source_line_map, vec![0, 1, 1, 2, 3]);
    }
}
