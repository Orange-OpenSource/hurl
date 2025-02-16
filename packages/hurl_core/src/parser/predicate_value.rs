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
use super::placeholder;
use crate::ast::PredicateValue;
use crate::combinator::choice;
use crate::parser::multiline::multiline_string;
use crate::parser::number::number;
use crate::parser::primitives::{base64, boolean, file, hex, null, regex};
use crate::parser::string::{backtick_template, quoted_template};
use crate::parser::{ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

pub fn predicate_value(reader: &mut Reader) -> ParseResult<PredicateValue> {
    choice(
        &[
            |p1| match null(p1) {
                Ok(()) => Ok(PredicateValue::Null),
                Err(e) => Err(e),
            },
            |p1| match boolean(p1) {
                Ok(value) => Ok(PredicateValue::Bool(value)),
                Err(e) => Err(e),
            },
            |p1| match number(p1) {
                Ok(value) => Ok(PredicateValue::Number(value)),
                Err(e) => Err(e),
            },
            |p1| match file(p1) {
                Ok(value) => Ok(PredicateValue::File(value)),
                Err(e) => Err(e),
            },
            |p1| match hex(p1) {
                Ok(value) => Ok(PredicateValue::Hex(value)),
                Err(e) => Err(e),
            },
            |p1| match base64(p1) {
                Ok(value) => Ok(PredicateValue::Base64(value)),
                Err(e) => Err(e),
            },
            |p1| match placeholder::parse(p1) {
                Ok(value) => Ok(PredicateValue::Placeholder(value)),
                Err(e) => Err(e),
            },
            |p1| match quoted_template(p1) {
                Ok(value) => Ok(PredicateValue::String(value)),
                Err(e) => Err(e),
            },
            |p1| match multiline_string(p1) {
                Ok(value) => Ok(PredicateValue::MultilineString(value)),
                Err(e) => Err(e),
            },
            |p1| match backtick_template(p1) {
                Ok(value) => Ok(PredicateValue::String(value)),
                Err(e) => Err(e),
            },
            |p1| match regex(p1) {
                Ok(value) => Ok(PredicateValue::Regex(value)),
                Err(e) => Err(e),
            },
        ],
        reader,
    )
    .map_err(|e| {
        let kind = if e.recoverable {
            ParseErrorKind::PredicateValue
        } else {
            e.kind
        };
        ParseError::new(e.pos, false, kind)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Float, Number, I64};
    use crate::parser::ParseErrorKind;
    use crate::reader::Pos;
    use crate::typing::ToSource;

    #[test]
    fn test_predicate_value() {
        let mut reader = Reader::new("true");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Bool(true)
        );

        let mut reader = Reader::new("1");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Number(Number::Integer(I64::new(1, "1".to_source())))
        );

        let mut reader = Reader::new("1.1");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Number(Number::Float(Float::new(1.1, "1.1".to_source())))
        );
    }

    #[test]
    fn test_predicate_value_error() {
        let mut reader = Reader::new("xx");
        let error = predicate_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::PredicateValue);
        assert!(!error.recoverable);
    }

    #[test]
    fn test_predicate_value_error_missing_quote() {
        let mut reader = Reader::new("\"not_found");
        let error = predicate_value(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 11
            }
        );
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\"".to_string()
            }
        );
        assert!(!error.recoverable);
    }
}
