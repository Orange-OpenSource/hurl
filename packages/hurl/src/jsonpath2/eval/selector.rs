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

use crate::jsonpath2::ast::expr::LogicalExpr;
use crate::jsonpath2::ast::selector::{
    ArraySliceSelector, FilterSelector, IndexSelector, NameSelector, Selector, WildcardSelector,
};
use crate::jsonpath2::eval::NodeList;

impl Selector {
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> NodeList {
        match self {
            Selector::Name(name_selector) => {
                name_selector.eval(current_value).into_iter().collect()
            }
            Selector::Wildcard(wildcard_selector) => wildcard_selector.eval(current_value),
            Selector::Index(index_selector) => {
                index_selector.eval(current_value).into_iter().collect()
            }
            Selector::ArraySlice(array_slice_selector) => array_slice_selector.eval(current_value),
            Selector::Filter(filter_selector) => filter_selector.eval(current_value, root_value),
        }
    }
}

impl NameSelector {
    pub fn eval(&self, current_value: &serde_json::Value) -> Option<serde_json::Value> {
        if let serde_json::Value::Object(key_values) = current_value {
            if let Some(value) = key_values.get(self.value()) {
                return Some(value.clone());
            }
        }
        None
    }
}

impl WildcardSelector {
    pub fn eval(&self, current_value: &serde_json::Value) -> NodeList {
        if let serde_json::Value::Object(key_values) = current_value {
            return key_values.values().cloned().collect::<NodeList>();
        } else if let serde_json::Value::Array(values) = current_value {
            return values.to_vec();
        }
        vec![]
    }
}

impl IndexSelector {
    pub fn eval(&self, current_value: &serde_json::Value) -> Option<serde_json::Value> {
        if let serde_json::Value::Array(values) = current_value {
            let index = if *self.value() < 0 {
                values.len() - ((*self.value()).unsigned_abs() as usize)
            } else {
                *self.value() as usize
            };
            if let Some(value) = values.get(index) {
                return Some(value.clone());
            }
        }
        None
    }
}

impl ArraySliceSelector {
    pub fn eval(&self, current_value: &serde_json::Value) -> NodeList {
        if let serde_json::Value::Array(values) = current_value {
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

impl FilterSelector {
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> NodeList {
        if let serde_json::Value::Object(key_values) = current_value {
            return key_values
                .values()
                .filter(|current_value| filter(current_value, root_value, self.expr()))
                .cloned()
                .collect::<NodeList>();
        } else if let serde_json::Value::Array(values) = current_value {
            return values
                .iter()
                .filter(|current_value| filter(current_value, root_value, self.expr()))
                .cloned()
                .collect::<NodeList>();
        }
        vec![]
    }
}

fn filter(
    current_value: &serde_json::Value,
    root_value: &serde_json::Value,
    logical_expr: &LogicalExpr,
) -> bool {
    logical_expr.eval(current_value, root_value)
}

fn normalize_index(i: i32, len: i32) -> i32 {
    if i >= 0 {
        i
    } else {
        len + i
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use serde_json::json;

    #[allow(unused_imports)]
    use crate::jsonpath2::ast::expr::{LogicalExpr, TestExpr, TestExprKind};
    #[allow(unused_imports)]
    use crate::jsonpath2::ast::query::{AbsoluteQuery, Query, RelativeQuery};
    #[allow(unused_imports)]
    use crate::jsonpath2::ast::segment::{ChildSegment, Segment};
    #[allow(unused_imports)]
    use crate::jsonpath2::ast::selector::{
        ArraySliceSelector, FilterSelector, IndexSelector, NameSelector, Selector, WildcardSelector,
    };

    #[test]
    fn test_selector() {
        let current_value = json!({"greeting": "Hello"});
        let root_value = json!("unused");
        assert_eq!(
            Selector::Name(NameSelector::new("greeting".to_string()))
                .eval(&current_value, &root_value),
            vec![json!("Hello")]
        );
    }

    #[test]
    fn test_name_selector() {
        let current_value = json!({"greeting": "Hello"});
        assert_eq!(
            NameSelector::new("greeting".to_string())
                .eval(&current_value)
                .unwrap(),
            json!("Hello")
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
        let current_value = json!(["a", "b"]);
        assert_eq!(
            IndexSelector::new(1).eval(&current_value).unwrap(),
            json!("b")
        );
        assert_eq!(
            IndexSelector::new(-2).eval(&current_value).unwrap(),
            json!("a")
        );
        assert!(IndexSelector::new(2).eval(&current_value).is_none());
    }

    #[test]
    fn test_array_slice_selector() {
        let current_value = json!(["a", "b", "c", "d", "e", "f", "g"]);

        assert!(ArraySliceSelector::new(Some(1), Some(3), 0)
            .eval(&current_value)
            .is_empty(),);

        let array_selector = ArraySliceSelector::new(Some(1), Some(3), 1);
        assert_eq!(array_selector.get_start(7), 1);
        assert_eq!(array_selector.get_end(7), 3);
        assert_eq!(array_selector.get_bounds(7), (1, 3));
        assert_eq!(
            array_selector.eval(&current_value),
            vec![json!("b"), json!("c")]
        );

        let array_selector = ArraySliceSelector::new(Some(5), None, 1);
        assert_eq!(array_selector.get_start(7), 5);
        assert_eq!(array_selector.get_end(7), 7);
        assert_eq!(array_selector.get_bounds(7), (5, 7));
        assert_eq!(
            array_selector.eval(&current_value),
            vec![json!("f"), json!("g")]
        );

        let array_selector = ArraySliceSelector::new(Some(1), Some(5), 2);
        assert_eq!(array_selector.get_start(7), 1);
        assert_eq!(array_selector.get_end(7), 5);
        assert_eq!(array_selector.get_bounds(7), (1, 5));
        assert_eq!(
            array_selector.eval(&current_value),
            vec![json!("b"), json!("d")]
        );

        let array_selector = ArraySliceSelector::new(Some(5), Some(1), -2);
        assert_eq!(array_selector.get_start(7), 5);
        assert_eq!(array_selector.get_end(7), 1);
        assert_eq!(array_selector.get_bounds(7), (1, 5));
        assert_eq!(
            array_selector.eval(&current_value),
            vec![json!("f"), json!("d")]
        );

        let array_selector = ArraySliceSelector::new(None, None, -1);
        assert_eq!(array_selector.get_start(7), 6);
        assert_eq!(array_selector.get_end(7), -8);
        assert_eq!(array_selector.get_bounds(7), (-1, 6));
        assert_eq!(
            array_selector.eval(&current_value),
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

    #[test]
    fn test_filter_selector() {
        let current_value = json!([3, 5, 1, 2, 4, 6,
         {"b": "j"},
         {"b": "k"},
         {"b": {}},
         {"b": "kilo"}
        ]);

        // @.b
        let filter_selector = FilterSelector::new(LogicalExpr::Test(TestExpr::new(
            false,
            TestExprKind::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                    "b".to_string(),
                ))])),
            ]))),
        )));
        let root_value = json!({});
        assert_eq!(
            filter_selector.eval(&current_value, &root_value),
            vec![
                json!({"b": "j"}),
                json!({"b": "k"}),
                json!({"b": {}}),
                json!({"b": "kilo"}),
            ]
        );
    }
}
