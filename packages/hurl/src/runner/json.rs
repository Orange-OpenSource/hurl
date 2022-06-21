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

use hurl_core::ast::{JsonListElement, JsonObjectElement, JsonValue, Template, TemplateElement};
use hurl_core::parser::{parse_json_boolean, parse_json_null, parse_json_number, Reader};

use super::core::{Error, RunnerError};
use super::value::Value;
use crate::runner::template::eval_expression;

pub fn eval_json_value(
    json_value: JsonValue,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match json_value {
        JsonValue::Null {} => Ok("null".to_string()),
        JsonValue::Number(s) => Ok(s),
        JsonValue::String(template) => {
            let s = eval_json_template(template, variables)?;
            Ok(format!("\"{}\"", s))
        }
        JsonValue::Boolean(v) => Ok(v.to_string()),
        JsonValue::List { space0, elements } => {
            let mut elems_string = vec![];
            for element in elements {
                let s = eval_json_list_element(element, variables)?;
                elems_string.push(s);
            }
            Ok(format!("[{}{}]", space0, elems_string.join(",")))
        }
        JsonValue::Object { space0, elements } => {
            let mut elems_string = vec![];
            for element in elements {
                let s = eval_json_object_element(element, variables)?;
                elems_string.push(s);
            }
            Ok(format!("{{{}{}}}", space0, elems_string.join(",")))
        }
        JsonValue::Expression(exp) => {
            let s = eval_expression(exp.clone(), variables)?;

            // The String can only be null, a bool, a number
            // It will be easier when your variables value have a type
            let mut reader = Reader::init(s.as_str());
            let start = reader.state.clone();
            if parse_json_number(&mut reader).is_ok() {
                return Ok(s);
            }
            reader.state = start.clone();
            if parse_json_boolean(&mut reader).is_ok() {
                return Ok(s);
            }
            reader.state = start;
            if parse_json_null(&mut reader).is_ok() {
                return Ok(s);
            }
            Err(Error {
                source_info: exp.variable.source_info,
                inner: RunnerError::InvalidJson { value: s },
                assert: false,
            })
        }
    }
}

pub fn eval_json_list_element(
    element: JsonListElement,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    let s = eval_json_value(element.value, variables)?;
    Ok(format!("{}{}{}", element.space0, s, element.space1))
}

pub fn eval_json_object_element(
    element: JsonObjectElement,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    let value = eval_json_value(element.value, variables)?;
    Ok(format!(
        "{}\"{}\"{}:{}{}{}",
        element.space0, element.name, element.space1, element.space2, value, element.space3
    ))
}

