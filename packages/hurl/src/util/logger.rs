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
use std::cmp::max;

use colored::*;
use hurl_core::ast::SourceInfo;
use hurl_core::error::DisplaySourceError;

use crate::runner::Value;
use crate::util::term::Stderr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorFormat {
    Short,
    Long,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Verbosity {
    Verbose,
    VeryVerbose,
}

impl Verbosity {
    pub fn from(verbose: bool, very_verbose: bool) -> Option<Verbosity> {
        match (verbose, very_verbose) {
            (_, true) => Some(Verbosity::VeryVerbose),
            (true, false) => Some(Verbosity::Verbose),
            _ => None,
        }
    }
}

/// A dedicated logger for an Hurl file. This logger can display rich parsing and runtime errors.
#[derive(Clone)]
pub struct Logger {
    pub(crate) color: bool,
    pub(crate) error_format: ErrorFormat,
    pub(crate) filename: String,
    pub(crate) verbosity: Option<Verbosity>,
    pub(crate) stderr: Stderr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoggerOptions {
    pub(crate) color: bool,
    pub(crate) error_format: ErrorFormat,
    pub(crate) filename: String,
    pub(crate) verbosity: Option<Verbosity>,

    // For --test reporting, will be cleaned later
    pub(crate) test: bool,
    pub(crate) progress_bar: bool,

    /// 0-based index of the running file in the total list files
    pub(crate) current_file: usize,
    /// Total number of files of this run
    pub(crate) total_files: usize,
}

pub struct LoggerOptionsBuilder {
    color: bool,
    error_format: ErrorFormat,
    filename: String,
    verbosity: Option<Verbosity>,

    test: bool,
    progress_bar: bool,
    current_file: usize,
    total_files: usize,
}

impl LoggerOptionsBuilder {
    /// Returns a new Logger builder with a default values.
    pub fn new() -> Self {
        LoggerOptionsBuilder::default()
    }

    /// Sets color usage.
    pub fn color(&mut self, color: bool) -> &mut Self {
        self.color = color;
        self
    }

    /// Control the format of error messages.
    /// If `error_format` is [`ErrorFormat::Long`], the HTTP request and response that has
    /// errors is displayed (headers, body, etc..)
    pub fn error_format(&mut self, error_format: ErrorFormat) -> &mut Self {
        self.error_format = error_format;
        self
    }

    /// Sets verbose logger.
    pub fn verbosity(&mut self, verbosity: Option<Verbosity>) -> &mut Self {
        self.verbosity = verbosity;
        self
    }

    /// Sets test mode.
    pub fn test(&mut self, test: bool) -> &mut Self {
        self.test = test;
        self
    }

    /// Sets progress bar.
    pub fn progress_bar(&mut self, progress_bar: bool) -> &mut Self {
        self.progress_bar = progress_bar;
        self
    }

    /// Sets filename.
    pub fn filename(&mut self, filename: &str) -> &mut Self {
        self.filename = filename.to_string();
        self
    }

    /// Set the 0-based index of the running file in the total list files.
    pub fn current_file(&mut self, current: usize) -> &mut Self {
        self.current_file = current;
        self
    }

    /// Set total number of files of this run.
    pub fn total_files(&mut self, total: usize) -> &mut Self {
        self.total_files = total;
        self
    }

    /// Creates a new logger.
    pub fn build(&self) -> LoggerOptions {
        LoggerOptions {
            color: self.color,
            error_format: self.error_format,
            filename: self.filename.clone(),
            verbosity: self.verbosity,
            test: self.test,
            progress_bar: self.progress_bar,
            current_file: self.current_file,
            total_files: self.total_files,
        }
    }
}

impl Default for LoggerOptionsBuilder {
    fn default() -> Self {
        LoggerOptionsBuilder {
            color: false,
            error_format: ErrorFormat::Short,
            filename: String::new(),
            test: false,
            progress_bar: false,
            verbosity: None,
            current_file: 0,
            total_files: 0,
        }
    }
}

impl Logger {
    /// Creates a new instance.
    pub fn new(options: &LoggerOptions, term: Stderr) -> Self {
        Logger {
            color: options.color,
            error_format: options.error_format,
            filename: options.filename.to_string(),
            verbosity: options.verbosity,
            stderr: term,
        }
    }

    pub fn info(&mut self, message: &str) {
        self.stderr.eprintln(message);
    }

    pub fn debug(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            let prefix = "*".blue().bold().to_string();
            self.stderr.eprintln_prefix(&prefix, message);
        } else {
            self.stderr.eprintln_prefix("*", message);
        }
    }

    pub fn debug_important(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            let prefix = "*".blue().bold().to_string();
            let message = message.bold().to_string();
            self.stderr.eprintln_prefix(&prefix, &message);
        } else {
            self.stderr.eprintln_prefix("*", message);
        }
    }

    pub fn debug_curl(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            let prefix = "**".blue().bold().to_string();
            self.stderr.eprintln_prefix(&prefix, message);
        } else {
            self.stderr.eprintln_prefix("**", message);
        }
    }

    pub fn debug_error<E: DisplaySourceError>(
        &mut self,
        content: &str,
        error: &E,
        entry_src_info: SourceInfo,
    ) {
        if self.verbosity.is_none() {
            return;
        }
        let message = error_string(
            &self.filename,
            content,
            error,
            Some(entry_src_info),
            self.color,
        );
        split_lines(&message).iter().for_each(|l| self.debug(l));
    }

    pub fn debug_headers_in(&mut self, headers: &[(&str, &str)]) {
        if self.verbosity.is_none() {
            return;
        }
        for (name, value) in headers {
            if self.color {
                self.stderr
                    .eprintln(&format!("< {}: {}", name.cyan().bold(), value));
            } else {
                self.stderr.eprintln(&format!("< {}: {}", name, value));
            }
        }
        self.stderr.eprintln("<");
    }

    pub fn debug_headers_out(&mut self, headers: &[(&str, &str)]) {
        if self.verbosity.is_none() {
            return;
        }
        for (name, value) in headers {
            if self.color {
                self.stderr
                    .eprintln(&format!("> {}: {}", name.cyan().bold(), value));
            } else {
                self.stderr.eprintln(&format!("> {}: {}", name, value));
            }
        }
        self.stderr.eprintln(">");
    }

    pub fn debug_status_version_in(&mut self, line: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            self.stderr.eprintln(&format!("< {}", line.green().bold()));
        } else {
            self.stderr.eprintln(&format!("< {line}"));
        }
    }

    pub fn warning(&mut self, message: &str) {
        if self.color {
            self.stderr.eprintln(&format!(
                "{}: {}",
                "warning".yellow().bold(),
                message.bold()
            ));
        } else {
            self.stderr.eprintln(&format!("warning: {message}"));
        }
    }

    pub fn error(&mut self, message: &str) {
        if self.color {
            self.stderr
                .eprintln(&format!("{}: {}", "error".red().bold(), message.bold()));
        } else {
            self.stderr.eprintln(&format!("error: {message}"));
        }
    }

    pub fn error_parsing_rich<E: DisplaySourceError>(&mut self, content: &str, error: &E) {
        let message = error_string(&self.filename, content, error, None, self.color);
        self.error_rich(&message);
    }

    pub fn error_runtime_rich<E: DisplaySourceError>(
        &mut self,
        content: &str,
        error: &E,
        entry_src_info: SourceInfo,
    ) {
        let message = error_string(
            &self.filename,
            content,
            error,
            Some(entry_src_info),
            self.color,
        );
        self.error_rich(&message);
    }

    fn error_rich(&mut self, message: &str) {
        if self.color {
            self.stderr
                .eprintln(&format!("{}: {message}\n", "error".red().bold()));
        } else {
            self.stderr.eprintln(&format!("error: {message}\n"));
        }
    }

    pub fn debug_method_version_out(&mut self, line: &str) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            self.stderr.eprintln(&format!("> {}", line.purple().bold()));
        } else {
            self.stderr.eprintln(&format!("> {line}"));
        }
    }

    pub fn capture(&mut self, name: &str, value: &Value) {
        if self.verbosity.is_none() {
            return;
        }
        if self.color {
            self.stderr.eprintln(&format!(
                "{} {}: {value}",
                "*".blue().bold(),
                name.yellow().bold()
            ));
        } else {
            self.stderr.eprintln(&format!("* {name}: {value}"));
        }
    }

    pub fn stderr(&self) -> &Stderr {
        &self.stderr
    }
}

