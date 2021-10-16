/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

pub use self::fs::read_to_string;
pub use self::logger::{
    error_string, log_info, make_logger_error_message, make_logger_parser_error,
    make_logger_runner_error, make_logger_verbose,
};
pub use self::options::app;
pub use self::options::output_color;
pub use self::options::parse_options;
pub use self::options::CliOptions;
pub use self::variables::parse as parse_variable;

mod fs;
pub mod interactive;
mod logger;
mod options;
mod variables;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliError {
    pub message: String,
}
