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

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use hurl_core::input::Input;
use hurl_core::parser::{self, ParseError};

use crate::{format, linter};

/// Represents a check error.
pub enum FormatError {
    IO(String), // TODO: Rajouter message / consistent with FormatError
    Parse {
        content: String,
        input_file: Input,
        error: ParseError,
    },
}

/// Run the format command for a list of input files
pub fn run(input_files: &[PathBuf]) -> Vec<FormatError> {
    let mut errors = vec![];
    for input_file in input_files {
        if let Err(e) = run_format(input_file) {
            errors.push(e);
        }
    }
    errors
}

/// Run the format command for one input file
fn run_format(input_file: &Path) -> Result<(), FormatError> {
    let content = fs::read_to_string(input_file.display().to_string())
        .map_err(|_| FormatError::IO(input_file.display().to_string()))?;
    let hurl_file = parser::parse_hurl_file(&content).map_err(|error| FormatError::Parse {
        content: content.clone(),
        input_file: Input::new(input_file.display().to_string().as_str()),
        error,
    })?;
    let hurl_file = linter::lint_hurl_file(&hurl_file);
    let formatted = format::format_text(&hurl_file, false);

    let mut file = match std::fs::File::create(input_file) {
        Err(_) => return Err(FormatError::IO(input_file.display().to_string())),
        Ok(file) => file,
    };
    file.write_all(formatted.as_bytes())
        .map_err(|_| FormatError::IO(input_file.display().to_string()))?;
    Ok(())
}
