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

use super::argument::{NodesTypeArgument, ValueTypeArgument};

/// Functions returning a LogicalType
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalTypeFunction {
    Match(ValueTypeArgument, ValueTypeArgument),
    Search(ValueTypeArgument, ValueTypeArgument),
}

/// Functions returning a ValueType
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueTypeFunction {
    Length(ValueTypeArgument),
    Count(NodesTypeArgument),
    Value(NodesTypeArgument),
}
