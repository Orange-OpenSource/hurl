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
use crate::ast::{Expr, ExprKind, SourceInfo, Variable};
use crate::parser::{function, ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

/// Parse an expression
///
/// Currently, an expression can only be found inside a placeholder
pub fn parse(reader: &mut Reader) -> ParseResult<Expr> {
    let start = reader.cursor().pos;
    let save_state = reader.cursor();
    let kind = match function::parse(reader) {
        Ok(function) => ExprKind::Function(function),
        Err(e) => {
            if e.recoverable {
                reader.seek(save_state);
                let variable = variable_name(reader)?;
                ExprKind::Variable(variable)
            } else {
                return Err(e);
            }
        }
    };
    let end = reader.cursor().pos;
    let source_info = SourceInfo::new(start, end);
    Ok(Expr { source_info, kind })
}

fn variable_name(reader: &mut Reader) -> ParseResult<Variable> {
    let start = reader.cursor();
    let name = reader.read_while(|c| c.is_alphanumeric() || c == '_' || c == '-');
    if name.is_empty() {
        return Err(ParseError::new(
            start.pos,
            false,
            ParseErrorKind::TemplateVariable,
        ));
    }
    Ok(Variable {
        name,
        source_info: SourceInfo::new(start.pos, reader.cursor().pos),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::Pos;

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
