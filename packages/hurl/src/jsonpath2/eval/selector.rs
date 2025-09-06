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

use crate::jsonpath2::{eval::NodeList, IndexSelector, NameSelector, Selector};

impl Selector {
    pub fn eval(&self, node: &serde_json::Value) -> NodeList {
        match self {
            Selector::Name(name_selector) => name_selector.eval(node),
            Selector::Wildcard(_wildcard_selector) => todo!(),
            Selector::Index(index_selector) => index_selector.eval(node),
            Selector::ArraySlice(_array_slice_selector) => todo!(),
            Selector::Filter(_filter_selector) => todo!(),
        }
    }
}

impl NameSelector {
    pub fn eval(&self, node: &serde_json::Value) -> NodeList {
        if let serde_json::Value::Object(key_values) = node {
            if let Some(value) = key_values.get(self.value()) {
                return vec![value.clone()];
            }
        }
        vec![]
    }
}

impl IndexSelector {
    pub fn eval(&self, node: &serde_json::Value) -> NodeList {
        if let serde_json::Value::Array(values) = node {
            let index = if *self.value() < 0 {
                values.len() - ((*self.value()).unsigned_abs() as usize)
            } else {
                *self.value() as usize
            };
            if let Some(value) = values.get(index) {
                return vec![value.clone()];
            }
        }
        vec![]
    }
}

mod tests {
    #[allow(unused_imports)]
    use serde_json::json;

    #[allow(unused_imports)]
    use crate::jsonpath2::{
        ChildSegment, IndexSelector, JsonPathExpr, NameSelector, Segment, Selector,
    };

    #[test]
    fn test_selector() {
        let value = json!({"greeting": "Hello"});
        assert_eq!(
            Selector::Name(NameSelector::new("greeting".to_string())).eval(&value),
            vec![json!("Hello")]
        );
    }

    #[test]
    fn test_name_selector() {
        let value = json!({"greeting": "Hello"});
        assert_eq!(
            NameSelector::new("greeting".to_string()).eval(&value),
            vec![json!("Hello")]
        );
    }

    #[test]
    fn test_index_selector() {
        let value = json!(["a", "b"]);
        assert_eq!(IndexSelector::new(1).eval(&value), vec![json!("b")]);
        assert_eq!(IndexSelector::new(-2).eval(&value), vec![json!("a")]);
        assert!(IndexSelector::new(2).eval(&value).is_empty());
    }
}
