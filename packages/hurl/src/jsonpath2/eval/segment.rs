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
use crate::jsonpath2::ast::segment::{ChildSegment, DescendantSegment, Segment};
use crate::jsonpath2::eval::NodeList;

impl Segment {
    /// Eval a `Segment` for the current `serde_json::Value` input.
    /// It returns a `NodeList`
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> NodeList {
        match self {
            Segment::Child(child_segment) => child_segment.eval(current_value, root_value),
            Segment::Descendant(descendant_segment) => {
                descendant_segment.eval(current_value, root_value)
            }
        }
    }
}

impl ChildSegment {
    /// Eval a `ChildSegment` for the current `serde_json::Value` input.
    /// It returns a `NodeList`
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> NodeList {
        let mut results = vec![];
        for selector in self.selectors() {
            results.append(&mut selector.eval(current_value, root_value));
        }
        results
    }
}

impl DescendantSegment {
    /// Eval a `DescendantSegment` for the current `serde_json::Value` input.
    /// It returns a `NodeList`
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> NodeList {
        let mut nodes = vec![];

        for descendent in &descendants(current_value) {
            for selector in self.selectors() {
                nodes.append(&mut selector.eval(descendent, root_value));
            }
        }
        nodes
    }
}

fn descendants(node: &serde_json::Value) -> NodeList {
    let mut nodes = vec![node.clone()];
    match node {
        serde_json::Value::Object(map) => {
            for (_, value) in map {
                nodes.append(&mut descendants(value));
            }
        }
        serde_json::Value::Array(values) => {
            for value in values {
                nodes.append(&mut descendants(value));
            }
        }
        _ => {}
    }
    nodes
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use crate::jsonpath2::ast::segment::{ChildSegment, Segment};
    #[allow(unused_imports)]
    use crate::jsonpath2::ast::selector::{
        IndexSelector, NameSelector, Selector, WildcardSelector,
    };
    #[allow(unused_imports)]
    use serde_json::json;

    #[test]
    fn test_segment() {
        let current_value = json!({"greeting": "Hello"});
        let root_value = json!("unused");
        assert_eq!(
            Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                "greeting".to_string()
            )),]))
            .eval(&current_value, &root_value),
            vec![json!("Hello")]
        );
    }

    #[test]
    fn test_child_segment() {
        let current_value = json!({"greeting": "Hello"});
        let root_value = json!("unused");
        assert_eq!(
            ChildSegment::new(vec![Selector::Name(NameSelector::new(
                "greeting".to_string()
            )),])
            .eval(&current_value, &root_value),
            vec![json!("Hello")]
        );
    }

    #[test]
    fn test_descendant_segment() {
        let current_value = json!({
          "o": {"j": 1, "k": 2},
          "a": [5, 3, [{"j": 4}, {"k": 6}]]
        });
        let root_value = json!("unused");
        assert_eq!(
            DescendantSegment::new(vec![Selector::Name(NameSelector::new("j".to_string()))])
                .eval(&current_value, &root_value),
            vec![json!(4), json!(1),]
        );
        assert_eq!(
            DescendantSegment::new(vec![Selector::Index(IndexSelector::new(0))])
                .eval(&current_value, &root_value),
            vec![json!(5), json!({"j": 4}),]
        );
        assert_eq!(
            DescendantSegment::new(vec![Selector::Wildcard(WildcardSelector)])
                .eval(&current_value, &root_value),
            vec![
                json!([5, 3, [{"j": 4}, {"k": 6}]]),
                json!({"j": 1, "k": 2}),
                json!(5),
                json!(3),
                json!([{"j": 4}, {"k": 6}]),
                json!({"j": 4}),
                json!({"k": 6}),
                json!(4),
                json!(6),
                json!(1),
                json!(2),
            ]
        );
    }

    #[test]
    fn test_descendants() {
        assert_eq!(descendants(&json!("Hello")), vec![json!("Hello")]);
        assert_eq!(
            descendants(&json!([1, 2, 3])),
            vec![json!([1, 2, 3]), json!(1), json!(2), json!(3)]
        );
        assert_eq!(
            descendants(&json!({"name": "Bob"})),
            vec![json!({"name": "Bob"}), json!("Bob")]
        );
    }
}
