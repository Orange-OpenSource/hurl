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
use crate::jsonpath2::ast::expr::{LogicalExpr, TestExpr, TestExprKind};
use crate::jsonpath2::parser::primitives::match_str;
use crate::jsonpath2::parser::query::try_filter_query;
use crate::jsonpath2::parser::{ParseError, ParseErrorKind, ParseResult};
use hurl_core::reader::Reader;

#[allow(dead_code)]
pub fn logical_or_expr(reader: &mut Reader) -> ParseResult<LogicalExpr> {
    // TODO: parse several operands
    logical_and_expr(reader)
}

fn logical_and_expr(reader: &mut Reader) -> ParseResult<LogicalExpr> {
    // TODO: parse several operands
    basic_expr(reader)
}

fn basic_expr(reader: &mut Reader) -> ParseResult<LogicalExpr> {
    let save = reader.cursor();
    if let Some(test_expr) = try_test_expr(reader)? {
        Ok(LogicalExpr::Test(test_expr))
    } else {
        Err(ParseError::new(
            save.pos,
            ParseErrorKind::Expecting("basic expression".to_string()),
        ))
    }
}

fn try_test_expr(reader: &mut Reader) -> ParseResult<Option<TestExpr>> {
    let not = match_str("!", reader);

    if let Some(query) = try_filter_query(reader)? {
        let kind = TestExprKind::FilterQuery(query);
        Ok(Some(TestExpr::new(not, kind)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::jsonpath2::ast::expr::{LogicalExpr, TestExpr, TestExprKind};
    use crate::jsonpath2::ast::query::{AbsoluteQuery, Query, RelativeQuery};
    use crate::jsonpath2::ast::segment::{ChildSegment, Segment};
    use crate::jsonpath2::ast::selector::{NameSelector, Selector};
    use hurl_core::reader::Reader;

    #[test]
    fn test_parse_logical_or_expr() {
        let mut reader = Reader::new("@.b");
        assert_eq!(
            logical_or_expr(&mut reader).unwrap(),
            LogicalExpr::Test(TestExpr::new(
                false,
                TestExprKind::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                    Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                        "b".to_string()
                    ))]))
                ])))
            ))
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(3));
    }

    #[test]
    fn test_parse_test_expr() {
        let mut reader = Reader::new("@.b");
        assert_eq!(
            try_test_expr(&mut reader).unwrap().unwrap(),
            TestExpr::new(
                false,
                TestExprKind::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                    Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                        "b".to_string()
                    ))]))
                ])))
            )
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(3));

        let mut reader = Reader::new("$");
        assert_eq!(
            try_test_expr(&mut reader).unwrap().unwrap(),
            TestExpr::new(
                false,
                TestExprKind::FilterQuery(Query::AbsoluteQuery(AbsoluteQuery::new(vec![])))
            )
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(1));
    }
}
