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
use hurl_core::ast::{Pos, SourceInfo};
use hurl_core::error::Error;

use crate::runner::HurlResult;
use crate::util::logger::Logger;
use crate::{output, runner};

/// Writes the `hurl_result` last body response to the file `filename_out`.
///
/// If `filename` is `None`, stdout is used. If `include_headers` is true, the last HTTP
/// response headers are written before the body response.
pub fn write_body(
    hurl_result: &HurlResult,
    filename_in: &str,
    include_headers: bool,
    color: bool,
    filename_out: &Option<String>,
    logger: &Logger,
) -> Result<(), output::error::Error> {
    // By default, we output the body response bytes of the last entry
    if let Some(entry_result) = hurl_result.entries.last() {
        if let Some(call) = entry_result.calls.last() {
            let response = &call.response;
            let mut output = vec![];

            // If include options is set, we output the HTTP response headers
            // with status and version (to mimic curl outputs)
            if include_headers {
                let mut text = response.get_status_line_headers(color);
                text.push('\n');
                output.append(&mut text.into_bytes());
            }
            let mut body = if entry_result.compressed {
                match response.uncompress_body() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        // FIXME: we convert to a runner::Error to be able to use fixme
                        // method. Can we do otherwise (without creating an artificial
                        // error a first character).
                        let source_info = SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0));
                        let error = runner::Error::new(source_info, e.into(), false);
                        let message = error.fixme();
                        return Err(output::error::Error::new(&message));
                    }
                }
            } else {
                response.body.clone()
            };
            output.append(&mut body);
            output::write_output(&output, filename_out)?;
        } else {
            logger.info("No response has been received");
        }
    } else {
        let source = if filename_in == "-" {
            String::new()
        } else {
            format!("for file {filename_in}")
        };
        logger.warning(format!("No entry have been executed {source}").as_str());
    }
    Ok(())
}
