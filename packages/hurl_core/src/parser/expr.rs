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
use crate::ast::{Expr, ExprKind, SourceInfo, Variable};
use crate::parser::primitives::variable_name;
use crate::parser::{ParseResult, function};
use crate::reader::Reader;

/// Parses an expression.
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
                let name = variable_name(reader)?;
                let variable = Variable {
                    name,
                    source_info: SourceInfo::new(start, reader.cursor().pos),
                };
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
