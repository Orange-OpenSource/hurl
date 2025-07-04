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
use std::{fmt, io};

use hurl::parallel::error::JobError;
use hurl::report;

/// All possibles errors from the command line.
///
/// Note that assert "errors" are not represented here: assert error occurred when the program
/// has successfully run and returned results with false assertions. We don't consider these false
/// asserts as program errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliError {
    /// An error has occurred during reading of an input.
    InputRead(String),
    /// The input is not a valid Hurl file.
    Parsing,
    /// An error has occurred during writing of an output.
    OutputWrite(String),
    /// A generic i/O error has happened.
    GenericIO(String),
}

impl From<report::ReportError> for CliError {
    fn from(error: report::ReportError) -> Self {
        CliError::GenericIO(error.to_string())
    }
}

impl From<JobError> for CliError {
    fn from(error: JobError) -> Self {
        match error {
            JobError::InputRead(message) => CliError::GenericIO(message),
            JobError::Parsing => CliError::Parsing,
            JobError::OutputWrite(message) => CliError::OutputWrite(message),
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::InputRead(message) => write!(f, "{message}"),
            CliError::Parsing => Ok(()),
            CliError::OutputWrite(message) => write!(f, "{message}"),
            CliError::GenericIO(message) => write!(f, "{message}"),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::GenericIO(error.to_string())
    }
}
