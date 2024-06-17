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
use hurl_core::ast::SourceInfo;
use hurl_core::error::DisplaySourceError;
use hurl_core::text::StyledString;

use crate::linter;
use crate::linter::LinterError;

///
/// Textual Output for linter errors
///
impl DisplaySourceError for linter::Error {
    fn source_info(&self) -> SourceInfo {
        self.source_info
    }

    fn description(&self) -> String {
        match self.inner {
            LinterError::UnnecessarySpace => "Unnecessary space".to_string(),
            LinterError::UnnecessaryJsonEncoding => "Unnecessary json encoding".to_string(),
            LinterError::OneSpace => "One space ".to_string(),
        }
    }

    fn fixme(&self, _lines: &[&str]) -> StyledString {
        let message = match self.inner {
            LinterError::UnnecessarySpace => "Remove space".to_string(),
            LinterError::UnnecessaryJsonEncoding => "Use Simple String".to_string(),
            LinterError::OneSpace => "Use only one space".to_string(),
        };
        let mut s = StyledString::new();
        s.push(&message);
        s
    }

    fn message(&self, content: &[&str]) -> StyledString {
        let mut text = StyledString::new();
        hurl_core::error::add_source_line(&mut text, content, self.source_info().start.line);
        text.append(self.fixme(content));
        text
    }
}
