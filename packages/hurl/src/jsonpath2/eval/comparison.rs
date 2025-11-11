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

use crate::jsonpath2::ast::comparison::{Comparable, ComparisonExpr, ComparisonOp};
use crate::jsonpath2::ast::literal::Literal;

impl ComparisonExpr {
    #[allow(dead_code)]
    pub fn eval(&self, current_value: &serde_json::Value, root_value: &serde_json::Value) -> bool {
        let left = self.left().eval(current_value, root_value);
        let right = self.right().eval(current_value, root_value);
        // !=, <=, >, and >= are defined in terms of the other comparison operators.
        match self.operator() {
            ComparisonOp::Equal => is_equal(&left, &right),
            ComparisonOp::Less => is_less(&left, &right),
            ComparisonOp::NotEqual => !is_equal(&left, &right),
            ComparisonOp::LessOrEqual => is_less(&left, &right) || is_equal(&left, &right),
            ComparisonOp::Greater => is_less(&right, &left),
            ComparisonOp::GreaterOrEqual => is_less(&right, &left) || is_equal(&left, &right),
        }
    }
}
fn is_equal(left: &Option<serde_json::Value>, right: &Option<serde_json::Value>) -> bool {
    left == right
}
fn is_less(left: &Option<serde_json::Value>, right: &Option<serde_json::Value>) -> bool {
    match (left, right) {
        (Some(serde_json::Value::String(left)), Some(serde_json::Value::String(right))) => {
            left < right
        }
        (Some(serde_json::Value::Number(left)), Some(serde_json::Value::Number(right))) => {
            if let (Some(left), Some(right)) = (left.as_f64(), right.as_f64()) {
                left < right
            } else {
                false
            }
        }
        _ => false,
    }
}

impl Comparable {
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> Option<serde_json::Value> {
        match self {
            Comparable::Literal(literal) => Some(literal.eval()),
            Comparable::SingularQuery(singular_query) => {
                singular_query.eval(current_value, root_value)
            }
        }
    }
}

impl Literal {
    pub fn eval(&self) -> serde_json::Value {
        match self {
            Literal::String(s) => serde_json::Value::String(s.clone()),
            Literal::Number(n) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap())
            }
            Literal::Bool(b) => serde_json::Value::Bool(*b),
            Literal::Null => serde_json::Value::Null,
            Literal::Integer(n) => serde_json::Number::from_i128(*n as i128).unwrap().into(),
        }
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::*;
    use crate::jsonpath2::ast::singular_query::SingularQuerySegment;
    use crate::jsonpath2::ast::{
        selector::NameSelector,
        singular_query::{AbsoluteSingularQuery, SingularQuery},
    };

    fn name_query(name: &str) -> SingularQuery {
        SingularQuery::Absolute(AbsoluteSingularQuery::new(vec![
            SingularQuerySegment::Name(NameSelector::new(name.to_string())),
        ]))
    }

    #[test]
    pub fn test_comparison() {
        let current_value = &serde_json::json!({});
        let root_value = json!({
          "obj": {"x": "y"},
          "arr": [2, 3]
        });
        // Empty nodelists
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("absent1")),
            Comparable::SingularQuery(name_query("absent2")),
            ComparisonOp::Equal
        )
        .eval(current_value, &root_value));

        // == implies <=
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("absent1")),
            Comparable::SingularQuery(name_query("absent2")),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // Empty nodelist
        assert!(!ComparisonExpr::new(
            Comparable::SingularQuery(name_query("absent")),
            Comparable::Literal(Literal::String("g".to_string())),
            ComparisonOp::Equal
        )
        .eval(current_value, &root_value));

        // Empty nodelists
        assert!(!ComparisonExpr::new(
            Comparable::SingularQuery(name_query("absent1")),
            Comparable::SingularQuery(name_query("absent2")),
            ComparisonOp::NotEqual
        )
        .eval(current_value, &root_value));

        // Empty nodelist
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("absent")),
            Comparable::Literal(Literal::String("g".to_string())),
            ComparisonOp::NotEqual
        )
        .eval(current_value, &root_value));

        // Numeric comparison
        assert!(ComparisonExpr::new(
            Comparable::Literal(Literal::Integer(1)),
            Comparable::Literal(Literal::Integer(2)),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // Numeric comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Integer(1)),
            Comparable::Literal(Literal::Integer(2)),
            ComparisonOp::Greater
        )
        .eval(current_value, &root_value));

        // Type mismatch
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Integer(13)),
            Comparable::Literal(Literal::String("13".to_string())),
            ComparisonOp::Equal
        )
        .eval(current_value, &root_value));

        // String comparison
        assert!(ComparisonExpr::new(
            Comparable::Literal(Literal::String("a".to_string())),
            Comparable::Literal(Literal::String("b".to_string())),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // String comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::String("a".to_string())),
            Comparable::Literal(Literal::String("b".to_string())),
            ComparisonOp::Greater
        )
        .eval(current_value, &root_value));

        // Type mismatch
        assert!(!ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::Equal
        )
        .eval(current_value, &root_value));

        // Type mismatch
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::NotEqual
        )
        .eval(current_value, &root_value));

        // Object comparison
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::SingularQuery(name_query("obj")),
            ComparisonOp::Equal
        )
        .eval(current_value, &root_value));

        // Object comparison
        assert!(!ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::SingularQuery(name_query("obj")),
            ComparisonOp::NotEqual
        )
        .eval(current_value, &root_value));

        // Array comparison
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("arr")),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::Equal
        )
        .eval(current_value, &root_value));

        // Array comparison
        assert!(!ComparisonExpr::new(
            Comparable::SingularQuery(name_query("arr")),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::NotEqual
        )
        .eval(current_value, &root_value));

        // Type mismatch
        assert!(!ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::Literal(Literal::Integer(17)),
            ComparisonOp::Equal
        )
        .eval(current_value, &root_value));

        // Type mismatch
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::Literal(Literal::Integer(17)),
            ComparisonOp::NotEqual
        )
        .eval(current_value, &root_value));

        // Objects and arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // Objects and arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::Less
        )
        .eval(current_value, &root_value));

        // == implies <=
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::SingularQuery(name_query("obj")),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // == implies <=
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("arr")),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // Arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Integer(1)),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // Arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Integer(1)),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::GreaterOrEqual
        )
        .eval(current_value, &root_value));

        // Arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Integer(1)),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::Greater
        )
        .eval(current_value, &root_value));

        // Arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Integer(1)),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::Less
        )
        .eval(current_value, &root_value));

        // == implies <=
        assert!(ComparisonExpr::new(
            Comparable::Literal(Literal::Bool(true)),
            Comparable::Literal(Literal::Bool(true)),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // Booleans do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Bool(true)),
            Comparable::Literal(Literal::Bool(true)),
            ComparisonOp::Greater
        )
        .eval(current_value, &root_value));
    }
}
