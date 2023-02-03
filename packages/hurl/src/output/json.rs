/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

use crate::cli::CliError;
use crate::output::write_output;
use crate::runner::HurlResult;

/// Writes the `hurl_result` JSON representation to the file `filename`.
///
/// If `filename` is `None`, stdout is used. The original content of the Hurl
/// file is necessary in order to construct error fields with column, line number etc... when
/// processing failed asserts and captures.
pub fn write_json(
    hurl_result: &HurlResult,
    content: &str,
    filename: &Option<String>,
) -> Result<(), CliError> {
    let json_result = hurl_result.to_json(content);
    let serialized = serde_json::to_string(&json_result).unwrap();
    let s = format!("{serialized}\n");
    write_output(&s.into_bytes(), filename)
}
