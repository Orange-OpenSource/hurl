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

use crate::jsonpath2::parser::primitives::{expect_str, match_str};
use crate::jsonpath2::parser::{ParseError, ParseErrorKind, ParseResult};
use hurl_core::reader::Reader;

/// Try to parse a string literal
/// if it does not start with a quote it returns `None` rather than a `ParseError`
pub fn try_parse(reader: &mut Reader) -> ParseResult<Option<String>> {
    if let Some(s) = try_double_quoted_string(reader)? {
        Ok(Some(s))
    } else if let Some(s) = try_single_quoted_string(reader)? {
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

/// Try to parse a double-quoted string
fn try_double_quoted_string(reader: &mut Reader) -> ParseResult<Option<String>> {
    if match_str("\"", reader) {
        let mut result = String::new();

        while !reader.is_eof() {
            let pos = reader.cursor().pos;
            let ch = reader.peek().unwrap_or('\0');

            if ch == '"' {
                // End of string
                reader.read();
                return Ok(Some(result));
            } else if ch == '\'' {
                // Single quote is allowed in double-quoted strings
                result.push(ch);
                reader.read();
            } else if ch == '\\' {
                // Escape sequence
                reader.read(); // consume backslash
                let escaped = parse_escape_sequence(reader, '"')?;

                result.push(escaped);
            } else if is_unescaped_char(ch) {
                result.push(ch);
                reader.read();
            } else {
                return Err(ParseError::new(pos, ParseErrorKind::InvalidCharacter(ch)));
            }
        }

        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("\"".to_string()),
        ))
    } else {
        Ok(None)
    }
}

/// Try to parse a single-quoted string literal
fn try_single_quoted_string(reader: &mut Reader) -> ParseResult<Option<String>> {
    if match_str("\'", reader) {
        let mut result = String::new();

        while !reader.is_eof() {
            let pos = reader.cursor().pos;
            let ch = reader.peek().unwrap_or('\0');

            if ch == '\'' {
                // End of string
                reader.read();
                return Ok(Some(result));
            } else if ch == '"' {
                // Double quote is allowed in single-quoted strings
                result.push(ch);
                reader.read();
            } else if ch == '\\' {
                // Escape sequence
                reader.read(); // consume backslash
                let escaped = parse_escape_sequence(reader, '\'')?;
                result.push(escaped);
            } else if is_unescaped_char(ch) {
                result.push(ch);
                reader.read();
            } else {
                return Err(ParseError::new(pos, ParseErrorKind::InvalidCharacter(ch)));
            }
        }

        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("'".to_string()),
        ))
    } else {
        Ok(None)
    }
}

/// Check if character is unescaped according to the spec
fn is_unescaped_char(ch: char) -> bool {
    let code = ch as u32;

    // unescaped = %x20-21 / %x23-26 / %x28-5B / %x5D-D7FF / %xE000-10FFFF
    (0x20..=0x21).contains(&code) ||  // omit 0x22 "
    (0x23..=0x26).contains(&code) ||  // omit 0x27 '
    (0x28..=0x5B).contains(&code) ||  // omit 0x5C \
    (0x5D..=0xD7FF).contains(&code) || // skip surrogate code points
    (0xE000..=0x10FFFF).contains(&code)
}

/// Parse escape sequence after backslash
fn parse_escape_sequence(reader: &mut Reader, quote_char: char) -> ParseResult<char> {
    let pos = reader.cursor().pos;

    let ch = if let Some(value) = reader.read() {
        value
    } else {
        return Err(ParseError::new(
            pos,
            ParseErrorKind::Expecting("escape character".to_string()),
        ));
    };

    match ch {
        'b' => Ok('\u{0008}'),                  // BS backspace
        'f' => Ok('\u{000C}'),                  // FF form feed
        'n' => Ok('\n'),                        // LF line feed
        'r' => Ok('\r'),                        // CR carriage return
        't' => Ok('\t'),                        // HT horizontal tab
        '/' => Ok('/'),                         // slash
        '\\' => Ok('\\'),                       // backslash
        '"' if quote_char == '"' => Ok('"'),    // escaped double quote in double-quoted string
        '\'' if quote_char == '\'' => Ok('\''), // escaped single quote in single-quoted string
        'u' => {
            // Unicode escape sequence \uXXXX
            parse_unicode_escape(reader)
        }
        _ => Err(ParseError::new(
            pos,
            ParseErrorKind::InvalidEscapeSequence(format!("\\{}", ch)),
        )),
    }
}

/// Parse Unicode escape sequence after \u
fn parse_unicode_escape(reader: &mut Reader) -> ParseResult<char> {
    if let Some(ch) = try_non_surrogate(reader)? {
        Ok(ch)
    } else if let Some(ch) = try_surrogate_pair(reader)? {
        Ok(ch)
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::InvalidUnicodeEscape("invalid unicode escape".to_string()),
        ))
    }
}

