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
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fmt, io};

use hurl_core::ast::SourceInfo;

use crate::runner::{RunnerError, RunnerErrorKind};
use crate::util::path::ContextDir;
use crate::util::term::Stdout;

/// Represents the output of write operation: can be either a file or standard output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Output {
    /// Write to file.
    File(PathBuf),
    /// Write to standard output.
    Stdout,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Output::File(file) => file.to_string_lossy().to_string(),
            Output::Stdout => "-".to_string(),
        };
        write!(f, "{output}")
    }
}

impl Output {
    /// Creates a new output from a string filename.
    pub fn new(filename: &str) -> Self {
        if filename == "-" {
            Output::Stdout
        } else {
            Output::File(PathBuf::from(filename))
        }
    }

    /// Writes these `bytes` to the output.
    ///
    /// If output is a standard output variant, `stdout` is used to write the bytes. If `append`
    /// the output file is created in append mode, else any existing file will be truncated.
    pub fn write(&self, bytes: &[u8], stdout: &mut Stdout, append: bool) -> Result<(), io::Error> {
        match self {
            Output::Stdout => stdout.write_all(bytes)?,
            Output::File(filename) => {
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(append)
                    .open(filename)?;
                file.write_all(bytes)?;
            }
        }
        Ok(())
    }

    /// Writes these `bytes` to the output.
    ///
    /// If output is a standard output variant, `stdout` is used to write the bytes.
    /// If output is a file variant, `context_dir` is used to check authorized write access.
    pub fn write_with_context_dir(
        &self,
        bytes: &[u8],
        stdout: &mut Stdout,
        context_dir: &ContextDir,
        source_info: SourceInfo,
    ) -> Result<(), RunnerError> {
        // TODO: Check if write method above can be reused
        match self {
            Output::Stdout => match stdout.write_all(bytes) {
                Ok(_) => Ok(()),
                Err(e) => {
                    let filename = Path::new("stdout");
                    Err(RunnerError::new_file_write_access(
                        filename,
                        &e.to_string(),
                        source_info,
                    ))
                }
            },
            Output::File(filename) => {
                if !context_dir.is_access_allowed(filename) {
                    return Err(RunnerError::new_unauthorized_file_access(
                        filename,
                        source_info,
                    ));
                }
                // we check if we can write to this filename and compute the new filename given this context dir.
                let filename = context_dir.resolved_path(filename);
                let mut file = match File::create(&filename) {
                    Ok(file) => file,
                    Err(e) => {
                        return Err(RunnerError::new_file_write_access(
                            &filename,
                            &e.to_string(),
                            source_info,
                        ))
                    }
                };
                match file.write_all(bytes) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(RunnerError::new_file_write_access(
                        &filename,
                        &e.to_string(),
                        source_info,
                    )),
                }
            }
        }
    }
}

impl RunnerError {
    /// Creates a new file write access error.
    fn new_file_write_access(path: &Path, error: &str, source_info: SourceInfo) -> RunnerError {
        let path = path.to_path_buf();
        let kind = RunnerErrorKind::FileWriteAccess {
            path,
            error: error.to_string(),
        };
        RunnerError::new(source_info, kind, false)
    }

    /// Creates a new authorization access error.
    fn new_unauthorized_file_access(path: &Path, source_info: SourceInfo) -> RunnerError {
        let path = path.to_path_buf();
        let kind = RunnerErrorKind::UnauthorizedFileAccess { path };
        RunnerError::new(source_info, kind, false)
    }
}
