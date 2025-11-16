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
use crate::jsonpath2::ast::expr::{AndExpr, LogicalExpr, NotExpr, OrExpr, TestExpr, TestExprKind};
use crate::jsonpath2::parser::comparison::try_parse as try_comparison;
use crate::jsonpath2::parser::primitives::match_str;
use crate::jsonpath2::parser::query::try_filter_query;
use crate::jsonpath2::parser::{ParseError, ParseErrorKind, ParseResult};
use hurl_core::reader::Reader;

#[allow(dead_code)]
pub fn logical_or_expr(reader: &mut Reader) -> ParseResult<LogicalExpr> {
    let mut operands = vec![];

    operands.push(logical_and_expr(reader)?);

    // Parse additional operands separated by "||"
    loop {
        if !match_str("||", reader) {
            break;
        }
        operands.push(logical_or_expr(reader)?);
    }

    // If we only have one operand, return it directly
    if operands.len() == 1 {
        Ok(operands.into_iter().next().unwrap())
    } else {
        Ok(LogicalExpr::Or(OrExpr::new(operands)))
    }
}

fn logical_and_expr(reader: &mut Reader) -> ParseResult<LogicalExpr> {
    let mut operands = vec![];

    operands.push(basic_expr(reader)?);

    // Parse additional operands separated by "&&"
    loop {
        if !match_str("&&", reader) {
            break;
        }
        operands.push(basic_expr(reader)?);
    }

    // If we only have one operand, return it directly
    if operands.len() == 1 {
        Ok(operands.into_iter().next().unwrap())
    } else {
        Ok(LogicalExpr::And(AndExpr::new(operands)))
    }
}

fn basic_expr(reader: &mut Reader) -> ParseResult<LogicalExpr> {
    let save = reader.cursor();
    if let Some(expr) = try_paren_expr(reader)? {
        Ok(expr)
    } else if let Some(comparison_expr) = try_comparison(reader)? {
        Ok(LogicalExpr::Comparison(comparison_expr))
    } else if let Some(test_expr) = try_test_expr(reader)? {
        Ok(LogicalExpr::Test(test_expr))
    } else {
        Err(ParseError::new(
            save.pos,
            ParseErrorKind::Expecting("basic expression".to_string()),
        ))
    }
}

fn try_paren_expr(reader: &mut Reader) -> ParseResult<Option<LogicalExpr>> {
    let save = reader.cursor();
    let not = match_str("!", reader);
    if match_str("(", reader) {
        let expr = logical_or_expr(reader)?;
        if match_str(")", reader) {
            let logical_expr = if not {
                LogicalExpr::Not(NotExpr::new(expr))
            } else {
                expr
            };
            Ok(Some(logical_expr))
        } else {
            Err(ParseError::new(
                reader.cursor().pos,
                ParseErrorKind::Expecting("')'".to_string()),
            ))
        }
    } else {
        reader.seek(save);
        Ok(None)
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

    use crate::jsonpath2::ast::comparison::{Comparable, ComparisonExpr, ComparisonOp};
    use crate::jsonpath2::ast::expr::{AndExpr, LogicalExpr, TestExpr, TestExprKind};
    use crate::jsonpath2::ast::literal::Literal;
    use crate::jsonpath2::ast::query::{AbsoluteQuery, Query, RelativeQuery};
    use crate::jsonpath2::ast::segment::{ChildSegment, Segment};
    use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
    use crate::jsonpath2::ast::singular_query::{RelativeSingularQuery, SingularQuery};
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
    fn test_parse_or_expression() {
        let mut reader = Reader::new("@<2||@>4");
        assert_eq!(
            logical_or_expr(&mut reader).unwrap(),
            LogicalExpr::Or(OrExpr::new(vec![
                LogicalExpr::Comparison(ComparisonExpr::new(
                    Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                        vec![]
                    ))),
                    Comparable::Literal(Literal::Integer(2)),
                    ComparisonOp::Less
                )),
                LogicalExpr::Comparison(ComparisonExpr::new(
                    Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                        vec![]
                    ))),
                    Comparable::Literal(Literal::Integer(4)),
                    ComparisonOp::Greater
                ))
            ]))
        );
    }

    #[test]
    fn test_parse_and_expression() {
        let mut reader = Reader::new("@>1&&@<4");
        assert_eq!(
            logical_and_expr(&mut reader).unwrap(),
            LogicalExpr::And(AndExpr::new(vec![
                LogicalExpr::Comparison(ComparisonExpr::new(
                    Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                        vec![]
                    ))),
                    Comparable::Literal(Literal::Integer(1)),
                    ComparisonOp::Greater
                )),
                LogicalExpr::Comparison(ComparisonExpr::new(
                    Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                        vec![]
                    ))),
                    Comparable::Literal(Literal::Integer(4)),
                    ComparisonOp::Less
                ))
            ]))
        );
    }

    #[test]
    fn test_parse_paren_expression() {
        let mut reader = Reader::new("!(@.b)");
        assert_eq!(
            try_paren_expr(&mut reader).unwrap().unwrap(),
            LogicalExpr::Not(NotExpr::new(LogicalExpr::Test(TestExpr::new(
                false,
                TestExprKind::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                    Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                        "b".to_string()
                    ))]))
                ])))
            ))))
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(6));
    }

    #[test]
    fn test_parse_paren_expression_none() {
        let mut reader = Reader::new("!@.b");
        assert!(try_paren_expr(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(0));
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

        let mut reader = Reader::new("$.*.a]");
        assert_eq!(
            try_test_expr(&mut reader).unwrap().unwrap(),
            TestExpr::new(
                false,
                TestExprKind::FilterQuery(Query::AbsoluteQuery(AbsoluteQuery::new(vec![
                    Segment::Child(ChildSegment::new(vec![Selector::Wildcard(
                        WildcardSelector
                    )])),
                    Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                        "a".to_string()
                    ))]))
                ])))
            )
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(5));

        let mut reader = Reader::new("!@.a");
        assert_eq!(
            try_test_expr(&mut reader).unwrap().unwrap(),
            TestExpr::new(
                true,
                TestExprKind::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                    Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                        "a".to_string()
                    ))]))
                ])))
            )
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(4));
    }
}
