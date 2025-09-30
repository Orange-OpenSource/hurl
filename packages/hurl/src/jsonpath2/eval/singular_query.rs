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

use crate::jsonpath2::ast::singular_query::{
    AbsoluteSingularQuery, RelativeSingularQuery, SingularQuery, SingularQuerySegment,
};

impl SingularQuery {
    #[allow(dead_code)]
    pub fn eval(
        &self,
        current_value: &serde_json::Value,
        root_value: &serde_json::Value,
    ) -> Option<serde_json::Value> {
        match self {
            SingularQuery::Absolute(absolute_singular_query) => {
                absolute_singular_query.eval(root_value)
            }
            SingularQuery::Relative(relative_singular_query) => {
                relative_singular_query.eval(current_value)
            }
        }
    }
}

impl AbsoluteSingularQuery {
    pub fn eval(&self, value: &serde_json::Value) -> Option<serde_json::Value> {
        let mut result = value.clone();
        for segment in self.segments() {
            if let Some(value) = segment.eval(&result) {
                result = value;
            } else {
                return None;
            }
        }
        Some(result.clone())
    }
}

impl RelativeSingularQuery {
    pub fn eval(&self, value: &serde_json::Value) -> Option<serde_json::Value> {
        let mut result = value.clone();
        for segment in self.segments() {
            if let Some(value) = segment.eval(&result) {
                result = value;
            } else {
                return None;
            }
        }
        Some(result.clone())
    }
}

impl SingularQuerySegment {
    pub fn eval(&self, value: &serde_json::Value) -> Option<serde_json::Value> {
        match self {
            SingularQuerySegment::Name(name_selector) => name_selector.eval(value),
            SingularQuerySegment::Index(index_selector) => index_selector.eval(value),
        }
    }
}
#[cfg(test)]
mod tests {}
