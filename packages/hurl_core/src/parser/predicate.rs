/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
use super::error::*;
use super::expr;
use super::primitives::*;
use super::reader::Reader;
use super::string::*;
use super::ParseResult;

pub fn predicate(reader: &mut Reader) -> ParseResult<'static, Predicate> {
    let (not, space0) = match try_literal("not", reader) {
        Err(_) => (
            false,
            Whitespace {
                value: String::from(""),
                source_info: SourceInfo {
                    start: reader.state.clone().pos,
                    end: reader.state.clone().pos,
                },
            },
        ),
        Ok(_) => (true, one_or_more_spaces(reader)?),
    };
    let func = predicate_func(reader)?;
    Ok(Predicate {
        not,
        space0,
        predicate_func: func,
    })
}

fn predicate_func(reader: &mut Reader) -> ParseResult<'static, PredicateFunc> {
    let start = reader.state.clone().pos;
    let value = predicate_func_value(reader)?;
    let end = reader.state.clone().pos;
    Ok(PredicateFunc {
        source_info: SourceInfo { start, end },
        value,
    })
}

fn predicate_func_value(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    let start = reader.state.clone();
    match choice(
        vec![
            equal_predicate,
            greater_or_equal_predicate,
            greater_predicate,
            less_or_equal_predicate,
            less_predicate,
            count_equal_predicate,
            start_with_predicate,
            contain_predicate,
            include_predicate,
            match_predicate,
            integer_predicate,
            float_predicate,
            boolean_predicate,
            string_predicate,
            collection_predicate,
            exist_predicate,
        ],
        reader,
    ) {
        Err(Error {
            recoverable: true, ..
        }) => Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Predicate,
        }),
        x => x,
    }
}

fn equal_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("equals", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let start = reader.state.clone();

    // TODO To be refactored - use idiomatic choice with correct recoverable behaviour
    match predicate_value(reader) {
        Ok(PredicateValue::Null {}) => Ok(PredicateFuncValue::EqualNull { space0 }),
        Ok(PredicateValue::Bool { value }) => Ok(PredicateFuncValue::EqualBool { space0, value }),
        Ok(PredicateValue::Int { value }) => Ok(PredicateFuncValue::EqualInt { space0, value }),
        Ok(PredicateValue::Float { value }) => Ok(PredicateFuncValue::EqualFloat { space0, value }),
        Ok(PredicateValue::Hex { value }) => Ok(PredicateFuncValue::EqualHex { space0, value }),
        Ok(PredicateValue::Expression { value }) => {
            Ok(PredicateFuncValue::EqualExpression { space0, value })
        }
        Ok(PredicateValue::Template { value }) => {
            Ok(PredicateFuncValue::EqualString { space0, value })
        }
        Err(e) => match e.inner {
            ParseError::EscapeChar {} => Err(e),
            _ => Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::PredicateValue {},
            }),
        },
    }
}

fn greater_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("greaterThan", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let start = reader.state.clone();

    match predicate_value(reader) {
        Ok(PredicateValue::Int { value }) => {
            Ok(PredicateFuncValue::GreaterThanInt { space0, value })
        }
        Ok(PredicateValue::Float { value }) => {
            Ok(PredicateFuncValue::GreaterThanFloat { space0, value })
        }
        Ok(_) => Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::PredicateValue {},
        }),
        Err(e) => match e.inner {
            ParseError::EscapeChar {} => Err(e),
            _ => Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::PredicateValue {},
            }),
        },
    }
}
fn greater_or_equal_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("greaterThanOrEquals", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let start = reader.state.clone();

    match predicate_value(reader) {
        Ok(PredicateValue::Int { value }) => {
            Ok(PredicateFuncValue::GreaterThanOrEqualInt { space0, value })
        }
        Ok(PredicateValue::Float { value }) => {
            Ok(PredicateFuncValue::GreaterThanOrEqualFloat { space0, value })
        }
        Ok(_) => Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::PredicateValue {},
        }),
        Err(e) => match e.inner {
            ParseError::EscapeChar {} => Err(e),
            _ => Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::PredicateValue {},
            }),
        },
    }
}

