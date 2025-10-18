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
use crate::ast::primitive::Placeholder;
use crate::ast::Template;
use crate::types::{SourceString, ToSource};

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

impl ToSource for JsonValue {
    fn to_source(&self) -> SourceString {
        match self {
            JsonValue::Placeholder(expr) => format!("{{{{{expr}}}}}").to_source(),
            JsonValue::Number(s) => s.to_source(),
            JsonValue::String(template) => template.to_source(),
            JsonValue::Boolean(value) => {
                if *value {
                    "true".to_source()
                } else {
                    "false".to_source()
                }
            }
            JsonValue::List { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_source())
                    .collect::<Vec<SourceString>>();
                format!("[{}{}]", space0, elements.join(",")).to_source()
            }
            JsonValue::Object { space0, elements } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_source())
                    .collect::<Vec<SourceString>>();
                format!("{{{}{}}}", space0, elements.join(",")).to_source()
            }
            JsonValue::Null => "null".to_source(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonListElement {
    pub space0: String,
    pub value: JsonValue,
    pub space1: String,
}

impl ToSource for JsonListElement {
    fn to_source(&self) -> SourceString {
        let mut s = SourceString::new();
        s.push_str(self.space0.as_str());
        s.push_str(self.value.to_source().as_str());
        s.push_str(self.space1.as_str());
        s
    }
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

impl ToSource for JsonObjectElement {
    fn to_source(&self) -> SourceString {
        let mut s = SourceString::new();
        s.push_str(self.space0.as_str());
        s.push_str(self.name.to_source().as_str());
        s.push_str(self.space1.as_str());
        s.push(':');
        s.push_str(self.space2.as_str());
        s.push_str(self.value.to_source().as_str());
        s.push_str(self.space3.as_str());
        s
    }
}
