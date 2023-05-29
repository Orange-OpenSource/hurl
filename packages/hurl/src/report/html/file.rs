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
use hurl_core::ast::HurlFile;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use hurl_core::parser;

use crate::report::html::Testcase;
use crate::report::Error;
use crate::runner::{EntryResult, Error as RunnerError};
use crate::util::logger;

impl Testcase {
    /// Exports a [`Testcase`] to HTML.
    ///
    /// It will create two HTML files:
    /// - an HTML for the Hurl source file (with potential errors and syntax colored),
    /// - an HTML for the entries waterfall
    pub fn write_html(
        &self,
        content: &str,
        entries: &[EntryResult],
        dir_path: &Path,
    ) -> Result<(), Error> {
        // We create the HTML Hurl source file.
        let output_file = dir_path.join("store").join(format!("{}.html", self.id));
        let mut file = File::create(output_file)?;

        // We parse the content as we'll reuse the AST to construct the HTML source file, and
        // the waterfall.
        // TODO: for the moment, we can only have parseable file.
        let hurl_file = parser::parse_hurl_file(content).unwrap();

        let html = self.get_file_html(&hurl_file, content);
        file.write_all(html.as_bytes())?;

        // Then we create the HTML entries waterfall.
        let output_file = dir_path
            .join("store")
            .join(format!("{}-waterfall.html", self.id));
        let mut file = File::create(output_file)?;
        let html = self.get_waterfall_html(&hurl_file, entries);
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    /// Returns the HTML string of the Hurl source file (syntax colored and errors).
    fn get_file_html(&self, hurl_file: &HurlFile, content: &str) -> String {
        let file_div = hurl_core::format::format_html(hurl_file, false);
        let file_div = underline_errors(&file_div, &self.errors);
        let lines_div = get_numbered_lines(content);
        let file_css = include_str!("resources/file.css");
        let status = if self.success {
            "<span class=\"success\">Success</span>"
        } else {
            "<span class=\"failure\">Failure</span>"
        };

        let errors = get_html_errors(&self.filename, content, &self.errors);
        let errors_count = if !errors.is_empty() {
            errors.len().to_string()
        } else {
            "-".to_string()
        };
        let hurl_css = hurl_core::format::hurl_css();
        let href_file = format!("{}.html", self.id);
        let href_waterfall = format!("{}-waterfall.html", self.id);
        format!(
            include_str!("resources/file.html"),
            file_css = file_css,
            hurl_css = hurl_css,
            lines_div = lines_div,
            file_div = file_div,
            errors_count = errors_count,
            errors = errors,
            filename = self.filename,
            status = status,
            href_file = href_file,
            href_waterfall = href_waterfall,
            duration = self.time_in_ms
        )
    }
}

/// Returns a list of lines number in HTML.
fn get_numbered_lines(content: &str) -> String {
    let mut lines =
        content
            .lines()
            .enumerate()
            .fold("<pre><code>".to_string(), |acc, (count, _)| -> String {
                let line = count + 1;
                acc + format!("<a id=\"l{line}\" href=\"#l{line}\">{line}</a>\n").as_str()
            });
    lines.push_str("</pre></code>");
    lines
}

/// Formats a list of Hurl errors to HTML snippet.
fn get_html_errors(filename: &str, content: &str, errors: &[RunnerError]) -> String {
    errors
        .iter()
        .map(|e| {
            let line = e.source_info.start.line;
            let column = e.source_info.start.column;
            let message = logger::error_string(filename, content, e, false);
            // We override the first part of the error string to add an anchor to
            // the error context.
            let old = format!("{filename}:{line}:{column}");
            let new = format!("<a href=\"#l{line}\">{filename}:{line}:{column}</a>");
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

/// Adds error class to `content` lines that triggers `errors`.
fn underline_errors(content: &str, errors: &[RunnerError]) -> String {
    // In nutshell, we're replacing line `<span class="line">...</span>`
    // with `<span class="line line-error">...</span>`.
    let re = Regex::new("<span class=\"line\">").unwrap();
    let mut line = 0;
    let error_lines = errors
        .iter()
        .map(|e| e.source_info.start.line - 1)
        .collect::<Vec<_>>();
    re.replace_all(content, |_: &Captures| {
        let str = if error_lines.contains(&line) {
            "<span class=\"line line-error\">"
        } else {
            "<span class=\"line\">"
        };
        line += 1;
        str
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::RunnerError::QueryHeaderNotFound;
    use hurl_core::ast::SourceInfo;

    #[test]
    fn add_underlined_errors() {
        let content = r#"
        <pre>
            <code class="language-hurl">
                <span class="hurl-entry">
                    <span class="request">
                        <span class="line">
                            <span class="method">GET</span> <span class="url">http://foo.com</span>
                        </span>
                        <span class="line">
                            <span class="string">x-bar</span><span>:</span> <span class="string">baz</span>
                        </span>
                    </span>
                    <span class="response">
                        <span class="line">
                            <span class="version">HTTP</span> <span class="number">200</span>
                        </span>
                    </span>
                </span>
                <span class="line">
                </span>
            </code>
        </pre>"#;

        let underlined_content = r#"
        <pre>
            <code class="language-hurl">
                <span class="hurl-entry">
                    <span class="request">
                        <span class="line">
                            <span class="method">GET</span> <span class="url">http://foo.com</span>
                        </span>
                        <span class="line line-error">
                            <span class="string">x-bar</span><span>:</span> <span class="string">baz</span>
                        </span>
                    </span>
                    <span class="response">
                        <span class="line">
                            <span class="version">HTTP</span> <span class="number">200</span>
                        </span>
                    </span>
                </span>
                <span class="line">
                </span>
            </code>
        </pre>"#;

        let errors = vec![RunnerError {
            source_info: SourceInfo::new(2, 1, 2, 4),
            inner: QueryHeaderNotFound,
            assert: true,
        }];

        assert_eq!(underlined_content, underline_errors(content, &errors));
    }
}
