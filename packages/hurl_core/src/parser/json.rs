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
use crate::ast::{JsonListElement, JsonObjectElement, JsonValue, Pos, SourceInfo, Template};

use super::combinators::*;
use super::error;
use super::primitives::*;
use super::reader::*;
use super::template::*;
use super::ParseResult;
use crate::parser::expr;

pub fn parse(reader: &mut Reader) -> ParseResult<'static, JsonValue> {
    choice(
        vec![
            null_value,
            boolean_value,
            string_value,
            number_value,
            expression_value,
            list_value,
            object_value,
        ],
        reader,
    )
}

pub fn null_value(reader: &mut Reader) -> ParseResult<'static, JsonValue> {
    try_literal("null", reader)?;
    Ok(JsonValue::Null {})
}

pub fn boolean_value(reader: &mut Reader) -> ParseResult<'static, JsonValue> {
    let value = boolean(reader)?;
    Ok(JsonValue::Boolean(value))
}

fn string_value(reader: &mut Reader) -> ParseResult<'static, JsonValue> {
    let template = string_template(reader)?;
    Ok(JsonValue::String(template))
}

fn string_template(reader: &mut Reader) -> ParseResult<'static, Template> {
    try_literal("\"", reader)?;
    let quotes = true;
    let mut chars = vec![];
    let start = reader.state.pos.clone();
    loop {
        if reader.remaining().starts_with('"') || reader.is_eof() {
            break;
        }
        let char = any_char(reader)?;
        chars.push(char);
    }
    let end = reader.state.pos.clone();

    let encoded_string = EncodedString {
        source_info: SourceInfo {
            start: start.clone(),
            end: end.clone(),
        },
        chars,
    };
    literal("\"", reader)?;
    let elements = templatize(encoded_string)?;

    let template = Template {
        quotes,
        elements,
        source_info: SourceInfo { start, end },
    };
    Ok(template)
}

fn any_char(reader: &mut Reader) -> ParseResult<'static, (char, String, Pos)> {
    let start = reader.state.clone();
    match escape_char(reader) {
        Ok(c) => Ok((c, reader.from(start.cursor), start.pos)),
        Err(e) => {
            if e.recoverable {
                reader.state = start.clone();
                match reader.read() {
                    None => Err(error::Error {
                        pos: start.pos,
                        recoverable: true,
                        inner: error::ParseError::Expecting {
                            value: "char".to_string(),
                        },
                    }),
                    Some(c) => {
                        if vec!['\\', '\x08', '\n', '\x0c', '\r', '\t'].contains(&c) {
                            Err(error::Error {
                                pos: start.pos,
                                recoverable: true,
                                inner: error::ParseError::Expecting {
                                    value: "char".to_string(),
                                },
                            })
                        } else {
                            Ok((c, reader.from(start.cursor), start.pos))
                        }
                    }
                }
            } else {
                Err(e)
            }
        }
    }
}

fn escape_char(reader: &mut Reader) -> ParseResult<'static, char> {
    try_literal("\\", reader)?;
    let start = reader.state.clone();
    match reader.read() {
        Some('"') => Ok('"'),
        Some('\\') => Ok('\\'),
        Some('/') => Ok('/'),
        Some('b') => Ok('\x08'),
        Some('n') => Ok('\n'),
        Some('f') => Ok('\x0c'),
        Some('r') => Ok('\r'),
        Some('t') => Ok('\t'),
        Some('u') => unicode(reader),
        _ => Err(error::Error {
            pos: start.pos,
            recoverable: false,
            inner: error::ParseError::EscapeChar {},
        }),
    }
}

fn unicode(reader: &mut Reader) -> ParseResult<'static, char> {
    let v = hex_value(reader)?;
    let c = match std::char::from_u32(v) {
        None => {
            return Err(error::Error {
                pos: reader.clone().state.pos,
                recoverable: false,
                inner: error::ParseError::Unicode {},
            })
        }
        Some(c) => c,
    };
    Ok(c)
}

fn hex_value(reader: &mut Reader) -> ParseResult<'static, u32> {
    let digit1 = nonrecover(hex_digit, reader)?;
    let digit2 = nonrecover(hex_digit, reader)?;
    let digit3 = nonrecover(hex_digit, reader)?;
    let digit4 = nonrecover(hex_digit, reader)?;
    let value = digit1 * (16 ^ 3) + digit2 * (16 ^ 2) + digit3 * 16 + digit4;
    Ok(value)
}

