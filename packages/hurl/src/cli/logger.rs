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

use colored::*;
use hurl_core::error::Error;
use hurl_core::parser;

use crate::runner;

pub fn make_logger_verbose(verbose: bool) -> impl Fn(&str) {
    move |message| log_verbose(verbose, message)
}

pub fn make_logger_error_message(color: bool) -> impl Fn(bool, &str) {
    move |warning, message| log_error_message(color, warning, message)
}

pub fn make_logger_parser_error(
    lines: Vec<String>,
    color: bool,
    filename: Option<String>,
) -> impl Fn(&parser::Error, bool) {
    move |error: &parser::Error, warning: bool| {
        log_error(lines.clone(), color, filename.clone(), error, warning)
    }
}

pub fn make_logger_runner_error(
    lines: Vec<String>,
    color: bool,
    filename: Option<String>,
) -> impl Fn(&runner::Error, bool) {
    move |error: &runner::Error, warning: bool| {
        log_error(lines.clone(), color, filename.clone(), error, warning)
    }
}

pub fn log_info(message: &str) {
    eprintln!("{}", message);
}

fn log_error_message(color: bool, warning: bool, message: &str) {
    let log_type = match (color, warning) {
        (false, true) => "warning".to_string(),
        (false, false) => "error".to_string(),
        (true, true) => "warning".yellow().bold().to_string(),
        (true, false) => "error".red().bold().to_string(),
    };
    eprintln!("{}: {}", log_type, message);
}

fn log_verbose(verbose: bool, message: &str) {
    if verbose {
        eprintln!("* {}", message);
    }
}

fn log_error(
    lines: Vec<String>,
    color: bool,
    filename: Option<String>,
    error: &dyn Error,
    warning: bool,
) {
    let error_type = if warning {
        String::from("warning")
    } else {
        String::from("error")
    };
    let error_type = if !color {
        error_type
    } else if warning {
        error_type.yellow().to_string()
    } else {
        error_type.red().to_string()
    };

    let filename = if let Some(filename) = filename {
        filename
    } else {
        "".to_string()
    };
    let error_message = error_string(&lines, filename, error);
    eprintln!("{}: {}\n", error_type, error_message);
}

pub fn error_string(lines: &[String], filename: String, error: &dyn Error) -> String {
    let line_number_size = if lines.len() < 100 {
        2
    } else if lines.len() < 1000 {
        3
    } else {
        4
    };

    let file_info = format!(
        "{}--> {}:{}:{}",
        " ".repeat(line_number_size).as_str(),
        filename,
        error.source_info().start.line,
        error.source_info().start.column,
    );

    let line = lines.get(error.source_info().start.line - 1).unwrap();
    let line = str::replace(line, "\t", "    "); // replace all your tabs with 4 characters

    // TODO: to clean/Refacto
    // specific case for assert errors
    let message = if error.source_info().start.column == 0 {
        let prefix = format!("{} |   ", " ".repeat(line_number_size).as_str());
        let fix_me = &error.fixme();
        add_line_prefix(fix_me, prefix)
    } else {
        let line = lines.get(error.source_info().start.line - 1).unwrap();
        let width = if error.source_info().end.column > error.source_info().start.column {
            (error.source_info().end.column - error.source_info().start.column) as usize
        } else {
            0
        };

        let mut tab_shift = 0;
        for (i, c) in line.chars().enumerate() {
            if i >= error.source_info().start.column - 1 {
                break;
            };
            if c == '\t' {
                tab_shift += 1;
            }
        }
        format!(
            "{} | {}{} {fixme}",
            " ".repeat(line_number_size).as_str(),
            " ".repeat(error.source_info().start.column - 1 + tab_shift * 3),
            "^".repeat(if width > 1 { width } else { 1 }),
            fixme = error.fixme().as_str(),
        )
    };

    format!(
        r#"{description}
{file_info}
{line_number_space} |
{line_number:>width$} |{line}
{message}
{line_number_space} |"#,
        description = error.description(),
        file_info = file_info,
        line_number = error.source_info().start.line,
        width = line_number_size,
        line = if line.is_empty() {
            line
        } else {
            format!(" {}", line)
        },
        message = message,
        line_number_space = " ".repeat(line_number_size)
    )
}

