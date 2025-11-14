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

use crate::jsonpath2::ast::comparison::Comparable;
use crate::jsonpath2::ast::comparison::ComparisonExpr;
use crate::jsonpath2::ast::comparison::ComparisonOp;
use crate::jsonpath2::parser::literal;
use crate::jsonpath2::parser::primitives::match_str;
use crate::jsonpath2::parser::singular_query::try_parse as try_singular_query;
use crate::jsonpath2::parser::ParseResult;
use crate::jsonpath2::parser::{ParseError, ParseErrorKind};

use hurl_core::reader::Reader;

#[allow(dead_code)]
/// Try to parse a comparison expression.
pub fn try_parse(reader: &mut Reader) -> ParseResult<Option<ComparisonExpr>> {
    let save = reader.cursor();
    let left = if let Some(value) = try_comparable(reader)? {
        value
    } else {
        return Ok(None);
    };
    let operator = if let Some(value) = try_comparison_op(reader) {
        value
    } else {
        reader.seek(save);
        return Ok(None);
    };
    let right = comparable(reader)?;
    Ok(Some(ComparisonExpr::new(left, right, operator)))
}

/// Parse a comparable.
fn comparable(reader: &mut Reader) -> ParseResult<Comparable> {
    if let Some(value) = try_comparable(reader)? {
        Ok(value)
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("comparable".to_string()),
        ))
    }
}

/// Try to parse a comparable.
fn try_comparable(reader: &mut Reader) -> ParseResult<Option<Comparable>> {
    if let Ok(literal) = literal::parse(reader) {
        Ok(Some(Comparable::Literal(literal)))
    } else if let Some(singular_query) = try_singular_query(reader)? {
        Ok(Some(Comparable::SingularQuery(singular_query)))
    } else {
        Ok(None)
    }
}

/// Try to parse a comparison operator.
/// It can not fail. Return None if no operator is found.
fn try_comparison_op(reader: &mut Reader) -> Option<ComparisonOp> {
    if match_str("==", reader) {
        Some(ComparisonOp::Equal)
    } else if match_str("!=", reader) {
        Some(ComparisonOp::NotEqual)
    } else if match_str("<=", reader) {
        Some(ComparisonOp::LessOrEqual)
    } else if match_str("<", reader) {
        Some(ComparisonOp::Less)
    } else if match_str(">=", reader) {
        Some(ComparisonOp::GreaterOrEqual)
    } else if match_str(">", reader) {
        Some(ComparisonOp::Greater)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::jsonpath2::ast::comparison::ComparisonOp;
    use crate::jsonpath2::ast::literal::Literal;
    use hurl_core::reader::{CharPos, Reader};

    use super::*;

    #[test]
    pub fn test_comparison_expr() {
        let mut reader = Reader::new("1<=2");
        assert_eq!(
            try_parse(&mut reader).unwrap().unwrap(),
            ComparisonExpr::new(
                Comparable::Literal(Literal::Integer(1)),
                Comparable::Literal(Literal::Integer(2)),
                ComparisonOp::LessOrEqual
            )
        );
        assert_eq!(reader.cursor().index, CharPos(4));
    }

    #[test]
    pub fn test_comparison_expr_none() {
        // This is a test expression, not a comparison expression
        let mut reader = Reader::new("@.b]");
        assert!(try_parse(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));

        // This is a test expression, not a comparison expression
        let mut reader = Reader::new("$.*.a]");
        assert!(try_parse(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    pub fn test_comparable() {
        let mut reader = Reader::new("1");
        assert_eq!(
            comparable(&mut reader).unwrap(),
            Comparable::Literal(Literal::Integer(1))
        );
        assert_eq!(reader.cursor().index, CharPos(1));
    }

    #[test]
    pub fn test_comparaison_op() {
        let mut reader = Reader::new("==");
        assert_eq!(try_comparison_op(&mut reader).unwrap(), ComparisonOp::Equal);
        assert_eq!(reader.cursor().index, CharPos(2));
    }

    #[test]
    pub fn test_comparaison_op_none() {
        let mut reader = Reader::new("]");
        assert!(try_comparison_op(&mut reader).is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }
}