pub fn number_value(reader: &mut Reader) -> ParseResult<'static, JsonValue> {
    let start = reader.state.clone();

    let sign = match try_literal("-", reader) {
        Err(_) => "".to_string(),
        Ok(_) => "-".to_string(),
    };

    let integer = match try_literal("0", reader) {
        Err(_) => {
            let digits = reader.read_while(|c| c.is_ascii_digit());
            if digits.is_empty() {
                return Err(error::Error {
                    pos: start.pos,
                    recoverable: true,
                    inner: error::ParseError::Expecting {
                        value: "number".to_string(),
                    },
                });
            } else {
                digits
            }
        }
        Ok(_) => "0".to_string(),
    };

    let fraction = match try_literal(".", reader) {
        Ok(_) => {
            let digits = reader.read_while(|c| c.is_ascii_digit());
            if digits.is_empty() {
                return Err(error::Error {
                    pos: reader.state.pos.clone(),
                    recoverable: false,
                    inner: error::ParseError::Expecting {
                        value: "digits".to_string(),
                    },
                });
            } else {
                format!(".{}", digits)
            }
        }
        Err(_) => "".to_string(),
    };

    let exponent = if reader.remaining().starts_with('e') || reader.remaining().starts_with('E') {
        reader.read();
        let exponent_sign = match try_literal("-", reader) {
            Ok(_) => "-".to_string(),
            Err(_) => match try_literal("+", reader) {
                Ok(_) => "+".to_string(),
                Err(_) => "".to_string(),
            },
        };
        let exponent_digits = reader.read_while(|c| c.is_ascii_digit());
        format!("e{}{}", exponent_sign, exponent_digits)
    } else {
        "".to_string()
    };

    Ok(JsonValue::Number(format!(
        "{}{}{}{}",
        sign, integer, fraction, exponent
    )))
}

fn expression_value(reader: &mut Reader) -> ParseResult<'static, JsonValue> {
    let exp = expr::parse(reader)?;
    Ok(JsonValue::Expression(exp))
}

fn list_value(reader: &mut Reader) -> ParseResult<'static, JsonValue> {
    try_literal("[", reader)?;
    let space0 = whitespace(reader);
    let mut elements = vec![];

    // at least one element
    if !reader.remaining().starts_with(']') {
        let first_element = list_element(reader)?;
        elements.push(first_element);

        loop {
            if reader.remaining().starts_with(']') {
                break;
            }
            if !reader.remaining().starts_with(',') {
                break;
            }
            literal(",", reader)?;
            let element = list_element(reader)?;
            elements.push(element);
        }
    }
    literal("]", reader)?;

    Ok(JsonValue::List { space0, elements })
}

fn list_element(reader: &mut Reader) -> ParseResult<'static, JsonListElement> {
    let save = reader.state.pos.clone();
    let space0 = whitespace(reader);
    let value = match parse(reader) {
        Ok(r) => r,
        Err(_) => {
            return Err(error::Error {
                pos: save,
                recoverable: false,
                inner: error::ParseError::Json {},
            })
        }
    };
    let space1 = whitespace(reader);
    Ok(JsonListElement {
        space0,
        value,
        space1,
    })
}

fn object_value(reader: &mut Reader) -> ParseResult<'static, JsonValue> {
    try_literal("{", reader)?;
    let space0 = whitespace(reader);
    let mut elements = vec![];
    if !reader.remaining().starts_with('}') {
        let first_element = object_element(reader)?;
        elements.push(first_element);

        loop {
            if reader.remaining().starts_with('}') {
                break;
            }
            if !reader.remaining().starts_with(',') {
                break;
            }
            literal(",", reader)?;
            let element = object_element(reader)?;
            elements.push(element);
        }
    }

    // at least one element

    literal("}", reader)?;

    Ok(JsonValue::Object { space0, elements })
}

fn key(reader: &mut Reader) -> ParseResult<'static, Template> {
    let save = reader.state.clone();
    let name = string_template(reader).map_err(|e| e.non_recoverable())?;
    if name.elements.is_empty() {
        Err(error::Error {
            pos: save.pos,
            recoverable: false,
            inner: error::ParseError::Json {},
        })
    } else {
        Ok(name)
    }
}
fn object_element(reader: &mut Reader) -> ParseResult<'static, JsonObjectElement> {
    let space0 = whitespace(reader);
    //literal("\"", reader)?;
    let name = key(reader)?;
    //literal("\"", reader)?;
    let space1 = whitespace(reader);
    literal(":", reader)?;
    let save = reader.state.pos.clone();
    let space2 = whitespace(reader);
    let value = match parse(reader) {
        Ok(r) => r,
        Err(_) => {
            return Err(error::Error {
                pos: save,
                recoverable: false,
                inner: error::ParseError::Json {},
            })
        }
    };
    let space3 = whitespace(reader);
    Ok(JsonObjectElement {
        space0,
        name,
        space1,
        space2,
        value,
        space3,
    })
}

