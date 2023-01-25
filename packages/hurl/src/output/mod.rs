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

mod json;
mod raw;
mod stdout;

pub use self::json::write_json;
pub use self::raw::write_body;
use crate::cli::CliError;
use crate::output::stdout::write_stdout;
use std::io::Write;
use std::path::Path;

/// Writes `bytes` to the file `filename` or stdout by default.
fn write_output(bytes: &Vec<u8>, filename: &Option<String>) -> Result<(), CliError> {
    match filename {
        None => write_stdout(bytes.as_slice()),
        Some(filename) => {
            let path = Path::new(filename.as_str());
            let mut file = match std::fs::File::create(path) {
                Err(why) => {
                    return Err(CliError {
                        message: format!("Issue writing to {}: {:?}", path.display(), why),
                    });
                }
                Ok(file) => file,
            };
            file.write_all(bytes.as_slice())
                .expect("writing bytes to file");
            Ok(())
        }
    }
}
