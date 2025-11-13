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

use crate::jsonpath2::ast::query::{AbsoluteQuery, Query, RelativeQuery};
use crate::jsonpath2::parser::primitives::{expect_str, match_str};
use crate::jsonpath2::parser::{segments, ParseError, ParseErrorKind, ParseResult};

pub fn parse(reader: &mut Reader) -> ParseResult<AbsoluteQuery> {
    expect_str("$", reader)?;
    let segments = segments::parse(reader)?;
    if reader.is_eof() {
        Ok(AbsoluteQuery::new(segments))
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("end of query".to_string()),
        ))
    }
}

#[allow(dead_code)]
pub fn try_filter_query(reader: &mut Reader) -> ParseResult<Option<Query>> {
    if let Some(relative_query) = try_relative_query(reader)? {
        Ok(Some(Query::RelativeQuery(relative_query)))
    } else if let Some(absolute_query) = try_absolute_query(reader)? {
        Ok(Some(Query::AbsoluteQuery(absolute_query)))
    } else {
        Ok(None)
    }
}

#[allow(dead_code)]
pub fn try_relative_query(reader: &mut Reader) -> ParseResult<Option<RelativeQuery>> {
    if match_str("@", reader) {
        let segments = segments::parse(reader)?;
        Ok(Some(RelativeQuery::new(segments)))
    } else {
        Ok(None)
    }
}

#[allow(dead_code)]
pub fn try_absolute_query(reader: &mut Reader) -> ParseResult<Option<AbsoluteQuery>> {
    if match_str("$", reader) {
        let segments = segments::parse(reader)?;
        Ok(Some(AbsoluteQuery::new(segments)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use super::super::{ParseError, ParseErrorKind};
    use crate::jsonpath2::ast::expr::{LogicalExpr, TestExpr, TestExprKind};
    use crate::jsonpath2::ast::query::{Query, RelativeQuery};
    use crate::jsonpath2::ast::segment::{ChildSegment, Segment};
    use crate::jsonpath2::ast::selector::{FilterSelector, NameSelector, Selector};
    use hurl_core::reader::{CharPos, Pos, Reader};

    use super::*;

    #[test]
    pub fn test_empty() {
        let mut reader = Reader::new("");
        assert_eq!(
            parse(&mut reader).unwrap_err(),
            ParseError::new(Pos::new(1, 1), ParseErrorKind::Expecting("$".to_string()))
        );
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    pub fn test_trailing_space() {
        let mut reader = Reader::new("$ ");
        assert_eq!(
            parse(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 2),
                ParseErrorKind::Expecting("end of query".to_string())
            )
        );
        assert_eq!(reader.cursor().index, CharPos(1));
    }

    #[test]
    pub fn test_root_identifier() {
        let mut reader = Reader::new("$");

        assert_eq!(parse(&mut reader).unwrap(), AbsoluteQuery::new(vec![]));
        assert_eq!(reader.cursor().index, CharPos(1));
    }

    #[test]
    pub fn test_child_segment() {
        let mut reader = Reader::new("$['store']");

        assert_eq!(
            parse(&mut reader).unwrap(),
            AbsoluteQuery::new(vec![Segment::Child(ChildSegment::new(vec![
                Selector::Name(NameSelector::new("store".to_string()))
            ]))])
        );
        assert_eq!(reader.cursor().index, CharPos(10));

        let mut reader = Reader::new("$[?@['isbn']]");
        assert_eq!(
            parse(&mut reader).unwrap(),
            AbsoluteQuery::new(vec![Segment::Child(ChildSegment::new(vec![
                Selector::Filter(FilterSelector::new(LogicalExpr::Test(TestExpr::new(
                    false,
                    TestExprKind::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                        Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                            "isbn".to_string()
                        ))]))
                    ])))
                ))))
            ]))])
        );
        assert_eq!(reader.cursor().index, CharPos(13));

        let mut reader = Reader::new("$.book");
        assert_eq!(
            parse(&mut reader).unwrap(),
            AbsoluteQuery::new(vec![Segment::Child(ChildSegment::new(vec![
                Selector::Name(NameSelector::new("book".to_string()))
            ]))])
        );
        assert_eq!(reader.cursor().index, CharPos(6));
    }

    #[test]
    pub fn test_relative_query() {
        let mut reader = Reader::new("@['isbn']");
        assert_eq!(
            try_relative_query(&mut reader).unwrap().unwrap(),
            RelativeQuery::new(vec![Segment::Child(ChildSegment::new(vec![
                Selector::Name(NameSelector::new("isbn".to_string()))
            ]))])
        );
        assert_eq!(reader.cursor().index, CharPos(9));
    }
}
