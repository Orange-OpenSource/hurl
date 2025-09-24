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
use crate::jsonpath2::parser::ParseResult;
use crate::jsonpath2::parser::{ParseError, ParseErrorKind};

use hurl_core::reader::Reader;

#[allow(dead_code)]
pub fn parse(reader: &mut Reader) -> ParseResult<ComparisonExpr> {
    let left = comparable(reader)?;
    let operator = comparison_op(reader)?;
    let right = comparable(reader)?;
    Ok(ComparisonExpr::new(left, right, operator))
}

fn comparable(reader: &mut Reader) -> ParseResult<Comparable> {
    if let Ok(literal) = literal::parse(reader) {
        return Ok(Comparable::Literal(literal));
    }
    todo!()
}

fn comparison_op(reader: &mut Reader) -> ParseResult<ComparisonOp> {
    if match_str("==", reader) {
        Ok(ComparisonOp::Equal)
    } else if match_str("!=", reader) {
        Ok(ComparisonOp::NotEqual)
    } else if match_str("<=", reader) {
        Ok(ComparisonOp::LessOrEqual)
    } else if match_str("<", reader) {
        Ok(ComparisonOp::Less)
    } else if match_str(">=", reader) {
        Ok(ComparisonOp::GreaterOrEqual)
    } else if match_str(">", reader) {
        Ok(ComparisonOp::Greater)
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("comparison operator".to_string()),
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::jsonpath2::ast::comparison::ComparisonOp;
    use crate::jsonpath2::ast::expr::Literal;
    use hurl_core::reader::{CharPos, Reader};

    use super::*;

    #[test]
    pub fn test_comparison_expr() {
        let mut reader = Reader::new("1<=2");
        assert_eq!(
            parse(&mut reader).unwrap(),
            ComparisonExpr::new(
                Comparable::Literal(Literal::Integer(1)),
                Comparable::Literal(Literal::Integer(2)),
                ComparisonOp::LessOrEqual
            )
        );
        assert_eq!(reader.cursor().index, CharPos(4));
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
        assert_eq!(comparison_op(&mut reader).unwrap(), ComparisonOp::Equal);
        assert_eq!(reader.cursor().index, CharPos(2));
    }
}