impl Stderr {
    fn eprintln_prefix(&mut self, prefix: &str, message: &str) {
        if message.is_empty() {
            self.eprintln(prefix);
        } else {
            self.eprintln(&format!("{prefix} {message}"));
        }
    }
}

/// Returns the string representation of an `error`, given `lines` of content and a `filename`.
///
/// The source information where the error occurred can be retrieved in `error`; optionally,
/// `entry_src_info` is the optional source information for the entry where the error happened.
/// If `colored` is true, the string use ANSI escape codes to add color and improve the readability
/// of the representation.
///
/// Example:
///
/// ```text
/// Assert status code
///  --> test.hurl:2:10
///   |
/// 2 | HTTP/1.0 200
///   |          ^^^ actual value is <404>
///   |
/// ```
pub(crate) fn error_string<E: DisplaySourceError>(
    filename: &str,
    content: &str,
    error: &E,
    entry_src_info: Option<SourceInfo>,
    colored: bool,
) -> String {
    let mut text = String::new();
    let lines = split_lines(content);
    let entry_line = entry_src_info.map(|e| e.start.line);
    let error_line = error.source_info().start.line;
    let error_column = error.source_info().start.column;
    // The number of digits of the lines count.
    let loc_max_width = max(lines.len().to_string().len(), 2);
    let separator = "|";

    let spaces = " ".repeat(loc_max_width);
    let prefix = format!("{spaces} {separator}");
    let prefix = if colored {
        prefix.blue().bold().to_string()
    } else {
        prefix.to_string()
    };

    let prefix_with_number = format!("{error_line:>loc_max_width$} {separator}");
    let prefix_with_number = if colored {
        prefix_with_number.blue().bold().to_string()
    } else {
        prefix_with_number.to_string()
    };

    // 1. First line is the description, ex. `Assert status code`.
    let description = if colored {
        error.description().bold().to_string()
    } else {
        error.description()
    };
    text.push_str(&description);
    text.push('\n');

    // 2. Second line is the filename info, ex. ` --> test.hurl:2:10`
    let arrow = "-->";
    let arrow = if colored {
        arrow.blue().bold().to_string()
    } else {
        arrow.to_string()
    };
    let line = format!("{spaces}{arrow} {filename}:{error_line}:{error_column}");
    text.push_str(&line);

    // 3. Appends additional empty line
    text.push('\n');
    text.push_str(&prefix);

    // 4. Appends the optional entry line.
    if let Some(entry_line) = entry_line {
        if entry_line != error_line {
            let line = lines.get(entry_line - 1).unwrap();
            let line = line.replace('\t', "    ");
            let line = if colored {
                line.bright_black().to_string()
            } else {
                line.to_string()
            };
            text.push('\n');
            text.push_str(&prefix);
            text.push(' ');
            text.push_str(&line);
        }
        if error_line - entry_line > 1 {
            text.push('\n');
            text.push_str(&prefix);
            let dots = " ...";
            let dots = if colored {
                dots.bright_black().to_string()
            } else {
                dots.to_string()
            };
            text.push_str(&dots);
        }
    }

    // 5. Appends the error message (one or more lines)
    // with the line number '|' prefix
    let message = get_message(error, &lines, colored);
    for (i, line) in split_lines(&message).iter().enumerate() {
        text.push('\n');
        text.push_str(if i == 0 { &prefix_with_number } else { &prefix });
        text.push_str(line);
    }

    // 6. Appends additional empty line
    if !message.ends_with('\n') {
        text.push('\n');
        text.push_str(&prefix);
    }

    text
}