fn less_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("lessThan", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let start = reader.state.clone();

    match predicate_value(reader) {
        Ok(PredicateValue::Int { value }) => Ok(PredicateFuncValue::LessThanInt { space0, value }),
        Ok(PredicateValue::Float { value }) => {
            Ok(PredicateFuncValue::LessThanFloat { space0, value })
        }
        Ok(_) => Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::PredicateValue {},
        }),
        Err(e) => match e.inner {
            ParseError::EscapeChar {} => Err(e),
            _ => Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::PredicateValue {},
            }),
        },
    }
}
fn less_or_equal_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("lessThanOrEquals", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let start = reader.state.clone();

    match predicate_value(reader) {
        Ok(PredicateValue::Int { value }) => {
            Ok(PredicateFuncValue::LessThanOrEqualInt { space0, value })
        }
        Ok(PredicateValue::Float { value }) => {
            Ok(PredicateFuncValue::LessThanOrEqualFloat { space0, value })
        }
        Ok(_) => Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::PredicateValue {},
        }),
        Err(e) => match e.inner {
            ParseError::EscapeChar {} => Err(e),
            _ => Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::PredicateValue {},
            }),
        },
    }
}

fn count_equal_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("countEquals", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let save = reader.state.clone();
    let value = match natural(reader) {
        Err(_) => {
            return Err(Error {
                pos: save.pos,
                recoverable: false,
                inner: ParseError::PredicateValue {},
            })
        }
        Ok(value) => value,
    };
    Ok(PredicateFuncValue::CountEqual { space0, value })
}

fn start_with_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("startsWith", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let value = quoted_template(reader)?;
    Ok(PredicateFuncValue::StartWith { space0, value })
}

fn contain_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("contains", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let value = quoted_template(reader)?;
    Ok(PredicateFuncValue::Contain { space0, value })
}

fn include_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("includes", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let start = reader.state.clone();
    match predicate_value(reader) {
        Ok(PredicateValue::Null {}) => Ok(PredicateFuncValue::IncludeNull { space0 }),
        Ok(PredicateValue::Bool { value }) => Ok(PredicateFuncValue::IncludeBool { space0, value }),
        Ok(PredicateValue::Int { value }) => Ok(PredicateFuncValue::IncludeInt { space0, value }),
        Ok(PredicateValue::Float { value }) => {
            Ok(PredicateFuncValue::IncludeFloat { space0, value })
        }
        Ok(PredicateValue::Template { value }) => {
            Ok(PredicateFuncValue::IncludeString { space0, value })
        }
        Ok(PredicateValue::Hex { value: _ }) => {
            todo!()
        }
        Ok(PredicateValue::Expression { value }) => {
            Ok(PredicateFuncValue::IncludeExpression { space0, value })
        }
        Err(e) => match e.inner {
            ParseError::EscapeChar {} => Err(e),
            _ => Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::PredicateValue {},
            }),
        },
    }
}

fn match_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("matches", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let value = quoted_template(reader)?;
    Ok(PredicateFuncValue::Match { space0, value })
}

fn integer_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("isInteger", reader)?;
    Ok(PredicateFuncValue::IsInteger {})
}

fn float_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("isFloat", reader)?;
    Ok(PredicateFuncValue::IsFloat {})
}

fn boolean_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("isBoolean", reader)?;
    Ok(PredicateFuncValue::IsBoolean {})
}

fn string_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("isString", reader)?;
    Ok(PredicateFuncValue::IsString {})
}

fn collection_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("isCollection", reader)?;
    Ok(PredicateFuncValue::IsCollection {})
}

fn exist_predicate(reader: &mut Reader) -> ParseResult<'static, PredicateFuncValue> {
    try_literal("exists", reader)?;
    Ok(PredicateFuncValue::Exist {})
}

/* internal to the parser */
#[derive(Clone, Debug, PartialEq, Eq)]
enum PredicateValue {
    Null {},
    Int { value: i64 },
    Float { value: Float },
    Bool { value: bool },
    Template { value: Template },
    Hex { value: Hex },
    Expression { value: Expr },
}

fn predicate_value(reader: &mut Reader) -> ParseResult<'static, PredicateValue> {
    choice(
        vec![
            |p1| match null(p1) {
                Ok(()) => Ok(PredicateValue::Null {}),
                Err(e) => Err(e),
            },
            |p1| match boolean(p1) {
                Ok(value) => Ok(PredicateValue::Bool { value }),
                Err(e) => Err(e),
            },
            |p1| match float(p1) {
                Ok(value) => Ok(PredicateValue::Float { value }),
                Err(e) => Err(e),
            },
            |p1| match integer(p1) {
                Ok(value) => Ok(PredicateValue::Int { value }),
                Err(e) => Err(e),
            },
            |p1| match hex(p1) {
                Ok(value) => Ok(PredicateValue::Hex { value }),
                Err(e) => Err(e),
            },
            |p1| match expr::parse(p1) {
                Ok(value) => Ok(PredicateValue::Expression { value }),
                Err(e) => Err(e),
            },
            |p1| match quoted_template(p1) {
                Ok(value) => Ok(PredicateValue::Template { value }),
                Err(e) => Err(e),
            },
        ],
        reader,
    )
}

