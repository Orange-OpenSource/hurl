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
use std::io;

use crate::runner::{HurlResult, Input, Output};
use crate::util::term::Stdout;

/// Writes the `hurl_result` JSON representation to the file `filename_out`.
///
/// If `filename_out` is `None`, stdout is used. If `append` is true, any existing file will
/// be appended instead of being truncated. The original `content` of the Hurl file and the
/// source `filename_in` is necessary in order to construct error fields with column, line number
/// etc... when processing failed asserts and captures.
pub fn write_json(
    hurl_result: &HurlResult,
    content: &str,
    filename_in: &Input,
    filename_out: Option<&Output>,
    stdout: &mut Stdout,
    append: bool,
) -> Result<(), io::Error> {
    let json_result = hurl_result.to_json(content, filename_in, None)?;
    let serialized = serde_json::to_string(&json_result).unwrap();
    let bytes = format!("{serialized}\n");
    let bytes = bytes.into_bytes();
    match filename_out {
        Some(out) => out.write(&bytes, stdout, append)?,
        None => Output::Stdout.write(&bytes, stdout, append)?,
    }
    Ok(())
}
