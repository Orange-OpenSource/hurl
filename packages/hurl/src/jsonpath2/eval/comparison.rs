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

/// Check equality between 2 serde_json::Value
/// Use numeric equality for numbers
/// 110 must be equal to 110.0
fn is_equal(left: &Option<serde_json::Value>, right: &Option<serde_json::Value>) -> bool {
    if left.is_none() && right.is_none() {
        true
    } else if let (Some(left), Some(right)) = (left, right) {
        match (left, right) {
            (serde_json::Value::Number(left_num), serde_json::Value::Number(right_num)) => {
                f64_equals(left_num.as_f64().unwrap(), right_num.as_f64().unwrap())
            }
            _ => left == right,
        }
    } else {
        false
    }
}

fn f64_equals(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-12
}

fn is_less(left: &Option<serde_json::Value>, right: &Option<serde_json::Value>) -> bool {
    match (left, right) {
        (Some(serde_json::Value::String(left)), Some(serde_json::Value::String(right))) => {
            left < right
        }
        (Some(serde_json::Value::Number(left)), Some(serde_json::Value::Number(right))) => {
            if let (Some(left), Some(right)) = (left.as_f64(), right.as_f64()) {
                f64_less(left, right)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn f64_less(a: f64, b: f64) -> bool {
    (b - a) > 1e-12
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
            Comparable::Function(value_type_function) => {
                value_type_function.eval(current_value, root_value)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::*;
    use crate::jsonpath2::ast::literal::{Literal, Number};
    use crate::jsonpath2::ast::singular_query::SingularQuerySegment;
    use crate::jsonpath2::ast::{
        selector::NameSelector,
        singular_query::{AbsoluteSingularQuery, RelativeSingularQuery, SingularQuery},
    };

    fn name_query(name: &str) -> SingularQuery {
        SingularQuery::Absolute(AbsoluteSingularQuery::new(vec![
            SingularQuerySegment::Name(NameSelector::new(name.to_string())),
        ]))
    }

    #[test]
    pub fn test_is_equal() {
        assert!(is_equal(&Some(json!(110)), &Some(json!(110.0))));
        assert!(is_equal(
            &Some(json!(110.0)),
            &Some(json!(110.00000000000001))
        ));
    }

    #[test]
    pub fn test_is_less() {
        assert!(!is_less(&Some(json!(110)), &Some(json!(110.0))));
        assert!(!is_less(
            &Some(json!(110.0)),
            &Some(json!(110.00000000000001))
        ));
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
            Comparable::Literal(Literal::Number(Number::Integer(1))),
            Comparable::Literal(Literal::Number(Number::Integer(2))),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // Numeric comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Number(Number::Integer(1))),
            Comparable::Literal(Literal::Number(Number::Integer(2))),
            ComparisonOp::Greater
        )
        .eval(current_value, &root_value));

        // Type mismatch
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Number(Number::Integer(13))),
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
            Comparable::Literal(Literal::Number(Number::Integer(17))),
            ComparisonOp::Equal
        )
        .eval(current_value, &root_value));

        // Type mismatch
        assert!(ComparisonExpr::new(
            Comparable::SingularQuery(name_query("obj")),
            Comparable::Literal(Literal::Number(Number::Integer(17))),
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
            Comparable::Literal(Literal::Number(Number::Integer(1))),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::LessOrEqual
        )
        .eval(current_value, &root_value));

        // Arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Number(Number::Integer(1))),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::GreaterOrEqual
        )
        .eval(current_value, &root_value));

        // Arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Number(Number::Integer(1))),
            Comparable::SingularQuery(name_query("arr")),
            ComparisonOp::Greater
        )
        .eval(current_value, &root_value));

        // Arrays do not offer < comparison
        assert!(!ComparisonExpr::new(
            Comparable::Literal(Literal::Number(Number::Integer(1))),
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

    #[test]
    pub fn test_number() {
        // @.a==110
        let comparison = ComparisonExpr::new(
            Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(vec![
                SingularQuerySegment::Name(NameSelector::new("a".to_string())),
            ]))),
            Comparable::Literal(Literal::Number(Number::Integer(110))),
            ComparisonOp::Equal,
        );
        assert!(comparison.eval(&serde_json::json!({"a":110}), &serde_json::json!({})));
        assert!(comparison.eval(&serde_json::json!({"a":110.0}), &serde_json::json!({})));
        assert!(comparison.eval(&serde_json::json!({"a":1.1e2}), &serde_json::json!({})));

        // @.a==110.0
        let comparison = ComparisonExpr::new(
            Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(vec![
                SingularQuerySegment::Name(NameSelector::new("a".to_string())),
            ]))),
            Comparable::Literal(Literal::Number(Number::Float(110.0))),
            ComparisonOp::Equal,
        );
        assert!(comparison.eval(&serde_json::json!({"a":110}), &serde_json::json!({})));
        assert!(comparison.eval(&serde_json::json!({"a":110.0}), &serde_json::json!({})));
        assert!(comparison.eval(&serde_json::json!({"a":1.1e2}), &serde_json::json!({})));
    }
}
