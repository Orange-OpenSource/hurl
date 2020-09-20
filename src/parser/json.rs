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
use crate::core::ast::Template;
use crate::core::common::Pos;
use crate::core::common::SourceInfo;
use crate::core::json;

use super::combinators::*;
use super::error;
use super::primitives::*;
use super::reader::*;
use super::template::*;
use super::ParseResult;

pub fn parse(reader: &mut Reader) -> ParseResult<'static, json::Value> {
    choice(
        vec![
            null_value,
            boolean_value,
            string_value,
            number_value,
            list_value,
            object_value,
        ],
        reader,
    )
}

fn null_value(reader: &mut Reader) -> ParseResult<'static, json::Value> {
    try_literal("null", reader)?;
    Ok(json::Value::Null {})
}

fn boolean_value(reader: &mut Reader) -> ParseResult<'static, json::Value> {
    let value = boolean(reader)?;
    Ok(json::Value::Boolean(value))
}

fn string_value(reader: &mut Reader) -> ParseResult<'static, json::Value> {
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
    Ok(json::Value::String(template))
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
    let digit1 = nonrecover(|r| hex_digit(r), reader)?;
    let digit2 = nonrecover(|r| hex_digit(r), reader)?;
    let digit3 = nonrecover(|r| hex_digit(r), reader)?;
    let digit4 = nonrecover(|r| hex_digit(r), reader)?;
    let value = digit1 * (16 ^ 3) + digit2 * (16 ^ 2) + digit3 * 16 + digit4;
    Ok(value)
}

fn number_value(reader: &mut Reader) -> ParseResult<'static, json::Value> {
    let start = reader.state.pos.clone();

    let sign = match try_literal("-", reader) {
        Err(_) => "".to_string(),
        Ok(_) => "-".to_string(),
    };

    let integer = match try_literal("0", reader) {
        Err(_) => {
            let digits = reader.read_while(|c| c.is_ascii_digit());
            if digits.is_empty() {
                return Err(error::Error {
                    pos: start,
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

    Ok(json::Value::Number(format!(
        "{}{}{}{}",
        sign, integer, fraction, exponent
    )))
}

fn list_value(reader: &mut Reader) -> ParseResult<'static, json::Value> {
    try_literal("[", reader)?;
    let space0 = whitespace(reader);
    let mut elements = vec![];

    // at least one element
    if !reader.remaining().starts_with(']') {
        let first_element = list_element(None, reader)?;
        elements.push(first_element.clone());

        loop {
            if reader.remaining().starts_with(']') {
                break;
            }
            if !reader.remaining().starts_with(',') {
                break;
            }
            literal(",", reader)?;
            let element = list_element(Some(first_element.value._type()), reader)?;
            elements.push(element);
        }
    }
    literal("]", reader)?;

    Ok(json::Value::List { space0, elements })
}

fn list_element(
    _type: Option<String>,
    reader: &mut Reader,
) -> ParseResult<'static, json::ListElement> {
    let save = reader.state.pos.clone();
    let space0 = whitespace(reader);
    let pos = reader.state.pos.clone();
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
    if let Some(t) = _type {
        if t != value._type() {
            return Err(error::Error {
                pos,
                recoverable: false,
                inner: error::ParseError::Expecting { value: t },
            });
        }
    }
    let space1 = whitespace(reader);
    Ok(json::ListElement {
        space0,
        value,
        space1,
    })
}

fn object_value(reader: &mut Reader) -> ParseResult<'static, json::Value> {
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

    Ok(json::Value::Object { space0, elements })
}

fn object_element(reader: &mut Reader) -> ParseResult<'static, json::ObjectElement> {
    let space0 = whitespace(reader);
    literal("\"", reader)?;
    let name = key(reader)?;
    literal("\"", reader)?;
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
    Ok(json::ObjectElement {
        space0,
        name,
        space1,
        space2,
        value,
        space3,
    })
}

fn key(reader: &mut Reader) -> ParseResult<'static, String> {
    let s = reader
        .read_while(|c| c.is_alphanumeric() || *c == '@' || *c == '.' || *c == '_' || *c == '-');
    if s.is_empty() {
        let pos = reader.state.pos.clone();
        Err(error::Error {
            pos,
            recoverable: false,
            inner: error::ParseError::Expecting {
                value: "key".to_string(),
            },
        })
    } else {
        Ok(s)
    }
}

