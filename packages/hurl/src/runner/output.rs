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
use std::fs::File;
#[cfg(target_family = "windows")]
use std::io::IsTerminal;
use std::io::Write;
use std::{fmt, io};

use crate::runner::{Error, RunnerError};
use hurl_core::ast::{Pos, SourceInfo};

/// Represents the output of write operation: can be either a file or stdout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Output {
    StdOut,
    File(String),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Output::StdOut => "-".to_string(),
            Output::File(file) => file.to_string(),
        };
        write!(f, "{output}")
    }
}

impl Output {
    /// Writes these `bytes` to the output.
    pub fn write(&self, bytes: &[u8]) -> Result<(), Error> {
        match self {
            Output::StdOut => match write_stdout(bytes) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::new_file_write_access("stdout", &e.to_string())),
            },
            Output::File(filename) => {
                let mut file = match File::create(filename) {
                    Ok(file) => file,
                    Err(e) => return Err(Error::new_file_write_access(filename, &e.to_string())),
                };
                match file.write_all(bytes) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::new_file_write_access(filename, &e.to_string())),
                }
            }
        }
    }
}

impl Error {
    /// Creates a new file write access error.
    fn new_file_write_access(filename: &str, error: &str) -> Error {
        // TODO: improve the error with a [`SourceInfo`] passed in parameter.
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

#[cfg(target_family = "unix")]
fn write_stdout(buf: &[u8]) -> Result<(), io::Error> {
    let mut handle = io::stdout().lock();
    handle.write_all(buf)?;
    Ok(())
}

#[cfg(target_family = "windows")]
fn write_stdout(buf: &[u8]) -> Result<(), io::Error> {
    if io::stdout().is_terminal() {
        println!("{}", String::from_utf8_lossy(buf));
    } else {
        let mut handle = io::stdout().lock();
        handle.write_all(buf)?;
    }
    Ok(())
}
