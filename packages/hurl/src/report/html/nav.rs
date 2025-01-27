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
use hurl_core::ast::SourceInfo;
use hurl_core::error::{DisplaySourceError, OutputFormat};

use crate::report::html::Testcase;
use crate::runner::RunnerError;
use crate::util::redacted::Redact;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tab {
    Timeline,
    Run,
    Source,
}

impl Testcase {
    /// Returns the HTML navigation component for a `tab`.
    /// This common component is used to get source information and errors.
    pub fn get_nav_html(&self, content: &str, tab: Tab, secrets: &[&str]) -> String {
        let status = get_status_html(self.success);
        let errors = self.get_errors_html(content, secrets);
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
    fn get_errors_html(&self, content: &str, secrets: &[&str]) -> String {
        self.errors
            .iter()
            .map(|(error, entry_src_info)| {
                let error = error_to_html(
                    error,
                    *entry_src_info,
                    content,
                    &self.filename,
                    &self.source_filename(),
                    secrets,
                );
                format!("<div class=\"error\"><div class=\"error-desc\">{error}</div></div>")
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

/// Returns an HTML `<pre>` tag representing this `error`.
fn error_to_html(
    error: &RunnerError,
    entry_src_info: SourceInfo,
    content: &str,
    filename: &str,
    source_filename: &str,
    secrets: &[&str],
) -> String {
    let line = error.source_info.start.line;
    let column = error.source_info.start.column;
    let message = error.to_string(
        filename,
        content,
        Some(entry_src_info),
        OutputFormat::Terminal(false),
    );
    let message = message.redact(secrets);
    let message = html_escape(&message);
    // We override the first part of the error string to add an anchor to
    // the error context.
    let old = format!("{filename}:{line}:{column}");
    let href = source_filename;
    let new = format!("<a href=\"{href}#l{line}\">{filename}:{line}:{column}</a>");
    let message = message.replace(&old, &new);
    format!("<pre><code>{message}</code></pre>")
}

/// Escapes '<' and '>' from `text`.
fn html_escape(text: &str) -> String {
    text.replace('<', "&lt;").replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::SourceInfo;
    use hurl_core::reader::Pos;

    use crate::report::html::nav::error_to_html;
    use crate::runner::{RunnerError, RunnerErrorKind};

    #[test]
    fn test_error_html() {
        let entry_src_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 39));
        let error = RunnerError::new(
            SourceInfo::new(Pos::new(4, 1), Pos::new(4, 9)),
            RunnerErrorKind::AssertFailure {
                actual: "<script>alert('Hi')</script>".to_string(),
                expected: "Hello world".to_string(),
                type_mismatch: false,
            },
            true,
        );
        let content = "GET http://localhost:8000/inline-script\n\
                       HTTP 200\n\
                       [Asserts]\n\
                       `Hello World`\n\
                      ";
        let filename = "a/b/c/foo.hurl";
        let source_filename = "abc-source.hurl";
        let html = error_to_html(
            &error,
            entry_src_info,
            content,
            filename,
            source_filename,
            &[],
        );
        assert_eq!(
            html,
            r##"<pre><code>Assert failure
  --&gt; <a href="abc-source.hurl#l4">a/b/c/foo.hurl:4:1</a>
   |
   | GET http://localhost:8000/inline-script
   | ...
 4 | `Hello World`
   |   actual:   &lt;script&gt;alert('Hi')&lt;/script&gt;
   |   expected: Hello world
   |</code></pre>"##
        );
    }
}
