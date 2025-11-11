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

use hurl_core::reader::Reader;

use crate::jsonpath2::ast::literal::Literal;
use crate::jsonpath2::parser::{
    primitives::{expect_str, match_str},
    ParseError, ParseErrorKind, ParseResult,
};

/// Parse a literal
/// This includes standard JSON primitives (number, string, bool or null)
/// with the addition of string literals with quotes
// TODO: implement full spec with number parsing
#[allow(dead_code)]
pub fn parse(reader: &mut Reader) -> ParseResult<Literal> {
    if try_null(reader) {
        Ok(Literal::Null)
    } else if let Some(value) = try_boolean(reader) {
        Ok(Literal::Bool(value))
    } else if let Some(value) = try_integer(reader)? {
        Ok(Literal::Integer(value))
    } else if let Some(value) = try_string_literal(reader)? {
        Ok(Literal::String(value))
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("a literal".to_string()),
        ))
    }
}

/// Try to parse a boolean literal
#[allow(dead_code)]
fn try_boolean(reader: &mut Reader) -> Option<bool> {
    if match_str("true", reader) {
        Some(true)
    } else if match_str("false", reader) {
        Some(false)
    } else {
        None
    }
}

/// Try to parse a null literal
#[allow(dead_code)]
fn try_null(reader: &mut Reader) -> bool {
    match_str("null", reader)
}

/// Try to parse a decimal integer
/// if it does not start with a minus sign or a digit
/// it returns `None` rather than a `ParseError`
///
// TODO: implement full spec
pub fn try_integer(reader: &mut Reader) -> ParseResult<Option<i32>> {
    if match_str("0", reader) {
        return Ok(Some(0));
    }
    let negative = match_str("-", reader);
    let saved_pos = reader.cursor().pos;
    let s = reader.read_while(|c| c.is_ascii_digit());

    if s.is_empty() || s.starts_with('0') {
        if negative {
            let kind = ParseErrorKind::Expecting("strictly positive digit".to_string());
            return Err(ParseError::new(saved_pos, kind));
        } else {
            return Ok(None);
        }
    }
    let sign = if negative { -1 } else { 1 };
    Ok(Some(sign * s.parse::<i32>().unwrap()))
}

/// Try to parse a string literal
/// if it does not start with a quote it returns `None` rather than a `ParseError`
///
// TODO: implement full spec with double-quoted and single-quoted parser
pub fn try_string_literal(reader: &mut Reader) -> ParseResult<Option<String>> {
    if match_str("\"", reader) {
        let s = reader.read_while(|c| c != '"');
        expect_str("\"", reader)?;
        Ok(Some(s))
    } else if match_str("'", reader) {
        let s = reader.read_while(|c| c != '\'');
        expect_str("'", reader)?;
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use hurl_core::reader::{CharPos, Pos, Reader};

    #[test]
    pub fn test_literal() {
        let mut reader = Reader::new("null");
        assert_eq!(parse(&mut reader).unwrap(), Literal::Null);
        assert_eq!(reader.cursor().index, CharPos(4));

        let mut reader = Reader::new("true");
        assert_eq!(parse(&mut reader).unwrap(), Literal::Bool(true));
        assert_eq!(reader.cursor().index, CharPos(4));
    }

    #[test]
    pub fn test_literal_error() {
        let mut reader = Reader::new("NULL");
        assert_eq!(
            parse(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 1),
                ParseErrorKind::Expecting("a literal".to_string())
            )
        );
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    fn test_string_literal() {
        let mut reader = Reader::new("'store'");
        assert_eq!(
            try_string_literal(&mut reader).unwrap().unwrap(),
            "store".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(7));
        let mut reader = Reader::new("\"store\"");
        assert_eq!(
            try_string_literal(&mut reader).unwrap().unwrap(),
            "store".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(7));

        let mut reader = Reader::new("0");
        assert!(try_string_literal(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    fn test_string_literal_error() {
        let mut reader = Reader::new("'store");
        assert_eq!(
            try_string_literal(&mut reader).unwrap_err(),
            ParseError::new(Pos::new(1, 7), ParseErrorKind::Expecting("'".to_string()))
        );
    }

    #[test]
    fn test_integer() {
        let mut reader = Reader::new("1");
        assert_eq!(try_integer(&mut reader).unwrap().unwrap(), 1);
        assert_eq!(reader.cursor().index, CharPos(1));
    }
}