/// Try to parse a non-surrogate Unicode code unit
fn try_non_surrogate(reader: &mut Reader) -> ParseResult<Option<char>> {
    let save = reader.cursor();
    let c1 = hex_digit(reader)?;
    if c1 == 13 {
        // D
        let c2 = hex_digit(reader)?;
        if c2 >= 8 {
            reader.seek(save);
            Ok(None)
        } else {
            let c3 = hex_digit(reader)?;
            let c4 = hex_digit(reader)?;
            let code_point = c1 * 4096 + c2 * 256 + c3 * 16 + c4;
            Ok(Some(char::from_u32(code_point).ok_or_else(|| {
                ParseError::new(
                    save.pos,
                    ParseErrorKind::InvalidUnicodeEscape(format!("{:04X}", code_point)),
                )
            })?))
        }
    } else {
        let c2 = hex_digit(reader)?;
        let c3 = hex_digit(reader)?;
        let c4 = hex_digit(reader)?;
        let code_point = c1 * 4096 + c2 * 256 + c3 * 16 + c4;
        Ok(Some(char::from_u32(code_point).ok_or_else(|| {
            ParseError::new(
                save.pos,
                ParseErrorKind::InvalidUnicodeEscape(format!("{:04X}", code_point)),
            )
        })?))
    }
}

/// Try to parse a surrogate pair Unicode code unit
fn try_surrogate_pair(reader: &mut Reader) -> ParseResult<Option<char>> {
    let pos = reader.cursor().pos;
    if let Some(high_surrogate) = try_high_surrogate(reader)? {
        expect_str("\\u", reader)?;
        let low_surrogate = low_surrogate(reader)?;
        let combined = 0x10000 + (high_surrogate << 10) + low_surrogate;
        Ok(Some(char::from_u32(combined).ok_or_else(|| {
            ParseError::new(
                pos,
                ParseErrorKind::InvalidUnicodeEscape(format!("{:06X}", combined)),
            )
        })?))
    } else {
        Ok(None)
    }
}

