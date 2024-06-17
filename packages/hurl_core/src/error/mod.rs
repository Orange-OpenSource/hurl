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
use crate::ast::SourceInfo;
use crate::text::{Format, StyledString};
use colored::Colorize;
use std::cmp::max;

pub trait DisplaySourceError {
    fn source_info(&self) -> SourceInfo;
    fn description(&self) -> String;
    fn fixme(&self, content: &[&str]) -> StyledString;

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
    fn message(&self, content: &[&str]) -> StyledString;
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
    for (i, line) in split_lines(message).iter().enumerate() {
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
pub fn error_string<E: DisplaySourceError>(
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
    let message = error.message(&lines);
    let message = if colored {
        message.to_string(Format::Ansi)
    } else {
        message.to_string(Format::Plain)
    };

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

/// Splits this `text` to a list of LF/CRLF separated lines.
pub fn split_lines(text: &str) -> Vec<&str> {
    regex::Regex::new(r"\n|\r\n").unwrap().split(text).collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;
    use crate::text::Style;

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
                self.fixme(lines)
            }
        }
        let error = E;

        colored::control::set_override(true);
        assert_eq!(
            error
                .message(&split_lines(content))
                .to_string(Format::Plain),
            r#" {
   "name": "John",
-  "age": 27
+  "age": 28
 }
"#
        );
        assert_eq!(
            error.message(&split_lines(content)).to_string(Format::Ansi),
            " {\n   \"name\": \"John\",\n\u{1b}[31m-  \"age\": 27\u{1b}[0m\n\u{1b}[32m+  \"age\": 28\u{1b}[0m\n }\n"
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
