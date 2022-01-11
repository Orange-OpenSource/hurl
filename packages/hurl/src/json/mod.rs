/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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
use std::io::prelude::*;
use std::path::PathBuf;

mod result;
mod value;

pub fn write_json_report(
    file_path: PathBuf,
    results: Vec<serde_json::Value>,
) -> Result<(), CliError> {
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            return Err(CliError {
                message: format!("Issue writing to {}: {:?}", file_path.display(), why),
            })
        }
        Ok(file) => file,
    };
    let serialized = serde_json::to_string_pretty(&results).unwrap();
    if let Err(why) = file.write_all(serialized.as_bytes()) {
        Err(CliError {
            message: format!("Issue writing to {}: {:?}", file_path.display(), why),
        })
    } else {
        Ok(())
    }
}

pub fn parse_json(path: PathBuf) -> Result<Vec<serde_json::Value>, CliError> {
    if path.exists() {
        let s = match std::fs::read_to_string(path.clone()) {
            Ok(s) => s,
            Err(why) => {
                return Err(CliError {
                    message: format!("Issue reading {} to string to {:?}", path.display(), why),
                });
            }
        };
        match serde_json::from_str(s.as_str()) {
            Ok(val) => Ok(val),
            Err(_) => {
                return Err(CliError {
                    message: format!("The file {} is not a valid json file", path.display()),
                })
            }
        }
    } else {
        Ok(vec![])
    }
}