/// Return the constructed message for the error
///
/// It may include:
/// - source line
/// - column position and number of characters (with one or more carets)
///
/// Examples:
///
/// GET abc
///     ^ expecting http://, https:// or {{
///
/// HTTP/1.0 200
///          ^^^ actual value is <404>
///
/// jsonpath "$.count" >= 5
///   actual:   int <2>
///   expected: greater than int <5>
///
/// {
///    "name": "John",
///-   "age": 27
///+   "age": 28
/// }
///
fn get_message<E: DisplaySourceError>(error: &E, lines: &[&str], colored: bool) -> String {
    let mut text = String::new();

    if error.show_source_line() {
        let line = lines.get(error.source_info().start.line - 1).unwrap();
        let line = line.replace('\t', "    ");
        text.push(' ');
        text.push_str(&line);
        text.push('\n');
    }
    let fixme = error.fixme(lines, colored);
    let lines = split_lines(&fixme);
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            text.push('\n');
        }
        text.push_str(line);
    }
    text
}

/// Splits this `text` to a list of LF/CRLF separated lines.
fn split_lines(text: &str) -> Vec<&str> {
    regex::Regex::new(r"\n|\r\n").unwrap().split(text).collect()
}

#[cfg(test)]
pub mod tests {

    use crate::http::HttpError;
    use hurl_core::ast::{Pos, SourceInfo};
    use hurl_core::parser::{ParseError, ParseErrorKind};

