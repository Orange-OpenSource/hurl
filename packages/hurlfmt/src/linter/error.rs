/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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
use hurl_core::error::Error;

use crate::linter;
use crate::linter::LinterError;

///
/// Textual Output for linter errors
///
///
impl Error for linter::Error {
    fn source_info(&self) -> SourceInfo {
        self.clone().source_info
    }

    fn description(&self) -> String {
        match self.inner {
            LinterError::UnneccessarySpace { .. } => "Unnecessary space".to_string(),
            LinterError::UnneccessaryJsonEncoding {} => "Unnecessary json encoding".to_string(),
            LinterError::OneSpace {} => "One space ".to_string(),
        }
    }

    fn fixme(&self) -> String {
        match self.inner {
            LinterError::UnneccessarySpace { .. } => "Remove space".to_string(),
            LinterError::UnneccessaryJsonEncoding {} => "Use Simple String".to_string(),
            LinterError::OneSpace {} => "Use only one space".to_string(),
        }
    }
}
