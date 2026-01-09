/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliOptionsError {
    DisplayHelp(String),
    DisplayVersion(String),
    NoInput(String),
    Error(String),
    InvalidInputFile(PathBuf),
}

impl CliOptionsError {
    /// Converts a clap error to an instance of [`CliOptionsError`].
    pub fn from_clap(error: clap::Error, allow_color: bool) -> Self {
        match error.kind() {
            clap::error::ErrorKind::DisplayVersion => {
                CliOptionsError::DisplayVersion(error.to_string())
            }
            clap::error::ErrorKind::DisplayHelp => {
                let help = if allow_color {
                    error.render().ansi().to_string()
                } else {
                    error.to_string()
                };
                CliOptionsError::DisplayHelp(help)
            }
            _ => {
                // Other clap errors are prefixed with "error ", we strip this prefix as we want to
                // have our own error prefix.
                let message = error.to_string();
                let message = message.strip_prefix("error: ").unwrap_or(&message);
                CliOptionsError::Error(message.to_string())
            }
        }
    }
}

impl fmt::Display for CliOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliOptionsError::DisplayHelp(message) => write!(f, "{message}"),
            CliOptionsError::DisplayVersion(message) => write!(f, "{message}"),
            CliOptionsError::NoInput(message) => write!(f, "{message}"),
            CliOptionsError::Error(message) => write!(f, "error: {message}"),
            CliOptionsError::InvalidInputFile(path) => write!(
                f,
                "error: Cannot access '{}': No such file or directory",
                path.display()
            ),
        }
    }
}
