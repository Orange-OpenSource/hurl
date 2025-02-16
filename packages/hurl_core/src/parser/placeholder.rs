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
use super::{expr, ParseResult};
use crate::ast::Placeholder;
use crate::parser::primitives::{literal, try_literal, zero_or_more_spaces};
use crate::reader::Reader;

/// Parse a placeholder {{ expr }}
pub fn parse(reader: &mut Reader) -> ParseResult<Placeholder> {
    try_literal("{{", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let expr = expr::parse(reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    literal("}}", reader)?;
    Ok(Placeholder {
        space0,
        expr,
        space1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expr, ExprKind, SourceInfo, Variable, Whitespace};
    use crate::parser::ParseErrorKind;
    use crate::reader::Pos;

    #[test]
    fn test_ok() {
        let mut reader = Reader::new("{{ name}}");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Placeholder {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 4)),
                },
                expr: Expr {
                    kind: ExprKind::Variable(Variable {
                        name: "name".to_string(),
                        source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 8)),
                    }),
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
    fn test_error() {
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
    fn test_error_eof() {
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
