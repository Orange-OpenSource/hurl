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
use std::io::Write;
use std::path::Path;

use hurl_core::parser;

use crate::report::html::Testcase;
use crate::report::Error;

impl Testcase {
    /// Exports a [`Testcase`] to HTML.
    ///
    /// For the moment, it's just an export of this HTML file, with syntax colored.
    pub fn write_html(&self, content: &str, dir_path: &Path) -> Result<(), Error> {
        let output_file = dir_path.join("store").join(format!("{}.html", self.id));

        let parent = output_file.parent().expect("a parent");
        std::fs::create_dir_all(parent).unwrap();
        let mut file = match std::fs::File::create(&output_file) {
            Err(why) => {
                return Err(Error {
                    message: format!("Issue writing to {}: {:?}", output_file.display(), why),
                });
            }
            Ok(file) => file,
        };
        let hurl_file = parser::parse_hurl_file(content).unwrap();
        let file_div = hurl_core::format::format_html(&hurl_file, false);
        let lines_div = lines(content);
        let file_css = include_str!("resources/file.css");
        let status = if self.success {
            "<span class=\"success\">Success</span>"
        } else {
            "<span class=\"failure\">Failure</span>"
        };
        let hurl_css = hurl_core::format::hurl_css();
        let href = format!("{}.html", self.id);
        let html = format!(
            include_str!("resources/file.html"),
            file_css = file_css,
            hurl_css = hurl_css,
            lines_div = lines_div,
            file_div = file_div,
            filename = self.filename,
            status = status,
            href = href,
            duration = self.time_in_ms
        );
        if let Err(why) = file.write_all(html.as_bytes()) {
            return Err(Error {
                message: format!("Issue writing to {}: {:?}", output_file.display(), why),
            });
        }
        Ok(())
    }
}

fn lines(content: &str) -> String {
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
