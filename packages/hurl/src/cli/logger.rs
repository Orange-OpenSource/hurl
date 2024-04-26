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
use colored::Colorize;

/// A simple logger to log app related event (start, high levels error, etc...).
pub struct BaseLogger {
    /// Uses ANSI color codes or not.
    pub color: bool,
    /// Prints debug message or not.
    pub verbose: bool,
}

impl BaseLogger {
    /// Creates a new base logger using `color` and `verbose`.
    pub fn new(color: bool, verbose: bool) -> BaseLogger {
        BaseLogger { color, verbose }
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
        if self.color {
            eprintln!("{} {message}", "*".blue().bold());
        } else {
            eprintln!("* {message}");
        }
    }

    /// Prints an error `message` on standard error.
    pub fn error(&self, message: &str) {
        if self.color {
            eprintln!("{}: {}", "error".red().bold(), message.bold());
        } else {
            eprintln!("error: {message}");
        }
    }
}
