/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

use hurl_core::ast::{JsonListElement, JsonObjectElement, JsonValue};

use super::core::Error;
use super::template::eval_template;
use super::value::Value;

pub fn eval_json_value(
    json_value: JsonValue,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match json_value {
        JsonValue::Null {} => Ok("null".to_string()),
        JsonValue::Number(s) => Ok(s),
        JsonValue::String(template) => {
            let s = eval_template(template, variables)?;
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
                name: "firstName".to_string(),
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
            "\"Hello Bob!\"".to_string()
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
                &variables
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
                            space1: "".to_string()
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: JsonValue::Number("-2".to_string()),
                            space1: "".to_string()
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: JsonValue::Number("3.0".to_string()),
                            space1: "".to_string()
                        },
                    ],
                },
                &variables
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
                            space1: "".to_string()
                        },
                        JsonListElement {
                            space0: " ".to_string(),
                            value: json_hello_world_value(),
                            space1: "".to_string()
                        },
                    ],
                },
                &variables
            )
            .unwrap(),
            "[\"Hi\", \"Hello Bob!\"]".to_string()
        );
    }

    #[test]
    fn test_object_value() {
        let variables = HashMap::new();
        assert_eq!(
            eval_json_value(
                JsonValue::Object {
                    space0: "".to_string(),
                    elements: vec![]
                },
                &variables
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
}
