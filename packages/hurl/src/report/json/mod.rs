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
//! JSON report.
//!
//! The JSON report is organised as follows:
//!
//! - `report.json`: list of each file of a run exported to JSON
//! - `store/foo_response.{json,xml,html}`: an HTTP response referenced in `report.json`
//!
//! ```text
//! report
//! ├── report.json
//! └── store
//!     ├── 1fe9d647-5689-4130-b4ea-dc120c2536ba_response.html
//!     ├── 35f49c69-15f9-43df-a672-a1ff5f68c935_response.json
//!     ...
//!     └── ce7f1326-2e2a-46e9-befd-ee0d85084814_response.json
//! ```
mod deserialize;

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use hurl_core::input::Input;

use crate::report::ReportError;
use crate::runner::HurlResult;

/// Exports a list of [`Testcase`] to a JSON file `filename`.
///
/// Response file are saved under the `response_dir` directory and referenced by path in JSON report
/// file. `secrets` strings are redacted from the JSON report fields.
pub fn write_report(
    filename: &Path,
    testcases: &[Testcase],
    response_dir: &Path,
    secrets: &[&str],
) -> Result<(), ReportError> {
    // We parse any potential existing report.
    let mut report = deserialize::parse_json_report(filename)?;

    // Serialize the new report, extended any exiting one.
    let json = testcases
        .iter()
        .map(|t| t.to_json(response_dir, secrets))
        .collect::<Result<Vec<_>, _>>()?;
    report.extend(json);

    let serialized = serde_json::to_string(&report)?;
    let bytes = format!("{serialized}\n");
    let bytes = bytes.into_bytes();
    let mut file_out = File::create(filename)?;
    match file_out.write_all(&bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(ReportError::from_error(
            e,
            filename,
            "Issue writing JSON report",
        )),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Testcase<'a> {
    result: &'a HurlResult,
    content: &'a str,
    filename: &'a Input,
}

impl<'a> Testcase<'a> {
    /// Creates a new `Testcase`.
    pub fn new(hurl_result: &'a HurlResult, content: &'a str, filename: &'a Input) -> Self {
        Testcase {
            result: hurl_result,
            content,
            filename,
        }
    }

    /// Serializes this testcase to JSON.
    ///
    /// `secrets` strings are redacted from the JSON fields.
    fn to_json(
        &self,
        response_dir: &Path,
        secrets: &[&str],
    ) -> Result<serde_json::Value, io::Error> {
        self.result
            .to_json(self.content, self.filename, Some(response_dir), secrets)
    }
}
