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

use super::functions::LogicalTypeFunction;
use super::functions::ValueTypeFunction;
use crate::jsonpath::ast::expr::LogicalExpr;
use crate::jsonpath::ast::literal::Literal;
use crate::jsonpath::ast::query::Query;
use crate::jsonpath::ast::singular_query::SingularQuery;
use regex::Regex;

/// Arguments with ValueType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueTypeArgument {
    Literal(Literal),
    SingularQuery(SingularQuery),
    Function(Box<ValueTypeFunction>),
}

/// Arguments with ValueType that should be a regex
#[derive(Clone, Debug)]
pub enum RegexValueTypeArgument {
    Literal(Regex),
    SingularQuery(SingularQuery),
    Function(Box<ValueTypeFunction>),
}

impl PartialEq for RegexValueTypeArgument {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0.to_string() == r0.to_string(),
            (Self::SingularQuery(l0), Self::SingularQuery(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl Eq for RegexValueTypeArgument {}

/// Arguments with LogicalType
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalTypeArgument {
    LogicalExpr(LogicalExpr),
    LogicalTypeFunction(Box<LogicalTypeFunction>),
}

/// Arguments with NodesType
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodesTypeArgument {
    FilterQuery(Query),
}
