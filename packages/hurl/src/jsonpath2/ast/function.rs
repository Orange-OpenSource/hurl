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

use crate::jsonpath2::ast::expr::LogicalExpr;
use crate::jsonpath2::ast::literal::Literal;
use crate::jsonpath2::ast::query::Query;

/// Function extensions:
/// https://www.rfc-editor.org/rfc/rfc9535.html#name-function-extensions
/// This module implements the functions defined in the RFC.
/// It defines the function types and their arguments.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]

pub enum LogicalTypeFunction {
    Match(ValueTypeArgument, ValueTypeArgument),
    Search(ValueTypeArgument, ValueTypeArgument),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]

pub enum ValueTypeFunction {
    Length(ValueTypeArgument),
    Count(NodesTypeArgument),
    Value(NodesTypeArgument),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ValueTypeArgument {
    Literal(Literal),
    FilterQuery(Query),
    Function(Box<ValueTypeFunction>),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalTypeArgument {
    LogicalExpr(LogicalExpr),
    Function(Box<LogicalTypeFunction>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NodesTypeArgument {
    FilterQuery(Query),
}
