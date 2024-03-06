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
use hurl::report;
use std::string::FromUtf8Error;
use std::{fmt, io};

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliError {
    IO(String),
    Parsing,
    Runtime(String),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IO(error.to_string())
    }
}

impl From<FromUtf8Error> for CliError {
    fn from(error: FromUtf8Error) -> Self {
        CliError::IO(error.to_string())
    }
}

impl From<report::Error> for CliError {
    fn from(error: report::Error) -> Self {
        CliError::IO(error.message)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::IO(message) => write!(f, "{}", message),
            CliError::Parsing => Ok(()),
            CliError::Runtime(message) => write!(f, "{}", message),
        }
    }
}
