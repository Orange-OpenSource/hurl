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

use crate::runner::{HurlResult, Value};
use colored::*;
use hurl_core::error::Error;
use std::cmp::max;

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
                debug: |_| {},
                warning: log_warning,
                error: log_error,
            },
            (false, false) => BaseLogger {
                info: log_info,
                debug: |_| {},
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
    pub debug_curl: fn(&str),
    pub debug_error: fn(&str, &str, &dyn Error),
    pub debug_header_in: fn(&str, &str),
    pub debug_header_out: fn(&str, &str),
    pub debug_important: fn(&str),
    pub debug_method_version_out: fn(&str),
    pub debug_status_version_in: fn(&str),
    pub warning: fn(&str),
    pub error: fn(&str),
    pub error_rich: fn(&str, &str, &dyn Error),
    pub capture: fn(&str, &Value),
    pub test_running: fn(&str, usize, usize),
    pub test_completed: fn(result: &HurlResult),
    pub color: bool,
    pub verbose: bool,
    pub filename: &'a str,
    pub content: &'a str,
}

impl<'a> Logger<'a> {
    /// Creates a new logger.
    pub fn new(color: bool, verbose: bool, filename: &'a str, content: &'a str) -> Logger<'a> {
        match (color, verbose) {
            (true, true) => Logger {
                info: log_info,
                debug: log_debug,
                debug_curl: log_debug_curl,
                debug_error: log_debug_error,
                debug_header_in: log_debug_header_in,
                debug_header_out: log_debug_header_out,
                debug_important: log_debug_important,
                debug_method_version_out: log_debug_method_version_out,
                debug_status_version_in: log_debug_status_version_in,
                warning: log_warning,
                error: log_error,
                error_rich: log_error_rich,
                capture: log_capture,
                test_running: log_test_running,
                test_completed: log_test_completed,
                color,
                verbose,
                filename,
                content,
            },
            (false, true) => Logger {
                info: log_info,
                debug: log_debug_no_color,
                debug_curl: log_debug_curl_no_color,
                debug_error: log_debug_error_no_color,
                debug_header_in: log_debug_header_in_no_color,
                debug_header_out: log_debug_header_out_no_color,
                debug_important: log_debug_no_color,
                debug_method_version_out: log_debug_method_version_out_no_color,
                debug_status_version_in: log_debug_status_version_in_no_color,
                warning: log_warning_no_color,
                error: log_error_no_color,
                error_rich: log_error_rich_no_color,
                capture: log_capture_no_color,
                test_running: log_test_running_no_color,
                test_completed: log_test_completed_no_color,
                color,
                verbose,
                filename,
                content,
            },
            (true, false) => Logger {
                info: log_info,
                debug: |_| {},
                debug_curl: |_| {},
                debug_error: |_, _, _| {},
                debug_header_in: |_, _| {},
                debug_header_out: |_, _| {},
                debug_important: |_| {},
                debug_method_version_out: |_| {},
                debug_status_version_in: |_| {},
                warning: log_warning,
                error: log_error,
                error_rich: log_error_rich,
                capture: |_, _| {},
                test_running: log_test_running,
                test_completed: log_test_completed,
                color,
                verbose,
                filename,
                content,
            },
            (false, false) => Logger {
                info: log_info,
                debug: |_| {},
                debug_curl: |_| {},
                debug_error: |_, _, _| {},
                debug_header_in: |_, _| {},
                debug_header_out: |_, _| {},
                debug_important: |_| {},
                debug_method_version_out: |_| {},
                debug_status_version_in: |_| {},
                warning: log_warning_no_color,
                error: log_error_no_color,
                error_rich: log_error_rich_no_color,
                capture: |_, _| {},
                test_running: log_test_running_no_color,
                test_completed: log_test_completed_no_color,
                color,
                verbose,
                filename,
                content,
            },
        }
    }

    pub fn info(&self, message: &str) {
        (self.info)(message)
    }

    pub fn debug(&self, message: &str) {
        (self.debug)(message)
    }

    pub fn debug_curl(&self, message: &str) {
        (self.debug_curl)(message)
    }

    pub fn debug_error(&self, error: &dyn Error) {
        (self.debug_error)(self.filename, self.content, error)
    }

    pub fn debug_header_in(&self, name: &str, value: &str) {
        (self.debug_header_in)(name, value)
    }

    pub fn debug_header_out(&self, name: &str, value: &str) {
        (self.debug_header_out)(name, value)
    }

    pub fn debug_important(&self, message: &str) {
        (self.debug_important)(message)
    }

    pub fn debug_status_version_in(&self, line: &str) {
        (self.debug_status_version_in)(line)
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
        (self.debug_method_version_out)(line)
    }

    pub fn capture(&self, name: &str, value: &Value) {
        (self.capture)(name, value)
    }

    pub fn test_running(&self, current: usize, total: usize) {
        (self.test_running)(self.filename, current, total)
    }

    pub fn test_completed(&self, result: &HurlResult) {
        (self.test_completed)(result)
    }
}

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

fn log_debug_curl(message: &str) {
    if message.is_empty() {
        eprintln!("{}", "**".blue().bold());
    } else {
        eprintln!("{} {}", "**".blue().bold(), message.green());
    }
}

