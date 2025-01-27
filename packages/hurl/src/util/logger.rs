/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
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
//! Log utilities.
use hurl_core::ast::SourceInfo;
use hurl_core::error::{DisplaySourceError, OutputFormat};
use hurl_core::input::Input;
use hurl_core::text::{Format, Style, StyledString};

use crate::runner::Value;
use crate::util::redacted::Redact;
use crate::util::term::{Stderr, WriteMode};

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
    pub(crate) verbosity: Option<Verbosity>,
    pub(crate) stderr: Stderr,
    secrets: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoggerOptions {
    color: bool,
    error_format: ErrorFormat,
    verbosity: Option<Verbosity>,
}

pub struct LoggerOptionsBuilder {
    color: bool,
    error_format: ErrorFormat,
    verbosity: Option<Verbosity>,
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

    /// Creates a new logger.
    pub fn build(&self) -> LoggerOptions {
        LoggerOptions {
            color: self.color,
            error_format: self.error_format,
            verbosity: self.verbosity,
        }
    }
}

impl Default for LoggerOptionsBuilder {
    fn default() -> Self {
        LoggerOptionsBuilder {
            color: false,
            error_format: ErrorFormat::Short,
            verbosity: None,
        }
    }
}

impl Logger {
    /// Creates a new instance.
    pub fn new(options: &LoggerOptions, term: Stderr, secrets: &[String]) -> Self {
        Logger {
            color: options.color,
            error_format: options.error_format,
            verbosity: options.verbosity,
            stderr: term,
            secrets: secrets.to_vec(),
        }
    }

    fn format(&self) -> Format {
        if self.color {
            Format::Ansi
        } else {
            Format::Plain
        }
    }

    /// Prints a given message to this logger [`Stderr`] instance, no matter what is the verbosity.
    pub fn info(&mut self, message: &str) {
        self.eprintln(message);
    }

    /// Prints a given debug message to this logger [`Stderr`] instance, in verbose and very verbose mode.
    ///
    /// Displayed debug messages start with `*`.
    pub fn debug(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let mut s = StyledString::new();
        s.push_with("*", Style::new().blue().bold());
        if !message.is_empty() {
            s.push(" ");
            s.push(message);
        }
        self.eprintln(&s.to_string(fmt));
    }

    /// Prints a given debug message in bold to this logger [`Stderr`] instance, in verbose and very verbose mode.
    ///
    /// Displayed debug messages start with `*`.
    pub fn debug_important(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let mut s = StyledString::new();
        s.push_with("*", Style::new().blue().bold());
        if !message.is_empty() {
            s.push(" ");
            s.push_with(message, Style::new().bold());
        }
        self.eprintln(&s.to_string(fmt));
    }

    /// Prints a given debug message from libcurl to this logger [`Stderr`] instance, in verbose and very verbose mode.
    ///
    /// Displayed libcurl debug messages start with `**`.
    pub fn debug_curl(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let mut s = StyledString::new();
        s.push_with("**", Style::new().blue().bold());
        if !message.is_empty() {
            s.push(" ");
            s.push(message);
        }
        self.eprintln(&s.to_string(fmt));
    }

    /// Prints an error (syntax error or runtime error) to this logger [`Stderr`] instance, in verbose and very verbose mode.
    pub fn debug_error<E: DisplaySourceError>(
        &mut self,
        content: &str,
        filename: Option<&Input>,
        error: &E,
        entry_src_info: SourceInfo,
    ) {
        if self.verbosity.is_none() {
            return;
        }
        let filename = filename.map_or(String::new(), |f| f.to_string());
        let message = error.to_string(
            &filename,
            content,
            Some(entry_src_info),
            OutputFormat::Terminal(self.color),
        );
        message.lines().for_each(|l| self.debug(l));
    }

    /// Prints a HTTP response header to this logger [`Stderr`] instance, in verbose and very verbose mode.
    ///
    /// Response HTTP headers start with `>`.
    pub fn debug_headers_in(&mut self, headers: &[(&str, &str)]) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();