pub fn add_line_prefix(s: &str, prefix: String) -> String {
    let lines: Vec<&str> = regex::Regex::new(r"\n|\r\n").unwrap().split(s).collect();
    lines
        .iter()
        .map(|line| format!("{}{}", prefix, line,))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::runner;
    use hurl_core::ast::SourceInfo;

    #[test]
    fn test_add_line_prefix() {
        assert_eq!(
            add_line_prefix("line1\nline2\nline3", ">".to_string()),
            ">line1\n>line2\n>line3"
        )
    }

    #[test]
    fn test_assert_error_status() {
        let lines = vec![
            "GET http://unknown".to_string(),
            "HTTP/1.0 200".to_string(),
            "".to_string(),
        ];
        let filename = "test.hurl".to_string();
        let error = runner::Error {
            source_info: SourceInfo::init(2, 10, 2, 13),
            inner: runner::RunnerError::AssertStatus {
                actual: "404".to_string(),
            },
            assert: true,
        };
        assert_eq!(
            error_string(&lines, filename, &error),
            r#"Assert Status
  --> test.hurl:2:10
   |
 2 | HTTP/1.0 200
   |          ^^^ actual value is <404>
   |"#
        )
    }

    #[test]
    fn test_invalid_xpath_expression() {
        let lines = vec![
            "GET http://example.com".to_string(),
            "HTTP/1.0 200".to_string(),
            "[Asserts]".to_string(),
            r#"xpath "strong(//head/title)" equals "Hello""#.to_string(),
        ];
        let filename = "test.hurl".to_string();
        let error = runner::Error {
            source_info: SourceInfo::init(4, 7, 4, 29),
            inner: runner::RunnerError::QueryInvalidXpathEval {},
            assert: true,
        };
        assert_eq!(
            error_string(&lines, filename, &error),
            r#"Invalid xpath expression
  --> test.hurl:4:7
   |
 4 | xpath "strong(//head/title)" equals "Hello"
   |       ^^^^^^^^^^^^^^^^^^^^^^ The xpath expression is not valid
   |"#
        )
    }

    #[test]
    fn test_assert_error_jsonpath() {
        let lines = vec![
            "GET http://api".to_string(),
            "HTTP/1.0 200".to_string(),
            "[Asserts]".to_string(),
            r#"jsonpath "$.count" >= 5"#.to_string(),
        ];
        let filename = "test.hurl".to_string();
        let error = runner::Error {
            source_info: SourceInfo::init(4, 0, 4, 0),
            inner: runner::RunnerError::AssertFailure {
                actual: "int <2>".to_string(),
                expected: "greater than int <5>".to_string(),
                type_mismatch: false,
            },
            assert: true,
        };
        assert_eq!(
            error_string(&lines, filename, &error),
            r#"Assert Failure
  --> test.hurl:4:0
   |
 4 | jsonpath "$.count" >= 5
   |   actual:   int <2>
   |   expected: greater than int <5>
   |"#
        )
    }

    #[test]
    fn test_assert_error_newline() {
        let lines = vec![
            "GET http://localhost".to_string(),
            "HTTP/1.0 200".to_string(),
            "```<p>Hello</p>".to_string(),
            "```".to_string(),
        ];
        let filename = "test.hurl".to_string();
        let error = runner::Error {
            source_info: SourceInfo::init(3, 4, 4, 1),
            inner: runner::RunnerError::AssertBodyValueError {
                actual: "<p>Hello</p>\n\n".to_string(),
                expected: "<p>Hello</p>\n".to_string(),
            },
            assert: true,
        };
        assert_eq!(
            error_string(&lines, filename, &error),
            r#"Assert Body Value
  --> test.hurl:3:4
   |
 3 | ```<p>Hello</p>
   |    ^ actual value is <<p>Hello</p>

>
   |"#
        )
    }
}
