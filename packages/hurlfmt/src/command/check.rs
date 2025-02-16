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

use hurl_core::input::Input;
use hurl_core::parser::{self, ParseError};

use crate::{format, linter};

/// Represents a check error.
pub enum CheckError {
    IO(String), // TODO: Rajouter message / consistent with FormatError
    Parse {
        content: String,
        input_file: Input,
        error: ParseError,
    },
    Unformatted(String),
}

/// Run the check command for a list of input files
pub fn run(input_files: &[Input]) -> Vec<CheckError> {
    let mut errors = vec![];
    for input_file in input_files {
        if let Err(e) = run_check(input_file) {
            errors.push(e);
        }
    }
    errors
}

/// Run the check command for one input file
fn run_check(input_file: &Input) -> Result<(), CheckError> {
    let content = input_file
        .read_to_string()
        .map_err(|_| CheckError::IO(input_file.to_string()))?;
    let hurl_file = parser::parse_hurl_file(&content).map_err(|error| CheckError::Parse {
        content: content.clone(),
        input_file: input_file.clone(),
        error,
    })?;
    let hurl_file = linter::lint_hurl_file(&hurl_file);
    let formatted = format::format_text(&hurl_file, false);
    if formatted == content {
        Ok(())
    } else {
        Err(CheckError::Unformatted(input_file.to_string()))
    }
}
