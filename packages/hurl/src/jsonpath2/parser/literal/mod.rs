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
pub(crate) mod number;
pub(crate) mod string;

use crate::jsonpath2::ast::literal::Literal;
use crate::jsonpath2::parser::primitives::match_str;
use crate::jsonpath2::parser::{ParseError, ParseErrorKind, ParseResult};
use hurl_core::reader::Reader;
use number::try_number;
use string::try_parse as try_string;

/// Parse a literal
/// This includes standard JSON primitives (number, string, bool or null)
/// with the addition of string literals with quotes
///
/// Number can be either integer, or floats
/// If the number contains a decimal point or an exponent, it is parsed as a float (Like Serde::json)
/// 110 is an integer, but 110.0 or 1.1e2 are floats
#[allow(dead_code)]
pub fn parse(reader: &mut Reader) -> ParseResult<Literal> {
    if try_null(reader) {
        Ok(Literal::Null)
    } else if let Some(value) = try_boolean(reader) {
        Ok(Literal::Bool(value))
    } else if let Some(value) = number::try_number(reader)? {
        Ok(Literal::Number(value))
    } else if let Some(value) = string::try_parse(reader)? {
        Ok(Literal::String(value))
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("a literal".to_string()),
        ))
    }
}

#[allow(dead_code)]
pub fn try_parse(reader: &mut Reader) -> ParseResult<Option<Literal>> {
    if try_null(reader) {
        Ok(Some(Literal::Null))
    } else if let Some(value) = try_boolean(reader) {
        Ok(Some(Literal::Bool(value)))
    } else if let Some(value) = try_number(reader)? {
        Ok(Some(Literal::Number(value)))
    } else if let Some(value) = try_string(reader)? {
        Ok(Some(Literal::String(value)))
    } else {
        Ok(None)
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

#[cfg(test)]
mod tests {

    use super::*;
    use hurl_core::reader::{CharPos, Pos, Reader};

    #[test]
    pub fn test_literal() {
        let mut reader = Reader::new("null");
        assert_eq!(try_parse(&mut reader).unwrap().unwrap(), Literal::Null);
        assert_eq!(reader.cursor().index, CharPos(4));

        let mut reader = Reader::new("true");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            Literal::Bool(true)
        );
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
}
