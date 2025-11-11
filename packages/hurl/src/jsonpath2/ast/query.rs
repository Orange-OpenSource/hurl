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

use std::fmt::Display;

use super::segment::Segment;

/// Generic Query
/// This query is only used inside the jsonpath module
/// can be either absolute with the root identifier $
/// or relative with the current node identifier @
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Query {
    AbsoluteQuery(AbsoluteQuery),
    RelativeQuery(RelativeQuery),
}

/// Absolute Query
/// This is the standard JsonPath Query starting with the root identifier $
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbsoluteQuery {
    segments: Vec<Segment>,
}

impl AbsoluteQuery {
    pub fn new(segments: Vec<Segment>) -> AbsoluteQuery {
        AbsoluteQuery { segments }
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

impl Display for AbsoluteQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let segments = self
            .segments
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "${segments}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]

/// RelativeQuery
/// This query is used inside a filter selector
pub struct RelativeQuery {
    segments: Vec<Segment>,
}

impl RelativeQuery {
    #[allow(dead_code)]
    pub fn new(segments: Vec<Segment>) -> RelativeQuery {
        RelativeQuery { segments }
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}
