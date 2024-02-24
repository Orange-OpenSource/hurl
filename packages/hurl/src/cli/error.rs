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
use std::error::Error;
use std::fmt;

use hurl::{output, report};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliError {
    pub message: String,
}

impl From<Box<dyn Error>> for CliError {
    fn from(e: Box<dyn Error>) -> Self {
        Self {
            message: format!("{e:?}"),
        }
    }
}

impl From<&str> for CliError {
    fn from(e: &str) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl From<String> for CliError {
    fn from(e: String) -> Self {
        Self { message: e }
    }
}

impl From<report::Error> for CliError {
    fn from(e: report::Error) -> Self {
        Self { message: e.message }
    }
}

impl From<output::Error> for CliError {
    fn from(e: output::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
