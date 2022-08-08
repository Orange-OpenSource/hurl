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
    pub warning: fn(&str),
    pub error: fn(&str),
}

impl BaseLogger {
    pub fn new(color: bool, verbose: bool) -> BaseLogger {
        match (color, verbose) {
            (true, true) => BaseLogger {
                info: log_info,
                debug: log_debug,
                warning: log_warning,
                error: log_error,
            },
            (false, true) => BaseLogger {
                info: log_info,
                debug: log_debug_no_color,
                warning: log_warning_no_color,
                error: log_error_no_color,
            },
            (true, false) => BaseLogger {
                info: log_info,
                debug: nop1,
                warning: log_warning,
                error: log_error,
            },
            (false, false) => BaseLogger {
                info: log_info,
                debug: nop1,
                warning: log_warning_no_color,
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

    pub fn warning(&self, message: &str) {
        (self.warning)(message)
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
    pub debug_important: fn(&str),
    pub warning: fn(&str),
    pub error: fn(&str),
    pub error_rich: fn(&str, &str, &dyn Error),
    pub method_version_out: fn(&str),
    pub status_version_in: fn(&str),
    pub header_in: fn(&str, &str),
    pub header_out: fn(&str, &str),
    pub test_running: fn(&str, usize, usize),
    pub test_completed: fn(&str, bool),
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
                debug_important: log_debug_important,
                warning: log_warning,
                error: log_error,
                error_rich: log_error_rich,
                method_version_out: log_method_version_out,
                status_version_in: log_status_version_in,
                header_in: log_header_in,
                header_out: log_header_out,
                test_running: log_test_running,
                test_completed: log_test_completed,
                content,
                filename,
            },
            (false, true) => Logger {
                info: log_info,
                debug: log_debug_no_color,
                debug_important: log_debug_no_color,
                warning: log_warning_no_color,
                error: log_error_no_color,
                error_rich: log_error_rich_no_color,
                method_version_out: log_method_version_out_no_color,
                status_version_in: log_status_version_in_no_color,
                header_in: log_header_in_no_color,
                header_out: log_header_out_no_color,
                test_running: log_test_running_no_color,
                test_completed: log_test_completed_no_color,
                content,
                filename,
            },
            (true, false) => Logger {
                info: log_info,
                debug: nop1,
                debug_important: nop1,
                warning: log_warning,
                error: log_error,
                error_rich: log_error_rich,
                method_version_out: nop1,
                status_version_in: nop1,
                header_in: nop2,
                header_out: nop2,
                test_running: log_test_running,
                test_completed: log_test_completed,
                content,
                filename,
            },
            (false, false) => Logger {
                info: log_info,
                debug: nop1,
                debug_important: nop1,
                warning: log_warning_no_color,
                error: log_error_no_color,
                error_rich: log_error_rich_no_color,
                method_version_out: nop1,
                status_version_in: nop1,
                header_in: nop2,
                header_out: nop2,
                test_running: log_test_running_no_color,
                test_completed: log_test_completed_no_color,
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

    pub fn debug_important(&self, message: &str) {
        (self.debug_important)(message)
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

    pub fn method_version_out(&self, line: &str) {
        (self.method_version_out)(line)
    }

    pub fn status_version_in(&self, line: &str) {
        (self.status_version_in)(line)
    }

    pub fn header_in(&self, name: &str, value: &str) {
        (self.header_in)(name, value)
    }

    pub fn header_out(&self, name: &str, value: &str) {
        (self.header_out)(name, value)
    }

    pub fn test_running(&self, current: usize, total: usize) {
        (self.test_running)(self.filename, current, total)
    }

    pub fn test_completed(&self, success: bool) {
        (self.test_completed)(self.filename, success)
    }
}

fn nop1(_one: &str) {}

fn nop2(_one: &str, _two: &str) {}

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

fn log_debug_important(message: &str) {
    if message.is_empty() {
        eprintln!("{}", "*".blue().bold());
    } else {
        eprintln!("{} {}", "*".blue().bold(), message.bold());
    }
}

fn log_warning(message: &str) {
    eprintln!("{}: {}", "warning".yellow().bold(), message);
}

fn log_warning_no_color(message: &str) {
    eprintln!("warning: {}", message);
}

fn log_error(message: &str) {
    eprintln!("{}: {}", "error".red().bold(), message.bold());
}

fn log_error_no_color(message: &str) {
    eprintln!("error: {}", message);
}

fn log_error_rich(filename: &str, content: &str, error: &dyn Error) {
    let message = error_string(filename, content, error, true);
    eprintln!("{}: {}\n", "error".red().bold(), &message)
}

fn log_error_rich_no_color(filename: &str, content: &str, error: &dyn Error) {
    let message = error_string(filename, content, error, false);
    eprintln!("error: {}\n", &message)
}

fn log_method_version_out(line: &str) {
    eprintln!("> {}", line.purple().bold())
}

fn log_method_version_out_no_color(line: &str) {
    eprintln!("> {}", line)
}

fn log_status_version_in(line: &str) {
    eprintln!("< {}", line.green().bold())
}

fn log_status_version_in_no_color(line: &str) {
    eprintln!("< {}", line)
}

fn log_header_in(name: &str, value: &str) {
    eprintln!("< {}: {}", name.cyan().bold(), value)
}

fn log_header_in_no_color(name: &str, value: &str) {
    eprintln!("< {}: {}", name, value)
}

fn log_header_out(name: &str, value: &str) {
    eprintln!("> {}: {}", name.cyan().bold(), value)
}

fn log_header_out_no_color(name: &str, value: &str) {
    eprintln!("> {}: {}", name, value)
}

fn log_test_running(filename: &str, current: usize, total: usize) {
    eprintln!(
        "{}: {} [{}/{}]",
        filename.bold(),
        "Running".cyan().bold(),
        current,
        total
    )
}

fn log_test_running_no_color(filename: &str, current: usize, total: usize) {
    eprintln!("{}: Running [{}/{}]", filename, current, total)
}

fn log_test_completed(filename: &str, success: bool) {
    let state = if success {
        "Success".green().bold()
    } else {
        "Failure".red().bold()
    };
    eprintln!("{}: {}", filename.bold(), state)
}

fn log_test_completed_no_color(filename: &str, success: bool) {
    let state = if success { "Success" } else { "Failure" };
    eprintln!("{}: {}", filename, state)
}

pub fn error_string_no_color(filename: &str, content: &str, error: &dyn Error) -> String {
    error_string(filename, content, error, false)
}

/// Returns an `error` as a string, given `lines` of content and a `filename`.
fn error_string(filename: &str, content: &str, error: &dyn Error, colored: bool) -> String {
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

    let mut arrow = "-->".to_string();
    if colored {
        arrow = arrow.blue().bold().to_string()
    }
    let line_number = error.source_info().start.line;
    let column_number = error.source_info().start.column;

    let file_info = format!(
        "{}{} {}:{}:{}",
        " ".repeat(line_number_size).as_str(),
        arrow,
        filename,
        line_number,
        column_number,
    );

    let line = lines.get(line_number - 1).unwrap();
    let line = str::replace(line, "\t", "    "); // replace all your tabs with 4 characters
    let mut separator = "|".to_string();
    if colored {
        separator = separator.blue().bold().to_string()
    }

    // TODO: to clean/Refacto
    // specific case for assert errors
    let message = if column_number == 0 {
        let prefix = format!("{} {}   ", " ".repeat(line_number_size).as_str(), separator);
        let fix_me = &error.fixme();
        add_line_prefix(fix_me, prefix)
    } else {
        let line = lines.get(line_number - 1).unwrap();
        let width = if error.source_info().end.column > column_number {
            (error.source_info().end.column - column_number) as usize
        } else {
            0
        };

        let mut tab_shift = 0;
        for (i, c) in line.chars().enumerate() {
            if i >= column_number - 1 {
                break;
            };
            if c == '\t' {
                tab_shift += 1;
            }
        }
        format!(
            "{} {} {}{} {fixme}",
            " ".repeat(line_number_size).as_str(),
            separator,
            " ".repeat(column_number - 1 + tab_shift * 3),
            "^".repeat(if width > 1 { width } else { 1 }),
            fixme = error.fixme().as_str(),
        )
    };

    let description = if colored {
        error.description().bold().to_string()
    } else {
        error.description()
    };

    let width = line_number_size;
    let mut line_number = format!(
        "{line_number:>width$}",
        line_number = line_number,
        width = width
    );
    if colored {
        line_number = line_number.blue().bold().to_string();
    }
    let line = if line.is_empty() {
        line
    } else {
        format!(" {}", line)
    };
    let message = if colored {
        message.red().bold().to_string()
    } else {
        message
    };

    format!(
        r#"{description}
{file_info}
{line_number_space} {separator}
{line_number} {separator}{line}
{message}
{line_number_space} {separator}"#,
        description = description,
        file_info = file_info,
        line_number = line_number,
        line = line,
        message = message,
        line_number_space = " ".repeat(line_number_size),
        separator = separator
    )
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
            error_string(filename, content, &error, false),
            r#"Assert status code
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
            error_string(filename, content, &error, false),
            r#"Invalid XPath expression
  --> test.hurl:4:7
   |
 4 | xpath "strong(//head/title)" equals "Hello"
   |       ^^^^^^^^^^^^^^^^^^^^^^ the XPath expression is not valid
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
            error_string(filename, content, &error, false),
            r#"Assert failure
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
            error_string(filename, content, &error, false),
            r#"Assert body value
  --> test.hurl:3:4
   |
 3 | ```<p>Hello</p>
   |    ^ actual value is <<p>Hello</p>

>
   |"#
        )
    }
}
