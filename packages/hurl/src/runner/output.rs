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
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::runner::{Error, RunnerError};
use crate::util::path::ContextDir;
use crate::util::term::Stdout;
use hurl_core::ast::{Pos, SourceInfo};

/// Represents the output of write operation: can be either a file or stdout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Output {
    StdOut,
    File(PathBuf),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Output::StdOut => "-".to_string(),
            Output::File(file) => file.to_string_lossy().to_string(),
        };
        write!(f, "{output}")
    }
}

impl Output {
    /// Creates a new output from a string filename.
    pub fn new(filename: &str) -> Self {
        if filename == "-" {
            Output::StdOut
        } else {
            Output::File(PathBuf::from(filename))
        }
    }

    /// Writes these `bytes` to the output.
    ///
    /// If output is a standard output variant, `stdout` is used to write the bytes.
    /// If output is a file variant, an optional `context_dir` can be used to check authorized
    /// write access.
    pub fn write(
        &self,
        bytes: &[u8],
        stdout: &mut Stdout,
        context_dir: Option<&ContextDir>,
    ) -> Result<(), Error> {
        match self {
            Output::StdOut => match stdout.write_all(bytes) {
                Ok(_) => Ok(()),
                Err(e) => {
                    let filename = Path::new("stdout");
                    Err(Error::new_file_write_access(filename, &e.to_string()))
                }
            },
            Output::File(filename) => {
                // If we have a context dir, we check if we can write to this filename and compute
                // the new filename given this context dir.
                let filename = match context_dir {
                    None => filename.clone(),
                    Some(context_dir) => {
                        if !context_dir.is_access_allowed(filename) {
                            return Err(Error::new_unauthorized_file_access(filename));
                        }
                        context_dir.resolved_path(filename)
                    }
                };
                let mut file = match File::create(&filename) {
                    Ok(file) => file,
                    Err(e) => return Err(Error::new_file_write_access(&filename, &e.to_string())),
                };
                match file.write_all(bytes) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::new_file_write_access(&filename, &e.to_string())),
                }
            }
        }
    }
}

// TODO: improve the error with a [`SourceInfo`] passed in parameter.
impl Error {
    /// Creates a new file write access error.
    fn new_file_write_access(path: &Path, error: &str) -> Error {
        let source_info = SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0));
        let path = path.to_path_buf();
        let inner = RunnerError::FileWriteAccess {
            path,
            error: error.to_string(),
        };
        Error::new(source_info, inner, false)
    }

    /// Creates a new authorization access error.
    fn new_unauthorized_file_access(path: &Path) -> Error {
        let source_info = SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0));
        let path = path.to_path_buf();
        let inner = RunnerError::UnauthorizedFileAccess { path };
        Error::new(source_info, inner, false)
    }
}
