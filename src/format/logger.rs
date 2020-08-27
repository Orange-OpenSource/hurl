/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

use super::error::Error;
use super::color::TerminalColor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Logger {
    pub filename: Option<String>,
    pub lines: Vec<String>,
    pub verbose: bool,
    pub color: bool,
}


impl Logger {

    pub fn info(&self, s: &str) {
        println!("{}", s);
    }

    pub fn verbose(&self, s: &str) {
        if self.verbose {
            eprintln!("* {}", s);
        }
    }

    pub fn send(&self, s: String) {
        if self.verbose {
            eprintln!("> {}", s);
        }
    }

    pub fn receive(&self, s: String) {
        if self.verbose {
            eprintln!("< {}", s);
        }
    }

    pub fn error_message(&self, s: String) {
        let error_type = if !self.color {
            "error".to_string()
        } else {
            TerminalColor::Red.format("error".to_string())
        };
        eprintln!("{}: {}", error_type, s);
    }

    pub fn warning_message(&self, s: String) {
        let error_type = if !self.color {
            "warning".to_string()
        } else {
            TerminalColor::Yellow.format("warning".to_string())
        };
        eprintln!("{}: {}", error_type, s);
    }

    pub fn error(&self, err: &Error) {
        let line_number_size = if self.lines.len() < 100 {
            2
        } else if self.lines.len() < 1000 {
            3
        } else {
            4
        };

        let error_type = if err.warning {
            String::from("warning")
        } else {
            String::from("error")
        };
        let error_type = if !self.color {
            error_type
        } else if err.warning {
            TerminalColor::Yellow.format(error_type)
        } else {
            TerminalColor::Red.format(error_type)
        };
        eprintln!("{}: {}", error_type, err.description);

        if let Some(filename) = self.filename.clone() {
            eprintln!(
                "{}--> {}:{}:{}",
                " ".repeat(line_number_size).as_str(),
                filename,
                err.source_info.start.line,
                err.source_info.start.column,
            );
        }
        eprintln!("{} |", " ".repeat(line_number_size));

        let line = self.lines.get(err.source_info.start.line - 1).unwrap();
        let line = str::replace(line, "\t", "    ");    // replace all your tabs with 4 characters
        eprintln!(
            "{line_number:>width$} |{line}",
            line_number = err.source_info.start.line,
            width = line_number_size,
            line = if line.is_empty() { line } else { format!(" {}", line) }
        );

        // TODO: to clean/Refacto
        // specific case for assert errors
        if err.source_info.start.column == 0 {
            let fixme_lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
                .unwrap()
                .split(&err.fixme)
                .collect();
            // edd an empty line at the end?
            for line in fixme_lines {
                eprintln!(
                    "{} |   {}",
                    " ".repeat(line_number_size).as_str(),
                    fixme = line,
                );
            }
        } else {
            let line = self.lines.get(err.source_info.start.line - 1).unwrap();
            let width = (err.source_info.end.column - err.source_info.start.column) as usize;

            let mut tab_shift = 0;
            for (i, c) in line.chars().enumerate() {
                if i >= err.source_info.start.column - 1 { break; };
                if c == '\t' {
                    tab_shift += 1;
                }
            }
            eprintln!(
                "{} | {}{} {fixme}",
                " ".repeat(line_number_size).as_str(),
                " ".repeat(err.source_info.start.column - 1 + tab_shift * 3),
                "^".repeat(if width > 1 { width } else { 1 }),
                fixme = err.fixme.as_str(),
            );
        }

        eprintln!("{} |\n", " ".repeat(line_number_size));
    }
}