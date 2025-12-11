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

use crate::jsonpath2::parser::primitives::{expect_str, match_str};
use crate::jsonpath2::parser::ParseResult;
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

fn try_double_quoted_string(reader: &mut Reader) -> ParseResult<Option<String>> {
    if match_str("\"", reader) {
        let s = reader.read_while(|c| c != '"');
        expect_str("\"", reader)?;
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

fn try_single_quoted_string(reader: &mut Reader) -> ParseResult<Option<String>> {
    if match_str("\'", reader) {
        let s = reader.read_while(|c| c != '\'');
        expect_str("\'", reader)?;
        Ok(Some(s))
    } else {
        Ok(None)
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
    fn test_string_literal_error() {
        let mut reader = Reader::new("'store");
        assert_eq!(
            try_parse(&mut reader).unwrap_err(),
            ParseError::new(Pos::new(1, 7), ParseErrorKind::Expecting("'".to_string()))
        );
    }
}
