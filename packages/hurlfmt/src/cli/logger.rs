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
use hurl_core::error::{DisplaySourceError, OutputFormat};
use hurl_core::input::Input;
use hurl_core::text::{Format, Style, StyledString};

/// A simple logger to log app related event (start, high levels error, etc...).
pub struct Logger {
    /// Format of the message in the terminal: ANSI or plain.
    format: Format,
}

impl Logger {
    /// Creates a new logger using `color`.
    pub fn new(color: bool) -> Self {
        let format = if color { Format::Ansi } else { Format::Plain };
        Logger { format }
    }

    /// Prints an error `message` on standard error.
    pub fn error(&self, message: &str) {
        let mut s = StyledString::new();
        s.push_with("error", Style::new().red().bold());
        s.push(": ");
        s.push_with(message, Style::new().bold());
        eprintln!("{}", s.to_string(self.format));
    }

    /// Displays a Hurl parsing error.
    pub fn error_parsing<E: DisplaySourceError>(&self, content: &str, file: &Input, error: &E) {
        // FIXME: peut-être qu'on devrait faire rentrer le prefix `error:` qui est
        // fournit par `self.error_rich` dans la méthode `error.to_string`
        let message = error.to_string(
            &file.to_string(),
            content,
            None,
            OutputFormat::Terminal(self.format == Format::Ansi),
        );

        let mut s = StyledString::new();
        s.push_with("error", Style::new().red().bold());
        s.push(": ");
        s.push(&message);
        s.push("\n");
        eprintln!("{}", s.to_string(self.format));
    }

    /// Displays a lint warning.
    pub fn warn_lint<E: DisplaySourceError>(&self, content: &str, file: &Input, error: &E) {
        let message = error.to_string(
            &file.to_string(),
            content,
            None,
            OutputFormat::Terminal(self.format == Format::Ansi),
        );

        let mut s = StyledString::new();
        s.push_with("warning", Style::new().yellow().bold());
        s.push(": ");
        s.push(&message);
        s.push("\n");
        eprintln!("{}", s.to_string(self.format));
    }
}
