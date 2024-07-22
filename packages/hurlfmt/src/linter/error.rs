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
use hurl_core::error;
use hurl_core::error::DisplaySourceError;
use hurl_core::text::{Style, StyledString};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinterError {
    pub source_info: SourceInfo,
    pub kind: LinterErrorKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LinterErrorKind {
    UnnecessarySpace,
    UnnecessaryJsonEncoding,
    OneSpace,
}

///
/// Textual Output for linter errors
///
impl DisplaySourceError for LinterError {
    fn source_info(&self) -> SourceInfo {
        self.source_info
    }

    fn description(&self) -> String {
        match self.kind {
            LinterErrorKind::UnnecessarySpace => "Unnecessary space".to_string(),
            LinterErrorKind::UnnecessaryJsonEncoding => "Unnecessary json encoding".to_string(),
            LinterErrorKind::OneSpace => "One space".to_string(),
        }
    }

    fn fixme(&self, content: &[&str]) -> StyledString {
        let message = match self.kind {
            LinterErrorKind::UnnecessarySpace => "Remove space".to_string(),
            LinterErrorKind::UnnecessaryJsonEncoding => "Use Simple String".to_string(),
            LinterErrorKind::OneSpace => "Use only one space".to_string(),
        };
        let mut s = StyledString::new();
        let message = error::add_carets(&message, self.source_info(), content);
        s.push_with(&message, Style::new().cyan());
        s
    }
}
