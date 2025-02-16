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
use std::fs;
use std::path::Path;

use hurl_core::ast::SourceInfo;
use hurl_core::input::Input;
use hurl_core::parser;
use uuid::Uuid;

use crate::runner::{EntryResult, HurlResult, RunnerError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Testcase {
    /// Unique identifier of this testcase.
    pub id: String,
    /// Source file name.
    pub filename: String,
    pub success: bool,
    pub time_in_ms: u128,
    /// The runtime errors, and the source information of the entry throwing this error.
    pub errors: Vec<(RunnerError, SourceInfo)>,
    pub timestamp: i64,
}

impl Testcase {
    /// Creates an HTML testcase.
    pub fn from(hurl_result: &HurlResult, filename: &Input) -> Testcase {
        let id = Uuid::new_v4();
        let errors = hurl_result
            .errors()
            .into_iter()
            .map(|(error, entry_src_info)| (error.clone(), entry_src_info))
            .collect();
        Testcase {
            id: id.to_string(),
            filename: filename.to_string(),
            time_in_ms: hurl_result.duration.as_millis(),
            success: hurl_result.success,
            errors,
            timestamp: hurl_result.timestamp,
        }
    }

    /// Exports a [`Testcase`] to HTML in the directory `dir`.
    ///
    /// It will create three HTML files:
    /// - an HTML view of the Hurl source file (with potential errors and syntax colored),
    /// - an HTML timeline view of the executed entries (with potential errors, waterfall)
    /// - an HTML view of the executed run (headers, cookies, etc...)
    ///
    /// `secrets` strings are redacted from the produced HTML.
    pub fn write_html(
        &self,
        content: &str,
        entries: &[EntryResult],
        dir: &Path,
        secrets: &[&str],
    ) -> Result<(), crate::report::ReportError> {
        // We parse the content as we'll reuse the AST to construct the HTML source file, and
        // the waterfall.
        // TODO: for the moment, we can only have parseable file.
        let hurl_file = parser::parse_hurl_file(content).unwrap();

        // We create the timeline view.
        let output_file = dir.join(self.timeline_filename());
        let html = self.get_timeline_html(&hurl_file, content, entries, secrets);
        fs::write(output_file, html.as_bytes())?;

        // Then create the run view.
        let output_file = dir.join(self.run_filename());
        let html = self.get_run_html(&hurl_file, content, entries, secrets);
        fs::write(output_file, html.as_bytes())?;

        // And create the source view.
        let output_file = dir.join(self.source_filename());
        let html = self.get_source_html(&hurl_file, content, secrets);
        fs::write(output_file, html.as_bytes())?;

        Ok(())
    }

    pub fn source_filename(&self) -> String {
        format!("{}-source.html", self.id)
    }

    pub fn timeline_filename(&self) -> String {
        format!("{}-timeline.html", self.id)
    }

    pub fn run_filename(&self) -> String {
        format!("{}-run.html", self.id)
    }
}
