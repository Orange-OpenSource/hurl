/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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

use super::combinators::*;
use super::expr;
use super::primitives::*;
use super::reader::Reader;
use super::string::*;
use super::ParseResult;
use crate::parser::{Error, ParseError};

pub fn predicate_value(reader: &mut Reader) -> ParseResult<'static, PredicateValue> {
    choice(
        vec![
            |p1| match null(p1) {
                Ok(()) => Ok(PredicateValue::Null {}),
                Err(e) => Err(e),
            },
            |p1| match boolean(p1) {
                Ok(value) => Ok(PredicateValue::Bool(value)),
                Err(e) => Err(e),
            },
            |p1| match float(p1) {
                Ok(value) => Ok(PredicateValue::Float(value)),
                Err(e) => Err(e),
            },
            |p1| match integer(p1) {
                Ok(value) => Ok(PredicateValue::Integer(value)),
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
            |p1| match expr::parse(p1) {
                Ok(value) => Ok(PredicateValue::Expression(value)),
                Err(e) => Err(e),
            },
            |p1| match quoted_template(p1) {
                Ok(value) => Ok(PredicateValue::String(value)),
                Err(e) => Err(e),
            },
            |p1| match raw_string(p1) {
                Ok(value) => Ok(PredicateValue::Raw(value)),
                Err(e) => Err(e),
            },
            |p1| match regex(p1) {
                Ok(value) => Ok(PredicateValue::Regex(value)),
                Err(e) => Err(e),
            },
        ],
        reader,
    )
    .map_err(|e| Error {
        pos: e.pos,
        recoverable: false,
        inner: if e.recoverable {
            ParseError::PredicateValue
        } else {
            e.inner
        },
    })
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::ParseError;

    #[test]
    fn test_predicate_value() {
        let mut reader = Reader::init("true");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Bool(true)
        );

        let mut reader = Reader::init("1");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Integer(1)
        );

        let mut reader = Reader::init("1.1");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Float(Float {
                value: 1.1,
                encoded: "1.1".to_string(),
            })
        );
    }

    #[test]
    fn test_predicate_value_error() {
        let mut reader = Reader::init("xx");
        let error = predicate_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::PredicateValue {});
        assert!(!error.recoverable);
    }

    #[test]
    fn test_predicate_value_error_missing_quote() {
        let mut reader = Reader::init("\"not_found");
        let error = predicate_value(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 11
            }
        );
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "\"".to_string()
            }
        );
        assert!(!error.recoverable);
    }
}