    use super::*;
    use crate::runner::{RunnerError, RunnerErrorKind};

    #[test]
    fn test_error_timeout() {
        let content = "GET http://unknown";
        let filename = "test.hurl";
        let kind = RunnerErrorKind::Http(HttpError::Libcurl {
            code: 6,
            description: "Could not resolve host: unknown".to_string(),
        });
        let error_source_info = SourceInfo::new(Pos::new(1, 5), Pos::new(1, 19));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19));
        let error = RunnerError::new(error_source_info, kind, true);
        assert_eq!(
            get_message(&error, &split_lines(content), false),
            " GET http://unknown\n     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown"
        );
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"HTTP connection
  --> test.hurl:1:5
   |
 1 | GET http://unknown
   |     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown
   |"#
        );
    }

    #[test]
    fn test_assert_error_status() {
        let content = r#"GET http://unknown
HTTP/1.0 200
"#;
        let filename = "test.hurl";
        let kind = RunnerErrorKind::AssertStatus {
            actual: "404".to_string(),
        };
        let error_source_info = SourceInfo::new(Pos::new(2, 10), Pos::new(2, 13));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 18));
        let error = RunnerError::new(error_source_info, kind, true);

        assert_eq!(
            get_message(&error, &split_lines(content), false),
            " HTTP/1.0 200\n          ^^^ actual value is <404>"
        );
        colored::control::set_override(true);
        assert_eq!(
            get_message(&error, &split_lines(content), true),
            " HTTP/1.0 200\n\u{1b}[1;31m          ^^^ actual value is <404>\u{1b}[0m"
        );

        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert status code
  --> test.hurl:2:10
   |
   | GET http://unknown
 2 | HTTP/1.0 200
   |          ^^^ actual value is <404>
   |"#
        );
    }

    #[test]
    fn test_invalid_xpath_expression() {
        let content = r#"GET http://example.com
HTTP/1.0 200
[Asserts]
xpath "strong(//head/title)" == "Hello"
"#;
        let filename = "test.hurl";
        let error_source_info = SourceInfo::new(Pos::new(4, 7), Pos::new(4, 29));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 22));
        let error = RunnerError::new(
            error_source_info,
            RunnerErrorKind::QueryInvalidXpathEval,
            true,
        );
        assert_eq!(
            get_message(&error, &split_lines(content), false),
            " xpath \"strong(//head/title)\" == \"Hello\"\n       ^^^^^^^^^^^^^^^^^^^^^^ the XPath expression is not valid"
        );
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Invalid XPath expression
  --> test.hurl:4:7
   |
   | GET http://example.com
   | ...
 4 | xpath "strong(//head/title)" == "Hello"
   |       ^^^^^^^^^^^^^^^^^^^^^^ the XPath expression is not valid
   |"#
        );
    }

    #[test]
    fn test_assert_error_jsonpath() {
        let content = r#"GET http://api
HTTP/1.0 200
[Asserts]
jsonpath "$.count" >= 5
"#;
        let filename = "test.hurl";
        let error_source_info = SourceInfo::new(Pos::new(4, 0), Pos::new(4, 0));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 14));
        let error = RunnerError {
            source_info: error_source_info,
            kind: RunnerErrorKind::AssertFailure {
                actual: "int <2>".to_string(),
                expected: "greater than int <5>".to_string(),
                type_mismatch: false,
            },
            assert: true,
        };

        assert_eq!(
            get_message(&error, &split_lines(content), false),
            r#" jsonpath "$.count" >= 5
   actual:   int <2>
   expected: greater than int <5>"#
        );

        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert failure
  --> test.hurl:4:0
   |
   | GET http://api
   | ...
 4 | jsonpath "$.count" >= 5
   |   actual:   int <2>
   |   expected: greater than int <5>
   |"#
        );
    }

    #[test]
    fn test_assert_error_newline() {
        let content = r#"GET http://localhost
HTTP/1.0 200
```<p>Hello</p>
```
"#;
        let filename = "test.hurl";
        let kind = RunnerErrorKind::AssertBodyValueError {
            actual: "<p>Hello</p>\n\n".to_string(),
            expected: "<p>Hello</p>\n".to_string(),
        };
        let error_source_info = SourceInfo::new(Pos::new(3, 4), Pos::new(4, 1));
        let entry_source_info = SourceInfo::new(Pos::new(1, 1), Pos::new(1, 20));
        let error = RunnerError::new(error_source_info, kind, true);

        assert_eq!(
            get_message(&error, &split_lines(content), false),
            " ```<p>Hello</p>\n    ^ actual value is <<p>Hello</p>\n\n      >"
        );
        assert_eq!(
            error_string(filename, content, &error, Some(entry_source_info), false),
            r#"Assert body value
  --> test.hurl:3:4
   |
   | GET http://localhost
   | ...
 3 | ```<p>Hello</p>
   |    ^ actual value is <<p>Hello</p>
   |
   |      >
   |"#
        );
    }

    #[test]
    fn test_parsing_error() {
        let content = "GET abc";
        let filename = "test.hurl";
        let error = ParseError {
            pos: Pos::new(1, 5),
            recoverable: false,
            kind: ParseErrorKind::UrlInvalidStart,
        };
        assert_eq!(
            error_string(filename, content, &error, None, false),
            r#"Parsing URL
  --> test.hurl:1:5
   |
 1 | GET abc
   |     ^ expecting http://, https:// or {{
   |"#
        );
    }

    #[test]
    fn test_diff_error() {
        let content = r#"GET http://localhost:8000/failed/multiline/json
HTTP 200
```
{
  "name": "John",
  "age": 27
}
```
"#;
        let filename = "test.hurl";
        struct E;
        impl DisplaySourceError for E {
            fn source_info(&self) -> SourceInfo {
                SourceInfo::new(Pos::new(4, 1), Pos::new(4, 0))
            }

            fn description(&self) -> String {
                "Assert body value".to_string()
            }

            fn fixme(&self, _lines: &[&str], _color: bool) -> String {
                r#" {
   "name": "John",
-  "age": 27
+  "age": 28
 }
"#
                .to_string()
            }

            fn show_source_line(&self) -> bool {
                false
            }
        }
        let error = E;

        assert_eq!(
            get_message(&error, &split_lines(content), false),
            r#" {
   "name": "John",
-  "age": 27
+  "age": 28
 }
"#
        );

        assert_eq!(
            error_string(filename, content, &error, None, false),
            r#"Assert body value
  --> test.hurl:4:1
   |
 4 | {
   |   "name": "John",
   |-  "age": 27
   |+  "age": 28
   | }
   |"#
        );
    }
}
