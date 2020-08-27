/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
use crate::core::common::{FormatError, SourceInfo};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub source_info: SourceInfo,
    pub inner: LinterError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinterError {
    UnneccessarySpace {},
    UnneccessaryJsonEncoding {},
    OneSpace {},
}


impl FormatError for Error {
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

pub trait Lintable<T> {
    fn errors(&self) -> Vec<Error>;
    fn lint(&self) -> T;
}
