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
use hurl_core::ast::{
    JsonListElement, JsonObjectElement, JsonValue, Placeholder, Template, TemplateElement,
};
use hurl_core::parser::{parse_json_boolean, parse_json_null, parse_json_number};
use hurl_core::reader::Reader;

use super::template::eval_template;
use crate::runner::error::{RunnerError, RunnerErrorKind};
use crate::runner::{expr, VariableSet};

/// Evaluates a JSON value to a string given a set of `variables`.
/// If `keep_whitespace` is true, whitespace is preserved from the JSonValue, otherwise
/// it is trimmed.
pub fn eval_json_value(
    json_value: &JsonValue,
    variables: &VariableSet,
    keep_whitespace: bool,
) -> Result<String, RunnerError> {
    match json_value {
        JsonValue::Null => Ok("null".to_string()),
        JsonValue::Number(s) => Ok(s.clone()),
        JsonValue::String(template) => {
            let s = eval_json_template(template, variables)?;
            Ok(format!("\"{s}\""))
        }
        JsonValue::Boolean(v) => Ok(v.to_string()),
        JsonValue::List { space0, elements } => {
            let mut elems_string = vec![];
            for element in elements {
                let s = eval_json_list_element(element, variables, keep_whitespace)?;
                elems_string.push(s);
            }
            if keep_whitespace {
                Ok(format!("[{}{}]", space0, elems_string.join(",")))
            } else {
                Ok(format!("[{}]", elems_string.join(",")))
            }
        }
        JsonValue::Object { space0, elements } => {
            let mut elems_string = vec![];
            for element in elements {
                let s = eval_json_object_element(element, variables, keep_whitespace)?;
                elems_string.push(s);
            }
            if keep_whitespace {
                Ok(format!("{{{}{}}}", space0, elems_string.join(",")))
            } else {
                Ok(format!("{{{}}}", elems_string.join(",")))
            }
        }
        JsonValue::Placeholder(Placeholder { expr, .. }) => {
            let s = expr::render(expr, variables)?;

            // The String can only be null, a bool, a number
            // It will be easier when your variables value have a type
            let mut reader = Reader::new(s.as_str());
            let start = reader.cursor();
            if parse_json_number(&mut reader).is_ok() {
                return Ok(s);
            }
            reader.seek(start);
            if parse_json_boolean(&mut reader).is_ok() {
                return Ok(s);
            }
            reader.seek(start);
            if parse_json_null(&mut reader).is_ok() {
                return Ok(s);
            }
            let kind = RunnerErrorKind::InvalidJson { value: s };
            Err(RunnerError::new(expr.source_info, kind, false))
        }
    }
}

/// Evaluates a JSON list to a string given a set of `variables`.
/// If `keep_whitespace` is true, whitespace is preserved from the JSonValue, otherwise
/// it is trimmed.
fn eval_json_list_element(
    element: &JsonListElement,
    variables: &VariableSet,
    keep_whitespace: bool,
) -> Result<String, RunnerError> {
    let s = eval_json_value(&element.value, variables, keep_whitespace)?;
    if keep_whitespace {
        Ok(format!("{}{}{}", element.space0, s, element.space1))
    } else {
        Ok(s)
    }
}

/// Renders a JSON object to a string given a set of `variables`.
/// If `keep_whitespace` is true, whitespace is preserved from the JSonValue, otherwise
/// it is trimmed.
fn eval_json_object_element(
    element: &JsonObjectElement,
    variables: &VariableSet,
    keep_whitespace: bool,
) -> Result<String, RunnerError> {
    let name = eval_template(&element.name, variables)?;
    let value = eval_json_value(&element.value, variables, keep_whitespace)?;
    if keep_whitespace {
        Ok(format!(
            "{}\"{}\"{}:{}{}{}",
            element.space0, name, element.space1, element.space2, value, element.space3
        ))
    } else {
        Ok(format!("\"{}\":{}", element.name, value))
    }
}

