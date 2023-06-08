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
use crate::report::html::Testcase;
use crate::util::logger;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tab {
    Timeline,
    Run,
    Source,
}

impl Testcase {
    /// Returns the HTML navigation component for a `tab`.
    /// This common component is used to get source information and errors.
    pub fn get_nav_html(&self, content: &str, tab: Tab) -> String {
        let status = get_status_html(self.success);
        let errors = self.get_errors_html(content);
        let errors_count = if !self.errors.is_empty() {
            self.errors.len().to_string()
        } else {
            "-".to_string()
        };
        format!(
            include_str!("resources/nav.html"),
            duration = self.time_in_ms,
            errors = errors,
            errors_count = errors_count,
            filename = self.filename,
            href_run = self.run_filename(),
            href_source = self.source_filename(),
            href_timeline = self.timeline_filename(),
            run_selected = tab == Tab::Run,
            source_selected = tab == Tab::Source,
            status = status,
            timeline_selected = tab == Tab::Timeline,
        )
    }

    /// Formats a list of Hurl errors to HTML snippet.
    fn get_errors_html(&self, content: &str) -> String {
        self.errors
            .iter()
            .map(|e| {
                let line = e.source_info.start.line;
                let column = e.source_info.start.column;
                let filename = &self.filename;
                let message = logger::error_string(filename, content, e, false);
                // We override the first part of the error string to add an anchor to
                // the error context.
                let old = format!("{filename}:{line}:{column}");
                let href = self.source_filename();
                let new = format!("<a href=\"{href}#l{line}\">{filename}:{line}:{column}</a>");
                let message = message.replace(&old, &new);
                format!(
                    "<div class=\"error\">\
                     <div class=\"error-desc\"><pre><code>{message}</code></pre></div>\
                 </div>"
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

fn get_status_html(success: bool) -> &'static str {
    if success {
        "<span class=\"success\">Success</span>"
    } else {
        "<span class=\"failure\">Failure</span>"
    }
}
