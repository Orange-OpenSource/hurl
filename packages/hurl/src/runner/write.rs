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
use crate::runner::{Error, RunnerError};
use hurl_core::ast::{Pos, SourceInfo};
use std::fs::File;
use std::io::Write;

// TODO: make functions for sdtout

/// Writes these `bytes` to the file `filename`.
pub fn write_file(bytes: &[u8], filename: &str) -> Result<(), Error> {
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(e) => return Err(Error::new_file_write_access(filename, &e.to_string())),
    };
    match file.write_all(bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new_file_write_access(filename, &e.to_string())),
    }
}

impl Error {
    /// Creates a new file write access error.
    fn new_file_write_access(filename: &str, error: &str) -> Error {
        // TODO: improve the error with a [`SourcInfo`] passed in parameter.
        let source_info = SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0));
        Error::new(
            source_info,
            RunnerError::FileWriteAccess {
                file: filename.to_string(),
                error: error.to_string(),
            },
            false,
        )
    }
}
