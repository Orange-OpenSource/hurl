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
use core::fmt;

use crate::ast::core::Template;
use crate::ast::{Placeholder, TemplateElement};

/// This the AST for the JSON used within Hurl (for instance in [implicit JSON body request](https://hurl.dev/docs/request.html#json-body)).
///
/// # Example
///
/// ```hurl
/// POST https://example.org/api/cats
/// {
///     "id": 42,
///     "lives": {{lives_count}},
///     "name": "{{name}}"
/// }
/// ```
///
/// It is a superset of the standard JSON spec. Strings have been replaced by Hurl [`Placeholder`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Placeholder(Placeholder),
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
    Null,
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Value::Placeholder(expr) => format!("{{{{{expr}}}}}"),
            Value::Number(s) => s.to_string(),
            Value::String(template) => format!("\"{template}\""),
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
            Value::Null => "null".to_string(),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for ListElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str(self.space0.as_str());
        s.push_str(self.value.to_string().as_str());
        s.push_str(self.space1.as_str());
        write!(f, "{s}")
    }
}

impl fmt::Display for ObjectElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str(self.space0.as_str());
        s.push('"');
        s.push_str(self.name.to_string().as_str());
        s.push('"');
        s.push_str(self.space1.as_str());
        s.push(':');
        s.push_str(self.space2.as_str());
        s.push_str(self.value.to_string().as_str());
        s.push_str(self.space3.as_str());
        write!(f, "{s}")
    }
}

impl Value {
    pub fn encoded(&self) -> String {
        match self {
            Value::Placeholder(expr) => format!("{{{{{expr}}}}}"),
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
            Value::Null => "null".to_string(),
        }
    }
}

impl ListElement {
    fn encoded(&self) -> String {
        let mut s = String::new();
        s.push_str(self.space0.as_str());
        s.push_str(self.value.encoded().as_str());
        s.push_str(self.space1.as_str());
        s
    }
}

impl ObjectElement {
    fn encoded(&self) -> String {
        let mut s = String::new();
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
        let mut s = String::new();
        if let Some(d) = self.delimiter {
            s.push(d);
        }
        let elements: Vec<String> = self.elements.iter().map(|e| e.encoded()).collect();
        s.push_str(elements.join("").as_str());
        if let Some(d) = self.delimiter {
            s.push(d);
        }
        s
    }
}

impl TemplateElement {
    fn encoded(&self) -> String {
        match self {
            TemplateElement::String { source, .. } => source.to_string(),
            TemplateElement::Placeholder(expr) => format!("{{{{{expr}}}}}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expr, ExprKind, SourceInfo, TemplateElement, Variable, Whitespace};
    use crate::reader::Pos;
    use crate::typing::ToSource;

    #[test]
    fn test_to_string() {
        assert_eq!(
            "{{x}}".to_string(),
            Value::Placeholder(Placeholder {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                expr: Expr {
                    kind: ExprKind::Variable(Variable {
                        name: "x".to_string(),
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    }),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            })
            .to_string()
        );
        assert_eq!("1".to_string(), Value::Number("1".to_string()).to_string());
        assert_eq!(
            "\"hello\"".to_string(),
            Value::String(Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "hello".to_string(),
                    source: "hello".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            })
            .to_string()
        );
        assert_eq!("true".to_string(), Value::Boolean(true).to_string());
        assert_eq!(
            "[]".to_string(),
            Value::List {
                space0: String::new(),
                elements: vec![],
            }
            .to_string()
        );
        assert_eq!(
            "[1, 2, 3]".to_string(),
            Value::List {
                space0: String::new(),
                elements: vec![
                    ListElement {
                        space0: String::new(),
                        value: Value::Number("1".to_string()),
                        space1: String::new(),
                    },
                    ListElement {
                        space0: " ".to_string(),
                        value: Value::Number("2".to_string()),
                        space1: String::new(),
                    },
                    ListElement {
                        space0: " ".to_string(),
                        value: Value::Number("3".to_string()),
                        space1: String::new(),
                    }
                ],
            }
            .to_string()
        );
        assert_eq!(
            "{}".to_string(),
            Value::Object {
                space0: String::new(),
                elements: vec![],
            }
            .to_string()
        );
        assert_eq!(
            "{ \"id\": 123 }".to_string(),
            Value::Object {
                space0: String::new(),
                elements: vec![ObjectElement {
                    space0: " ".to_string(),
                    name: Template {
                        delimiter: Some('"'),
                        elements: vec![TemplateElement::String {
                            value: "id".to_string(),
                            source: "id".to_source(),
                        }],
                        source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    },
                    space1: String::new(),
                    space2: " ".to_string(),
                    value: Value::Number("123".to_string()),
                    space3: " ".to_string(),
                }],
            }
            .to_string()
        );
        assert_eq!("null".to_string(), Value::Null.to_string());
    }

    #[test]
    fn test_encoded() {
        assert_eq!(
            TemplateElement::Placeholder(Placeholder {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                expr: Expr {
                    kind: ExprKind::Variable(Variable {
                        name: "name".to_string(),
                        source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    }),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
            })
            .encoded(),
            "{{name}}".to_string()
        );
        assert_eq!(
            Template {
                delimiter: None,
                elements: vec![TemplateElement::Placeholder(Placeholder {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    },
                    expr: Expr {
                        kind: ExprKind::Variable(Variable {
                            name: "name".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                        }),
                        source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    },
                    space1: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    },
                })],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            }
            .encoded(),
            "{{name}}".to_string()
        );
    }
}
