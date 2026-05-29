/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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

use super::ConfigFileError;
use hurl_core::reader::Reader;

/// Skip whitespaces and comments
pub fn skip_whitespace_and_comments(reader: &mut Reader) {
    loop {
        reader.read_while(|c: char| c.is_whitespace());
        if reader.is_eof() {
            break;
        }
        if reader.peek() == Some('#') {
            // Skip comment line
            reader.read_while(|c: char| c != '\n');
            if reader.peek() == Some('\n') {
                reader.read(); // consume newline
            }
        } else {
            break;
        }
    }
}

/// Expect end-line after an option value, skipping any trailing whitespace and comments
pub fn expect_no_value(reader: &mut Reader) -> Result<(), ConfigFileError> {
    loop {
        reader.read_while(|c: char| c == ' ' || c == '\t');
        if reader.is_eof() {
            break;
        }
        if reader.peek() == Some('#') {
            // Skip comment line
            reader.read_while(|c: char| c != '\n');
            if reader.peek() == Some('\n') {
                reader.read(); // consume newline
            }
        } else if reader.peek() == Some('\n') {
            reader.read(); // consume newline
            break;
        } else {
            return Err(ConfigFileError::new(
                reader.cursor().pos,
                "Not expecting a value for this option",
            ));
        }
    }
    Ok(())
}

/// Parse an option value on the same as the option
/// It is separated by one or more spaces or '='
pub fn parse_value(reader: &mut Reader) -> Result<String, ConfigFileError> {
    if let Some('"') = reader.peek() {
        reader.read(); // consume opening quote
        let value = reader.read_while(|c| c != '"');
        if reader.peek() == Some('"') {
            reader.read(); // consume closing quote
            // Allow trailing spaces and inline comment, but nothing else on the same line.
            reader.read_while(|c: char| c == ' ' || c == '\t');
            if reader.is_eof() {
                Ok(value.trim().to_string())
            } else if reader.peek() == Some('#') {
                reader.read_while(|c: char| c != '\n');
                if reader.peek() == Some('\n') {
                    reader.read(); // consume newline
                }
                Ok(value.trim().to_string())
            } else if reader.peek() == Some('\n') {
                reader.read(); // consume newline
                Ok(value.trim().to_string())
            } else {
                Err(ConfigFileError::new(
                    reader.cursor().pos,
                    "characters after the closing quote",
                ))
            }
        } else {
            Err(ConfigFileError::new(
                reader.cursor().pos,
                "Missing closing quote",
            ))
        }
    } else {
        let s = reader.read_while(|c| c != '\n').trim().to_string();
        Ok(s)
    }
}

/// The separator must one or more spaces or '=' character
pub fn parse_value_separator(reader: &mut Reader) -> Result<(), ConfigFileError> {
    let s = spaces(reader);
    if s.is_empty() {
        if reader.peek() == Some('=') {
            reader.read(); // consume '='

            Ok(())
        } else {
            Err(ConfigFileError::new(
                reader.cursor().pos,
                "Expecting a value using space or '=' separator",
            ))
        }
    } else {
        Ok(())
    }
}

/// parse one or more spaces
fn spaces(reader: &mut Reader) -> String {
    reader.read_while(|c: char| c == ' ' || c == '\t')
}

#[cfg(test)]
mod tests {
    use hurl_core::reader::{CharPos, Pos};

    use super::*;

    #[test]
    fn test_parse_value_separator() {
        let mut reader = Reader::new("  x");
        parse_value_separator(&mut reader).unwrap();
        assert_eq!((reader.cursor().pos), Pos::new(1, 3));

        let mut reader = Reader::new("= x");
        parse_value_separator(&mut reader).unwrap();
        assert_eq!((reader.cursor().pos), Pos::new(1, 2));
    }

    #[test]
    fn test_parse_value_separator_error() {
        let mut reader = Reader::new("x");
        assert_eq!(
            parse_value_separator(&mut reader).unwrap_err(),
            ConfigFileError::new(
                Pos::new(1, 1),
                "Expecting a value using space or '=' separator"
            )
        );
    }

    #[test]
    fn test_skip_whitespace_and_comments() {
        let mut reader =
            Reader::new("# This is a comment\n   # Another comment\n   option=value\n");
        skip_whitespace_and_comments(&mut reader);
        assert_eq!((reader.cursor().pos), Pos::new(3, 4));
    }

    #[test]
    fn test_expect_end_of_line() {
        let mut reader = Reader::new("\n");
        expect_no_value(&mut reader).unwrap();
        assert_eq!((reader.cursor().pos), Pos::new(2, 1));

        let mut reader = Reader::new("  \n");
        expect_no_value(&mut reader).unwrap();
        assert_eq!((reader.cursor().pos), Pos::new(2, 1));

        let mut reader = Reader::new("  ");
        expect_no_value(&mut reader).unwrap();
        assert_eq!((reader.cursor().pos), Pos::new(1, 3));

        let mut reader = Reader::new(" # Comment\n");
        expect_no_value(&mut reader).unwrap();
        assert_eq!((reader.cursor().pos), Pos::new(2, 1));
    }

    #[test]
    fn test_expect_end_of_line_error() {
        let mut reader = Reader::new("x\n");
        assert_eq!(
            expect_no_value(&mut reader).unwrap_err(),
            ConfigFileError::new(Pos::new(1, 1), "Not expecting a value for this option")
        );
    }

    #[test]
    fn test_parse_value() {
        // You can have an empty value
        let mut reader = Reader::new("\n");
        assert_eq!(parse_value(&mut reader).unwrap(), "");
        assert_eq!((reader.cursor().index), CharPos(0));

        // Unquoted String
        let mut reader = Reader::new("1\n");
        assert_eq!(parse_value(&mut reader).unwrap(), "1");
        assert_eq!((reader.cursor().index), CharPos(1));

        // Quoted String
        let mut reader = Reader::new("\"Hello\"\n");
        assert_eq!(parse_value(&mut reader).unwrap(), "Hello");
        assert_eq!((reader.cursor().index), CharPos(8));

        // Value containing newline
        let mut reader = Reader::new("\"Hello\nBob!\"\n");
        assert_eq!(parse_value(&mut reader).unwrap(), "Hello\nBob!");
        assert_eq!((reader.cursor().index), CharPos(13));
    }

    #[test]
    fn test_inline_value_error() {
        let mut reader = Reader::new("\"1\n");
        assert_eq!(
            parse_value(&mut reader).unwrap_err(),
            ConfigFileError::new(Pos::new(2, 1), "Missing closing quote")
        );
        assert_eq!((reader.cursor().index), CharPos(3));
    }

    #[test]
    fn test_parse_quoted_value_with_trailing_whitespace_or_comment() {
        let mut reader = Reader::new("\"Hello\"   \n");
        assert_eq!(parse_value(&mut reader).unwrap(), "Hello");
        assert_eq!((reader.cursor().index), CharPos(11));

        let mut reader = Reader::new("\"Hello\" # this is a comment\n");
        assert_eq!(parse_value(&mut reader).unwrap(), "Hello");
        assert_eq!((reader.cursor().index), CharPos(28));
    }

    #[test]
    fn test_parse_quoted_value_error_trailing_chars() {
        let mut reader = Reader::new("\"Hello\" world\n");
        assert_eq!(
            parse_value(&mut reader).unwrap_err(),
            ConfigFileError::new(Pos::new(1, 9), "characters after the closing quote")
        );
    }
}
