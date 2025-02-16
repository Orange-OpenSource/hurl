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

use crate::cli::options::{InputFormat, OutputFormat};
use crate::{curl, format, linter};

/// Represents an export error.
pub enum ExportError {
    IO {
        filename: String,
        message: String,
    },
    Parse {
        content: String,
        input_file: Input,
        error: ParseError,
    },
    Curl(String),
}

/// Run the export command for a list of input files
pub fn run(
    input_files: &[Input],
    input_format: &InputFormat,
    output_format: &OutputFormat,
    standalone: bool,
    color: bool,
) -> Vec<Result<String, ExportError>> {
    input_files
        .iter()
        .map(|input_file| run_export(input_file, input_format, output_format, standalone, color))
        .collect()
}

/// Run the export command for one input file
fn run_export(
    input_file: &Input,
    input_format: &InputFormat,
    output_format: &OutputFormat,
    standalone: bool,
    color: bool,
) -> Result<String, ExportError> {
    let content = input_file.read_to_string().map_err(|e| ExportError::IO {
        filename: input_file.to_string(),
        message: e.to_string(),
    })?;

    // Parse input curl or Hurl file
    let input = match input_format {
        InputFormat::Hurl => content.to_string(),
        InputFormat::Curl => curl::parse(&content).map_err(ExportError::Curl)?,
    };
    let hurl_file = parser::parse_hurl_file(&input).map_err(|error| ExportError::Parse {
        content: input.clone(),
        input_file: input_file.clone(),
        error,
    })?;

    let output = match output_format {
        OutputFormat::Hurl => {
            let hurl_file = linter::lint_hurl_file(&hurl_file);
            format::format_text(&hurl_file, color)
        }
        OutputFormat::Json => format::format_json(&hurl_file),
        OutputFormat::Html => hurl_core::format::format_html(&hurl_file, standalone),
    };
    Ok(output)
}
