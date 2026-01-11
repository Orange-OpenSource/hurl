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

use super::expr::LogicalExpr;

/// Selector
/// https://www.rfc-editor.org/rfc/rfc9535.html#name-selectors-2
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector {
    Name(NameSelector),
    Wildcard(WildcardSelector),
    Index(IndexSelector),
    ArraySlice(ArraySliceSelector),
    Filter(FilterSelector),
}

/// Name selector
/// selects at most one object member value
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameSelector {
    value: String,
}

impl NameSelector {
    pub fn new(value: String) -> NameSelector {
        NameSelector { value }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Wildcard selector
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WildcardSelector;

/// Index selector
/// matches at most one array element value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexSelector {
    value: i64,
}
impl IndexSelector {
    pub fn new(value: i64) -> IndexSelector {
        IndexSelector { value }
    }
    pub fn value(&self) -> &i64 {
        &self.value
    }
}

/// Array slice selector
/// <start>:<end>:<step>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArraySliceSelector {
    start: Option<i64>,
    end: Option<i64>,
    step: i64,
}
impl ArraySliceSelector {
    pub fn new(start: Option<i64>, end: Option<i64>, step: i64) -> ArraySliceSelector {
        ArraySliceSelector { start, end, step }
    }
    pub fn start(&self) -> Option<i64> {
        self.start
    }
    pub fn end(&self) -> Option<i64> {
        self.end
    }
    pub fn step(&self) -> i64 {
        self.step
    }
}

/// Filter selector
/// used to iterate over the elements or members of structured values,
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterSelector {
    expr: LogicalExpr,
}

impl FilterSelector {
    pub fn new(expr: LogicalExpr) -> FilterSelector {
        FilterSelector { expr }
    }

    pub fn expr(&self) -> &LogicalExpr {
        &self.expr
    }
}