fn whitespace(reader: &mut Reader) -> String {
    reader.read_while(|c| *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_parse_error() {
        let mut reader = Reader::init("{ \"a\":\n}");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert!(!error.recoverable);

        let mut reader = Reader::init("[0,1,]");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 6 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert!(!error.recoverable);
    }

    #[test]
    fn test_null_value() {
        let mut reader = Reader::init("null");
        assert_eq!(null_value(&mut reader).unwrap(), JsonValue::Null {});
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("true");
        let error = null_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "null".to_string()
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_boolean_value() {
        let mut reader = Reader::init("true");
        assert_eq!(
            boolean_value(&mut reader).unwrap(),
            JsonValue::Boolean(true)
        );
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("1");
        let error = boolean_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "true|false".to_string()
            }
        );
        assert!(error.recoverable);
    }

    pub fn json_hello_world_value() -> JsonValue {
        // "hello\u0020{{name}}!"
        JsonValue::String(Template {
            quotes: true,
            elements: vec![
                TemplateElement::String {
                    value: "Hello ".to_string(),
                    encoded: "Hello\\u0020".to_string(),
                },
                TemplateElement::Expression(Expr {
                    space0: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(1, 15, 1, 15),
                    },
                    variable: Variable {
                        name: "name".to_string(),
                        source_info: SourceInfo::init(1, 15, 1, 19),
                    },
                    space1: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(1, 19, 1, 19),
                    },
                }),
                TemplateElement::String {
                    value: "!".to_string(),
                    encoded: "!".to_string(),
                },
            ],
            source_info: SourceInfo::init(1, 2, 1, 22),
        })
    }

    #[test]
    fn test_string_value() {
        let mut reader = Reader::init("\"\"");
        assert_eq!(
            string_value(&mut reader).unwrap(),
            JsonValue::String(Template {
                quotes: true,
                elements: vec![],
                source_info: SourceInfo::init(1, 2, 1, 2),
            })
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("\"Hello\\u0020{{name}}!\"");
        assert_eq!(string_value(&mut reader).unwrap(), json_hello_world_value());
        assert_eq!(reader.state.cursor, 22);

        let mut reader = Reader::init("\"{}\"");
        assert_eq!(
            string_value(&mut reader).unwrap(),
            JsonValue::String(Template {
                quotes: true,
                elements: vec![TemplateElement::String {
                    value: "{}".to_string(),
                    encoded: "{}".to_string(),
                }],
                source_info: SourceInfo::init(1, 2, 1, 4),
            })
        );
        assert_eq!(reader.state.cursor, 4);
    }

    #[test]
    fn test_string_value_error() {
        let mut reader = Reader::init("1");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "\"".to_string()
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("\"1");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "\"".to_string()
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::init("\"{{x\"");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "}}".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_any_char() {
        let mut reader = Reader::init("a");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            ('a', "a".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init(" ");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            (' ', " ".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init("\\u0020 ");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            (' ', "\\u0020".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.state.cursor, 6);

        let mut reader = Reader::init("\\t");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            ('\t', "\\t".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("#");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            ('#', "#".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.state.cursor, 1);
    }

    #[test]
    fn test_any_char_error() {
        let mut reader = Reader::init("");
        let error = any_char(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::init("\t");
        let error = any_char(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);
    }

    #[test]
    fn test_escape_char() {
        let mut reader = Reader::init("\\n");
        assert_eq!(escape_char(&mut reader).unwrap(), '\n');
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("\\u000a");
        assert_eq!(escape_char(&mut reader).unwrap(), '\n');
        assert_eq!(reader.state.cursor, 6);

        let mut reader = Reader::init("x");
        let error = escape_char(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "\\".to_string()
            }
        );
        assert!(error.recoverable);
        assert_eq!(reader.state.cursor, 0);
    }

    #[test]
    fn test_unicode() {
        let mut reader = Reader::init("000a");
        assert_eq!(unicode(&mut reader).unwrap(), '\n');
        assert_eq!(reader.state.cursor, 4);
    }

    #[test]
    fn test_hex_value() {
        let mut reader = Reader::init("0020x");
        assert_eq!(hex_value(&mut reader).unwrap(), 32);

        let mut reader = Reader::init("x");
        let error = hex_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, error::ParseError::HexDigit);
        assert!(!error.recoverable);
    }

    #[test]
    fn test_number_value() {
        let mut reader = Reader::init("100");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("100".to_string())
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("1.333");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("1.333".to_string())
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("-1");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("-1".to_string())
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("00");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("0".to_string())
        );
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init("1e0");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("1e0".to_string())
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("1e005");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("1e005".to_string())
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("1e-005");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("1e-005".to_string())
        );
        assert_eq!(reader.state.cursor, 6);
    }

    #[test]
    fn test_number_value_error() {
        let mut reader = Reader::init("true");
        let error = number_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "number".to_string()
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("1.x");
        let error = number_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "digits".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_expression_value() {
        let mut reader = Reader::init("{{n}}");
        assert_eq!(
            expression_value(&mut reader).unwrap(),
            JsonValue::Expression(Expr {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 3, 1, 3)
                },
                variable: Variable {
                    name: "n".to_string(),
                    source_info: SourceInfo::init(1, 3, 1, 4)
                },
                space1: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 4, 1, 4)
                }
            })
        );
        assert_eq!(reader.state.cursor, 5);
    }

    #[test]
    fn test_list_value() {
        let mut reader = Reader::init("[]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            JsonValue::List {
                space0: "".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("[ ]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            JsonValue::List {
                space0: " ".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("[true]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            JsonValue::List {
                space0: "".to_string(),
                elements: vec![JsonListElement {
                    space0: "".to_string(),
                    value: JsonValue::Boolean(true),
                    space1: "".to_string(),
                }],
            }
        );
        assert_eq!(reader.state.cursor, 6);
    }

    #[test]
    fn test_list_error() {
        let mut reader = Reader::init("true");
        let error = list_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "[".to_string()
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("[1, 2,]");
        let error = list_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert!(!error.recoverable);
    }

    #[test]
    fn test_list_element() {
        let mut reader = Reader::init("true");
        assert_eq!(
            list_element(&mut reader).unwrap(),
            JsonListElement {
                space0: "".to_string(),
                value: JsonValue::Boolean(true),
                space1: "".to_string(),
            }
        );
        assert_eq!(reader.state.cursor, 4);
    }

    #[test]
    fn test_object_value() {
        let mut reader = Reader::init("{}");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            JsonValue::Object {
                space0: "".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("{ }");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            JsonValue::Object {
                space0: " ".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("{\n  \"a\": true\n}");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            JsonValue::Object {
                space0: "\n  ".to_string(),
                elements: vec![JsonObjectElement {
                    space0: "".to_string(),
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "a".to_string(),
                            encoded: "a".to_string()
                        }],
                        source_info: SourceInfo::init(2, 4, 2, 5)
                    },
                    space1: "".to_string(),
                    space2: " ".to_string(),
                    value: JsonValue::Boolean(true),
                    space3: "\n".to_string(),
                }],
            }
        );
        assert_eq!(reader.state.cursor, 15);

        let mut reader = Reader::init("true");
        let error = object_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "{".to_string()
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_object_error() {
        let mut reader = Reader::init("{ \"a\":\n}");
        let error = object_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert!(!error.recoverable);
    }

    #[test]
    fn test_object_element() {
        let mut reader = Reader::init("\"a\": true");
        assert_eq!(
            object_element(&mut reader).unwrap(),
            JsonObjectElement {
                space0: "".to_string(),
                name: Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "a".to_string(),
                        encoded: "a".to_string()
                    }],
                    source_info: SourceInfo::init(1, 2, 1, 3)
                },
                space1: "".to_string(),
                space2: " ".to_string(),
                value: JsonValue::Boolean(true),
                space3: "".to_string(),
            }
        );
        assert_eq!(reader.state.cursor, 9);
    }

    #[test]
    fn test_object_element_error() {
        let mut reader = Reader::init(":");
        let error = object_element(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "\"".to_string()
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::init("\"name\":\n");
        let error = object_element(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert!(!error.recoverable);
    }

    #[test]
    fn test_whitespace() {
        let mut reader = Reader::init("");
        assert_eq!(whitespace(&mut reader), "".to_string());
        assert_eq!(reader.state.cursor, 0);

        let mut reader = Reader::init(" x");
        assert_eq!(whitespace(&mut reader), " ".to_string());
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init("\n  x");
        assert_eq!(whitespace(&mut reader), "\n  ".to_string());
        assert_eq!(reader.state.cursor, 3);
    }
}
