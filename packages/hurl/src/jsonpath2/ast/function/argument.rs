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

use super::functions::LogicalTypeFunction;
use super::functions::ValueTypeFunction;
use crate::jsonpath2::ast::expr::LogicalExpr;
use crate::jsonpath2::ast::literal::Literal;
use crate::jsonpath2::ast::query::Query;
use crate::jsonpath2::ast::singular_query::SingularQuery;

/// Arguments with ValueType
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ValueTypeArgument {
    Literal(Literal),
    SingularQuery(SingularQuery),
    Function(Box<ValueTypeFunction>),
}

/// Arguments with LogicalType
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalTypeArgument {
    LogicalExpr(LogicalExpr),
    LogicalTypeFunction(Box<LogicalTypeFunction>),
}

/// Arguments with NodesType
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodesTypeArgument {
    FilterQuery(Query),
}
