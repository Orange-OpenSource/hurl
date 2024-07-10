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
use std::path::PathBuf;

use hurl_core::error::{DisplaySourceError, OutputFormat};
use hurl_core::text::{Format, Style, StyledString};

use crate::linter;

/// A simple logger to log app related event (start, high levels error, etc...).
pub struct Logger {
    /// Format of the messaeg in the terminal: ANSI or plain.
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

    /// Display a Hurl parsing error.
    pub fn error_parsing_rich<E: DisplaySourceError>(
        &mut self,
        content: &str,
        filename: &str,
        error: &E,
    ) {
        // FIXME: peut-être qu'on devrait faire rentrer le prefix `error:` qui est
        // fournit par `self.error_rich` dans la méthode `error.to_string`
        let message = error.to_string(
            filename,
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
}

pub fn make_logger_linter_error(
    lines: Vec<String>,
    color: bool,
    filename: Option<PathBuf>,
) -> impl Fn(&linter::Error, bool) {
    move |error: &linter::Error, warning: bool| {
        let filename = match &filename {
            None => "-".to_string(),
            Some(value) => value.display().to_string(),
        };
        let content = lines.join("\n");
        let message = error.to_string(&filename, &content, None, OutputFormat::Terminal(color));
        eprintln!("{}: {}\n", get_prefix(warning, color), message);
    }
}
fn get_prefix(warning: bool, color: bool) -> String {
    let mut message = StyledString::new();
    if warning {
        message.push_with("warning", Style::new().yellow().bold());
    } else {
        message.push_with("error", Style::new().red());
    };
    let fmt = if color { Format::Ansi } else { Format::Plain };
    message.to_string(fmt)
}
