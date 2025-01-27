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
use hurl_core::ast::HurlFile;

use crate::http::Call;
use crate::report::html::nav::Tab;
use crate::report::html::timeline::unit::Pixel;
use crate::report::html::Testcase;
use crate::runner::EntryResult;

mod calls;
mod nice;
mod svg;
mod unit;
mod util;
mod waterfall;

/// Some common constants used to construct our SVG timeline.
const CALL_HEIGHT: Pixel = Pixel(24.0);
const CALL_INSET: Pixel = Pixel(3.0);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CallContextKind {
    Success, // call context parent entry is successful
    Failure, // call context parent entry is in error and has not been retried
    Retry,   // call context parent entry is in error and has been retried
}
/// A structure that holds information to construct a SVG view
/// of a [`Call`]
pub struct CallContext {
    pub kind: CallContextKind, // If the parent entry is successful, retried or in error.
    pub line: usize,           // Line number of the source entry (1-based)
    pub entry_index: usize,    // Index of the runtime EntryResult
    pub call_entry_index: usize, // Index of the runtime Call in the current entry
    pub call_index: usize,     // Index of the runtime Call in the whole run
    pub source_filename: String,
    pub run_filename: String,
}

impl Testcase {
    /// Returns the HTML timeline of these `entries`.
    /// The AST `hurl_file` is used to construct URL with line numbers to the corresponding
    /// entry in the colored HTML source file.
    pub fn get_timeline_html(
        &self,
        hurl_file: &HurlFile,
        content: &str,
        entries: &[EntryResult],
        secrets: &[&str],
    ) -> String {
        let calls = entries
            .iter()
            .flat_map(|e| &e.calls)
            .collect::<Vec<&Call>>();

        let call_ctxs = self.get_call_contexts(hurl_file, entries);
        let timeline_css = include_str!("../resources/timeline.css");
        let nav = self.get_nav_html(content, Tab::Timeline, secrets);
        let nav_css = include_str!("../resources/nav.css");
        let calls_svg = self.get_calls_svg(&calls, &call_ctxs, secrets);
        let waterfall_svg = self.get_waterfall_svg(&calls, &call_ctxs, secrets);
        format!(
            include_str!("../resources/timeline.html"),
            calls = calls_svg,
            filename = self.filename,
            nav = nav,
            nav_css = nav_css,
            timeline_css = timeline_css,
            waterfall = waterfall_svg,
        )
    }

    /// Constructs a list of call contexts to record source line code, runtime entry and call indices.
    fn get_call_contexts(&self, hurl_file: &HurlFile, entries: &[EntryResult]) -> Vec<CallContext> {
        let mut calls_ctx = vec![];

        for (entry_index, e) in entries.iter().enumerate() {
            let next_e = entries.get(entry_index + 1);
            let retry = match next_e {
                None => false, // last entry of the whole run can't be retried
                Some(next_e) => e.entry_index == next_e.entry_index,
            };
            let kind = match (e.errors.is_empty(), retry) {
                (true, _) => CallContextKind::Success,
                (false, true) => CallContextKind::Retry,
                (false, false) => CallContextKind::Failure,
            };
            for (call_entry_index, _) in e.calls.iter().enumerate() {
                let entry_src_index = e.entry_index - 1;
                let entry_src = hurl_file.entries.get(entry_src_index).unwrap();
                let line = entry_src.source_info().start.line;
                let ctx = CallContext {
                    kind,
                    line,
                    entry_index: entry_index + 1,
                    call_entry_index: call_entry_index + 1,
                    call_index: calls_ctx.len() + 1,
                    source_filename: self.source_filename(),
                    run_filename: self.run_filename(),
                };
                calls_ctx.push(ctx);
            }
        }
        calls_ctx
    }
}
