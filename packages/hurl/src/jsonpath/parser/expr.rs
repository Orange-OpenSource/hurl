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
use crate::jsonpath::ast::expr::{AndExpr, LogicalExpr, NotExpr, OrExpr, TestExpr, TestExprKind};
use crate::jsonpath::parser::comparison::try_parse as try_comparison;
use crate::jsonpath::parser::function::functions;
use crate::jsonpath::parser::primitives::{match_str, skip_whitespace};
use crate::jsonpath::parser::query::try_filter_query;
use crate::jsonpath::parser::{ParseError, ParseErrorKind, ParseResult};
use hurl_core::reader::Reader;

pub(super) const MAX_NESTING_DEPTH: usize = 64;

/// Parse a logical expression using the depth shared with nested filters and functions.
pub fn logical_or_expr(reader: &mut Reader, depth: usize) -> ParseResult<LogicalExpr> {
    check_depth(reader, depth)?;

    let mut operands = vec![];

    operands.push(logical_and_expr(reader, depth)?);

    // Parse additional operands separated by "||"
    loop {
        skip_whitespace(reader);
        if !match_str("||", reader) {
            break;
        }
        skip_whitespace(reader);
        operands.push(logical_or_expr(reader, depth + 1)?);
    }

    // If we only have one operand, return it directly
    if operands.len() == 1 {
        Ok(operands.into_iter().next().unwrap())
    } else {
        Ok(LogicalExpr::Or(OrExpr::new(operands)))
    }
}

pub(super) fn check_depth(reader: &Reader, depth: usize) -> ParseResult<()> {
    if depth >= MAX_NESTING_DEPTH {
        return Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::MaxDepthExceeded {
                max_depth: MAX_NESTING_DEPTH,
            },
        ));
    }

    Ok(())
}

fn logical_and_expr(reader: &mut Reader, depth: usize) -> ParseResult<LogicalExpr> {
    let mut operands = vec![];

    operands.push(basic_expr(reader, depth)?);

    // Parse additional operands separated by "&&"
    loop {
        skip_whitespace(reader);
        if !match_str("&&", reader) {
            break;
        }
        skip_whitespace(reader);
        operands.push(basic_expr(reader, depth)?);
    }

    // If we only have one operand, return it directly
    if operands.len() == 1 {
        Ok(operands.into_iter().next().unwrap())
    } else {
        Ok(LogicalExpr::And(AndExpr::new(operands)))
    }
}

fn basic_expr(reader: &mut Reader, depth: usize) -> ParseResult<LogicalExpr> {
    let save = reader.cursor();
    if let Some(expr) = try_paren_expr(reader, depth)? {
        Ok(expr)
    } else if let Some(comparison_expr) = try_comparison(reader, depth)? {
        Ok(LogicalExpr::Comparison(comparison_expr))
    } else if let Some(test_expr) = try_test_expr(reader, depth)? {
        Ok(LogicalExpr::Test(test_expr))
    } else {
        Err(ParseError::new(
            save.pos,
            ParseErrorKind::Expecting("basic expression".to_string()),
        ))
    }
}

