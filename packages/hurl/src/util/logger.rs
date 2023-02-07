/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
    pub color: bool,
    pub verbose: bool,
}

impl BaseLogger {
    pub fn new(color: bool, verbose: bool) -> BaseLogger {
        BaseLogger { color, verbose }
    }

    pub fn info(&self, message: &str) {
        log_info(message)
    }

    pub fn debug(&self, message: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug(message)
        } else {
            log_debug_no_color(message)
        }
    }

    pub fn warning(&self, message: &str) {
        if self.color {
            log_warning(message)
        } else {
            log_warning_no_color(message)
        }
    }

    pub fn error(&self, message: &str) {
        if self.color {
            log_error(message)
        } else {
            log_error_no_color(message)
        }
    }
}

/// A Hurl dedicated logger for an Hurl file. Contrary to [`BaseLogger`], this logger can display
/// rich error for parsing and runtime errors. As the rich errors can display user content,
/// this logger should have access to the content of the file being run.
pub struct Logger<'a> {
    pub(crate) color: bool,
    pub(crate) verbose: bool,
    pub(crate) progress_bar: bool,
    pub(crate) test: bool,
    pub(crate) filename: &'a str,
    pub(crate) content: &'a str,
}

#[derive(Default)]
pub struct LoggerBuilder<'a> {
    color: bool,
    verbose: bool,
    progress_bar: bool,
    test: bool,
    filename: Option<&'a str>,
    content: Option<&'a str>,
}

impl<'a> LoggerBuilder<'a> {
    /// Returns a new Logger builder with a default values.
    pub fn new() -> Self {
        LoggerBuilder::default()
    }

    /// Sets color usage.
    pub fn color(&mut self, color: bool) -> &mut Self {
        self.color = color;
        self
    }

    /// Sets verbose logger.
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Sets the filename used to display error, warning etc...
    pub fn filename(&mut self, filename: &'a str) -> &mut Self {
        self.filename = Some(filename);
        self
    }

    /// Sets the content used to display error, warning etc...
    pub fn content(&mut self, content: &'a str) -> &mut Self {
        self.content = Some(content);
        self
    }

    /// Sets progress bar.
    pub fn progress_bar(&mut self, progress_bar: bool) -> &mut Self {
        self.progress_bar = progress_bar;
        self
    }

    /// Sets test.
    pub fn test(&mut self, test: bool) -> &mut Self {
        self.test = test;
        self
    }

    /// Creates a new logger.
    pub fn build(&self) -> Result<Logger, &'static str> {
        if self.filename.is_none() {
            return Err("filename is not set");
        }
        if self.content.is_none() {
            return Err("content is not set");
        }

        Ok(Logger {
            color: self.color,
            verbose: self.verbose,
            progress_bar: self.progress_bar,
            test: self.test,
            filename: self.filename.unwrap(),
            content: self.content.unwrap(),
        })
    }
}

impl<'a> Logger<'a> {
    pub fn info(&self, message: &str) {
        log_info(message)
    }

    pub fn debug(&self, message: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug(message)
        } else {
            log_debug_no_color(message)
        }
    }

    pub fn debug_curl(&self, message: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug_curl(message)
        } else {
            log_debug_curl_no_color(message)
        }
    }

    pub fn debug_error(&self, error: &dyn Error) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug_error(self.filename, self.content, error)
        } else {
            log_debug_error_no_color(self.filename, self.content, error)
        }
    }

    pub fn debug_header_in(&self, name: &str, value: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug_header_in(name, value)
        } else {
            log_debug_header_in_no_color(name, value)
        }
    }

    pub fn debug_header_out(&self, name: &str, value: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug_header_out(name, value)
        } else {
            log_debug_header_out_no_color(name, value)
        }
    }

    pub fn debug_important(&self, message: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug_important(message)
        } else {
            log_debug_no_color(message)
        }
    }

    pub fn debug_status_version_in(&self, line: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug_status_version_in(line)
        } else {
            log_debug_status_version_in_no_color(line)
        }
    }

    pub fn warning(&self, message: &str) {
        if self.color {
            log_warning(message)
        } else {
            log_warning_no_color(message)
        }
    }

    pub fn error(&self, message: &str) {
        if self.color {
            log_error(message)
        } else {
            log_error_no_color(message)
        }
    }

    pub fn error_rich(&self, error: &dyn Error) {
        if self.color {
            log_error_rich(self.filename, self.content, error)
        } else {
            log_error_rich_no_color(self.filename, self.content, error)
        }
    }

    pub fn debug_method_version_out(&self, line: &str) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_debug_method_version_out(line)
        } else {
            log_debug_method_version_out_no_color(line)
        }
    }

    pub fn capture(&self, name: &str, value: &Value) {
        if !self.verbose {
            return;
        }
        if self.color {
            log_capture(name, value)
        } else {
            log_capture_no_color(name, value)
        }
    }

    pub fn test_running(&self, current: usize, total: usize) {
        if !self.test {
            return;
        }
        if self.color {
            log_test_running(self.filename, current, total)
        } else {
            log_test_running_no_color(self.filename, current, total)
        }
    }

    pub fn test_progress(&self, entry_index: usize, count: usize) {
        if !self.progress_bar {
            return;
        }
        log_test_progress(entry_index, count)
    }

    pub fn test_completed(&self, result: &HurlResult) {
        if !self.test {
            return;
        }
        if self.color {
            log_test_completed(result)
        } else {
            log_test_completed_no_color(result)
        }
    }

    pub fn test_erase_line(&self) {
        if !self.progress_bar {
            return;
        }
        // This is the "EL - Erase in Line" sequence. It clears from the cursor
        // to the end of line.
        // https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_sequences
        eprint!("\x1B[K");
    }
}