#[cfg(test)]
mod tests {
    use crate::ast::Pos;

    use super::*;

    #[test]
    fn test_predicate() {
        let mut reader = Reader::init("not equals true");
        assert_eq!(
            predicate(&mut reader).unwrap(),
            Predicate {
                not: true,
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 4, 1, 5),
                },
                predicate_func: PredicateFunc {
                    source_info: SourceInfo::init(1, 5, 1, 16),
                    value: PredicateFuncValue::EqualBool {
                        space0: Whitespace {
                            value: String::from(" "),
                            source_info: SourceInfo::init(1, 11, 1, 12),
                        },
                        value: true,
                    },
                },
            }
        );
    }

    #[test]
    fn test_predicate_error() {
        let mut reader = Reader::init("countEquals true");
        let error = predicate(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 13
            }
        );
        assert_eq!(error.recoverable, false);
        assert_eq!(error.inner, ParseError::PredicateValue {});
    }

    #[test]
    fn test_predicate_func() {
        let mut reader = Reader::init("tata equals 1");
        let error = predicate_func(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.recoverable, false);
        assert_eq!(error.inner, ParseError::Predicate {});
    }

    #[test]
    fn test_equal_predicate() {
        let mut reader = Reader::init("equals  true");
        assert_eq!(
            equal_predicate(&mut reader).unwrap(),
            PredicateFuncValue::EqualBool {
                value: true,
                space0: Whitespace {
                    value: String::from("  "),
                    source_info: SourceInfo::init(1, 7, 1, 9),
                },
            }
        );

        let mut reader = Reader::init("equals 1.1");
        assert_eq!(
            equal_predicate(&mut reader).unwrap(),
            PredicateFuncValue::EqualFloat {
                value: Float {
                    int: 1,
                    decimal: 100_000_000_000_000_000,
                    decimal_digits: 1
                },
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 7, 1, 8),
                },
            }
        );

        let mut reader = Reader::init("equals 2");
        assert_eq!(
            equal_predicate(&mut reader).unwrap(),
            PredicateFuncValue::EqualInt {
                value: 2,
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 7, 1, 8),
                },
            }
        );

        let mut reader = Reader::init("equals \"Bob\"");
        assert_eq!(
            equal_predicate(&mut reader).unwrap(),
            PredicateFuncValue::EqualString {
                value: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "Bob".to_string(),
                        encoded: "Bob".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 8, 1, 13),
                },
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 7, 1, 8),
                },
            }
        );
    }

    #[test]
    fn test_equal_expression_predicate() {
        let mut reader = Reader::init("equals {{count}}");
        assert_eq!(
            equal_predicate(&mut reader).unwrap(),
            PredicateFuncValue::EqualExpression {
                value: Expr {
                    space0: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 10, 1, 10),
                    },
                    variable: Variable {
                        name: "count".to_string(),
                        source_info: SourceInfo::init(1, 10, 1, 15),
                    },
                    space1: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 15, 1, 15),
                    },
                },
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 7, 1, 8),
                },
            }
        );
    }

    #[test]
    fn test_count_equal_predicate() {
        let mut reader = Reader::init("countEquals 2");
        assert_eq!(
            count_equal_predicate(&mut reader).unwrap(),
            PredicateFuncValue::CountEqual {
                value: 2,
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 12, 1, 13),
                },
            }
        );

        let mut reader = Reader::init("countEquals true");
        let error = count_equal_predicate(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 13
            }
        );
        assert_eq!(error.recoverable, false);
        assert_eq!(error.inner, ParseError::PredicateValue {});
    }

    #[test]
    fn test_start_with_predicate() {
        let mut reader = Reader::init("startsWith 2");
        let error = start_with_predicate(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 12
            }
        );
        assert_eq!(error.recoverable, false);
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "\"".to_string()
            }
        );
    }

    #[test]
    fn test_predicate_value() {
        let mut reader = Reader::init("true");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Bool { value: true }
        );

        let mut reader = Reader::init("1");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Int { value: 1 }
        );

        let mut reader = Reader::init("1.1");
        assert_eq!(
            predicate_value(&mut reader).unwrap(),
            PredicateValue::Float {
                value: Float {
                    int: 1,
                    decimal: 100_000_000_000_000_000,
                    decimal_digits: 1
                }
            }
        );
    }
}
