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
use super::ast::Template;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Number(String),
    String(Template),
    Boolean(bool),
    List {
        space0: String,
        elements: Vec<ListElement>,
    },
    Object {
        space0: String,
        elements: Vec<ObjectElement>,
    },
    Null {},
}

impl Value {
    pub fn _type(&self) -> String {
        match self {
            Value::Number(_) => "number".to_string(),
            Value::Null {} => "null".to_string(),
            Value::Boolean(_) => "boolean".to_string(),
            Value::List { .. } => "list".to_string(),
            Value::Object { .. } => "object".to_string(),
            Value::String(_) => "string".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListElement {
    pub space0: String,
    pub value: Value,
    pub space1: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectElement {
    pub space0: String,
    pub name: String,
    pub space1: String,
    pub space2: String,
    pub value: Value,
    pub space3: String,
}

#[cfg(test)]
pub mod tests {
    use super::super::ast::{Expr, TemplateElement, Variable, Whitespace};
    use super::super::common::SourceInfo;
    use super::*;

    pub fn person_value() -> Value {
        Value::Object {
            space0: "\n    ".to_string(),
            elements: vec![ObjectElement {
                space0: "".to_string(),
                name: "firstName".to_string(),
                space1: "".to_string(),
                space2: " ".to_string(),
                value: Value::String(Template {
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

    pub fn hello_world_value() -> Value {
        // "hello\u0020{{name}}!"
        Value::String(Template {
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
}
