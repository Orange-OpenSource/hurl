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
use super::core::Template;
use crate::ast::{Expr, JsonValue, TemplateElement};
use core::fmt;

///
/// This the AST for the JSON used within hurl
///
/// It is a superset of the standard json spec.
/// Strings have been replaced by hurl template.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Expression(Expr),
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
            Value::Expression(_) => "expression".to_string(),
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
    pub name: Template,
    pub space1: String,
    pub space2: String,
    pub value: Value,
    pub space3: String,
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Value::Expression(expr) => format!("{{{{{}}}}}", expr),
            Value::Number(s) => s.to_string(),
            Value::String(template) => format!("\"{}\"", template),
            Value::Boolean(value) => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Value::List { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>();
                format!("[{}{}]", space0, elements.join(","))
            }
            Value::Object { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>();
                format!("{{{}{}}}", space0, elements.join(","))
            }
            Value::Null { .. } => "null".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for ListElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        s.push_str(self.space0.as_str());
        s.push_str(self.value.to_string().as_str());
        s.push_str(self.space1.as_str());
        write!(f, "{}", s)
    }
}

impl fmt::Display for ObjectElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        s.push_str(self.space0.as_str());
        s.push('"');
        s.push_str(self.name.to_string().as_str());
        s.push('"');
        s.push_str(self.space1.as_str());
        s.push(':');
        s.push_str(self.space2.as_str());
        s.push_str(self.value.to_string().as_str());
        s.push_str(self.space3.as_str());
        write!(f, "{}", s)
    }
}

impl JsonValue {
    pub fn encoded(&self) -> String {
        match self {
            Value::Expression(expr) => format!("{{{{{}}}}}", expr),
            Value::Number(s) => s.to_string(),
            Value::String(template) => template.encoded(),
            Value::Boolean(value) => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Value::List { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.encoded())
                    .collect::<Vec<String>>();
                format!("[{}{}]", space0, elements.join(","))
            }
            Value::Object { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.encoded())
                    .collect::<Vec<String>>();
                format!("{{{}{}}}", space0, elements.join(","))
            }
            Value::Null { .. } => "null".to_string(),
        }
    }
}

impl ListElement {
    fn encoded(&self) -> String {
        let mut s = "".to_string();
        s.push_str(self.space0.as_str());
        s.push_str(self.value.encoded().as_str());
        s.push_str(self.space1.as_str());
        s
    }
}

impl ObjectElement {
    fn encoded(&self) -> String {
        let mut s = "".to_string();
        s.push_str(self.space0.as_str());
        s.push_str(self.name.encoded().as_str());
        s.push_str(self.space1.as_str());
        s.push(':');
        s.push_str(self.space2.as_str());
        s.push_str(self.value.encoded().as_str());
        s.push_str(self.space3.as_str());
        s
    }
}

impl Template {
    fn encoded(&self) -> String {
        let mut s = "".to_string();
        if self.quotes {
            s.push('"')
        }
        let elements: Vec<String> = self.elements.iter().map(|e| e.encoded()).collect();
        s.push_str(elements.join("").as_str());
        if self.quotes {
            s.push('"')
        }
        s
    }
}

impl TemplateElement {
    fn encoded(&self) -> String {
        match self {
            TemplateElement::String { encoded, .. } => encoded.to_string(),
            TemplateElement::Expression(expr) => format!("{{{{{}}}}}", expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{SourceInfo, TemplateElement, Variable, Whitespace};

    #[test]
    fn test_to_string() {
        assert_eq!(
            "{{x}}".to_string(),
            JsonValue::Expression(Expr {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                variable: Variable {
                    name: "x".to_string(),
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                space1: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
            })
            .to_string()
        );
        assert_eq!(
            "1".to_string(),
            JsonValue::Number("1".to_string()).to_string()
        );
        assert_eq!(
            "\"hello\"".to_string(),
            JsonValue::String(Template {
                quotes: false,
                elements: vec![TemplateElement::String {
                    value: "hello".to_string(),
                    encoded: "hello".to_string(),
                }],
                source_info: SourceInfo::init(0, 0, 0, 0),
            })
            .to_string()
        );
        assert_eq!("true".to_string(), JsonValue::Boolean(true).to_string());
        assert_eq!(
            "[]".to_string(),
            JsonValue::List {
                space0: "".to_string(),
                elements: vec![],
            }
            .to_string()
        );
        assert_eq!(
            "[1, 2, 3]".to_string(),
            JsonValue::List {
                space0: "".to_string(),
                elements: vec![
                    ListElement {
                        space0: "".to_string(),
                        value: JsonValue::Number("1".to_string()),
                        space1: "".to_string(),
                    },
                    ListElement {
                        space0: " ".to_string(),
                        value: JsonValue::Number("2".to_string()),
                        space1: "".to_string(),
                    },
                    ListElement {
                        space0: " ".to_string(),
                        value: JsonValue::Number("3".to_string()),
                        space1: "".to_string(),
                    }
                ],
            }
            .to_string()
        );
        assert_eq!(
            "{}".to_string(),
            JsonValue::Object {
                space0: "".to_string(),
                elements: vec![],
            }
            .to_string()
        );
        assert_eq!(
            "{ \"id\": 123 }".to_string(),
            JsonValue::Object {
                space0: "".to_string(),
                elements: vec![ObjectElement {
                    space0: " ".to_string(),
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "id".to_string(),
                            encoded: "id".to_string(),
                        }],
                        source_info: SourceInfo::init(1, 1, 1, 1),
                    },
                    space1: "".to_string(),
                    space2: " ".to_string(),
                    value: JsonValue::Number("123".to_string()),
                    space3: " ".to_string(),
                }],
            }
            .to_string()
        );
        assert_eq!("null".to_string(), JsonValue::Null {}.to_string());
    }

    #[test]
    fn test_encoded() {
        assert_eq!(
            TemplateElement::Expression(Expr {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 1, 1, 1),
                },
                variable: Variable {
                    name: "name".to_string(),
                    source_info: SourceInfo::init(1, 1, 1, 1),
                },
                space1: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 1, 1, 1),
                },
            })
            .encoded(),
            "{{name}}".to_string()
        );
        assert_eq!(
            Template {
                quotes: false,
                elements: vec![TemplateElement::Expression(Expr {
                    space0: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(1, 1, 1, 1),
                    },
                    variable: Variable {
                        name: "name".to_string(),
                        source_info: SourceInfo::init(1, 1, 1, 1),
                    },
                    space1: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(1, 1, 1, 1),
                    },
                })],
                source_info: SourceInfo::init(1, 1, 1, 1),
            }
            .encoded(),
            "{{name}}".to_string()
        );
    }
}
