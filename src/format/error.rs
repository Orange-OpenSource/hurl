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
use crate::core::common::SourceInfo;

use super::color::TerminalColor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    //pub exit_code: usize,
    pub source_info: SourceInfo,
    pub description: String,
    pub fixme: String,
    pub lines: Vec<String>,
    pub filename: String,
    pub warning: bool,
    pub color: bool,
}


impl Error {
    pub fn format(self) -> String {
        let mut s = "".to_string();
        let line_number_size = if self.lines.len() < 100 {
            2
        } else if self.lines.len() < 1000 {
            3
        } else {
            4
        };

        let error_type = if self.warning {
            String::from("warning")
        } else {
            String::from("error")
        };
        let error_type = if !self.color {
            error_type
        } else if self.warning {
            TerminalColor::Yellow.format(error_type)
        } else {
            TerminalColor::Red.format(error_type)
        };
        s.push_str(format!("{}: {}\n", error_type, self.description).as_str());

        if self.filename != "-" {
            s.push_str(
                format!(
                    "{}--> {}:{}:{}\n",
                    " ".repeat(line_number_size).as_str(),
                    self.filename,
                    self.source_info.start.line,
                    self.source_info.start.column,
                ).as_str(),
            );
        }


        s.push_str(format!("{} |\n", " ".repeat(line_number_size)).as_str());

        let line = self.lines.get(self.source_info.start.line - 1).unwrap();
        let line = str::replace(line, "\t", "    ");    // replace all your tabs with 4 characters
        s.push_str(
            format!(
                "{line_number:>width$} |{line}\n",
                line_number = self.source_info.start.line,
                width = line_number_size,
                line = if line.is_empty() { line } else { format!(" {}", line) }
            )
                .as_str(),
        );

        // TODO: to clean/Refacto
        // specific case for assert errors
        if self.source_info.start.column == 0 {
            let fixme_lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
                .unwrap()
                .split(&self.fixme)
                .collect();
            // edd an empty line at the end?
            for line in fixme_lines {
                s.push_str(
                    format!(
                        "{} |   {}\n",
                        " ".repeat(line_number_size).as_str(),
                        fixme = line,
                    )
                        .as_str(),
                );
            }
        } else {
            let line = self.lines.get(self.source_info.start.line - 1).unwrap();
            let width = (self.source_info.end.column - self.source_info.start.column) as usize;

            let mut tab_shift = 0;
            for (i, c) in line.chars().enumerate() {
                if i >= self.source_info.start.column - 1 { break; };
                if c == '\t' {
                    tab_shift += 1;
                }
            }
            s.push_str(
                format!(
                    "{} | {}{} {fixme}\n",
                    " ".repeat(line_number_size).as_str(),
                    " ".repeat(self.source_info.start.column - 1 + tab_shift * 3),
                    "^".repeat(if width > 1 { width } else { 1 }),
                    fixme = self.fixme.as_str(),
                )
                    .as_str(),
            );
        }


        s.push_str(format!("{} |\n", " ".repeat(line_number_size)).as_str());

        s
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let filename = String::from("integration/hurl_error_lint/spaces.hurl");
        let lines = vec![
            String::from("GET\thttp://localhost:8000/hello")
        ];
        let error = Error {
            source_info: SourceInfo::init(1, 4, 1, 5),
            description: String::from("One space"),
            fixme: String::from("Use only one space"),
            lines,
            filename,
            warning: true,
            color: false,
        };
        assert_eq!(error.format(),
                   String::from(r#"warning: One space
  --> integration/hurl_error_lint/spaces.hurl:1:4
   |
 1 | GET    http://localhost:8000/hello
   |    ^ Use only one space
   |
"#)
        );
    }

    #[test]
    fn test_with_tabs() {
        let filename = String::from("integration/hurl_error_lint/spaces.hurl");
        let lines = vec![
            String::from("GET\thttp://localhost:8000/hello ")
        ];
        let error = Error {
            source_info: SourceInfo::init(1, 32, 1, 32),
            description: String::from("Unnecessary space"),
            fixme: String::from("Remove space"),
            lines,
            filename,
            warning: true,
            color: false,
        };
        assert_eq!(error.format(),
                   concat!(
"warning: Unnecessary space\n",
"  --> integration/hurl_error_lint/spaces.hurl:1:32\n",
"   |\n",
" 1 | GET    http://localhost:8000/hello \n",
"   |                                   ^ Remove space\n",
"   |\n")
        );
    }


    #[test]
    fn test_end_of_file() {

        // todo: improve error location

        let filename = String::from("hurl_error_parser/json_unexpected_eof.hurl");
        let lines = vec![
            String::from("POST http://localhost:8000/data\n"),
            String::from("{ \"name\":\n"),
            String::from("")
        ];
        let error = Error {
            source_info: SourceInfo::init(3, 1, 3, 1),
            description: String::from("Parsing json"),
            fixme: String::from("json error"),
            lines,
            filename,
            warning: true,
            color: false,
        };
        assert_eq!(error.format(),
                   String::from(r#"warning: Parsing json
  --> hurl_error_parser/json_unexpected_eof.hurl:3:1
   |
 3 |
   | ^ json error
   |
"#)
        );
    }

    #[test]
    fn test_assert_error() {
        let filename = String::from("hurl_error_parser/json_unexpected_eof.hurl");
        let lines = vec![
            String::from("...\n"),
            String::from("[Asserts]\n"),
            String::from("jsonpath \"$.message\" startsWith \"hello\"")
        ];
        let _error = Error {
            source_info: SourceInfo::init(3, 0, 3, 0),
            description: String::from("Assert Error"),
            fixme: String::from("actual:   string <tutu>\nexpected: starts with string <toto>"),
            lines,
            filename,
            warning: false,
            color: false,
        };

        //assert_eq!(1,2);
    }
}
