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
use crate::parser::error::*;
use crate::parser::primitives::*;
use crate::parser::reader::Reader;
use crate::parser::ParseResult;

pub fn parse(reader: &mut Reader) -> ParseResult<Expr> {
    try_literal("{{", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let variable = variable_name(reader)?;
    let space1 = zero_or_more_spaces(reader)?;

    if try_literal("}}}", reader).is_err() {
        literal("}}", reader)?
    }

    Ok(Expr {
        space0,
        variable,
        space1,
    })
}

pub fn parse2(reader: &mut Reader) -> ParseResult<Expr> {
    let space0 = zero_or_more_spaces(reader)?;
    let variable = variable_name(reader)?;
    let space1 = zero_or_more_spaces(reader)?;

    Ok(Expr {
        space0,
        variable,
        space1,
    })
}

fn variable_name(reader: &mut Reader) -> ParseResult<Variable> {
    let start = reader.state;
    let name = reader.read_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-');
    if name.is_empty() {
        return Err(Error::new(start.pos, false, ParseError::TemplateVariable));
    }
    Ok(Variable {
        name,
        source_info: SourceInfo::new(start.pos, reader.state.pos),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;

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
            error.inner,
            ParseError::Expecting {
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
            error.inner,
            ParseError::Expecting {
                value: String::from("}}")
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_variable() {
        let mut reader = Reader::new("name");
        assert_eq!(
            variable_name(&mut reader).unwrap(),
            Variable {
                name: String::from("name"),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 5)),
            }
        );

        let mut reader = Reader::new("my-id");
        assert_eq!(
            variable_name(&mut reader).unwrap(),
            Variable {
                name: String::from("my-id"),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6)),
            }
        );
    }
}
