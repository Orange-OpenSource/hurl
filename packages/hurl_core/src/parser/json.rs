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
use crate::ast::{JsonListElement, JsonObjectElement, JsonValue, SourceInfo, Template};
use crate::combinator::{choice, non_recover, ParseError as ParseErrorTrait};
use crate::parser::primitives::{boolean, hex_digit, literal, try_literal};
use crate::parser::template::EncodedString;
use crate::parser::{templatize, JsonErrorVariant, ParseError, ParseErrorKind, ParseResult};
use crate::reader::{Pos, Reader};

pub fn parse(reader: &mut Reader) -> ParseResult<JsonValue> {
    choice(
        &[
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

/// Helper for parse, but already knowing that we are inside a JSON body.
fn parse_in_json(reader: &mut Reader) -> ParseResult<JsonValue> {
    if let Some(c) = reader.peek() {
        if c == ',' {
            let kind = ParseErrorKind::Json(JsonErrorVariant::EmptyElement);
            return Err(ParseError::new(reader.cursor().pos, false, kind));
        }
    }
    match parse(reader) {
        Ok(r) => Ok(r),
        // The only error that is recoverable is caused by reaching object_value try_literal('{'),
        // but this is not recoverable in this case, because we already know that we are in a JSON
        // body. So, we change the error to CannotResolve for the object found.
        Err(e) => match e {
            ParseError {
                recoverable: true, ..
            } => {
                let kind = ParseErrorKind::Json(JsonErrorVariant::ExpectingElement);
                Err(ParseError::new(e.pos, false, kind))
            }
            _ => Err(ParseError::new(e.pos, false, e.kind)),
        },
    }
}

pub fn null_value(reader: &mut Reader) -> ParseResult<JsonValue> {
    try_literal("null", reader)?;
    Ok(JsonValue::Null)
}

pub fn boolean_value(reader: &mut Reader) -> ParseResult<JsonValue> {
    let value = boolean(reader)?;
    Ok(JsonValue::Boolean(value))
}

fn string_value(reader: &mut Reader) -> ParseResult<JsonValue> {
    let template = string_template(reader)?;
    Ok(JsonValue::String(template))
}

fn string_template(reader: &mut Reader) -> ParseResult<Template> {
    try_literal("\"", reader)?;
    let delimiter = Some('"');
    let mut chars = vec![];
    let start = reader.cursor();
    loop {
        if reader.peek() == Some('"') || reader.is_eof() {
            break;
        }
        let char = any_char(reader)?;
        chars.push(char);
    }
    let end = reader.cursor();

    let encoded_string = EncodedString {
        source_info: SourceInfo::new(start.pos, end.pos),
        chars,
    };
    literal("\"", reader)?;
    let elements = templatize(encoded_string)?;

    let template = Template {
        delimiter,
        elements,
        source_info: SourceInfo::new(start.pos, end.pos),
    };
    Ok(template)
}

fn any_char(reader: &mut Reader) -> ParseResult<(char, String, Pos)> {
    let start = reader.cursor();
    match escape_char(reader) {
        Ok(c) => Ok((c, reader.read_from(start.index), start.pos)),
        Err(e) => {
            if e.recoverable {
                reader.seek(start);
                match reader.read() {
                    None => {
                        let kind = ParseErrorKind::Expecting {
                            value: "char".to_string(),
                        };
                        Err(ParseError::new(start.pos, true, kind))
                    }
                    Some(c) => {
                        if ['\\', '\x08', '\n', '\x0c', '\r', '\t'].contains(&c) {
                            let kind = ParseErrorKind::Expecting {
                                value: "char".to_string(),
                            };
                            Err(ParseError::new(start.pos, true, kind))
                        } else {
                            Ok((c, reader.read_from(start.index), start.pos))
                        }
                    }
                }
            } else {
                Err(e)
            }
        }
    }
}

fn escape_char(reader: &mut Reader) -> ParseResult<char> {
    try_literal("\\", reader)?;
    let start = reader.cursor();
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
        _ => Err(ParseError::new(
            start.pos,
            false,
            ParseErrorKind::EscapeChar,
        )),
    }
}

fn unicode(reader: &mut Reader) -> ParseResult<char> {
    let start = reader.cursor();
    let cp1 = hex_value(reader)?;
    let cp = if is_surrogate(cp1) {
        literal("\\u", reader)?;
        let start = reader.cursor();
        let cp2 = hex_value(reader)?;
        match cp_surrogate_pair(cp1, cp2) {
            None => return Err(ParseError::new(start.pos, false, ParseErrorKind::Unicode)),
            Some(cp) => cp,
        }
    } else {
        cp1
    };
    let c = match char::from_u32(cp) {
        None => return Err(ParseError::new(start.pos, false, ParseErrorKind::Unicode)),
        Some(c) => c,
    };
    Ok(c)
}

// 0xd800-0xdc00 encodes the high 10 bits of a pair.
// 0xdc00-0xe000 encodes the low 10 bits of a pair.
// the value is those 20 bits plus 0x10000.
const SURR1: u32 = 0xd800;
const SURR2: u32 = 0xdc00;
const SURR3: u32 = 0xe000;
const SURR_SELF: u32 = 0x10000;

/// Returns whether the specified Unicode code point can appear in a surrogate pair.
fn is_surrogate(cp: u32) -> bool {
    (SURR1..SURR3).contains(&cp)
}

fn cp_surrogate_pair(cp1: u32, cp2: u32) -> Option<u32> {
    if (SURR1..SURR2).contains(&cp1) && (SURR2..SURR3).contains(&cp2) {
        Some(((cp1 - SURR1) << 10) | ((cp2 - SURR2) + SURR_SELF))
    } else {
        None
    }
}

fn hex_value(reader: &mut Reader) -> ParseResult<u32> {
    let digit1 = non_recover(hex_digit, reader)?;
    let digit2 = non_recover(hex_digit, reader)?;
    let digit3 = non_recover(hex_digit, reader)?;
    let digit4 = non_recover(hex_digit, reader)?;
    let value = digit1 * (16 * 16 * 16) + digit2 * (16 * 16) + digit3 * 16 + digit4;
    Ok(value)
}

pub fn number_value(reader: &mut Reader) -> ParseResult<JsonValue> {
    let sign = match try_literal("-", reader) {
        Err(_) => String::new(),
        Ok(_) => "-".to_string(),
    };
    let int = integer(reader)?;
    let frac = fraction(reader)?;
    let exp = exponent(reader)?;
    Ok(JsonValue::Number(format!("{sign}{int}{frac}{exp}")))
}

fn integer(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.cursor();
    match try_literal("0", reader) {
        Err(_) => {
            let digits = reader.read_while(|c| c.is_ascii_digit());
            if digits.is_empty() {
                let kind = ParseErrorKind::Expecting {
                    value: "number".to_string(),
                };
                Err(ParseError::new(start.pos, true, kind))
            } else {
                Ok(digits)
            }
        }
        Ok(_) => Ok("0".to_string()),
    }
}

fn fraction(reader: &mut Reader) -> ParseResult<String> {
    match try_literal(".", reader) {
        Ok(_) => {
            let digits = reader.read_while(|c| c.is_ascii_digit());
            if digits.is_empty() {
                let kind = ParseErrorKind::Expecting {
                    value: "digits".to_string(),
                };
                Err(ParseError::new(reader.cursor().pos, false, kind))
            } else {
                Ok(format!(".{digits}"))
            }
        }
        Err(_) => Ok(String::new()),
    }
}

fn exponent(reader: &mut Reader) -> ParseResult<String> {
    if reader.peek() == Some('e') || reader.peek() == Some('E') {
        reader.read();
        let exponent_sign = match try_literal("-", reader) {
            Ok(_) => "-".to_string(),
            Err(_) => match try_literal("+", reader) {
                Ok(_) => "+".to_string(),
                Err(_) => String::new(),
            },
        };
        let exponent_digits = reader.read_while(|c| c.is_ascii_digit());
        Ok(format!("e{exponent_sign}{exponent_digits}"))
    } else {
        Ok(String::new())
    }
}

fn expression_value(reader: &mut Reader) -> ParseResult<JsonValue> {
    let exp = placeholder::parse(reader)?;
    Ok(JsonValue::Placeholder(exp))
}

fn list_value(reader: &mut Reader) -> ParseResult<JsonValue> {
    try_literal("[", reader)?;
    let space0 = whitespace(reader);
    let mut elements = vec![];

    // at least one element
    if reader.peek() != Some(']') {
        let first_element = list_element(reader)?;
        elements.push(first_element);

        loop {
            if reader.peek() == Some(']') {
                break;
            }
            // Reports "expecting ']'" in case the user forgot to add the last ']', e.g
            // `[1, 2`
            if reader.peek() != Some(',') {
                break;
            }
            // The reader advances after literal(","), so this saves its position to report an
            // error in case it happens.
            let save = reader.cursor();
            literal(",", reader)?;
            // If there is one more comma, e.g. [1, 2,], it's better to report to the user because
            // this occurrence is common.
            if reader.peek_if(|c| !is_whitespace(c)) == Some(']') {
                let kind = ParseErrorKind::Json(JsonErrorVariant::TrailingComma);
                return Err(ParseError::new(save.pos, false, kind));
            }
            let element = list_element(reader)?;
            elements.push(element);
        }
    }
    literal("]", reader)?;

    Ok(JsonValue::List { space0, elements })
}

fn list_element(reader: &mut Reader) -> ParseResult<JsonListElement> {
    let space0 = whitespace(reader);
    let value = parse_in_json(reader)?;
    let space1 = whitespace(reader);
    Ok(JsonListElement {
        space0,
        value,
        space1,
    })
}

pub fn object_value(reader: &mut Reader) -> ParseResult<JsonValue> {
    try_literal("{", reader)?;
    let space0 = whitespace(reader);
    let mut elements = vec![];
    if reader.peek() != Some('}') {
        let first_element = object_element(reader)?;
        elements.push(first_element);

        loop {
            if reader.peek() == Some('}') {
                break;
            }
            // Reports "expecting '}'" in case the user forgot to add the last '}', e.g
            // `{"name": "abc"`
            if reader.peek() != Some(',') {
                break;
            }
            // The reader advances after literal(","), so this saves its position to report an
            // error in case it happens.
            let save = reader.cursor();
            literal(",", reader)?;
            // If there is one more comma, e.g. {"a": "b",}, it's better to report to the user
            // because this occurrence is common.
            if reader.peek_if(|c| !is_whitespace(c)) == Some('}') {
                let kind = ParseErrorKind::Json(JsonErrorVariant::TrailingComma);
                return Err(ParseError::new(save.pos, false, kind));
            }
            let element = object_element(reader)?;
            elements.push(element);
        }
    }

    // at least one element

    literal("}", reader)?;

    Ok(JsonValue::Object { space0, elements })
}

fn key(reader: &mut Reader) -> ParseResult<Template> {
    let name = string_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(name)
}

fn object_element(reader: &mut Reader) -> ParseResult<JsonObjectElement> {
    let space0 = whitespace(reader);
    //literal("\"", reader)?;
    let name = key(reader)?;
    //literal("\"", reader)?;
    let space1 = whitespace(reader);
    literal(":", reader)?;
    let save = reader.cursor();
    let space2 = whitespace(reader);
    // Checks if there is no element after ':'. In this case, a special error must be reported
    // because this is a common occurrence.
    let next_char = reader.peek();
    // Comparing to None because `next_char` can be EOF.
    if next_char == Some('}') || next_char.is_none() {
        let kind = ParseErrorKind::Json(JsonErrorVariant::EmptyElement);
        return Err(ParseError::new(save.pos, false, kind));
    }
    let value = parse_in_json(reader)?;
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

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fn whitespace(reader: &mut Reader) -> String {
    reader.read_while(is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::typing::ToSource;

    #[test]
    fn test_parse_error() {
        let mut reader = Reader::new("{ \"a\":\n}");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Json(JsonErrorVariant::EmptyElement)
        );
        assert!(!error.recoverable);

        let mut reader = Reader::new("[0,1,]");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Json(JsonErrorVariant::TrailingComma),
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_null_value() {
        let mut reader = Reader::new("null");
        assert_eq!(null_value(&mut reader).unwrap(), JsonValue::Null);
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("true");
        let error = null_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "null".to_string()
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_integer() {
        let mut reader = Reader::new("0");
        assert_eq!(integer(&mut reader).unwrap(), "0".to_string());
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("123");
        assert_eq!(integer(&mut reader).unwrap(), "123".to_string());
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("0123");
        assert_eq!(integer(&mut reader).unwrap(), "0".to_string());
        assert_eq!(reader.cursor().index, 1);
    }

    #[test]
    fn test_fraction() {
        let mut reader = Reader::new(".5");
        assert_eq!(fraction(&mut reader).unwrap(), ".5".to_string());
        assert_eq!(reader.cursor().index, 2);
    }

    #[test]
    fn test_exponent() {
        let mut reader = Reader::new("e2");
        assert_eq!(exponent(&mut reader).unwrap(), "e2".to_string());
        assert_eq!(reader.cursor().index, 2);
    }

    #[test]
    fn test_boolean_value() {
        let mut reader = Reader::new("true");
        assert_eq!(
            boolean_value(&mut reader).unwrap(),
            JsonValue::Boolean(true)
        );
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("1");
        let error = boolean_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "true|false".to_string()
            }
        );
        assert!(error.recoverable);
    }

    pub fn json_hello_world_value() -> JsonValue {
        // "hello\u0020{{name}}!"
        JsonValue::String(Template {
            delimiter: Some('"'),
            elements: vec![
                TemplateElement::String {
                    value: "Hello ".to_string(),
                    source: "Hello\\u0020".to_source(),
                },
                TemplateElement::Placeholder(Placeholder {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                    },
                    expr: Expr {
                        kind: ExprKind::Variable(Variable {
                            name: "name".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 19)),
                        }),
                        source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 19)),
                    },
                    space1: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 19), Pos::new(1, 19)),
                    },
                }),
                TemplateElement::String {
                    value: "!".to_string(),
                    source: "!".to_source(),
                },
            ],
            source_info: SourceInfo::new(Pos::new(1, 2), Pos::new(1, 22)),
        })
    }

    #[test]
    fn test_string_value() {
        let mut reader = Reader::new("\"\"");
        assert_eq!(
            string_value(&mut reader).unwrap(),
            JsonValue::String(Template {
                delimiter: Some('"'),
                elements: vec![],
                source_info: SourceInfo::new(Pos::new(1, 2), Pos::new(1, 2)),
            })
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("\"Hello\\u0020{{name}}!\"");
        assert_eq!(string_value(&mut reader).unwrap(), json_hello_world_value());
        assert_eq!(reader.cursor().index, 22);

        let mut reader = Reader::new("\"{}\"");
        assert_eq!(
            string_value(&mut reader).unwrap(),
            JsonValue::String(Template {
                delimiter: Some('"'),
                elements: vec![TemplateElement::String {
                    value: "{}".to_string(),
                    source: "{}".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 2), Pos::new(1, 4)),
            })
        );
        assert_eq!(reader.cursor().index, 4);
    }

    #[test]
    fn test_string_value_error() {
        let mut reader = Reader::new("1");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\"".to_string()
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("\"1");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\"".to_string()
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::new("\"{{x\"");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "}}".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_any_char() {
        let mut reader = Reader::new("a");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            ('a', "a".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new(" ");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            (' ', " ".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("\\u0020 ");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            (' ', "\\u0020".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("\\t");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            ('\t', "\\t".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("#");
        assert_eq!(
            any_char(&mut reader).unwrap(),
            ('#', "#".to_string(), Pos { line: 1, column: 1 })
        );
        assert_eq!(reader.cursor().index, 1);
    }

    #[test]
    fn test_any_char_error() {
        let mut reader = Reader::new("");
        let error = any_char(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("\t");
        let error = any_char(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);
    }

    #[test]
    fn test_escape_char() {
        let mut reader = Reader::new("\\n");
        assert_eq!(escape_char(&mut reader).unwrap(), '\n');
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("\\u000a");
        assert_eq!(escape_char(&mut reader).unwrap(), '\n');
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("x");
        let error = escape_char(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\\".to_string()
            }
        );
        assert!(error.recoverable);
        assert_eq!(reader.cursor().index, 0);
    }

    #[test]
    fn test_unicode() {
        let mut reader = Reader::new("000a");
        assert_eq!(unicode(&mut reader).unwrap(), '\n');
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("c350");
        assert_eq!(unicode(&mut reader).unwrap(), '썐');
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("d83c\\udf78");
        assert_eq!(unicode(&mut reader).unwrap(), '🍸');
        assert_eq!(reader.cursor().index, 10);

        let mut reader = Reader::new("d800");
        let error = unicode(&mut reader).unwrap_err();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\\u".to_string()
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::new("d800\\ud800");
        let error = unicode(&mut reader).unwrap_err();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(error.kind, ParseErrorKind::Unicode);
        assert!(!error.recoverable);
    }

    #[test]
    fn test_hex_value() {
        let mut reader = Reader::new("0020x");
        assert_eq!(hex_value(&mut reader).unwrap(), 32);

        let mut reader = Reader::new("d800");
        assert_eq!(hex_value(&mut reader).unwrap(), 55296);

        let mut reader = Reader::new("x");
        let error = hex_value(&mut reader).unwrap_err();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::HexDigit);
        assert!(!error.recoverable);
    }

    #[test]
    fn test_number_value() {
        let mut reader = Reader::new("100");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("100".to_string())
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("1.333");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("1.333".to_string())
        );
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("-1");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("-1".to_string())
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("00");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("0".to_string())
        );
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("1e0");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("1e0".to_string())
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("1e005");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("1e005".to_string())
        );
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("1e-005");
        assert_eq!(
            number_value(&mut reader).unwrap(),
            JsonValue::Number("1e-005".to_string())
        );
        assert_eq!(reader.cursor().index, 6);
    }

    #[test]
    fn test_number_value_error() {
        let mut reader = Reader::new("true");
        let error = number_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "number".to_string()
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("1.x");
        let error = number_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "digits".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_expression_value() {
        let mut reader = Reader::new("{{n}}");
        assert_eq!(
            expression_value(&mut reader).unwrap(),
            JsonValue::Placeholder(Placeholder {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 3))
                },
                expr: Expr {
                    kind: ExprKind::Variable(Variable {
                        name: "n".to_string(),
                        source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 4))
                    }),
                    source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 4))
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4))
                }
            })
        );
        assert_eq!(reader.cursor().index, 5);
    }

    #[test]
    fn test_list_value() {
        let mut reader = Reader::new("[]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            JsonValue::List {
                space0: String::new(),
                elements: vec![]
            }
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("[ ]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            JsonValue::List {
                space0: " ".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("[true, false]");
        assert_eq!(
            list_value(&mut reader).unwrap(),
            JsonValue::List {
                space0: String::new(),
                elements: vec![
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Boolean(true),
                        space1: String::new(),
                    },
                    JsonListElement {
                        space0: String::from(" "),
                        value: JsonValue::Boolean(false),
                        space1: String::new(),
                    }
                ],
            }
        );
        assert_eq!(reader.cursor().index, 13);
    }

    #[test]
    fn test_list_error() {
        let mut reader = Reader::new("true");
        let error = list_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "[".to_string()
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("[1, 2,]");
        let error = list_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 6 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Json(JsonErrorVariant::TrailingComma),
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_list_element() {
        let mut reader = Reader::new("true");
        assert_eq!(
            list_element(&mut reader).unwrap(),
            JsonListElement {
                space0: String::new(),
                value: JsonValue::Boolean(true),
                space1: String::new(),
            }
        );
        assert_eq!(reader.cursor().index, 4);
    }

    #[test]
    fn test_object_value() {
        let mut reader = Reader::new("{}");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            JsonValue::Object {
                space0: String::new(),
                elements: vec![]
            }
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("{ }");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            JsonValue::Object {
                space0: " ".to_string(),
                elements: vec![]
            }
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("{\n  \"a\": true\n}");
        assert_eq!(
            object_value(&mut reader).unwrap(),
            JsonValue::Object {
                space0: "\n  ".to_string(),
                elements: vec![JsonObjectElement {
                    space0: String::new(),
                    name: Template {
                        delimiter: Some('"'),
                        elements: vec![TemplateElement::String {
                            value: "a".to_string(),
                            source: "a".to_source()
                        }],
                        source_info: SourceInfo::new(Pos::new(2, 4), Pos::new(2, 5))
                    },
                    space1: String::new(),
                    space2: " ".to_string(),
                    value: JsonValue::Boolean(true),
                    space3: "\n".to_string(),
                }],
            }
        );
        assert_eq!(reader.cursor().index, 15);

        let mut reader = Reader::new("true");
        let error = object_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "{".to_string()
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_object_error() {
        let mut reader = Reader::new("{ \"a\":\n}");
        let error = object_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Json(JsonErrorVariant::EmptyElement)
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_object_element() {
        let mut reader = Reader::new("\"a\": true");
        assert_eq!(
            object_element(&mut reader).unwrap(),
            JsonObjectElement {
                space0: String::new(),
                name: Template {
                    delimiter: Some('"'),
                    elements: vec![TemplateElement::String {
                        value: "a".to_string(),
                        source: "a".to_source()
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 2), Pos::new(1, 3))
                },
                space1: String::new(),
                space2: " ".to_string(),
                value: JsonValue::Boolean(true),
                space3: String::new(),
            }
        );
        assert_eq!(reader.cursor().index, 9);
    }

    #[test]
    fn test_object_element_error() {
        let mut reader = Reader::new(":");
        let error = object_element(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\"".to_string()
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::new("\"name\":\n");
        let error = object_element(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Json(JsonErrorVariant::EmptyElement),
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_whitespace() {
        let mut reader = Reader::new("");
        assert_eq!(whitespace(&mut reader), String::new());
        assert_eq!(reader.cursor().index, 0);

        let mut reader = Reader::new(" x");
        assert_eq!(whitespace(&mut reader), " ".to_string());
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("\n  x");
        assert_eq!(whitespace(&mut reader), "\n  ".to_string());
        assert_eq!(reader.cursor().index, 3);
    }
}
