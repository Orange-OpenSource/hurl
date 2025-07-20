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

use crate::report::html::nav::Tab;
use crate::report::html::Testcase;
use crate::runner::RunnerError;
use crate::util::redacted::Redact;

impl Testcase {
    /// Returns the HTML string of the Hurl source file (syntax colored and errors).
    /// Secret values are redacted from the output.
    pub fn get_source_html(&self, hurl_file: &HurlFile, content: &str, secrets: &[&str]) -> String {
        let nav = self.get_nav_html(content, Tab::Source, secrets);
        let nav_css = include_str!("resources/nav.css");
        let source_div = hurl_core::format::format_html(hurl_file, false);
        let lines_div = get_numbered_lines(content, &self.errors);
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
        .redact(secrets)
    }
}

/// Returns a list of lines number in HTML.
fn get_numbered_lines(content: &str, errors: &[(RunnerError, SourceInfo)]) -> String {
    let errors = errors
        .iter()
        .map(|(error, _)| error.source_info.start.line)
        .collect::<Vec<_>>();
    let mut lines =
        content
            .lines()
            .enumerate()
            .fold("<pre><code>".to_string(), |acc, (count, _)| -> String {
                let line = count + 1;
                let tag = if errors.contains(&line) {
                    format!("<a id=\"l{line}\" href=\"#l{line}\" class=\"line-error\">{line}</a>\n")
                } else {
                    format!("<a id=\"l{line}\" href=\"#l{line}\">{line}</a>\n")
                };
                acc + &tag
            });
    lines.push_str("</pre></code>");
    lines
}

#[cfg(test)]
mod tests {
    use crate::report::html::Testcase;
    use crate::runner::HurlResult;
    use hurl_core::input::Input;
    use hurl_core::parser;

    #[test]
    fn secrets_redacted_in_source_html() {
        let content = r#"
GET https://localhost
X-Secret: secret
HTTP 200
"#;
        let hurl_file = parser::parse_hurl_file(content).unwrap();
        let filename = Input::new("test.hurl");
        let testcase = Testcase::from(&HurlResult::default(), &filename);
        let html = testcase.get_source_html(&hurl_file, content, &["secret"]);
        assert!(html.contains("***"));
        assert!(!html.contains("secret"));
    }
}
