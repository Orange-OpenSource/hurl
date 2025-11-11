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

use crate::jsonpath2::ast::literal::Literal;
use crate::jsonpath2::ast::singular_query::SingularQuery;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct ComparisonExpr {
    left: Comparable,
    right: Comparable,
    operator: ComparisonOp,
}

impl ComparisonExpr {
    pub fn new(left: Comparable, right: Comparable, operator: ComparisonOp) -> ComparisonExpr {
        ComparisonExpr {
            left,
            right,
            operator,
        }
    }
    pub fn left(&self) -> &Comparable {
        &self.left
    }

    pub fn right(&self) -> &Comparable {
        &self.right
    }
    pub fn operator(&self) -> &ComparisonOp {
        &self.operator
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Comparable {
    Literal(Literal),
    SingularQuery(SingularQuery),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    LessOrEqual,
    Less,
    GreaterOrEqual,
    Greater,
}
