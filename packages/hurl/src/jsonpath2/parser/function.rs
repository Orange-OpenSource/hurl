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

use crate::jsonpath2::ast::literal::Literal;
use crate::jsonpath2::parser::primitives::{expect_str, match_str};

use hurl_core::reader::Reader;
// use crate::jsonpath2::parser::reader::Reader;
use crate::jsonpath2::ast::function::{LogicalTypeFunction, ValueTypeArgument, ValueTypeFunction};
use crate::jsonpath2::parser::{ParseError, ParseErrorKind, ParseResult};

#[allow(dead_code)]
pub fn function_logicaltype(reader: &mut Reader) -> ParseResult<LogicalTypeFunction> {
    let initial_state = reader.cursor();
    if match_str("match", reader) {
        expect_str("(", reader)?;
        let argument1 = argument_valuetype(reader)?;
        expect_str(",", reader)?;
        let argument2 = argument_valuetype(reader)?;
        expect_str(")", reader)?;
        Ok(LogicalTypeFunction::Match(argument1, argument2))
    } else {
        Err(ParseError::new(
            initial_state.pos,
            ParseErrorKind::Expecting("a function that returns a LogicalType".to_string()),
        ))
    }
}

#[allow(dead_code)]
pub fn function_valuetype(reader: &mut Reader) -> ParseResult<ValueTypeFunction> {
    let initial_state = reader.cursor();
    if match_str("length", reader) {
        expect_str("(", reader)?;
        let argument = argument_valuetype(reader)?;
        expect_str(")", reader)?;
        Ok(ValueTypeFunction::Length(argument))
    } else {
        Err(ParseError::new(
            initial_state.pos,
            ParseErrorKind::Expecting("a function that returns a ValueType".to_string()),
        ))
    }
}

#[allow(dead_code)]
fn argument_valuetype(_reader: &mut Reader) -> ParseResult<ValueTypeArgument> {
    Ok(ValueTypeArgument::Literal(Literal::String(
        "abc".to_string(),
    )))
}