fn log_info(message: &str) {
    eprintln!("{message}");
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
        eprintln!("* {message}");
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
        eprintln!("** {message}");
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
    eprintln!("< {name}: {value}")
}

fn log_debug_header_out(name: &str, value: &str) {
    eprintln!("> {}: {}", name.cyan().bold(), value)
}

fn log_debug_header_out_no_color(name: &str, value: &str) {
    eprintln!("> {name}: {value}")
}

fn log_debug_method_version_out(line: &str) {
    eprintln!("> {}", line.purple().bold())
}

fn log_debug_method_version_out_no_color(line: &str) {
    eprintln!("> {line}")
}

fn log_debug_status_version_in(line: &str) {
    eprintln!("< {}", line.green().bold())
}

fn log_debug_status_version_in_no_color(line: &str) {
    eprintln!("< {line}")
}

fn log_warning(message: &str) {
    eprintln!("{}: {}", "warning".yellow().bold(), message.bold());
}

fn log_warning_no_color(message: &str) {
    eprintln!("warning: {message}");
}

fn log_error(message: &str) {
    eprintln!("{}: {}", "error".red().bold(), message.bold());
}

fn log_error_no_color(message: &str) {
    eprintln!("error: {message}");
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
    eprintln!("* {name}: {value}")
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
    eprintln!("{filename}: Running [{current}/{total}]")
}

fn log_test_progress(entry_index: usize, count: usize) {
    let progress = progress_string(entry_index, count);
    eprint!(" {progress}\r");
}

/// Returns the progress string with the current entry at `entry_index`.
fn progress_string(entry_index: usize, count: usize) -> String {
    const WIDTH: usize = 24;
    // We report the number of entries already processed.
    let progress = (entry_index - 1) as f64 / count as f64;
    let col = (progress * WIDTH as f64) as usize;
    let completed = if col > 0 {
        "=".repeat(col)
    } else {
        "".to_string()
    };
    let void = " ".repeat(WIDTH - col - 1);
    format!("[{completed}>{void}] {entry_index}/{count}")
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
            error.source_info().end.column - column_number
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
    let mut line_number = format!("{line_number:>width$}");
    if colored {
        line_number = line_number.blue().bold().to_string();
    }
    let line = if line.is_empty() {
        line
    } else {
        format!(" {line}")
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
                format!("{prefix}{line}")
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

    #[rustfmt::skip]
    #[test]
    fn test_progress_string() {
        // Progress strings with 20 entries:
        assert_eq!(progress_string(1,  20), "[>                       ] 1/20");
        assert_eq!(progress_string(2,  20), "[=>                      ] 2/20");
        assert_eq!(progress_string(5,  20), "[====>                   ] 5/20");
        assert_eq!(progress_string(10, 20), "[==========>             ] 10/20");
        assert_eq!(progress_string(15, 20), "[================>       ] 15/20");
        assert_eq!(progress_string(20, 20), "[======================> ] 20/20");

        // Progress strings with 3 entries:
        assert_eq!(progress_string(1, 3), "[>                       ] 1/3");
        assert_eq!(progress_string(2, 3), "[========>               ] 2/3");
        assert_eq!(progress_string(3, 3), "[================>       ] 3/3");

        // Progress strings with 1 entries:
        assert_eq!(progress_string(1, 1), "[>                       ] 1/1");
    }
}