fn whitespace(reader: &mut Reader) -> String {
    reader.read_while(|c| *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r')
}

#[cfg(test)]
mod tests {
    use crate::core::ast::TemplateElement;

    use super::*;

    #[test]
    fn test_parse_error() {
        let mut reader = Reader::init("{ \"a\":\n}");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert_eq!(error.recoverable, false);

        let mut reader = Reader::init("[0,1,]");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 6 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert_eq!(error.recoverable, false);
    }

    #[test]
    fn test_null_value() {
        let mut reader = Reader::init("null");
        assert_eq!(null_value(&mut reader).unwrap(), json::Value::Null {});
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
        assert_eq!(error.recoverable, true);
    }

    #[test]
    fn test_boolean_value() {
        let mut reader = Reader::init("true");
        assert_eq!(
            boolean_value(&mut reader).unwrap(),
            json::Value::Boolean(true)
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
        assert_eq!(error.recoverable, true);
    }

    #[test]
    fn test_string_value() {
        let mut reader = Reader::init("\"\"");
        assert_eq!(
            string_value(&mut reader).unwrap(),
            json::Value::String(Template {
                quotes: true,
                elements: vec![],
                source_info: SourceInfo::init(1, 2, 1, 2),
            })
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("\"Hello\\u0020{{name}}!\"");
        assert_eq!(
            string_value(&mut reader).unwrap(),
            json::tests::hello_world_value()
        );
        assert_eq!(reader.state.cursor, 22);

        let mut reader = Reader::init("\"{}\"");
        assert_eq!(
            string_value(&mut reader).unwrap(),
            json::Value::String(Template {
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
        assert_eq!(error.recoverable, true);

        let mut reader = Reader::init("\"1");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "\"".to_string()
            }
        );
        assert_eq!(error.recoverable, false);

        let mut reader = Reader::init("\"{{x\"");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "}}".to_string()
            }
        );
        assert_eq!(error.recoverable, false);
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
        assert_eq!(error.recoverable, true);

        let mut reader = Reader::init("\t");
        let error = any_char(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.recoverable, true);
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
        assert_eq!(error.recoverable, true);
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
        assert_eq!(error.recoverable, false);
    }

    #[test]
    fn test_number_value() {
        let mut reader = Reader::init("100");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            json::Value::Number("100".to_string())
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("1.333");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            json::Value::Number("1.333".to_string())
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("-1");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            json::Value::Number("-1".to_string())
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("00");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            json::Value::Number("0".to_string())
        );
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init("1e0");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            json::Value::Number("1e0".to_string())
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("1e005");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            json::Value::Number("1e005".to_string())
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("1e-005");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            json::Value::Number("1e-005".to_string())
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
        assert_eq!(error.recoverable, true);

        let mut reader = Reader::init("1.x");
        let error = number_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "digits".to_string()
            }
        );
        assert_eq!(error.recoverable, false);
    }

    #[test]
    fn test_list_value() {
        let mut reader = Reader::init("[]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            json::Value::List {
                space0: "".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("[ ]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            json::Value::List {
                space0: " ".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("[true]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            json::Value::List {
                space0: "".to_string(),
                elements: vec![json::ListElement {
                    space0: "".to_string(),
                    value: json::Value::Boolean(true),
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
        assert_eq!(error.recoverable, true);

        let mut reader = Reader::init("[1, true]");
        let error = list_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "number".to_string()
            }
        );
        assert_eq!(error.recoverable, false);

        let mut reader = Reader::init("[1, 2,]");
        let error = list_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert_eq!(error.recoverable, false);
    }

    #[test]
    fn test_list_element() {
        let mut reader = Reader::init("true");
        assert_eq!(
            list_element(None, &mut reader).unwrap(),
            json::ListElement {
                space0: "".to_string(),
                value: json::Value::Boolean(true),
                space1: "".to_string(),
            }
        );
        assert_eq!(reader.state.cursor, 4);
    }

    #[test]
    fn test_list_element_error() {
        let mut reader = Reader::init("true");
        let error = list_element(Some("number".to_string()), &mut reader)
            .err()
            .unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            error::ParseError::Expecting {
                value: "number".to_string()
            }
        );
        assert_eq!(error.recoverable, false);

        let mut reader = Reader::init("\n]");
        let error = list_element(Some("number".to_string()), &mut reader)
            .err()
            .unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert_eq!(error.recoverable, false);
    }

    #[test]
    fn test_object_value() {
        let mut reader = Reader::init("{}");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            json::Value::Object {
                space0: "".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("{ }");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            json::Value::Object {
                space0: " ".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("{\n  \"a\": true\n}");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            json::Value::Object {
                space0: "\n  ".to_string(),
                elements: vec![json::ObjectElement {
                    space0: "".to_string(),
                    name: "a".to_string(),
                    space1: "".to_string(),
                    space2: " ".to_string(),
                    value: json::Value::Boolean(true),
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
        assert_eq!(error.recoverable, true);
    }

    #[test]
    fn test_object_error() {
        let mut reader = Reader::init("{ \"a\":\n}");
        let error = object_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert_eq!(error.recoverable, false);
    }

    #[test]
    fn test_object_element() {
        let mut reader = Reader::init("\"a\": true");
        assert_eq!(
            object_element(&mut reader).unwrap(),
            json::ObjectElement {
                space0: "".to_string(),
                name: "a".to_string(),
                space1: "".to_string(),
                space2: " ".to_string(),
                value: json::Value::Boolean(true),
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
        assert_eq!(error.recoverable, false);

        let mut reader = Reader::init("\"name\":\n");
        let error = object_element(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
        assert_eq!(error.inner, error::ParseError::Json {});
        assert_eq!(error.recoverable, false);
    }

    #[test]
    fn test_key() {
        let mut reader = Reader::init("name");
        assert_eq!(key(&mut reader).unwrap(), "name".to_string());
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("@timestamp");
        assert_eq!(key(&mut reader).unwrap(), "@timestamp".to_string());
        assert_eq!(reader.state.cursor, 10);

        let mut reader = Reader::init("data.q_client-dns");
        assert_eq!(key(&mut reader).unwrap(), "data.q_client-dns".to_string());
        assert_eq!(reader.state.cursor, 17);
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