/// Eval a JSON template to a valid JSON string
/// The variable are replaced by their value and encoded into JSON
///
/// # Arguments
///
/// * `template` - An Hurl Template
/// * `variables` - A map of input variables
///
/// # Example
///
/// The template "Hello {{quote}}" with variable quote="
/// will be evaluated to the JSON String "Hello \""
///
pub fn eval_json_template(
    template: Template,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    let Template { elements, .. } = template;
    {
        let mut value = String::from("");
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
    template_element: TemplateElement,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match template_element {
        TemplateElement::String { encoded, .. } => Ok(encoded),
        TemplateElement::Expression(expr) => {
            let s = eval_expression(expr, variables)?;
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
    use super::super::core::RunnerError;
    use super::*;
    use hurl_core::ast::*;

    pub fn json_hello_world_value() -> JsonValue {
        // "hello\u0020{{name}}!"
        JsonValue::String(Template {
            quotes: true,
            elements: vec![
                TemplateElement::String {
                    value: "Hello ".to_string(),
                    encoded: "Hello\\u0020".to_string(),
                },
                TemplateElement::Expression(Expr {
                    space0: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(1, 15, 1, 15),
                    },
                    variable: Variable {
                        name: "name".to_string(),
                        source_info: SourceInfo::init(1, 15, 1, 19),
                    },
                    space1: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(1, 19, 1, 19),
                    },
                }),
                TemplateElement::String {
                    value: "!".to_string(),
                    encoded: "!".to_string(),
                },
            ],
            source_info: SourceInfo::init(1, 2, 1, 22),
        })
    }

    pub fn json_person_value() -> JsonValue {
        JsonValue::Object {
            space0: "\n    ".to_string(),
            elements: vec![JsonObjectElement {
                space0: "".to_string(),
                name: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "firstName".to_string(),
                        encoded: "firstName".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 1, 1, 1),
                },
                space1: "".to_string(),
                space2: " ".to_string(),
                value: JsonValue::String(Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "John".to_string(),
                        encoded: "John".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 1, 1, 1),
                }),
                space3: "\n".to_string(),
            }],
        }
    }

    #[test]
    fn test_scalar_value() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), Value::String("Bob".to_string()));
        assert_eq!(
            eval_json_value(JsonValue::Null {}, &variables).unwrap(),
            "null".to_string()
        );
        assert_eq!(
            eval_json_value(JsonValue::Number("3.14".to_string()), &variables).unwrap(),
            "3.14".to_string()
        );
        assert_eq!(
            eval_json_value(JsonValue::Boolean(false), &variables).unwrap(),
            "false".to_string()
        );
        assert_eq!(
            eval_json_value(json_hello_world_value(), &variables).unwrap(),
            "\"Hello\\u0020Bob!\"".to_string()
        );
    }

    #[test]
    fn test_error() {
        let variables = HashMap::new();
        let error = eval_json_value(json_hello_world_value(), &variables)
            .err()
            .unwrap();
        assert_eq!(error.source_info, SourceInfo::init(1, 15, 1, 19));
        assert_eq!(
            error.inner,
            RunnerError::TemplateVariableNotDefined {
                name: "name".to_string()
            }
        );
    }

    #[test]
    fn test_list_value() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), Value::String("Bob".to_string()));
        assert_eq!(
            eval_json_value(
                JsonValue::List {
                    space0: "".to_string(),
                    elements: vec![],
                },
                &variables,
            )
            .unwrap(),
            "[]".to_string()
        );

        assert_eq!(
            eval_json_value(
                JsonValue::List {
                    space0: "".to_string(),
                    elements: vec![
                        JsonListElement {
                            space0: "".to_string(),
                            value: JsonValue::Number("1".to_string()),
                            space1: "".to_string(),
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: JsonValue::Number("-2".to_string()),
                            space1: "".to_string(),
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: JsonValue::Number("3.0".to_string()),
                            space1: "".to_string(),
                        },
                    ],
                },
                &variables,
            )
            .unwrap(),
            "[1, -2, 3.0]".to_string()
        );

        let template = Template {
            quotes: true,
            elements: vec![TemplateElement::String {
                encoded: "Hi".to_string(),
                value: "Hi".to_string(),
            }],
            source_info: SourceInfo::init(0, 0, 0, 0),
        };
        assert_eq!(
            eval_json_value(
                JsonValue::List {
                    space0: "".to_string(),
                    elements: vec![
                        JsonListElement {
                            space0: "".to_string(),
                            value: JsonValue::String(template),
                            space1: "".to_string(),
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: json_hello_world_value(),
                            space1: "".to_string(),
                        },
                    ],
                },
                &variables,
            )
            .unwrap(),
            "[\"Hi\", \"Hello\\u0020Bob!\"]".to_string()
        );
    }

    #[test]
    fn test_object_value() {
        let variables = HashMap::new();
        assert_eq!(
            eval_json_value(
                JsonValue::Object {
                    space0: "".to_string(),
                    elements: vec![],
                },
                &variables,
            )
            .unwrap(),
            "{}".to_string()
        );
        assert_eq!(
            eval_json_value(json_person_value(), &variables).unwrap(),
            r#"{
    "firstName": "John"
}"#
            .to_string()
        );
    }

    #[test]
    fn test_escape_sequence() {
        let variables = HashMap::new();
        assert_eq!(
            eval_json_value(
                JsonValue::String(Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "\n".to_string(),
                        encoded: "\\n".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 1, 1, 1),
                }),
                &variables,
            )
            .unwrap(),
            "\"\\n\"".to_string()
        );
    }

    #[test]
    fn test_eval_json_template() {
        let variables = HashMap::new();
        assert_eq!(
            eval_json_template(
                Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "\n".to_string(),
                        encoded: "\\n".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 1, 1, 1),
                },
                &variables,
            )
            .unwrap(),
            "\\n".to_string()
        );

        let mut variables = HashMap::new();
        variables.insert("quote".to_string(), Value::String("\"".to_string()));
        assert_eq!(
            eval_json_template(
                Template {
                    quotes: true,
                    elements: vec![
                        TemplateElement::String {
                            value: "Hello ".to_string(),
                            encoded: "Hello ".to_string(),
                        },
                        TemplateElement::Expression(Expr {
                            space0: whitespace(),
                            variable: Variable {
                                name: "quote".to_string(),
                                source_info: SourceInfo::init(0, 0, 0, 0),
                            },
                            space1: whitespace(),
                        }),
                    ],
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                &variables,
            )
            .unwrap(),
            "Hello \\\"".to_string()
        );
    }

    fn whitespace() -> Whitespace {
        Whitespace {
            value: "".to_string(),
            source_info: SourceInfo::init(0, 0, 0, 0),
        }
    }

    #[test]
    fn test_encode_json_string() {
        assert_eq!(encode_json_string("a"), "a");
        assert_eq!(encode_json_string("\""), "\\\"");
        assert_eq!(encode_json_string("\\"), "\\\\");
    }
}
