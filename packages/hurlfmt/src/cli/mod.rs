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

pub use self::fs::read_to_string;
pub use self::logger::{
    log_info, make_logger_error_message, make_logger_linter_error, make_logger_parser_error,
    make_logger_verbose,
};

mod fs;
mod logger;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliError {
    pub message: String,
}
