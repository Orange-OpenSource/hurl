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
use super::core::Template;

///
/// This the AST for the JSON used within hurl
///
/// It is a superset of the standard json spec.
/// Strings have been replaced by hurl template.
///

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
