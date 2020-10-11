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

use crate::ast::{JsonListElement, JsonObjectElement, JsonValue};

use super::core::Error;
use super::value::Value;

impl JsonValue {
    pub fn eval(self, variables: &HashMap<String, Value>) -> Result<String, Error> {
        match self {
            JsonValue::Null {} => Ok("null".to_string()),
            JsonValue::Number(s) => Ok(s),
            JsonValue::String(template) => {
                let s = template.eval(variables)?;
                Ok(format!("\"{}\"", s))
            }
            JsonValue::Boolean(v) => Ok(v.to_string()),
            JsonValue::List { space0, elements } => {
                let mut elems_string = vec![];
                for element in elements {
                    let s = element.eval(variables)?;
                    elems_string.push(s);
                }
                Ok(format!("[{}{}]", space0, elems_string.join(",")))
            }
            JsonValue::Object { space0, elements } => {
                let mut elems_string = vec![];
                for element in elements {
                    let s = element.eval(variables)?;
                    elems_string.push(s);
                }
                Ok(format!("{{{}{}}}", space0, elems_string.join(",")))
            }
        }
    }
}

impl JsonListElement {
    pub fn eval(self, variables: &HashMap<String, Value>) -> Result<String, Error> {
        let s = self.value.eval(variables)?;
        Ok(format!("{}{}{}", self.space0, s, self.space1))
    }
}

impl JsonObjectElement {
    pub fn eval(self, variables: &HashMap<String, Value>) -> Result<String, Error> {
        let value = self.value.eval(variables)?;
        Ok(format!(
            "{}\"{}\"{}:{}{}{}",
            self.space0, self.name, self.space1, self.space2, value, self.space3
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::core::RunnerError;
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_scalar_value() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), Value::String("Bob".to_string()));
        assert_eq!(
            JsonValue::Null {}.eval(&variables).unwrap(),
            "null".to_string()
        );
        assert_eq!(
            JsonValue::Number("3.14".to_string())
                .eval(&variables)
                .unwrap(),
            "3.14".to_string()
        );
        assert_eq!(
            JsonValue::Boolean(false).eval(&variables).unwrap(),
            "false".to_string()
        );
        assert_eq!(
            json_hello_world_value().eval(&variables).unwrap(),
            "\"Hello Bob!\"".to_string()
        );
    }

    #[test]
    fn test_error() {
        let variables = HashMap::new();
        let error = json_hello_world_value().eval(&variables).err().unwrap();
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
            JsonValue::List {
                space0: "".to_string(),
                elements: vec![],
            }
            .eval(&variables)
            .unwrap(),
            "[]".to_string()
        );

        assert_eq!(
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
            }
            .eval(&variables)
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
            }
            .eval(&variables)
            .unwrap(),
            "[\"Hi\", \"Hello Bob!\"]".to_string()
        );
    }

    #[test]
    fn test_object_value() {
        let variables = HashMap::new();
        assert_eq!(
            JsonValue::Object {
                space0: "".to_string(),
                elements: vec![]
            }
            .eval(&variables)
            .unwrap(),
            "{}".to_string()
        );
        assert_eq!(
            json_person_value().eval(&variables).unwrap(),
            r#"{
    "firstName": "John"
}"#
            .to_string()
        );
    }
}
