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

use crate::jsonpath2::ast::expr::{LogicalExpr, TestExpr, TestExprKind};

impl LogicalExpr {
    #[allow(dead_code)]
    pub fn eval(&self, current_value: &serde_json::Value, root_value: &serde_json::Value) -> bool {
        match self {
            LogicalExpr::Comparison(_comparison_expr) => todo!(),
            LogicalExpr::Test(test_expr) => test_expr.eval(current_value, root_value),
            LogicalExpr::And(_and_expr) => todo!(),
            LogicalExpr::Or(_or_expr) => todo!(),
        }
    }
}

impl TestExpr {
    #[allow(dead_code)]
    pub fn eval(&self, current_value: &serde_json::Value, root_value: &serde_json::Value) -> bool {
        match self.kind() {
            TestExprKind::FilterQuery(filter_query) => {
                !filter_query.eval(current_value, root_value).is_empty()
            }
            TestExprKind::FunctionExpr(_function_expr) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::jsonpath2::ast::expr::{TestExpr, TestExprKind};
    use crate::jsonpath2::ast::selector::{NameSelector, Selector};

    #[test]
    fn test_eval_test_expr() {
        // @.b
        let test_expr = TestExpr::new(
            false,
            TestExprKind::FilterQuery(crate::jsonpath2::ast::query::Query::RelativeQuery(
                crate::jsonpath2::ast::query::RelativeQuery::new(vec![
                    crate::jsonpath2::ast::segment::Segment::Child(
                        crate::jsonpath2::ast::segment::ChildSegment::new(vec![Selector::Name(
                            NameSelector::new("b".to_string()),
                        )]),
                    ),
                ]),
            )),
        );
        assert!(test_expr.eval(&serde_json::json!({"b": "j"}), &serde_json::json!({})));
        assert!(!test_expr.eval(&serde_json::json!(3), &serde_json::json!({})));
    }
}
