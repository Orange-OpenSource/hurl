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
use crate::jsonpath2::Query;

mod segment;
mod selector;

#[allow(dead_code)]
pub type NodeList = Vec<serde_json::Value>;

impl Query {
    /// Eval a JSONPath `Query` for a `serde_json::Value` input.
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

mod tests {
    #[allow(unused_imports)]
    use crate::{json, jsonpath2::Query};
    #[allow(unused_imports)]
    use serde_json::json;

    #[test]
    fn test_root_identifier() {
        let value = json!({"greeting": "Hello"});
        let root_identifier = Query::new(vec![]);
        assert_eq!(root_identifier.eval(&value), vec![value]);
    }
}
