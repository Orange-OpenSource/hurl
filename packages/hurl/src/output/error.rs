/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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

use crate::http::HttpError;
use colored::Colorize;
use hurl_core::ast::SourceInfo;
use hurl_core::error::{get_message, split_lines, DisplaySourceError};
use std::cmp::max;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputError {
    pub source_info: SourceInfo,
    pub kind: OutputErrorKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputErrorKind {
    Http(HttpError),
    Binary,
    Io(String),
}

impl OutputError {
    pub fn new(source_info: SourceInfo, kind: OutputErrorKind) -> OutputError {
        OutputError { source_info, kind }
    }
}

/// Textual Output for runner errors
impl DisplaySourceError for OutputError {
    fn source_info(&self) -> SourceInfo {
        self.source_info
    }

    fn description(&self) -> String {
        match &self.kind {
            OutputErrorKind::Http(http_error) => http_error.description(),
            OutputErrorKind::Binary => "Binary Error".to_string(),
            OutputErrorKind::Io(_) => "IO Error".to_string(),
        }
    }

    fn fixme(&self, content: &[&str], color: bool) -> (String, usize) {
        let message = match &self.kind {
            OutputErrorKind::Http(http_error) => {
                let message = http_error.message();
                let message = hurl_core::error::add_carets(&message, self.source_info, content);

                if color {
                    message.red().bold().to_string()
                } else {
                    message
                }
            }
            OutputErrorKind::Binary => {
                let message = "Binary output can mess up your terminal. Use \"--output -\" to tell Hurl to output it to your terminal anyway, or consider \"--output\" to save to a file.";

                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            OutputErrorKind::Io(message) => {
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
        };
        (message, 0)
    }

    fn show_source_line(&self) -> bool {
        true
    }

    fn info(&self, content: &[&str], color: bool) -> String {
        let error_line = self.source_info().start.line;
        // The number of digits of the lines count.
        let loc_max_width = max(content.len().to_string().len(), 2);
        let separator = "|";

        let spaces = " ".repeat(loc_max_width);
        let prefix = format!("{spaces} {separator}");
        let prefix = if color {
            prefix.blue().bold().to_string()
        } else {
            prefix.to_string()
        };

        let (message, offset) = get_message(self, content, color);

        let error_line = error_line + offset;
        let prefix_with_number = format!("{error_line:>loc_max_width$} {separator}");
        let prefix_with_number = if color {
            prefix_with_number.blue().bold().to_string()
        } else {
            prefix_with_number.to_string()
        };

        let mut text = String::new();
        for (i, line) in split_lines(&message).iter().enumerate() {
            text.push('\n');
            text.push_str(if i == 0 { &prefix_with_number } else { &prefix });
            text.push_str(line);
        }
        text
    }
}
