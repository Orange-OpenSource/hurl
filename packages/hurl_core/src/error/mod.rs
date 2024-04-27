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

pub trait Error {
    fn source_info(&self) -> SourceInfo;
    fn description(&self) -> String;
    fn fixme(&self, content: &[&str], color: bool) -> String;
    fn show_source_line(&self) -> bool;
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

/// Splits this `text` to a list of LF/CRLF separated lines.
fn split_lines(text: &str) -> Vec<&str> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;

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
}
