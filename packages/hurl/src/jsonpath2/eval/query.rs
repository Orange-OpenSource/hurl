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

use crate::jsonpath2::ast::query::{AbsoluteQuery, Query, RelativeQuery};
use crate::jsonpath2::eval::NodeList;

impl Query {
    /// Eval a `Query`
    /// It returns a `NodeList`
    ///
    /// Note that the absolute and relative queries are not symmetrical
    /// An absolute query only need the root value
    /// while the relative query needs both the current value and the root value
    #[allow(dead_code)]
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> NodeList {
        match self {
            Query::AbsoluteQuery(absolute_query) => absolute_query.eval(root_value),
            Query::RelativeQuery(relative_query) => relative_query.eval(current_value, root_value),
        }
    }
}

impl RelativeQuery {
    /// Eval a `RelativeQuery` for the current `serde_json::Value` input.
    /// It returns a `NodeList`{
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> NodeList {
        let mut results = vec![current_value.clone()];
        for segment in self.segments() {
            results = results
                .iter()
                .flat_map(|current_value| segment.eval(current_value, root_value))
                .collect();
        }
        results
    }
}

impl AbsoluteQuery {
    /// Eval a JSONPath `Query` for a root `serde_json::Value` input.
    /// It returns a `NodeList`
    #[allow(dead_code)]
    pub fn eval(&self, root_value: &serde_json::Value) -> NodeList {
        let mut results = vec![root_value.clone()];
        for segment in self.segments() {
            results = results
                .iter()
                .flat_map(|current_value| segment.eval(current_value, root_value))
                .collect();
        }
        results
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::json;
    #[allow(unused_imports)]
    use crate::jsonpath2::ast::query::AbsoluteQuery;
    #[allow(unused_imports)]
    use serde_json::json;

    #[test]
    fn test_root_identifier() {
        let root_value = json!({"greeting": "Hello"});
        let root_identifier = AbsoluteQuery::new(vec![]);
        assert_eq!(root_identifier.eval(&root_value), vec![root_value]);
    }
}
