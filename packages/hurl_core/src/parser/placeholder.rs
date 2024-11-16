use super::{expr, ParseResult};
/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use crate::ast::*;
use crate::parser::primitives::*;
use crate::reader::Reader;

/// Parse a placeholder {{ expr }}
pub fn parse(reader: &mut Reader) -> ParseResult<Expr> {
    try_literal("{{", reader)?;
    let expression = expr::parse(reader)?;
    literal("}}", reader)?;
    Ok(expression)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::ParseErrorKind, reader::Pos};

    #[test]
    fn test_expr() {
        let mut reader = Reader::new("{{ name}}");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Expr {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 4)),
                },
                variable: Variable {
                    name: String::from("name"),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 8)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 8)),
                },
            }
        );
    }

    #[test]
    fn test_expr_error() {
        let mut reader = Reader::new("{{host>}}");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("}}")
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_expr_error_eof() {
        let mut reader = Reader::new("{{host");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("}}")
            }
        );
        assert!(!error.recoverable);
    }
}
