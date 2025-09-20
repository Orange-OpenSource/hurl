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

use super::selector::Selector;

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

    pub fn selectors(&self) -> &[Selector] {
        &self.selectors
    }
}

impl Display for DescendantSegment {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
