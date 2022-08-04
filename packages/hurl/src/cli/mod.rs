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

use std::error::Error;

pub use self::fs::read_to_string;
pub use self::options::app;
pub use self::options::output_color;
pub use self::options::parse_options;
pub use self::options::{CliOptions, OutputType};
pub use self::variables::parse as parse_variable;
pub use self::variables::parse_value as parse_variable_value;
pub use crate::util::logger::{error_string_no_color, Logger};

mod fs;
pub mod interactive;
mod options;
mod variables;
pub use self::options::{get_strings, has_flag};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliError {
    pub message: String,
}

impl From<Box<dyn Error>> for CliError {
    fn from(e: Box<dyn Error>) -> Self {
        Self {
            message: format!("{:?}", e),
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
        Self {
            message: format!("{:?}", e),
        }
    }
}
