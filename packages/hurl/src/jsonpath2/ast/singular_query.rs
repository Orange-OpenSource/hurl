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

use crate::jsonpath2::ast::selector::{IndexSelector, NameSelector};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SingularQuery {
    Absolute(AbsoluteSingularQuery),
    Relative(RelativeSingularQuery),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbsoluteSingularQuery {
    segments: Vec<SingularQuerySegment>,
}

impl AbsoluteSingularQuery {
    pub fn new(segments: Vec<SingularQuerySegment>) -> AbsoluteSingularQuery {
        AbsoluteSingularQuery { segments }
    }

    pub fn segments(&self) -> &[SingularQuerySegment] {
        &self.segments
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelativeSingularQuery {
    segments: Vec<SingularQuerySegment>,
}

impl RelativeSingularQuery {
    pub fn new(segments: Vec<SingularQuerySegment>) -> RelativeSingularQuery {
        RelativeSingularQuery { segments }
    }

    pub fn segments(&self) -> &[SingularQuerySegment] {
        &self.segments
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SingularQuerySegment {
    Name(NameSelector),
    Index(IndexSelector),
}
