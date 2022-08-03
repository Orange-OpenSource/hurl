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

/// A simple logger to log app related event (start, high levels error, etc...).
/// When we run an [`hurl_core::ast::HurlFile`], user has to provide a dedicated Hurl logger (see [`Logger`]).
pub struct BaseLogger {
    pub info: fn(&str),
    pub debug: fn(&str),
    pub error: fn(&str),
}

impl BaseLogger {
    pub fn new(color: bool, verbose: bool) -> BaseLogger {
        match (color, verbose) {
            (true, true) => BaseLogger {
                info: log_info,
                debug: log_debug,
                error: log_error,
            },
            (false, true) => BaseLogger {
                info: log_info,
                debug: log_debug_no_color,
                error: log_error_no_color,
            },
            (true, false) => BaseLogger {
                info: log_info,
                debug: nop,
                error: log_error,
            },
            (false, false) => BaseLogger {
                info: log_info,
                debug: nop,
                error: log_error_no_color,
            },
        }
    }

    pub fn info(&self, message: &str) {
        (self.info)(message)
    }

    pub fn debug(&self, message: &str) {
        (self.debug)(message)
    }

    pub fn error(&self, message: &str) {
        (self.error)(message)
    }
}

/// A Hurl dedicated logger for an Hurl file. Contrary to [`BaseLogger`], this logger can display
/// rich error for parsing and runtime errors. As the rich errors can display user content,
/// this logger should have access to the content of the file being run.
pub struct Logger<'a> {
    pub info: fn(&str),
    pub debug: fn(&str),
    pub warning: fn(&str),
    pub error: fn(&str),
    pub error_rich: fn(&str, &str, &dyn Error),
    pub content: &'a str,
    pub filename: &'a str,
}

impl<'a> Logger<'a> {
    /// Creates a new logger.
    pub fn new(color: bool, verbose: bool, filename: &'a str, content: &'a str) -> Logger<'a> {
        match (color, verbose) {
            (true, true) => Logger {
                info: log_info,
                debug: log_debug,
                warning: log_warning,
                error: log_error,
                error_rich: log_error_rich,
                content,
                filename,
            },
            (false, true) => Logger {
                info: log_info,
                debug: log_debug_no_color,
                warning: log_warning_no_color,
                error: log_error_no_color,
                error_rich: log_error_rich_no_color,
                content,
                filename,
            },
            (true, false) => Logger {
                info: log_info,
                debug: nop,
                warning: log_warning,
                error: log_error,
                error_rich: log_error_rich,
                content,
                filename,
            },
            (false, false) => Logger {
                info: log_info,
                debug: nop,
                warning: log_warning_no_color,
                error: log_error_no_color,
                error_rich: log_error_rich_no_color,
                content,
                filename,
            },
        }
    }

    pub fn info(&self, message: &str) {
        (self.info)(message)
    }

    pub fn debug(&self, message: &str) {
        (self.debug)(message)
    }

    pub fn warning(&self, message: &str) {
        (self.warning)(message)
    }

    pub fn error(&self, message: &str) {
        (self.error)(message)
    }

    pub fn error_rich(&self, error: &dyn Error) {
        (self.error_rich)(self.filename, self.content, error)
    }
}

fn nop(_message: &str) {}

fn log_info(message: &str) {
    eprintln!("{}", message);
}

fn log_debug(message: &str) {
    if message.is_empty() {
        eprintln!("{}", "*".blue().bold());
    } else {
        eprintln!("{} {}", "*".blue().bold(), message);
    }
}

fn log_debug_no_color(message: &str) {
    if message.is_empty() {
        eprintln!("*");
    } else {
        eprintln!("* {}", message);
    }
}

fn log_warning(message: &str) {
    eprintln!("{}: {}", "warning".yellow().bold(), message);
}

fn log_warning_no_color(message: &str) {
    eprintln!("warning: {}", message);
}

fn log_error(message: &str) {
    eprintln!("{}: {}", "error".red().bold(), message);
}

fn log_error_no_color(message: &str) {
    eprintln!("error: {}", message);
}

fn log_error_rich(filename: &str, content: &str, error: &dyn Error) {
    let message = error_string(filename, content, error);
    log_error(format!("{}\n", &message).as_str())
}

fn log_error_rich_no_color(filename: &str, content: &str, error: &dyn Error) {
    let message = error_string_no_color(filename, content, error);
    log_error_no_color(format!("{}\n", &message).as_str())
}

/// Returns an `error` as a string, given `lines` of content and a `filename`.
pub fn error_string(filename: &str, content: &str, error: &dyn Error) -> String {
    let lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(content)
        .collect();

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

pub fn error_string_no_color(filename: &str, content: &str, error: &dyn Error) -> String {
    error_string(filename, content, error)
}

fn add_line_prefix(s: &str, prefix: String) -> String {
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
        let content = r#"GET http://unknown
HTTP/1.0 200
"#;
        let filename = "test.hurl";
        let error = runner::Error {
            source_info: SourceInfo::init(2, 10, 2, 13),
            inner: runner::RunnerError::AssertStatus {
                actual: "404".to_string(),
            },
            assert: true,
        };
        assert_eq!(
            error_string(filename, content, &error),
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
        let content = r#"GET http://example.com
HTTP/1.0 200
[Asserts]
xpath "strong(//head/title)" equals "Hello"
"#;
        let filename = "test.hurl";
        let error = runner::Error {
            source_info: SourceInfo::init(4, 7, 4, 29),
            inner: runner::RunnerError::QueryInvalidXpathEval {},
            assert: true,
        };
        assert_eq!(
            error_string(filename, content, &error),
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
        let content = r#"GET http://api
HTTP/1.0 200
[Asserts]
jsonpath "$.count" >= 5
"#;
        let filename = "test.hurl";
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
            error_string(filename, content, &error),
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
        let content = r#"GET http://localhost
HTTP/1.0 200
```<p>Hello</p>
```
"#;
        let filename = "test.hurl";
        let error = runner::Error {
            source_info: SourceInfo::init(3, 4, 4, 1),
            inner: runner::RunnerError::AssertBodyValueError {
                actual: "<p>Hello</p>\n\n".to_string(),
                expected: "<p>Hello</p>\n".to_string(),
            },
            assert: true,
        };
        assert_eq!(
            error_string(filename, content, &error),
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
