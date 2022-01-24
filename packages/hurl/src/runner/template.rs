/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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

use super::core::{Error, RunnerError};
use super::value::Value;

/// Renders to string a `template` given a map of variables.
pub fn eval_template(
    template: &Template,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    let Template { elements, .. } = template;
    let mut value = String::from("");
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
) -> Result<String, Error> {
    match template_element {
        TemplateElement::String { value, .. } => Ok(value.clone()),
        TemplateElement::Expression(expr) => eval_expression(expr.clone(), variables),
    }
}

pub fn eval_expression(expr: Expr, variables: &HashMap<String, Value>) -> Result<String, Error> {
    let source_info = expr.variable.source_info;
    let name = expr.variable.name;
    match variables.get(name.as_str()) {
        Some(value) => {
            if value.is_renderable() {
                Ok(value.clone().to_string())
            } else {
                Err(Error {
                    source_info,
                    inner: RunnerError::UnrenderableVariable {
                        value: value.to_string(),
                    },
                    assert: false,
                })
            }
        }
        _ => Err(Error {
            source_info,
            inner: RunnerError::TemplateVariableNotDefined { name },
            assert: false,
        }),
    }
}

impl Value {
    pub fn is_renderable(&self) -> bool {
        matches!(
            self,
            Value::Integer(_) | Value::Bool(_) | Value::Float(_) | Value::String(_) | Value::Null
        )
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::SourceInfo;

    use super::*;

    fn template_element_expression() -> TemplateElement {
        // {{name}}
        TemplateElement::Expression(Expr {
            space0: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::init(1, 3, 1, 3),
            },
            variable: Variable {
                name: "name".to_string(),
                source_info: SourceInfo::init(1, 3, 1, 7),
            },
            space1: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::init(1, 7, 1, 7),
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
            Value::List(vec![Value::Integer(1), Value::Integer(2)]),
        );
        let error = eval_template_element(&template_element_expression(), &variables)
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 3, 1, 7));
        assert_eq!(
            error.inner,
            RunnerError::UnrenderableVariable {
                value: "[1,2]".to_string()
            }
        );
    }
}