fn log_debug_curl_no_color(message: &str) {
    if message.is_empty() {
        eprintln!("**");
    } else {
        eprintln!("** {}", message);
    }
}

fn log_debug_important(message: &str) {
    if message.is_empty() {
        eprintln!("{}", "*".blue().bold());
    } else {
        eprintln!("{} {}", "*".blue().bold(), message.bold());
    }
}

fn log_debug_error(filename: &str, content: &str, error: &dyn Error) {
    let message = error_string(filename, content, error, true);
    get_lines(&message).iter().for_each(|l| log_debug(l));
}

fn log_debug_error_no_color(filename: &str, content: &str, error: &dyn Error) {
    let message = error_string(filename, content, error, false);
    get_lines(&message)
        .iter()
        .for_each(|l| log_debug_no_color(l));
}

fn log_debug_header_in(name: &str, value: &str) {
    eprintln!("< {}: {}", name.cyan().bold(), value)
}

fn log_debug_header_in_no_color(name: &str, value: &str) {
    eprintln!("< {}: {}", name, value)
}

fn log_debug_header_out(name: &str, value: &str) {
    eprintln!("> {}: {}", name.cyan().bold(), value)
}

fn log_debug_header_out_no_color(name: &str, value: &str) {
    eprintln!("> {}: {}", name, value)
}

fn log_debug_method_version_out(line: &str) {
    eprintln!("> {}", line.purple().bold())
}

fn log_debug_method_version_out_no_color(line: &str) {
    eprintln!("> {}", line)
}

fn log_debug_status_version_in(line: &str) {
    eprintln!("< {}", line.green().bold())
}

fn log_debug_status_version_in_no_color(line: &str) {
    eprintln!("< {}", line)
}

fn log_warning(message: &str) {
    eprintln!("{}: {}", "warning".yellow().bold(), message.bold());
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

fn log_capture(name: &str, value: &Value) {
    eprintln!("{} {}: {}", "*".blue().bold(), name.yellow().bold(), value)
}

fn log_capture_no_color(name: &str, value: &Value) {
    eprintln!("* {}: {}", name, value)
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

fn log_test_completed(result: &HurlResult) {
    let state = if result.success {
        "Success".green().bold()
    } else {
        "Failure".red().bold()
    };
    let count = result.entries.iter().flat_map(|r| &r.calls).count();
    eprintln!(
        "{}: {} ({} request(s) in {} ms)",
        result.filename.bold(),
        state,
        count,
        result.time_in_ms
    )
}

fn log_test_completed_no_color(result: &HurlResult) {
    let state = if result.success { "Success" } else { "Failure" };
    let count = result.entries.iter().flat_map(|r| &r.calls).count();
    eprintln!(
        "{}: {} ({} request(s) in {} ms)",
        result.filename, state, count, result.time_in_ms
    )
}

pub fn error_string_no_color(filename: &str, content: &str, error: &dyn Error) -> String {
    error_string(filename, content, error, false)
}

/// Returns an `error` as a string, given `lines` of content and a `filename`.
fn error_string(filename: &str, content: &str, error: &dyn Error, colored: bool) -> String {
    let lines = get_lines(content);
    let line_number_size = max(lines.len().to_string().len(), 2);
    let arrow = if colored {
        "-->".blue().bold().to_string()
    } else {
        "-->".to_string()
    };
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
    let separator = if colored {
        "|".blue().bold().to_string()
    } else {
        "|".to_string()
    };

    // TODO: to clean/Refacto
    // specific case for assert errors
    let message = if column_number == 0 {
        let prefix = format!("{} {}   ", " ".repeat(line_number_size).as_str(), separator);
        let fix_me = error.fixme();
        add_line_prefix(&fix_me, &prefix, colored)
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
        let mut underline = "^".repeat(if width > 1 { width } else { 1 });
        if colored {
            underline = underline.red().bold().to_string();
        }

        let mut fix_me = error.fixme();
        if colored {
            fix_me = fix_me.red().bold().to_string();
        }
        format!(
            "{} {} {}{} {fixme}",
            " ".repeat(line_number_size).as_str(),
            separator,
            " ".repeat(column_number - 1 + tab_shift * 3),
            underline,
            fixme = fix_me,
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

fn add_line_prefix(s: &str, prefix: &str, colored: bool) -> String {
    get_lines(s)
        .iter()
        .map(|line| {
            if colored {
                format!("{}{}", prefix, line.red().bold())
            } else {
                format!("{}{}", prefix, line)
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_lines(text: &str) -> Vec<&str> {
    regex::Regex::new(r"\n|\r\n").unwrap().split(text).collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::runner;
    use hurl_core::ast::SourceInfo;

    #[test]
    fn test_add_line_prefix_no_colored() {
        assert_eq!(
            add_line_prefix("line1\nline2\nline3", ">", false),
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
            source_info: SourceInfo::new(2, 10, 2, 13),
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
            source_info: SourceInfo::new(4, 7, 4, 29),
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
            source_info: SourceInfo::new(4, 0, 4, 0),
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
            source_info: SourceInfo::new(3, 4, 4, 1),
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
