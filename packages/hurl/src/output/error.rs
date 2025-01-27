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
use hurl_core::error::DisplaySourceError;
use hurl_core::text::{Style, StyledString};

use crate::http::HttpError;

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

    fn fixme(&self, content: &[&str]) -> StyledString {
        match &self.kind {
            OutputErrorKind::Http(http_error) => {
                let message = http_error.message();
                let message = hurl_core::error::add_carets(&message, self.source_info, content);
                color_red(&message)
            }
            OutputErrorKind::Binary => {
                let message = "Binary output can mess up your terminal. Use \"--output -\" to tell Hurl to output it to your terminal anyway, or consider \"--output\" to save to a file.";

                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red(&message)
            }
            OutputErrorKind::Io(message) => {
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                color_red(&message)
            }
        }
    }
}

fn color_red(message: &str) -> StyledString {
    let mut s = StyledString::new();
    s.push_with(message, Style::new().red().bold());
    s
}
