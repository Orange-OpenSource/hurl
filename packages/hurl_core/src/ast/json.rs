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
use std::fmt;

use crate::ast::primitive::Placeholder;
use crate::ast::Template;
use crate::typing::{SourceString, ToSource};

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
pub enum JsonValue {
    Placeholder(Placeholder),
    Number(String),
    String(Template),
    Boolean(bool),
    List {
        space0: String,
        elements: Vec<JsonListElement>,
    },
    Object {
        space0: String,
        elements: Vec<JsonObjectElement>,
    },
    Null,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonListElement {
    pub space0: String,
    pub value: JsonValue,
    pub space1: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonObjectElement {
    pub space0: String,
    pub name: Template,
    pub space1: String,
    pub space2: String,
    pub value: JsonValue,
    pub space3: String,
}

impl fmt::Display for JsonObjectElement {
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

impl ToSource for JsonObjectElement {
    fn to_source(&self) -> SourceString {
        let mut s = SourceString::new();
        s.push_str(self.space0.as_str());
        s.push_str(self.name.to_source().as_str());
        s.push_str(self.space1.as_str());
        s.push(':');
        s.push_str(self.space2.as_str());
        s.push_str(self.value.encoded().as_str());
        s.push_str(self.space3.as_str());
        s
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            JsonValue::Placeholder(expr) => format!("{{{{{expr}}}}}"),
            JsonValue::Number(s) => s.to_string(),
            JsonValue::String(template) => format!("\"{template}\""),
            JsonValue::Boolean(value) => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            JsonValue::List { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>();
                format!("[{}{}]", space0, elements.join(","))
            }
            JsonValue::Object { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>();
                format!("{{{}{}}}", space0, elements.join(","))
            }
            JsonValue::Null => "null".to_string(),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for JsonListElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str(self.space0.as_str());
        s.push_str(self.value.to_string().as_str());
        s.push_str(self.space1.as_str());
        write!(f, "{s}")
    }
}

impl JsonValue {
    pub fn encoded(&self) -> String {
        match self {
            JsonValue::Placeholder(expr) => format!("{{{{{expr}}}}}"),
            JsonValue::Number(s) => s.to_string(),
            JsonValue::String(template) => template.to_source().to_string(),
            JsonValue::Boolean(value) => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            JsonValue::List { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.encoded())
                    .collect::<Vec<String>>();
                format!("[{}{}]", space0, elements.join(","))
            }
            JsonValue::Object { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_source().to_string())
                    .collect::<Vec<String>>();
                format!("{{{}{}}}", space0, elements.join(","))
            }
            JsonValue::Null => "null".to_string(),
        }
    }
}

impl JsonListElement {
    fn encoded(&self) -> String {
        let mut s = String::new();
        s.push_str(self.space0.as_str());
        s.push_str(self.value.encoded().as_str());
        s.push_str(self.space1.as_str());
        s
    }
}
