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
use hurl_core::text::{Format, Style, StyledString};

/// A simple logger to log app related event (start, high levels error, etc...).
pub struct BaseLogger {
    /// Format of the message in the terminal: ANSI or plain.
    format: Format,
    /// Prints debug message or not.
    verbose: bool,
}

impl BaseLogger {
    /// Creates a new base logger using `color` and `verbose`.
    pub fn new(color: bool, verbose: bool) -> BaseLogger {
        let format = if color { Format::Ansi } else { Format::Plain };
        BaseLogger { format, verbose }
    }

    /// Prints an informational `message` on standard error.
    pub fn info(&self, message: &str) {
        eprintln!("{message}");
    }

    /// Prints a debug `message` on standard error if the logger is in verbose mode.
    pub fn debug(&self, message: &str) {
        if !self.verbose {
            return;
        }
        let mut s = StyledString::new();
        s.push_with("*", Style::new().blue().bold());
        if !message.is_empty() {
            s.push(&format!(" {message}"));
        }
        eprintln!("{}", s.to_string(self.format));
    }

    /// Prints an error `message` on standard error.
    pub fn error(&self, message: &str) {
        let mut s = StyledString::new();
        s.push_with("error", Style::new().red().bold());
        s.push(": ");
        s.push_with(message, Style::new().bold());
        eprintln!("{}", s.to_string(self.format));
    }
}