/// Try to parse a high surrogate code unit
/// If found, returns the value of the high surrogate 10 bits
fn try_high_surrogate(reader: &mut Reader) -> ParseResult<Option<u32>> {
    if match_str("D", reader) {
        let c1 = hex_digit(reader)?;
        if (8..=11).contains(&c1) {
            let c2 = hex_digit(reader)?;
            let c3 = hex_digit(reader)?;
            Ok(Some((c1 - 8) * 256 + c2 * 16 + c3))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

/// Parse a low surrogate code unit
/// If found, returns the value of the low surrogate 10 bits
fn low_surrogate(reader: &mut Reader) -> ParseResult<u32> {
    let pos = reader.cursor().pos;
    expect_str("D", reader).map_err(|_| {
        ParseError::new(pos, ParseErrorKind::Expecting("low surrogate".to_string()))
    })?;
    let c1 = hex_digit(reader)?;
    if c1 >= 12 {
        let c2 = hex_digit(reader)?;
        let c3 = hex_digit(reader)?;
        Ok((c1 - 12) * 256 + c2 * 16 + c3)
    } else {
        Err(ParseError::new(
            pos,
            ParseErrorKind::Expecting("low surrogate".to_string()),
        ))
    }
}

/// Parse a single hex digit and return its value
fn hex_digit(reader: &mut Reader) -> ParseResult<u32> {
    let pos = reader.cursor().pos;
    if let Some(ch) = reader.read() {
        if ch.is_ascii_hexdigit() {
            let value = ch.to_digit(16).unwrap();
            Ok(value)
        } else {
            Err(ParseError::new(
                pos,
                ParseErrorKind::Expecting("hex digit".to_string()),
            ))
        }
    } else {
        Err(ParseError::new(
            pos,
            ParseErrorKind::Expecting("hex digit".to_string()),
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::jsonpath2::parser::{ParseError, ParseErrorKind};
    use hurl_core::reader::{CharPos, Pos, Reader};

    #[test]
    fn test_string_literal() {
        let mut reader = Reader::new("'store'");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "store".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(7));

        let mut reader = Reader::new("\"store\"");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "store".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(7));

        let mut reader = Reader::new("0");
        assert!(try_parse(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    fn test_escape_character() {
        // Test escaped quotes
        let mut reader = Reader::new("'quoted\\' literal'");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "quoted' literal".to_string()
        );

        let mut reader = Reader::new("\"quoted\\\" literal\"");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "quoted\" literal".to_string()
        );

        // Test standard escape sequences
        let mut reader = Reader::new("\"line1\\nline2\"");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "line1\nline2".to_string()
        );

        let mut reader = Reader::new("\"tab\\there\"");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "tab\there".to_string()
        );

        let mut reader = Reader::new("\"back\\\\slash\"");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "back\\slash".to_string()
        );

        let mut reader = Reader::new("\"slash\\/here\"");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "slash/here".to_string()
        );
    }

    #[test]
    fn test_unicode_escape() {
        // Basic Unicode escape
        let mut reader = Reader::new("\"Hello \\u0041\"");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "Hello A".to_string()
        );

        // Test valid 4-digit Unicode character (π - pi symbol)
        let mut reader = Reader::new("\"\\u03C0\"");
        assert_eq!(try_parse(&mut reader).unwrap().unwrap(), "π".to_string());

        // Test another valid 4-digit Unicode character (© - copyright symbol)
        let mut reader = Reader::new("\"\\u00A9\"");
        assert_eq!(try_parse(&mut reader).unwrap().unwrap(), "©".to_string());

        // Unicode surrogate pair - emoji
        let mut reader = Reader::new("\"\\uD83D\\uDE00\"");
        assert_eq!(try_parse(&mut reader).unwrap().unwrap(), "😀".to_string());
    }

    #[test]
    fn test_mixed_quotes() {
        // Single quote inside double-quoted string
        let mut reader = Reader::new("\"it's fine\"");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "it's fine".to_string()
        );

        // Double quote inside single-quoted string
        let mut reader = Reader::new("'say \"hello\"'");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            "say \"hello\"".to_string()
        );
    }

    #[test]
    fn test_string_literal_error() {
        let mut reader = Reader::new("'store");
        assert_eq!(
            try_parse(&mut reader).unwrap_err(),
            ParseError::new(Pos::new(1, 7), ParseErrorKind::Expecting("'".to_string()))
        );

        let mut reader = Reader::new("\"store");
        assert_eq!(
            try_parse(&mut reader).unwrap_err(),
            ParseError::new(Pos::new(1, 7), ParseErrorKind::Expecting("\"".to_string()))
        );
    }

    #[test]
    fn test_invalid_escape_sequences() {
        let mut reader = Reader::new("\"invalid\\x escape\"");
        assert!(try_parse(&mut reader).is_err());

        let mut reader = Reader::new("\"incomplete\\u123\"");
        assert!(try_parse(&mut reader).is_err());

        let mut reader = Reader::new("\"incomplete\\u\"");
        assert!(try_parse(&mut reader).is_err());
    }

    #[test]
    fn test_parse_escape_sequence() {
        let mut reader = Reader::new("/");
        assert_eq!(parse_escape_sequence(&mut reader, '"').unwrap(), '/');
        assert_eq!(reader.cursor().index, CharPos(1));

        // Unicode Character 'GRINNING FACE' (U+1F600)
        let mut reader = Reader::new("uD83D\\uDE00\"");
        assert_eq!(parse_escape_sequence(&mut reader, '"').unwrap(), '😀');
        assert_eq!(reader.cursor().index, CharPos(11));
    }

    #[test]
    fn test_parse_unicode_escape() {
        let mut reader = Reader::new("00E9");
        assert_eq!(parse_unicode_escape(&mut reader).unwrap(), 'é');
        assert_eq!(reader.cursor().index, CharPos(4));

        // Unicode Character 'GRINNING FACE' (U+1F600)
        let mut reader = Reader::new("D83D\\uDE00\"");
        assert_eq!(parse_unicode_escape(&mut reader).unwrap(), '😀');
        assert_eq!(reader.cursor().index, CharPos(10));
    }

    #[test]
    fn test_non_surrogate() {
        let mut reader = Reader::new("00E9");
        assert_eq!(try_non_surrogate(&mut reader).unwrap().unwrap(), 'é');
        assert_eq!(reader.cursor().index, CharPos(4));

        let mut reader = Reader::new("D83D");
        assert!(try_non_surrogate(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    fn test_surrogate_pairs() {
        // Unicode Character 'GRINNING FACE' (U+1F600)
        let mut reader = Reader::new("D83D\\uDE00\"");
        assert_eq!(try_surrogate_pair(&mut reader).unwrap().unwrap(), '😀');
        assert_eq!(reader.cursor().index, CharPos(10));

        let mut reader = Reader::new("00E9");
        assert!(try_surrogate_pair(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));

        let mut reader = Reader::new("D83D\\u00E9\"");
        assert_eq!(
            try_surrogate_pair(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 7),
                ParseErrorKind::Expecting("low surrogate".to_string())
            )
        );
    }

    #[test]
    fn test_high_surrogate() {
        let mut reader = Reader::new("D83D");
        assert_eq!(try_high_surrogate(&mut reader).unwrap().unwrap(), 61);
        assert_eq!(reader.cursor().index, CharPos(4));

        let mut reader = Reader::new("00E9");
        assert!(try_high_surrogate(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    fn test_low_surrogate() {
        let mut reader = Reader::new("DE00");
        assert_eq!(low_surrogate(&mut reader).unwrap(), 512);
        assert_eq!(reader.cursor().index, CharPos(4));

        let mut reader = Reader::new("00E9");
        assert_eq!(
            low_surrogate(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 1),
                ParseErrorKind::Expecting("low surrogate".to_string())
            )
        );
    }

    #[test]
    fn test_hex_digit() {
        let mut reader = Reader::new("D83D");
        assert_eq!(hex_digit(&mut reader).unwrap(), 13);
        assert_eq!(reader.cursor().index, CharPos(1));

        let mut reader = Reader::new("x");
        assert_eq!(
            hex_digit(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 1),
                ParseErrorKind::Expecting("hex digit".to_string())
            )
        );
    }
}
