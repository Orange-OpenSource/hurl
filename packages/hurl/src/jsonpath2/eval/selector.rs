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

use std::cmp::{max, min};

use crate::jsonpath2::{
    eval::NodeList, ArraySliceSelector, IndexSelector, NameSelector, Selector, WildcardSelector,
};

impl Selector {
    pub fn eval(&self, node: &serde_json::Value) -> NodeList {
        match self {
            Selector::Name(name_selector) => name_selector.eval(node),
            Selector::Wildcard(wildcard_selector) => wildcard_selector.eval(node),
            Selector::Index(index_selector) => index_selector.eval(node),
            Selector::ArraySlice(array_slice_selector) => array_slice_selector.eval(node),
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

impl WildcardSelector {
    pub fn eval(&self, node: &serde_json::Value) -> NodeList {
        if let serde_json::Value::Object(key_values) = node {
            return key_values.values().cloned().collect::<NodeList>();
        } else if let serde_json::Value::Array(values) = node {
            return values.to_vec();
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

impl ArraySliceSelector {
    pub fn eval(&self, node: &serde_json::Value) -> NodeList {
        if let serde_json::Value::Array(values) = node {
            if self.step() == 0 {
                return vec![];
            }
            let len = values.len() as i32;
            let (lower, upper) = self.get_bounds(len);
            let mut results = vec![];
            if self.step() > 0 {
                let mut i = lower;
                while i < upper {
                    results.push(values.get(i as usize).unwrap().clone());
                    i += self.step();
                }
            } else {
                let mut i = upper;
                while lower < i {
                    results.push(values.get(i as usize).unwrap().clone());
                    i += self.step();
                }
            }
            return results;
        }
        vec![]
    }

    pub fn get_start(&self, len: i32) -> i32 {
        if let Some(value) = self.start() {
            value
        } else if self.step() >= 0 {
            0
        } else {
            len - 1
        }
    }

    pub fn get_end(&self, len: i32) -> i32 {
        if let Some(value) = self.end() {
            value
        } else if self.step() >= 0 {
            len
        } else {
            -len - 1
        }
    }

    fn get_bounds(&self, len: i32) -> (i32, i32) {
        let n_start = normalize_index(self.get_start(len), len);
        let n_end = normalize_index(self.get_end(len), len);
        if self.step() > 0 {
            (min(max(n_start, 0), len), min(max(n_end, 0), len))
        } else {
            (min(max(n_end, -1), len - 1), min(max(n_start, -1), len - 1))
        }
    }
}

fn normalize_index(i: i32, len: i32) -> i32 {
    if i >= 0 {
        i
    } else {
        len + i
    }
}

mod tests {
    #[allow(unused_imports)]
    use serde_json::json;

    #[allow(unused_imports)]
    use crate::jsonpath2::{
        ArraySliceSelector, ChildSegment, IndexSelector, JsonPathExpr, NameSelector, Segment,
        Selector, WildcardSelector,
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
    fn test_wildcard_selector() {
        assert_eq!(
            WildcardSelector {}.eval(&json!({
              "o": {"j": 1, "k": 2},
              "a": [5, 3]
            })),
            vec![json!([5, 3]), json!({"j": 1, "k": 2})]
        );
        assert_eq!(
            WildcardSelector {}.eval(&json!({"j": 1, "k": 2})),
            vec![json!(1), json!(2)]
        );
        assert_eq!(
            WildcardSelector {}.eval(&json!([5, 3])),
            vec![json!(5), json!(3)]
        );
    }

    #[test]
    fn test_index_selector() {
        let value = json!(["a", "b"]);
        assert_eq!(IndexSelector::new(1).eval(&value), vec![json!("b")]);
        assert_eq!(IndexSelector::new(-2).eval(&value), vec![json!("a")]);
        assert!(IndexSelector::new(2).eval(&value).is_empty());
    }

    #[test]
    fn test_array_slice_selector() {
        let value = json!(["a", "b", "c", "d", "e", "f", "g"]);

        assert!(ArraySliceSelector::new(Some(1), Some(3), 0)
            .eval(&value)
            .is_empty(),);

        let array_selector = ArraySliceSelector::new(Some(1), Some(3), 1);
        assert_eq!(array_selector.get_start(7), 1);
        assert_eq!(array_selector.get_end(7), 3);
        assert_eq!(array_selector.get_bounds(7), (1, 3));
        assert_eq!(array_selector.eval(&value), vec![json!("b"), json!("c")]);

        let array_selector = ArraySliceSelector::new(Some(5), None, 1);
        assert_eq!(array_selector.get_start(7), 5);
        assert_eq!(array_selector.get_end(7), 7);
        assert_eq!(array_selector.get_bounds(7), (5, 7));
        assert_eq!(array_selector.eval(&value), vec![json!("f"), json!("g")]);

        let array_selector = ArraySliceSelector::new(Some(1), Some(5), 2);
        assert_eq!(array_selector.get_start(7), 1);
        assert_eq!(array_selector.get_end(7), 5);
        assert_eq!(array_selector.get_bounds(7), (1, 5));
        assert_eq!(array_selector.eval(&value), vec![json!("b"), json!("d")]);

        let array_selector = ArraySliceSelector::new(Some(5), Some(1), -2);
        assert_eq!(array_selector.get_start(7), 5);
        assert_eq!(array_selector.get_end(7), 1);
        assert_eq!(array_selector.get_bounds(7), (1, 5));
        assert_eq!(array_selector.eval(&value), vec![json!("f"), json!("d")]);

        let array_selector = ArraySliceSelector::new(None, None, -1);
        assert_eq!(array_selector.get_start(7), 6);
        assert_eq!(array_selector.get_end(7), -8);
        assert_eq!(array_selector.get_bounds(7), (-1, 6));
        assert_eq!(
            array_selector.eval(&value),
            vec![
                json!("g"),
                json!("f"),
                json!("e"),
                json!("d"),
                json!("c"),
                json!("b"),
                json!("a")
            ]
        );
    }
}
