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

use hurl_core::ast::*;

use crate::runner::error::RunnerError;
use crate::runner::expr;
use crate::runner::Value;

/// Renders to string a `template` given a map of variables.
pub fn eval_template(
    template: &Template,
    variables: &HashMap<String, Value>,
) -> Result<String, RunnerError> {
    let Template { elements, .. } = template;
    let mut value = String::new();
    for elem in elements {
        match eval_template_element(elem, variables) {
            Ok(v) => value.push_str(v.as_str()),
            Err(e) => return Err(e),
        }
    }
    Ok(value)
}

fn eval_template_element(
    template_element: &TemplateElement,
    variables: &HashMap<String, Value>,
) -> Result<String, RunnerError> {
    match template_element {
        TemplateElement::String { value, .. } => Ok(value.clone()),
        TemplateElement::Expression(expr) => expr::render(expr, variables),
    }
}

impl Value {
    pub fn is_renderable(&self) -> bool {
        matches!(
            self,
            Value::Number(_) | Value::Bool(_) | Value::String(_) | Value::Null
        )
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::SourceInfo;
    use hurl_core::reader::Pos;

    use super::*;
    use crate::runner::{Number, RunnerErrorKind};

    fn template_element_expression() -> TemplateElement {
        // {{name}}
        TemplateElement::Expression(Expr {
            space0: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 3)),
            },
            variable: Variable {
                name: "name".to_string(),
                source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 7)),
            },
            space1: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 7)),
            },
        })
    }

    #[test]
    fn test_template_element() {
        let variables = HashMap::new();
        assert_eq!(
            eval_template_element(
                &TemplateElement::String {
                    value: "World".to_string(),
                    encoded: "World".to_string(),
                },
                &variables
            )
            .unwrap(),
            "World".to_string()
        );

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), Value::String("World".to_string()));
        assert_eq!(
            eval_template_element(&template_element_expression(), &variables).unwrap(),
            "World".to_string()
        );
    }

    #[test]
    fn test_template_element_error() {
        let mut variables = HashMap::new();
        variables.insert(
            "name".to_string(),
            Value::List(vec![
                Value::Number(Number::Integer(1)),
                Value::Number(Number::Integer(2)),
            ]),
        );
        let error = eval_template_element(&template_element_expression(), &variables)
            .err()
            .unwrap();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 3), Pos::new(1, 7))
        );
        assert_eq!(
            error.kind,
            RunnerErrorKind::UnrenderableVariable {
                name: "name".to_string(),
                value: "[1,2]".to_string()
            }
        );
    }
}