fn try_paren_expr(reader: &mut Reader, depth: usize) -> ParseResult<Option<LogicalExpr>> {
    let save = reader.cursor();
    let not = match_str("!", reader);
    skip_whitespace(reader);
    if match_str("(", reader) {
        let expr = logical_or_expr(reader, depth + 1)?;
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

fn try_test_expr(reader: &mut Reader, depth: usize) -> ParseResult<Option<TestExpr>> {
    let not = match_str("!", reader);
    skip_whitespace(reader);
    if let Some(query) = try_filter_query(reader, depth + 1)? {
        let kind = TestExprKind::FilterQuery(query);
        Ok(Some(TestExpr::new(not, kind)))
    } else if let Some(logical_type_function) = functions::try_logical_type_function(reader, depth)?
    {
        let kind = TestExprKind::LogicalTypeFunction(Box::new(logical_type_function));
        Ok(Some(TestExpr::new(not, kind)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::jsonpath::ast::comparison::{Comparable, ComparisonExpr, ComparisonOp};
    use crate::jsonpath::ast::expr::{AndExpr, LogicalExpr, TestExpr, TestExprKind};
    use crate::jsonpath::ast::function::argument::{RegexValueTypeArgument, ValueTypeArgument};
    use crate::jsonpath::ast::function::functions::LogicalTypeFunction;
    use crate::jsonpath::ast::literal::{Literal, Number};
    use crate::jsonpath::ast::query::{AbsoluteQuery, Query, RelativeQuery};
    use crate::jsonpath::ast::segment::{ChildSegment, Segment};
    use crate::jsonpath::ast::selector::{NameSelector, Selector, WildcardSelector};
    use crate::jsonpath::ast::singular_query::{
        RelativeSingularQuery, SingularQuery, SingularQuerySegment,
    };
    use hurl_core::reader::Reader;
    use regex::Regex;

    #[test]
    fn test_parse_logical_or_expr() {
        let mut reader = Reader::new("@.b");
        assert_eq!(
            logical_or_expr(&mut reader, 0).unwrap(),
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
        let mut reader = Reader::new("@<2 || @>4");
        assert_eq!(
            logical_or_expr(&mut reader, 0).unwrap(),
            LogicalExpr::Or(OrExpr::new(vec![
                LogicalExpr::Comparison(ComparisonExpr::new(
                    Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                        vec![]
                    ))),
                    Comparable::Literal(Literal::Number(Number::Integer(2))),
                    ComparisonOp::Less
                )),
                LogicalExpr::Comparison(ComparisonExpr::new(
                    Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                        vec![]
                    ))),
                    Comparable::Literal(Literal::Number(Number::Integer(4))),
                    ComparisonOp::Greater
                ))
            ]))
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(10));
    }

    #[test]
    fn test_max_nesting_depth() {
        let valid_or = vec!["@"; MAX_NESTING_DEPTH].join(" || ");
        assert!(logical_or_expr(&mut Reader::new(&valid_or), 0).is_ok());

        let too_deep_or = vec!["@"; MAX_NESTING_DEPTH + 1].join(" || ");
        let mut reader = Reader::new(&too_deep_or);
        assert_eq!(
            logical_or_expr(&mut reader, 0).unwrap_err(),
            ParseError::new(
                reader.cursor().pos,
                ParseErrorKind::MaxDepthExceeded {
                    max_depth: MAX_NESTING_DEPTH
                }
            )
        );

        let valid_parentheses = format!(
            "{}@{}",
            "(".repeat(MAX_NESTING_DEPTH - 1),
            ")".repeat(MAX_NESTING_DEPTH - 1)
        );
        assert!(logical_or_expr(&mut Reader::new(&valid_parentheses), 0).is_ok());

        let too_deep_parentheses = format!(
            "{}@{}",
            "(".repeat(MAX_NESTING_DEPTH),
            ")".repeat(MAX_NESTING_DEPTH)
        );
        let mut reader = Reader::new(&too_deep_parentheses);
        assert_eq!(
            logical_or_expr(&mut reader, 0).unwrap_err(),
            ParseError::new(
                reader.cursor().pos,
                ParseErrorKind::MaxDepthExceeded {
                    max_depth: MAX_NESTING_DEPTH
                }
            )
        );
    }

    #[test]
    fn test_parse_and_expression() {
        let mut reader = Reader::new("@>1&&@<4");
        assert_eq!(
            logical_and_expr(&mut reader, 0).unwrap(),
            LogicalExpr::And(AndExpr::new(vec![
                LogicalExpr::Comparison(ComparisonExpr::new(
                    Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                        vec![]
                    ))),
                    Comparable::Literal(Literal::Number(Number::Integer(1))),
                    ComparisonOp::Greater
                )),
                LogicalExpr::Comparison(ComparisonExpr::new(
                    Comparable::SingularQuery(SingularQuery::Relative(RelativeSingularQuery::new(
                        vec![]
                    ))),
                    Comparable::Literal(Literal::Number(Number::Integer(4))),
                    ComparisonOp::Less
                ))
            ]))
        );
    }

    #[test]
    fn test_parse_paren_expression() {
        let mut reader = Reader::new("!(@.b)");
        assert_eq!(
            try_paren_expr(&mut reader, 0).unwrap().unwrap(),
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
        assert!(try_paren_expr(&mut reader, 0).unwrap().is_none());
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(0));
    }

    #[test]
    fn test_parse_test_expr() {
        let mut reader = Reader::new("@.b");
        assert_eq!(
            try_test_expr(&mut reader, 0).unwrap().unwrap(),
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
            try_test_expr(&mut reader, 0).unwrap().unwrap(),
            TestExpr::new(
                false,
                TestExprKind::FilterQuery(Query::AbsoluteQuery(AbsoluteQuery::new(vec![])))
            )
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(1));

        let mut reader = Reader::new("$.*.a]");
        assert_eq!(
            try_test_expr(&mut reader, 0).unwrap().unwrap(),
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
            try_test_expr(&mut reader, 0).unwrap().unwrap(),
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

        let mut reader = Reader::new("match(@.a, 'a.*')");
        assert_eq!(
            try_test_expr(&mut reader, 0).unwrap().unwrap(),
            TestExpr::new(
                false,
                TestExprKind::LogicalTypeFunction(Box::new(LogicalTypeFunction::Match(
                    ValueTypeArgument::SingularQuery(SingularQuery::Relative(
                        RelativeSingularQuery::new(vec![SingularQuerySegment::Name(
                            NameSelector::new("a".to_string())
                        )])
                    )),
                    RegexValueTypeArgument::Literal(Regex::new("a.*").unwrap())
                )))
            )
        );
        assert_eq!(reader.cursor().index, hurl_core::reader::CharPos(17));
    }
}
