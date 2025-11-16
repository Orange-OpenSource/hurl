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

use super::comparison::ComparisonExpr;
use super::query::Query;

/// Logical expression
/// evaluates to a boolean value
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalExpr {
    /// Comparison expression (e.g., @.price > 10)
    #[allow(dead_code)]
    Comparison(ComparisonExpr),

    /// Test expression (e.g., @.name)
    #[allow(dead_code)]
    Test(TestExpr),

    /// Logical AND expression (e.g., expr1 && expr2 && expr3)
    #[allow(dead_code)]
    And(AndExpr),

    /// Logical OR expression (e.g., expr1 || expr2 || expr3)
    #[allow(dead_code)]
    Or(OrExpr),

    /// Logical NOT expression (e.g., !expr)
    #[allow(dead_code)]
    Not(NotExpr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestExpr {
    not: bool,
    kind: TestExprKind,
}
impl TestExpr {
    #[allow(dead_code)]
    pub fn new(not: bool, kind: TestExprKind) -> Self {
        Self { not, kind }
    }

    #[allow(dead_code)]
    pub fn not(&self) -> bool {
        self.not
    }
    #[allow(dead_code)]
    pub fn kind(&self) -> &TestExprKind {
        &self.kind
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TestExprKind {
    #[allow(dead_code)]
    FilterQuery(Query),
    #[allow(dead_code)]
    FunctionExpr(FunctionExpr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionExpr;

/// Logical AND expression that can handle multiple operands
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AndExpr {
    operands: Vec<LogicalExpr>,
}

impl AndExpr {
    #[allow(dead_code)]
    pub fn new(operands: Vec<LogicalExpr>) -> Self {
        Self { operands }
    }

    #[allow(dead_code)]
    pub fn operands(&self) -> &Vec<LogicalExpr> {
        &self.operands
    }
}
/// Logical OR expression that can handle multiple operands
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrExpr {
    operands: Vec<LogicalExpr>,
}

impl OrExpr {
    #[allow(dead_code)]
    pub fn new(operands: Vec<LogicalExpr>) -> Self {
        Self { operands }
    }

    #[allow(dead_code)]
    pub fn operands(&self) -> &Vec<LogicalExpr> {
        &self.operands
    }
}

/// Logical AND expression
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotExpr {
    expr: Box<LogicalExpr>,
}

impl NotExpr {
    #[allow(dead_code)]
    pub fn new(expr: LogicalExpr) -> Self {
        Self {
            expr: Box::new(expr),
        }
    }

    #[allow(dead_code)]
    pub fn expr(&self) -> &LogicalExpr {
        &self.expr
    }
}