        for (name, value) in headers {
            let mut s = StyledString::new();
            s.push("< ");
            s.push_with(name, Style::new().cyan().bold());
            s.push(": ");
            s.push(value);
            self.eprintln(&s.to_string(fmt));
        }
        self.eprintln("<");
    }

    /// Prints a HTTP request header to this logger [`Stderr`] instance, in verbose and very verbose mode.
    ///
    /// Request HTTP headers start with `>`.
    pub fn debug_headers_out(&mut self, headers: &[(&str, &str)]) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();

        for (name, value) in headers {
            let mut s = StyledString::new();
            s.push("> ");
            s.push_with(name, Style::new().cyan().bold());
            s.push(": ");
            s.push(value);
            self.eprintln(&s.to_string(fmt));
        }
        self.eprintln(">");
    }

    /// Prints a HTTP response status code to this logger [`Stderr`] instance, in verbose and very verbose mode.
    pub fn debug_status_version_in(&mut self, line: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let mut s = StyledString::new();
        s.push("< ");
        s.push_with(line, Style::new().green().bold());
        self.eprintln(&s.to_string(fmt));
    }

    /// Prints a warning given message to this logger [`Stderr`] instance, no matter what is the verbosity.
    ///
    /// Displayed warning messages start with `warning:`.
    pub fn warning(&mut self, message: &str) {
        let fmt = self.format();
        let mut s = StyledString::new();
        s.push_with("warning", Style::new().yellow().bold());
        s.push(": ");
        s.push_with(message, Style::new().bold());
        self.eprintln(&s.to_string(fmt));
    }

    pub fn error_parsing_rich<E: DisplaySourceError>(
        &mut self,
        content: &str,
        filename: Option<&Input>,
        error: &E,
    ) {
        // FIXME: peut-être qu'on devrait faire rentrer le prefix `error:` qui est
        // fournit par `self.error_rich` dans la méthode `error.to_string`
        let filename = filename.map_or(String::new(), |f| f.to_string());
        let message = error.to_string(&filename, content, None, OutputFormat::Terminal(self.color));
        self.error_rich(&message);
    }

    /// Prints a runtime error to this logger [`Stderr`] instance, no matter what is the verbosity.
    pub fn error_runtime_rich<E: DisplaySourceError>(
        &mut self,
        content: &str,
        filename: Option<&Input>,
        error: &E,
        entry_src_info: SourceInfo,
    ) {
        let filename = filename.map_or(String::new(), |f| f.to_string());
        let message = error.to_string(
            &filename,
            content,
            Some(entry_src_info),
            OutputFormat::Terminal(self.color),
        );
        self.error_rich(&message);
    }

    fn error_rich(&mut self, message: &str) {
        let fmt = self.format();
        let mut s = StyledString::new();
        s.push_with("error", Style::new().red().bold());
        s.push(": ");
        s.push(message);
        s.push("\n");
        self.eprintln(&s.to_string(fmt));
    }

    /// Prints the request method and HTTP  version to this logger [`Stderr`] instance, in verbose and very verbose mode.
    pub fn debug_method_version_out(&mut self, line: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let mut s = StyledString::new();
        s.push("> ");
        s.push_with(line, Style::new().purple().bold());
        self.eprintln(&s.to_string(fmt));
    }

    /// Prints a capture to this logger [`Stderr`] instance, in verbose and very verbose mode.
    pub fn capture(&mut self, name: &str, value: &Value) {
        if self.verbosity.is_none() {
            return;
        }
        let value = value.to_string();
        let fmt = self.format();
        let mut s = StyledString::new();
        s.push_with("*", Style::new().blue().bold());
        s.push(" ");
        s.push_with(name, Style::new().yellow().bold());
        s.push(": ");
        s.push(&value);
        self.eprintln(&s.to_string(fmt));
    }

    /// Update logger with new `secrets`.
    pub fn set_secrets(&mut self, secrets: Vec<String>) {
        if self.secrets == secrets {
            return;
        }
        self.secrets = secrets;

        // When secrets are updated, we need to rewrite the buffered `StdErr` as new secrets
        // needs to be redacted.
        if matches!(self.stderr.mode(), WriteMode::Buffered) {
            let old_buffer = self.stderr.buffer();
            let new_buffer = old_buffer.redact(&self.secrets);
            self.stderr.set_buffer(new_buffer);
        }
    }

    fn eprintln(&mut self, message: &str) {
        if self.secrets.is_empty() {
            self.stderr.eprintln(message);
            return;
        }
        let redacted = message.redact(&self.secrets);
        self.stderr.eprintln(&redacted);
    }
}
