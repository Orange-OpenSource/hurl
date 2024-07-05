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
use std::path::PathBuf;

use crate::linter;
use colored::*;
use hurl_core::error::{DisplaySourceError, OutputFormat};
use hurl_core::parser;
use hurl_core::text::{Format, Style, StyledString};

pub fn make_logger_verbose(verbose: bool) -> impl Fn(&str) {
    move |message| log_verbose(verbose, message)
}

pub fn make_logger_error_message(color: bool) -> impl Fn(bool, &str) {
    move |warning, message| log_error_message(color, warning, message)
}

pub fn make_logger_parser_error(
    lines: Vec<String>,
    color: bool,
    filename: Option<PathBuf>,
) -> impl Fn(&parser::ParseError, bool) {
    move |error: &parser::ParseError, warning: bool| {
        let filename = match &filename {
            None => "-".to_string(),
            Some(value) => value.display().to_string(),
        };
        let content = lines.join("\n");
        let message = error.to_string(&filename, &content, None, OutputFormat::Terminal(color));
        eprintln!("{}: {}\n", get_prefix(warning, color), message);
    }
}

pub fn make_logger_linter_error(
    lines: Vec<String>,
    color: bool,
    filename: Option<PathBuf>,
) -> impl Fn(&linter::Error, bool) {
    move |error: &linter::Error, warning: bool| {
        let filename = match &filename {
            None => "-".to_string(),
            Some(value) => value.display().to_string(),
        };
        let content = lines.join("\n");
        let message = error.to_string(&filename, &content, None, OutputFormat::Terminal(color));
        eprintln!("{}: {}\n", get_prefix(warning, color), message);
    }
}
fn get_prefix(warning: bool, color: bool) -> String {
    let mut message = StyledString::new();
    if warning {
        message.push_with("warning", Style::new().yellow().bold());
    } else {
        message.push_with("error", Style::new().red());
    };
    let fmt = if color { Format::Ansi } else { Format::Plain };
    message.to_string(fmt)
}

pub fn log_info(message: &str) {
    eprintln!("{message}");
}

fn log_error_message(color: bool, warning: bool, message: &str) {
    let log_type = match (color, warning) {
        (false, true) => "warning".to_string(),
        (false, false) => "error".to_string(),
        (true, true) => "warning".yellow().bold().to_string(),
        (true, false) => "error".red().bold().to_string(),
    };
    eprintln!("{log_type}: {message}");
}

fn log_verbose(verbose: bool, message: &str) {
    if verbose {
        if message.is_empty() {
            eprintln!("*");
        } else {
            eprintln!("* {message}");
        }
    }
}
