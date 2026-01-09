/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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

use crate::jsonpath2::ast::function::argument::{NodesTypeArgument, ValueTypeArgument};
use crate::jsonpath2::ast::function::{NodesType, ValueType};

impl ValueTypeArgument {
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> ValueType {
        match self {
            ValueTypeArgument::SingularQuery(filter_query) => {
                filter_query.eval(current_value, root_value)
            }
            ValueTypeArgument::Function(value_type_function) => {
                value_type_function.eval(current_value, root_value)
            }
            ValueTypeArgument::Literal(value) => Some(value.eval()),
        }
    }
}

impl NodesTypeArgument {
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> NodesType {
        match self {
            NodesTypeArgument::FilterQuery(query) => query.eval(current_value, root_value),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::jsonpath2::ast::literal::{Literal, Number};
    use crate::jsonpath2::ast::query::{Query, RelativeQuery};
    use crate::jsonpath2::ast::segment::{ChildSegment, Segment};
    use crate::jsonpath2::ast::selector::{NameSelector, Selector};
    use crate::jsonpath2::ast::singular_query::{
        RelativeSingularQuery, SingularQuery, SingularQuerySegment,
    };
    use crate::jsonpath2::eval::function::argument::{NodesTypeArgument, ValueTypeArgument};
    use serde_json::json;

    #[test]
    fn test_value_type_argument() {
        // "hello"
        let argument = ValueTypeArgument::Literal(Literal::String("hello".to_string()));
        assert_eq!(argument.eval(&json!({}), &json!({})), Some(json!("hello")));

        // 1
        let argument = ValueTypeArgument::Literal(Literal::Number(Number::Integer(1)));
        assert_eq!(argument.eval(&json!({}), &json!({})), Some(json!(1)));

        // @.authors
        let argument = ValueTypeArgument::SingularQuery(SingularQuery::Relative(
            RelativeSingularQuery::new(vec![SingularQuerySegment::Name(NameSelector::new(
                "authors".to_string(),
            ))]),
        ));
        assert_eq!(
            argument.eval(&json!({"authors": ["a", "b", "c"]}), &json!({})),
            Some(json!(["a", "b", "c"]))
        );
    }

    #[test]
    fn test_nodes_type_argument() {
        // @.books
        let argument =
            NodesTypeArgument::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                    "books".to_string(),
                ))])),
            ])));
        assert_eq!(
            argument.eval(&json!({"books": ["book1", "book2"]}), &json!({})),
            vec![json!(["book1", "book2"])]
        );
        assert!(argument.eval(&json!({}), &json!({})).is_empty());
    }
}
