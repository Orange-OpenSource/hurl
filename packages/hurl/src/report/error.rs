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
use std::path::{Path, PathBuf};
use std::{fmt, io};

#[derive(Debug)]
pub enum ReportError {
    IO {
        inner: io::Error,
        file: PathBuf,
        message: String,
    },
    Message(String),
}

impl ReportError {
    /// Creates a new error instance.
    pub fn from_string(message: &str) -> Self {
        ReportError::Message(message.to_string())
    }

    /// Creates a new error instance.
    pub fn from_error(error: io::Error, file: &Path, message: &str) -> Self {
        ReportError::IO {
            inner: error,
            file: file.to_path_buf(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReportError::IO {
                inner,
                file,
                message,
            } => write!(f, "{message} {} ({inner})", file.display()),
            ReportError::Message(message) => write!(f, "{message}"),
        }
    }
}

impl From<io::Error> for ReportError {
    fn from(e: io::Error) -> Self {
        ReportError::from_string(&e.to_string())
    }
}
