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

use crate::jsonpath2::ast::expr::{AndExpr, LogicalExpr, NotExpr, OrExpr, TestExpr, TestExprKind};

impl LogicalExpr {
    #[allow(dead_code)]
    pub fn eval(&self, current_value: &serde_json::Value, root_value: &serde_json::Value) -> bool {
        match self {
            LogicalExpr::Comparison(comparison_expr) => {
                comparison_expr.eval(current_value, root_value)
            }
            LogicalExpr::Test(test_expr) => test_expr.eval(current_value, root_value),
            LogicalExpr::And(and_expr) => and_expr.eval(current_value, root_value),
            LogicalExpr::Or(or_expr) => or_expr.eval(current_value, root_value),
            LogicalExpr::Not(not_expr) => not_expr.eval(current_value, root_value),
        }
    }
}

impl TestExpr {
    /// eval the test expression to a boolean value
    #[allow(dead_code)]
    pub fn eval(&self, current_value: &serde_json::Value, root_value: &serde_json::Value) -> bool {
        let value = match self.kind() {
            TestExprKind::FilterQuery(filter_query) => {
                !filter_query.eval(current_value, root_value).is_empty()
            }
            TestExprKind::FunctionExpr(_function_expr) => todo!(),
        };
        if self.not() {
            !value
        } else {
            value
        }
    }
}

impl AndExpr {
    /// eval and end expression to a boolean value
    #[allow(dead_code)]
    pub fn eval(&self, current_value: &serde_json::Value, root_value: &serde_json::Value) -> bool {
        for operand in self.operands() {
            if !operand.eval(current_value, root_value) {
                return false;
            }
        }
        true
    }
}

impl OrExpr {
    /// eval or expression to a boolean value
    #[allow(dead_code)]
    pub fn eval(&self, current_value: &serde_json::Value, root_value: &serde_json::Value) -> bool {
        for operand in self.operands() {
            if operand.eval(current_value, root_value) {
                return true;
            }
        }
        false
    }
}

impl NotExpr {
    /// eval not expression to a boolean value
    #[allow(dead_code)]
    pub fn eval(&self, current_value: &serde_json::Value, root_value: &serde_json::Value) -> bool {
        !self.expr().eval(current_value, root_value)
    }
}

#[cfg(test)]
mod tests {
    use crate::jsonpath2::ast::comparison::{Comparable, ComparisonExpr, ComparisonOp};
    use crate::jsonpath2::ast::expr::{AndExpr, LogicalExpr, OrExpr, TestExpr, TestExprKind};
    use crate::jsonpath2::ast::literal::Literal;
    use crate::jsonpath2::ast::query::{AbsoluteQuery, Query, RelativeQuery};
    use crate::jsonpath2::ast::segment::{ChildSegment, Segment};
    use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
    use crate::jsonpath2::ast::singular_query::{RelativeSingularQuery, SingularQuery};
    use serde_json::json;

    #[test]
    fn test_eval_test_expr() {
        // @.b
        let test_expr = TestExpr::new(
            false,
            TestExprKind::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                    "b".to_string(),
                ))])),
            ]))),
        );
        assert!(test_expr.eval(&serde_json::json!({"b": "j"}), &serde_json::json!({})));
        assert!(!test_expr.eval(&serde_json::json!(3), &serde_json::json!({})));

        // $.*.name
        let test_expr = TestExpr::new(
            false,
            TestExprKind::FilterQuery(Query::AbsoluteQuery(AbsoluteQuery::new(vec![
                Segment::Child(ChildSegment::new(vec![Selector::Wildcard(
                    WildcardSelector,
                )])),
                Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                    "name".to_string(),
                ))])),
            ]))),
        );
        assert!(test_expr.eval(
            &serde_json::json!({"name": "bob"}),
            &serde_json::json!([1, {"name": "bob"}])
        ));
        assert!(!test_expr.eval(
            &serde_json::json!({"name": "bob"}),
            &serde_json::json!([1, 2])
        ));
    }

    #[test]
    fn test_eval_or_expr() {
        // @<2 || @>4
        let or_expr = LogicalExpr::Or(OrExpr::new(vec![
            LogicalExpr::Comparison(ComparisonExpr::new(
                Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                    vec![],
                ))),
                Comparable::Literal(Literal::Integer(2)),
                ComparisonOp::Less,
            )),
            LogicalExpr::Comparison(ComparisonExpr::new(
                Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                    vec![],
                ))),
                Comparable::Literal(Literal::Integer(4)),
                ComparisonOp::Greater,
            )),
        ]));
        assert!(or_expr.eval(&json!(1), &json!([1, 2, 3, 4, 5, 6])));
        assert!(!or_expr.eval(&json!(3), &json!([1, 2, 3, 4, 5, 6])));
    }

    #[test]
    fn test_eval_and_expr() {
        // @>1 && @<4
        let and_expr = LogicalExpr::And(AndExpr::new(vec![
            LogicalExpr::Comparison(ComparisonExpr::new(
                Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                    vec![],
                ))),
                Comparable::Literal(Literal::Integer(1)),
                ComparisonOp::Greater,
            )),
            LogicalExpr::Comparison(ComparisonExpr::new(
                Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                    vec![],
                ))),
                Comparable::Literal(Literal::Integer(4)),
                ComparisonOp::Less,
            )),
        ]));
        assert!(!and_expr.eval(&json!(1), &json!([1, 2, 3, 4, 5, 6])));
        assert!(and_expr.eval(&json!(2), &json!([1, 2, 3, 4, 5, 6])));
    }
}