/// Evaluates a JSON template to a string given a set of `variables`
///
/// # Example
///
/// The template "Hello {{quote}}" with variable quote="
/// will be evaluated to the JSON String "Hello \""
pub fn eval_json_template(
    template: &Template,
    variables: &VariableSet,
) -> Result<String, RunnerError> {
    let Template { elements, .. } = template;
    {
        let mut value = String::new();
        for elem in elements {
            match eval_json_template_element(elem, variables) {
                Ok(v) => value.push_str(v.as_str()),
                Err(e) => return Err(e),
            }
        }
        Ok(value)
    }
}

fn eval_json_template_element(
    template_element: &TemplateElement,
    variables: &VariableSet,
) -> Result<String, RunnerError> {
    match template_element {
        TemplateElement::String { source, .. } => Ok(source.to_string()),
        TemplateElement::Placeholder(Placeholder { expr, .. }) => {
            let s = expr::render(expr, variables)?;
            Ok(encode_json_string(&s))
        }
    }
}

fn encode_json_string(s: &str) -> String {
    s.chars().map(encode_json_char).collect()
}

fn encode_json_char(c: char) -> String {
    match c {
        '"' => "\\\"".to_string(),
        '\\' => "\\\\".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        c => c.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::*;
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use super::super::error::RunnerErrorKind;
    use super::*;
    use crate::runner::Value;

    pub fn json_hello_world_value() -> JsonValue {
        // "hello\u0020{{name}}!"
        JsonValue::String(Template {
            delimiter: Some('"'),
            elements: vec![
                TemplateElement::String {
                    value: "Hello ".to_string(),
                    source: "Hello\\u0020".to_source(),
                },
                TemplateElement::Placeholder(Placeholder {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                    },
                    expr: Expr {
                        kind: ExprKind::Variable(Variable {
                            name: "name".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 19)),
                        }),
                        source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 19)),
                    },
                    space1: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 19), Pos::new(1, 19)),
                    },
                }),
                TemplateElement::String {
                    value: "!".to_string(),
                    source: "!".to_source(),
                },
            ],
            source_info: SourceInfo::new(Pos::new(1, 2), Pos::new(1, 22)),
        })
    }

    pub fn json_person_value() -> JsonValue {
        JsonValue::Object {
            space0: "\n    ".to_string(),
            elements: vec![JsonObjectElement {
                space0: String::new(),
                name: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "firstName".to_string(),
                        source: "firstName".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                space1: String::new(),
                space2: " ".to_string(),
                value: JsonValue::String(Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "John".to_string(),
                        source: "John".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                }),
                space3: "\n".to_string(),
            }],
        }
    }

    #[test]
    fn test_scalar_value() {
        let mut variables = VariableSet::new();
        variables
            .insert("name".to_string(), Value::String("Bob".to_string()))
            .unwrap();
        assert_eq!(
            eval_json_value(&JsonValue::Null, &variables, true).unwrap(),
            "null".to_string()
        );
        assert_eq!(
            eval_json_value(&JsonValue::Number("3.14".to_string()), &variables, true).unwrap(),
            "3.14".to_string()
        );
        assert_eq!(
            eval_json_value(&JsonValue::Boolean(false), &variables, true).unwrap(),
            "false".to_string()
        );
        assert_eq!(
            eval_json_value(&json_hello_world_value(), &variables, true).unwrap(),
            "\"Hello\\u0020Bob!\"".to_string()
        );
    }

    #[test]
    fn test_error() {
        let variables = VariableSet::new();
        let error = eval_json_value(&json_hello_world_value(), &variables, true)
            .err()
            .unwrap();
        assert_eq!(
            error.source_info,
            SourceInfo::new(Pos::new(1, 15), Pos::new(1, 19))
        );
        assert_eq!(
            error.kind,
            RunnerErrorKind::TemplateVariableNotDefined {
                name: "name".to_string()
            }
        );
    }

    #[test]
    fn test_list_value() {
        let mut variables = VariableSet::new();
        variables
            .insert("name".to_string(), Value::String("Bob".to_string()))
            .unwrap();
        assert_eq!(
            eval_json_value(
                &JsonValue::List {
                    space0: String::new(),
                    elements: vec![],
                },
                &variables,
                true,
            )
            .unwrap(),
            "[]".to_string()
        );

        assert_eq!(
            eval_json_value(
                &JsonValue::List {
                    space0: String::new(),
                    elements: vec![
                        JsonListElement {
                            space0: String::new(),
                            value: JsonValue::Number("1".to_string()),
                            space1: String::new(),
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: JsonValue::Number("-2".to_string()),
                            space1: String::new(),
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: JsonValue::Number("3.0".to_string()),
                            space1: String::new(),
                        },
                    ],
                },
                &variables,
                true
            )
            .unwrap(),
            "[1, -2, 3.0]".to_string()
        );

        let template = Template {
            delimiter: Some('"'),
            elements: vec![TemplateElement::String {
                value: "Hi".to_string(),
                source: "Hi".to_source(),
            }],
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        };
        assert_eq!(
            eval_json_value(
                &JsonValue::List {
                    space0: String::new(),
                    elements: vec![
                        JsonListElement {
                            space0: String::new(),
                            value: JsonValue::String(template),
                            space1: String::new(),
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: json_hello_world_value(),
                            space1: String::new(),
                        },
                    ],
                },
                &variables,
                true
            )
            .unwrap(),
            "[\"Hi\", \"Hello\\u0020Bob!\"]".to_string()
        );
    }

    #[test]
    fn test_object_value() {
        let variables = VariableSet::new();
        assert_eq!(
            eval_json_value(
                &JsonValue::Object {
                    space0: String::new(),
                    elements: vec![],
                },
                &variables,
                true
            )
            .unwrap(),
            "{}".to_string()
        );
        assert_eq!(
            eval_json_value(&json_person_value(), &variables, true).unwrap(),
            r#"{
    "firstName": "John"
}"#
            .to_string()
        );
    }

    #[test]
    fn test_escape_sequence() {
        let variables = VariableSet::new();
        assert_eq!(
            eval_json_value(
                &JsonValue::String(Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\n".to_string(),
                        source: "\\n".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                }),
                &variables,
                true
            )
            .unwrap(),
            "\"\\n\"".to_string()
        );
    }

    #[test]
    fn test_eval_json_template() {
        let variables = VariableSet::new();
        assert_eq!(
            eval_json_template(
                &Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\n".to_string(),
                        source: "\\n".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                &variables,
            )
            .unwrap(),
            "\\n".to_string()
        );

        let mut variables = VariableSet::new();
        variables
            .insert("quote".to_string(), Value::String("\"".to_string()))
            .unwrap();
        assert_eq!(
            eval_json_template(
                &Template {
                    delimiter: Some('"'),
                    elements: vec![
                        TemplateElement::String {
                            value: "Hello ".to_string(),
                            source: "Hello ".to_source(),
                        },
                        TemplateElement::Placeholder(Placeholder {
                            space0: whitespace(),
                            expr: Expr {
                                kind: ExprKind::Variable(Variable {
                                    name: "quote".to_string(),
                                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                                }),
                                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                            },
                            space1: whitespace(),
                        }),
                    ],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                &variables,
            )
            .unwrap(),
            "Hello \\\"".to_string()
        );
    }

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    #[test]
    fn test_encode_json_string() {
        assert_eq!(encode_json_string("a"), "a");
        assert_eq!(encode_json_string("\""), "\\\"");
        assert_eq!(encode_json_string("\\"), "\\\\");
    }

    #[test]
    fn test_not_preserving_spaces() {
        let variables = VariableSet::new();
        assert_eq!(
            eval_json_value(&json_person_value(), &variables, false).unwrap(),
            r#"{"firstName":"John"}"#.to_string()
        );
    }
}
