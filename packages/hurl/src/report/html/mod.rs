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
//! HTML report.
mod nav;
mod report;
mod run;
mod source;
mod testcase;
mod timeline;

pub use report::write_report;
pub use testcase::Testcase;

/// The test result to be displayed in an HTML page
#[derive(Clone, Debug, PartialEq, Eq)]
struct HTMLResult {
    /// Original filename, as given in the run execution
    filename: String,
    /// The id of the corresponding [`Testcase`]
    id: String,
    time_in_ms: u128,
    success: bool,
    timestamp: i64,
}

impl HTMLResult {
    /// Creates a new HTMLResult from a [`Testcase`].
    fn from(testcase: &Testcase) -> Self {
        HTMLResult {
            filename: testcase.filename.clone(),
            id: testcase.id.clone(),
            time_in_ms: testcase.time_in_ms,
            success: testcase.success,
            timestamp: testcase.timestamp,
        }
    }
}
