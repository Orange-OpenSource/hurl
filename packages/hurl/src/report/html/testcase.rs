/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use uuid::Uuid;

use crate::runner::{Error, HurlResult};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Testcase {
    pub id: String,
    pub filename: String,
    pub success: bool,
    pub time_in_ms: u128,
    pub errors: Vec<Error>,
}

impl Testcase {
    /// Creates an HTML testcase.
    pub fn from(hurl_result: &HurlResult, filename: &str) -> Testcase {
        let id = Uuid::new_v4();
        Testcase {
            id: id.to_string(),
            filename: filename.to_string(),
            time_in_ms: hurl_result.time_in_ms,
            success: hurl_result.success,
            errors: hurl_result
                .entries
                .iter()
                .flat_map(|e| e.errors.clone())
                .collect(),
        }
    }
}
