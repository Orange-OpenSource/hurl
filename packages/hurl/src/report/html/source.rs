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
use hurl_core::ast::{HurlFile, SourceInfo};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::report::html::nav::Tab;
use crate::report::html::Testcase;
use crate::runner::RunnerError;

impl Testcase {
    /// Returns the HTML string of the Hurl source file (syntax colored and errors).
    pub fn get_source_html(&self, hurl_file: &HurlFile, content: &str, secrets: &[&str]) -> String {
        let nav = self.get_nav_html(content, Tab::Source, secrets);
        let nav_css = include_str!("resources/nav.css");
        let source_div = hurl_core::format::format_html(hurl_file, false);
        let source_div = underline_errors(&source_div, &self.errors);
        let lines_div = get_numbered_lines(content);
        let source_css = include_str!("resources/source.css");
        let hurl_css = hurl_core::format::hurl_css();
        format!(
            include_str!("resources/source.html"),
            filename = self.filename,
            hurl_css = hurl_css,
            lines_div = lines_div,
            nav = nav,
            nav_css = nav_css,
            source_div = source_div,
            source_css = source_css,
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

lazy_static! {
    static ref LINES_RE: Regex = Regex::new("<span class=\"line\">").unwrap();
}

/// Adds error class to `content` lines that triggers `errors`.
fn underline_errors(content: &str, errors: &[(RunnerError, SourceInfo)]) -> String {
    // In nutshell, we're replacing line `<span class="line">...</span>`
    // with `<span class="line line-error">...</span>`.
    let mut line = 0;
    let error_lines = errors
        .iter()
        .map(|(error, _)| error.source_info.start.line - 1)
        .collect::<Vec<_>>();
    LINES_RE
        .replace_all(content, |_: &Captures| {
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
    use hurl_core::ast::SourceInfo;
    use hurl_core::reader::Pos;

    use super::*;
    use crate::runner::RunnerErrorKind::QueryHeaderNotFound;

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

        let error = RunnerError {
            source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 4)),
            kind: QueryHeaderNotFound,
            assert: true,
        };
        let entry_src_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 18));
        let errors = [(error, entry_src_info)];
        assert_eq!(underlined_content, underline_errors(content, &errors));
    }
}
