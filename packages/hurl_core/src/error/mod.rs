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
use std::cmp::max;

use crate::ast::SourceInfo;
use crate::text::{Format, Style, StyledString};

pub trait DisplaySourceError {
    fn source_info(&self) -> SourceInfo;
    fn description(&self) -> String;
    fn fixme(&self, content: &[&str]) -> StyledString;

    /// Returns the constructed message for the error
    ///
    /// It may include:
    ///
    /// - source line
    /// - column position and number of characters (with one or more carets)
    ///
    /// Examples:
    ///
    /// ```text
    /// GET abc
    ///     ^ expecting http://, https:// or {{
    /// ```
    ///
    /// ```text
    /// HTTP/1.0 200
    ///          ^^^ actual value is <404>
    /// ```
    ///
    /// ```text
    /// jsonpath "$.count" >= 5
    ///   actual:   integer <2>
    ///   expected: greater than integer <5>
    /// ```
    ///
    /// ```text
    /// {
    ///    "name": "John",
    ///-   "age": 27
    ///+   "age": 28
    /// }
    /// ```
    fn message(&self, content: &[&str]) -> StyledString {
        let mut text = StyledString::new();
        add_source_line(&mut text, content, self.source_info().start.line);
        text.append(self.fixme(content));

        let error_line = self.source_info().start.line;
        add_line_info_prefix(&text, content, error_line)
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
    ///
    ///
    /// ```text
    /// Assert status code                        | > description()
    ///  --> test.hurl:2:10                       | > add_filename_with_sourceinfo()
    ///   |                                       |
    ///   | GET http://foo.com                    | > add_entry_line()
    ///   | ...                                   |
    /// 2 | HTTP/1.0 200                          | > message()
    ///   |          ^^^ actual value is <404>    | >
    ///   |                                       |
    /// ```
    fn to_string(
        &self,
        filename: &str,
        content: &str,
        entry_src_info: Option<SourceInfo>,
        format: OutputFormat,
    ) -> String {
        let mut text = StyledString::new();
        let lines = content.lines().collect::<Vec<_>>();

        let error_line = self.source_info().start.line;
        let error_column = self.source_info().start.column;

        // Push one-line description of the error
        text.push_with(&self.description(), Style::new().bold());
        text.push("\n");

        // We build the prefix
        let loc_max_width = max(lines.len().to_string().len(), 2);
        let spaces = " ".repeat(loc_max_width);
        let separator = "|";
        let mut prefix = StyledString::new();
        prefix.push_with(&format!("{spaces} {separator}"), Style::new().blue().bold());

        // Add filename, with a left margin space for error line number.
        add_filename_with_sourceinfo(&mut text, &spaces, filename, error_line, error_column);

        // First line of the error message
        text.append(prefix.clone());

        // We can have an optional source line for context
        let entry_line = entry_src_info.map(|e| e.start.line);
        if let Some(entry_line) = entry_line {
            add_entry_line(&mut text, &lines, error_line, entry_line, &prefix);
        }

        text.append(self.message(&lines));
        format_error_message(&text, format)
    }
}

/// Show column position with carets
pub fn add_carets(message: &str, source_info: SourceInfo, content: &[&str]) -> String {
    let error_line = source_info.start.line;
    let error_column = source_info.start.column;
    // Error source info start and end can be on different lines, we insure a minimum width.
    let width = if source_info.end.column > error_column {
        source_info.end.column - error_column
    } else {
        1
    };
    let line_raw = content.get(error_line - 1).unwrap();
    let prefix = get_carets(line_raw, error_column, width);

    let mut s = String::new();
    for (i, line) in message.lines().enumerate() {
        if i == 0 {
            s.push_str(format!("{prefix}{line}").as_str());
        } else {
            s.push('\n');
            if !line.is_empty() {
                s.push_str(" ".repeat(prefix.len()).as_str());
            }
            s.push_str(line);
        };
    }
    s
}

/// Format used by to_string
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Plain,
    Terminal(bool), // Replace \r\n by \n
}

pub fn add_line_info_prefix(
    text: &StyledString,
    content: &[&str],
    error_line: usize,
) -> StyledString {
    let text = text.clone();
    let separator = "|";

    let loc_max_width = max(content.len().to_string().len(), 2);
    let spaces = " ".repeat(loc_max_width);
    let mut prefix = StyledString::new();
    prefix.push_with(
        format!("{spaces} {separator}").as_str(),
        Style::new().blue().bold(),
    );
    let mut prefix_with_number = StyledString::new();
    prefix_with_number.push_with(
        format!("{error_line:>loc_max_width$} {separator}").as_str(),
        Style::new().blue().bold(),
    );

    let mut text2 = StyledString::new();
    for (i, line) in text.split('\n').iter().enumerate() {
        text2.push("\n");
        text2.append(if i == 0 {
            prefix_with_number.clone()
        } else {
            prefix.clone()
        });
        text2.append(line.clone());
    }

    //  Appends additional empty line
    if !text2.ends_with("|") {
        text2.push("\n");
        text2.append(prefix.clone());
    }

    text2
}

