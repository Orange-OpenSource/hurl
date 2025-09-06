use crate::jsonpath2::{JsonPathExpr, Segment};

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
mod selector;

#[allow(dead_code)]
pub type NodeList = Vec<serde_json::Value>;

impl JsonPathExpr {
    /// Eval a `JsonPathExpr` for a `serde_json::Value` input.
    /// It returns a `NodeList`
    #[allow(dead_code)]
    pub fn eval(&self, value: &serde_json::Value) -> NodeList {
        let mut results = vec![value.clone()];
        for segment in self.segments() {
            results = results.iter().flat_map(|node| segment.eval(node)).collect();
        }
        results
    }
}

impl Segment {
    /// Eval a `JsonPathExpr` for a `serde_json::Value` input.
    /// It returns a `NodeList`
    pub fn eval(&self, node: &serde_json::Value) -> NodeList {
        match self {
            Segment::Child(child_segment) => {
                let mut results = vec![];
                for selector in child_segment.selectors() {
                    results.append(&mut selector.eval(node));
                }
                results
            }
            Segment::Descendant(_descendant_segment) => todo!(),
        }
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
    fn test_root_identifier() {
        let value = json!({"greeting": "Hello"});
        let root_identifier = JsonPathExpr::new(vec![]);
        assert_eq!(root_identifier.eval(&value), vec![value]);
    }

    #[test]
    fn test_child_segment() {
        let value = json!({"greeting": "Hello"});
        let jsonpath1 = JsonPathExpr::new(vec![Segment::Child(ChildSegment::new(vec![
            Selector::Name(NameSelector::new("greeting".to_string())),
        ]))]);
        assert_eq!(jsonpath1.eval(&value), vec![json!("Hello")]);
    }
}
