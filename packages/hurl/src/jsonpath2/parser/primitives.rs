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

use super::{ParseError, ParseErrorKind, ParseResult};

pub fn literal(s: &str, reader: &mut Reader) -> ParseResult<()> {
    // does not return a value
    // => use combinator recover to make it recoverable
    let start = reader.cursor();
    if reader.is_eof() {
        let kind = ParseErrorKind::Expecting(s.to_string());
        let error = ParseError::new(start.pos, kind);
        return Err(error);
    }
    for c in s.chars() {
        match reader.read() {
            None => {
                let kind = ParseErrorKind::Expecting(s.to_string());
                let error = ParseError::new(start.pos, kind);
                return Err(error);
            }
            Some(x) => {
                if x != c {
                    let kind = ParseErrorKind::Expecting(s.to_string());
                    let error = ParseError::new(start.pos, kind);
                    return Err(error);
                } else {
                    continue;
                }
            }
        }
    }
    Ok(())
}

pub fn try_literal(s: &str, reader: &mut Reader) -> bool {
    let initial_state = reader.cursor();
    if literal(s, reader).is_ok() {
        true
    } else {
        reader.seek(initial_state);
        false
    }
}

/// Try to parse a string literal
/// if it does not start with a quote it returns `None` rather than a `ParseError`
///
// TODO: implement full spec with double-quoted and single-quoted parser
pub fn string_literal(reader: &mut Reader) -> ParseResult<Option<String>> {
    if try_literal("\"", reader) {
        let s = reader.read_while(|c| c != '"');
        literal("\"", reader)?;
        Ok(Some(s))
    } else if try_literal("'", reader) {
        let s = reader.read_while(|c| c != '\'');
        literal("'", reader)?;
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::reader::{CharPos, Pos};

    use super::*;

    #[test]
    fn test_literal() {
        let mut reader = Reader::new("hello");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.cursor().index, CharPos(5));

        let mut reader = Reader::new("hello ");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.cursor().index, CharPos(5));

        let mut reader = Reader::new("");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(
            error,
            ParseError::new(
                Pos { line: 1, column: 1 },
                ParseErrorKind::Expecting("hello".to_string())
            )
        );
        assert_eq!(reader.cursor().index, CharPos(0));

        let mut reader = Reader::new("hi");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(
            error,
            ParseError::new(
                Pos { line: 1, column: 1 },
                ParseErrorKind::Expecting("hello".to_string())
            )
        );

        assert_eq!(reader.cursor().index, CharPos(2));

        let mut reader = Reader::new("he");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(
            error,
            ParseError::new(
                Pos { line: 1, column: 1 },
                ParseErrorKind::Expecting("hello".to_string())
            )
        );
        assert_eq!(reader.cursor().index, CharPos(2));
    }

    #[test]
    fn test_string_literal() {
        let mut reader = Reader::new("'store'");
        assert_eq!(
            string_literal(&mut reader).unwrap().unwrap(),
            "store".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(7));
        let mut reader = Reader::new("\"store\"");
        assert_eq!(
            string_literal(&mut reader).unwrap().unwrap(),
            "store".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(7));

        let mut reader = Reader::new("0");
        assert!(string_literal(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    fn test_string_literal_error() {
        let mut reader = Reader::new("'store");
        assert_eq!(
            string_literal(&mut reader).unwrap_err(),
            ParseError::new(Pos::new(1, 7), ParseErrorKind::Expecting("'".to_string()))
        );
    }
}
