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
//! Log utilities.
use hurl_core::ast::SourceInfo;
use hurl_core::error::{error_string, split_lines, DisplaySourceError};
use hurl_core::richtext::{Format, RichText};

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

    fn format(&self) -> Format {
        if self.color {
            Format::Ansi
        } else {
            Format::Plain
        }
    }

    pub fn info(&mut self, message: &str) {
        self.stderr.eprintln(message);
    }

    pub fn debug(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let prefix = RichText::new().text("*").blue().bold().to_string(fmt);
        self.stderr.eprintln_prefix(&prefix, message);
    }

    pub fn debug_important(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let prefix = RichText::new().text("*").blue().bold().to_string(fmt);
        let message = RichText::new().text(message).bold().to_string(fmt);
        self.stderr.eprintln_prefix(&prefix, &message);
    }

    pub fn debug_curl(&mut self, message: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let prefix = RichText::new().text("**").blue().bold().to_string(fmt);
        self.stderr.eprintln_prefix(&prefix, message);
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
        let fmt = self.format();

        for (name, value) in headers {
            let message = RichText::new()
                .text("< ")
                .text(name)
                .cyan()
                .bold()
                .text(&format!(": {value}"))
                .to_string(fmt);
            self.stderr.eprintln(&message);
        }
        self.stderr.eprintln("<");
    }

    pub fn debug_headers_out(&mut self, headers: &[(&str, &str)]) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();

        for (name, value) in headers {
            let message = RichText::new()
                .text("> ")
                .text(name)
                .cyan()
                .bold()
                .text(&format!(": {value}"))
                .to_string(fmt);
            self.stderr.eprintln(&message);
        }
        self.stderr.eprintln(">");
    }

    pub fn debug_status_version_in(&mut self, line: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let message = RichText::new()
            .text("< ")
            .text(line)
            .green()
            .bold()
            .to_string(fmt);
        self.stderr.eprintln(&message);
    }

    pub fn warning(&mut self, message: &str) {
        let fmt = self.format();
        let message = RichText::new()
            .text("warning")
            .yellow()
            .bold()
            .text(": ")
            .text(message)
            .bold()
            .to_string(fmt);
        self.stderr.eprintln(&message);
    }

    pub fn error(&mut self, message: &str) {
        let fmt = self.format();
        let message = RichText::new()
            .text("error")
            .red()
            .bold()
            .text(": ")
            .text(message)
            .bold()
            .to_string(fmt);
        self.stderr.eprintln(&message);
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
        let fmt = self.format();
        let message = RichText::new()
            .text("error")
            .red()
            .bold()
            .text(": ")
            .text(message)
            .to_string(fmt);
        self.stderr.eprintln(&message);
    }

    pub fn debug_method_version_out(&mut self, line: &str) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let message = RichText::new()
            .text("> ")
            .text(line)
            .purple()
            .bold()
            .to_string(fmt);
        self.stderr.eprintln(&message);
    }

    pub fn capture(&mut self, name: &str, value: &Value) {
        if self.verbosity.is_none() {
            return;
        }
        let fmt = self.format();
        let message = RichText::new()
            .text("*")
            .blue()
            .bold()
            .text(" ")
            .text(name)
            .yellow()
            .bold()
            .text(&format!(": {value}"))
            .to_string(fmt);
        self.stderr.eprintln(&message);
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
