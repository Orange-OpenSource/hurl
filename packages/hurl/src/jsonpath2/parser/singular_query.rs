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

use crate::jsonpath2::ast::selector::{IndexSelector, NameSelector};
use crate::jsonpath2::ast::singular_query::{
    AbsoluteSingularQuery, RelativeSingularQuery, SingularQuery, SingularQuerySegment,
};
use crate::jsonpath2::parser::literal::try_integer;
use crate::jsonpath2::parser::primitives::expect_str;
use crate::jsonpath2::parser::primitives::match_str;
use crate::jsonpath2::parser::selectors::try_name_selector;
use crate::jsonpath2::parser::ParseResult;
use crate::jsonpath2::parser::{ParseError, ParseErrorKind};
use hurl_core::reader::Reader;

/// Try to parse a singular query.
/// It differs from a regular query in that it only matches a single node.
#[allow(dead_code)]
pub fn try_parse(reader: &mut Reader) -> ParseResult<Option<SingularQuery>> {
    if match_str("$", reader) {
        let segments = singular_query_segments(reader)?;
        Ok(Some(SingularQuery::Absolute(AbsoluteSingularQuery::new(
            segments,
        ))))
    } else if match_str("@", reader) {
        let segments = singular_query_segments(reader)?;
        Ok(Some(SingularQuery::Relative(RelativeSingularQuery::new(
            segments,
        ))))
    } else {
        Ok(None)
    }
}

/// Parse singular segments.
fn singular_query_segments(reader: &mut Reader) -> ParseResult<Vec<SingularQuerySegment>> {
    let mut segments = vec![];
    while let Some(segment) = try_singular_query_segment(reader)? {
        segments.push(segment);
    }
    Ok(segments)
}

/// Try to parse a singular query segment.
fn try_singular_query_segment(reader: &mut Reader) -> ParseResult<Option<SingularQuerySegment>> {
    let save = reader.cursor();
    if match_str("[", reader) {
        if let Some(name) = try_name_selector(reader)? {
            expect_str("]", reader)?;
            Ok(Some(SingularQuerySegment::Name(name)))
        } else if let Some(value) = try_integer(reader)? {
            expect_str("]", reader)?;
            Ok(Some(SingularQuerySegment::Index(IndexSelector::new(value))))
        } else {
            Ok(None)
        }
    } else if match_str(".", reader) {
        match member_name_shorthand(reader) {
            Ok(name) => Ok(Some(SingularQuerySegment::Name(NameSelector::new(name)))),
            Err(_) => {
                reader.seek(save);
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

fn member_name_shorthand(reader: &mut Reader) -> ParseResult<String> {
    let mut name = alpha(reader)?.to_string();
    name.push_str(&reader.read_while(|c| c.is_alphanumeric()));
    Ok(name)
}

fn alpha(reader: &mut Reader) -> ParseResult<char> {
    let pos = reader.cursor().pos;
    if let Some(c) = reader.read() {
        if c.is_alphabetic() {
            Ok(c)
        } else {
            let kind = ParseErrorKind::Expecting("a character".to_string());
            Err(ParseError::new(pos, kind))
        }
    } else {
        let kind = ParseErrorKind::Expecting("a character".to_string());
        Err(ParseError::new(pos, kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jsonpath2::ast::singular_query::{
        AbsoluteSingularQuery, SingularQuery, SingularQuerySegment,
    };
    use hurl_core::reader::{CharPos, Reader};

    #[test]
    pub fn test_singular_query() {
        let mut reader = Reader::new("$.store");

        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            SingularQuery::Absolute(AbsoluteSingularQuery::new(vec![
                SingularQuerySegment::Name(NameSelector::new("store".to_string()))
            ]))
        );
        assert_eq!(reader.cursor().index, CharPos(7));
    }

    #[test]
    pub fn test_singular_query_none() {
        let mut reader = Reader::new("1");
        assert!(try_parse(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    pub fn test_singular_query_segment() {
        let mut reader = Reader::new(".store");
        assert_eq!(
            try_singular_query_segment(&mut reader).unwrap().unwrap(),
            SingularQuerySegment::Name(NameSelector::new("store".to_string()))
        );
        assert_eq!(reader.cursor().index, CharPos(6));

        let mut reader = Reader::new("]");
        assert!(try_singular_query_segment(&mut reader).unwrap().is_none(),);
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    pub fn test_singular_query_segment_error() {
        let mut reader = Reader::new(".*");
        assert!(try_singular_query_segment(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }
}