/// Generate carets for the given source_line/source_info
fn get_carets(line_raw: &str, error_column: usize, width: usize) -> String {
    //  We take tabs into account because we have normalize the display of the error line by replacing
    //  tabs with 4 spaces.
    let mut tab_shift = 0;
    for (i, c) in line_raw.chars().enumerate() {
        if i >= (error_column - 1) {
            break;
        };
        if c == '\t' {
            tab_shift += 1;
        }
    }

    let mut prefix = " ".repeat(error_column + tab_shift * 3);
    prefix.push_str("^".repeat(width).as_str());
    prefix.push(' ');
    prefix
}

pub fn add_source_line(text: &mut StyledString, content: &[&str], line: usize) {
    let line = content.get(line - 1).unwrap();
    let line = line.replace('\t', "    ");
    text.push(" ");
    text.push(&line);
    text.push("\n");
}

fn add_filename_with_sourceinfo(
    text: &mut StyledString,
    spaces: &str,
    filename: &str,
    error_line: usize,
    error_column: usize,
) {
    text.push(spaces);
    text.push_with("-->", Style::new().blue().bold());
    text.push(format!(" {filename}:{error_line}:{error_column}").as_str());
    text.push("\n");
}

fn add_entry_line(
    text: &mut StyledString,
    lines: &[&str],
    error_line: usize,
    entry_line: usize,
    prefix: &StyledString,
) {
    if entry_line != error_line {
        let line = lines.get(entry_line - 1).unwrap();
        let line = line.replace('\t', "    ");
        text.push("\n");
        text.append(prefix.clone());
        text.push(" ");
        text.push_with(&line, Style::new().bright_black());
    }

    if error_line - entry_line > 1 {
        text.push("\n");
        text.append(prefix.clone());
        text.push_with(" ...", Style::new().bright_black());
    }
}

fn format_error_message(message: &StyledString, format: OutputFormat) -> String {
    let colored = format == OutputFormat::Terminal(true);
    let message = if colored {
        message.to_string(Format::Ansi)
    } else {
        message.to_string(Format::Plain)
    };

    match format {
        OutputFormat::Terminal(_) => {
            message.replace("\r\n", "\n") // CRLF must be replaced by LF in the terminal
        }
        OutputFormat::Plain => message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::Pos;
    use crate::text::{Format, Style};

    #[test]
    fn test_add_carets() {
        // `Hello World`
        // ^^^^^^^^^^^^^ actual value is <Hello World!>
        assert_eq!(
            add_carets(
                "actual value is <Hello World!>",
                SourceInfo::new(Pos::new(1, 1), Pos::new(1, 14)),
                &["`Hello World`"]
            ),
            " ^^^^^^^^^^^^^ actual value is <Hello World!>".to_string()
        );
    }

    #[test]
    fn test_get_carets() {
        // `Hello World`
        // ^^^^^^^^^^^^^ actual value is <Hello World!>
        assert_eq!(
            get_carets("`Hello World`", 1, 13),
            " ^^^^^^^^^^^^^ ".to_string()
        );

        // Content-Length: 200
        //                 ^^^ actual value is <12>
        assert_eq!(
            get_carets("Content-Length: 200", 17, 3),
            "                 ^^^ ".to_string()
        );

        // With a tab instead of a space
        // Content-Length:    200
        //                    ^^^ actual value is <12>
        assert_eq!(
            get_carets("Content-Length:\t200", 17, 3),
            "                    ^^^ ".to_string()
        );
    }

    #[test]
    fn test_diff_error() {
        crate::text::init_crate_colored();

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

            fn fixme(&self, _lines: &[&str]) -> StyledString {
                let mut diff = StyledString::new();
                diff.push(" {\n   \"name\": \"John\",\n");
                diff.push_with("-  \"age\": 27", Style::new().red());
                diff.push("\n");
                diff.push_with("+  \"age\": 28", Style::new().green());
                diff.push("\n }\n");
                diff
            }
            fn message(&self, lines: &[&str]) -> StyledString {
                let s = self.fixme(lines);
                add_line_info_prefix(&s, &[], 4)
            }
        }
        let error = E;

        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(
            error.message(&lines).to_string(Format::Plain),
            r#"
 4 | {
   |   "name": "John",
   |-  "age": 27
   |+  "age": 28
   | }
   |"#
        );
        assert_eq!(
            error.message(&lines).to_string(Format::Ansi),
            "\n\u{1b}[1;34m 4 |\u{1b}[0m {\n\u{1b}[1;34m   |\u{1b}[0m   \"name\": \"John\",\n\u{1b}[1;34m   |\u{1b}[0m\u{1b}[31m-  \"age\": 27\u{1b}[0m\n\u{1b}[1;34m   |\u{1b}[0m\u{1b}[32m+  \"age\": 28\u{1b}[0m\n\u{1b}[1;34m   |\u{1b}[0m }\n\u{1b}[1;34m   |\u{1b}[0m"
        );

        assert_eq!(
            error.to_string(filename, content, None, OutputFormat::Terminal(false)),
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
