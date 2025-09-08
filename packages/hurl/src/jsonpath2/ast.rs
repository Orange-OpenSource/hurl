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

use std::fmt::{Display, Formatter};

/// JSONPath Query
/// https://www.rfc-editor.org/rfc/rfc9535.html#name-overview-of-jsonpath-expres
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    segments: Vec<Segment>,
}

impl Query {
    pub fn new(segments: Vec<Segment>) -> Query {
        Query { segments }
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let segments = self
            .segments
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "${segments}",)
    }
}

/// JSONPath segment
/// https://www.rfc-editor.org/rfc/rfc9535.html#name-segments-2
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    Child(ChildSegment),
    Descendant(DescendantSegment),
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Segment::Child(child_segment) => write!(f, "{}", child_segment),
            Segment::Descendant(descendant_segment) => write!(f, "{}", descendant_segment),
        }
    }
}

/// Child segment
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildSegment {
    selectors: Vec<Selector>,
}

impl ChildSegment {
    pub fn new(selectors: Vec<Selector>) -> ChildSegment {
        ChildSegment { selectors }
    }

    pub fn selectors(&self) -> &[Selector] {
        &self.selectors
    }
}

impl Display for ChildSegment {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Descendant segment
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DescendantSegment {
    selectors: Vec<Selector>,
}

impl DescendantSegment {
    pub fn new(selectors: Vec<Selector>) -> DescendantSegment {
        DescendantSegment { selectors }
    }
}

impl Display for DescendantSegment {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

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
    value: i32,
}
impl IndexSelector {
    pub fn new(value: i32) -> IndexSelector {
        IndexSelector { value }
    }
    pub fn value(&self) -> &i32 {
        &self.value
    }
}

/// Array slice selector
/// <start>:<end>:<step>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArraySliceSelector {
    start: Option<i32>,
    end: Option<i32>,
    step: i32,
}
impl ArraySliceSelector {
    pub fn new(start: Option<i32>, end: Option<i32>, step: i32) -> ArraySliceSelector {
        ArraySliceSelector { start, end, step }
    }
    pub fn start(&self) -> Option<i32> {
        self.start
    }
    pub fn end(&self) -> Option<i32> {
        self.end
    }
    pub fn step(&self) -> i32 {
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

/// Logical expression
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogicalExpr {
    value: RelQuery,
}
impl LogicalExpr {
    pub fn new(value: RelQuery) -> LogicalExpr {
        LogicalExpr { value }
    }
    pub fn value(&self) -> &RelQuery {
        &self.value
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FilterQuery {
    Rel(RelQuery),
    Jsonpath(Query),
}

#[derive(Clone, Debug, PartialEq, Eq)]

pub struct RelQuery {
    segments: Vec<Segment>,
}

impl RelQuery {
    pub fn new(segments: Vec<Segment>) -> RelQuery {
        RelQuery { segments }
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}
