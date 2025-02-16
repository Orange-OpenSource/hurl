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
use crate::ast::{SourceInfo, Template};
use crate::combinator::one_or_more;
use crate::parser::primitives::{hex_digit, literal, try_literal};
use crate::parser::{template, ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

/// Steps:
/// 1- parse String until end of stream, end of line
///    the string does not contain trailing space
/// 2- templatize
pub fn unquoted_template(reader: &mut Reader) -> ParseResult<Template> {
    let start = reader.cursor();
    let mut chars = vec![];
    let mut spaces = vec![];
    let mut end = start;
    loop {
        let pos = reader.cursor().pos;
        match any_char(&['#'], reader) {
            Err(e) => {
                if e.recoverable {
                    break;
                } else {
                    return Err(e);
                }
            }
            Ok((c, s)) => {
                if s == "\n" {
                    break;
                }
                if s == " " {
                    spaces.push((c, s, pos));
                } else {
                    if !spaces.is_empty() {
                        chars.append(&mut spaces);
                        spaces = vec![];
                    }
                    chars.push((c, s, pos));
                    end = reader.cursor();
                }
            }
        }
    }
    reader.seek(end);
    let encoded_string = template::EncodedString {
        source_info: SourceInfo::new(start.pos, end.pos),
        chars,
    };
    let elements = template::templatize(encoded_string)?;
    Ok(Template {
        delimiter: None,
        elements,
        source_info: SourceInfo::new(start.pos, end.pos),
    })
}

// TODO: should return an EncodedString
// (decoding escape sequence)
pub fn quoted_oneline_string(reader: &mut Reader) -> ParseResult<String> {
    literal("\"", reader)?;
    let s = reader.read_while(|c| c != '"' && c != '\n');
    literal("\"", reader)?;
    Ok(s)
}

pub fn quoted_template(reader: &mut Reader) -> ParseResult<Template> {
    let start = reader.cursor();
    let mut end = start;
    try_literal("\"", reader)?;
    let mut chars = vec![];
    loop {
        let pos = reader.cursor().pos;
        let save = reader.cursor();
        match any_char(&['"'], reader) {
            Err(e) => {
                if e.recoverable {
                    reader.seek(save);
                    break;
                } else {
                    return Err(e);
                }
            }
            Ok((c, s)) => {
                chars.push((c, s, pos));
                end = reader.cursor();
            }
        }
    }
    literal("\"", reader)?;
    let encoded_string = template::EncodedString {
        source_info: SourceInfo::new(start.pos, end.pos),
        chars,
    };
    let elements = template::templatize(encoded_string)?;
    Ok(Template {
        delimiter: Some('"'),
        elements,
        source_info: SourceInfo::new(start.pos, reader.cursor().pos),
    })
}

pub fn backtick_template(reader: &mut Reader) -> ParseResult<Template> {
    let delimiter = Some('`');
    let start = reader.cursor();
    let mut end = start;
    try_literal("`", reader)?;
    let mut chars = vec![];
    loop {
        let pos = reader.cursor().pos;
        let save = reader.cursor();
        match any_char(&['`', '\n'], reader) {
            Err(e) => {
                if e.recoverable {
                    reader.seek(save);
                    break;
                } else {
                    return Err(e);
                }
            }
            Ok((c, s)) => {
                chars.push((c, s, pos));
                end = reader.cursor();
            }
        }
    }
    literal("`", reader)?;
    let encoded_string = template::EncodedString {
        source_info: SourceInfo::new(start.pos, end.pos),
        chars,
    };
    let elements = template::templatize(encoded_string)?;
    Ok(Template {
        delimiter,
        elements,
        source_info: SourceInfo::new(start.pos, reader.cursor().pos),
    })
}

fn any_char(except: &[char], reader: &mut Reader) -> ParseResult<(char, String)> {
    let start = reader.cursor();
    match escape_char(reader) {
        Ok(c) => Ok((c, reader.read_from(start.index))),
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
                        if except.contains(&c)
                            || ['\\', '\x08', '\n', '\x0c', '\r', '\t'].contains(&c)
                        {
                            let kind = ParseErrorKind::Expecting {
                                value: "char".to_string(),
                            };
                            Err(ParseError::new(start.pos, true, kind))
                        } else {
                            Ok((c, reader.read_from(start.index)))
                        }
                    }
                }
            } else {
                Err(e)
            }
        }
    }
}

pub fn escape_char(reader: &mut Reader) -> ParseResult<char> {
    try_literal("\\", reader)?;
    let start = reader.cursor();
    match reader.read() {
        Some('#') => Ok('#'),
        Some('"') => Ok('"'),
        Some('`') => Ok('`'),
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

pub(crate) fn unicode(reader: &mut Reader) -> ParseResult<char> {
    literal("{", reader)?;
    let v = hex_value(reader)?;
    let c = match std::char::from_u32(v) {
        None => {
            return Err(ParseError::new(
                reader.cursor().pos,
                false,
                ParseErrorKind::Unicode,
            ))
        }
        Some(c) => c,
    };
    literal("}", reader)?;
    Ok(c)
}

fn hex_value(reader: &mut Reader) -> ParseResult<u32> {
    let mut digits = one_or_more(hex_digit, reader)?;
    let mut v = 0;
    let mut weight = 1;
    digits.reverse();
    for d in digits.iter() {
        v += weight * d;
        weight *= 16;
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    use crate::ast::{Expr, ExprKind, Placeholder, TemplateElement, Variable, Whitespace};
    use crate::reader::Pos;
    use crate::typing::ToSource;

    #[test]
    fn test_unquoted_template_empty() {
        let mut reader = Reader::new("");
        assert_eq!(
            unquoted_template(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            }
        );
        assert_eq!(reader.cursor().index, 0);
    }

    #[test]
    fn test_unquoted_template_with_hash() {
        let mut reader = Reader::new("a#");
        assert_eq!(
            unquoted_template(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "a".to_string(),
                    source: "a".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 2)),
            }
        );
        assert_eq!(reader.cursor().index, 1);
    }

    #[test]
    fn test_unquoted_template_with_encoded_hash() {
        let mut reader = Reader::new("a\\u{23}");
        assert_eq!(
            unquoted_template(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "a#".to_string(),
                    source: "a\\u{23}".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 8)),
            }
        );
        assert_eq!(reader.cursor().index, 7);
    }

    #[test]
    fn test_unquoted_template_with_quote() {
        let mut reader = Reader::new("\"hi\"");
        assert_eq!(
            unquoted_template(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "\"hi\"".to_string(),
                    source: "\"hi\"".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 5)),
            }
        );
        assert_eq!(reader.cursor().index, 4);
    }

    #[test]
    fn test_unquoted_template_hello_world() {
        let mut reader = Reader::new("hello\\u{20}{{name}}!");
        assert_eq!(
            unquoted_template(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![
                    TemplateElement::String {
                        value: "hello ".to_string(),
                        source: "hello\\u{20}".to_source(),
                    },
                    TemplateElement::Placeholder(Placeholder {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 14), Pos::new(1, 14)),
                        },
                        expr: Expr {
                            kind: ExprKind::Variable(Variable {
                                name: "name".to_string(),
                                source_info: SourceInfo::new(Pos::new(1, 14), Pos::new(1, 18)),
                            }),
                            source_info: SourceInfo::new(Pos::new(1, 14), Pos::new(1, 18)),
                        },
                        space1: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 18), Pos::new(1, 18)),
                        },
                    }),
                    TemplateElement::String {
                        value: "!".to_string(),
                        source: "!".to_source(),
                    },
                ],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 21)),
            }
        );
        assert_eq!(reader.cursor().index, 20);
    }

    #[test]
    fn test_quoted_template() {
        let mut reader = Reader::new("\"\"");
        assert_eq!(
            quoted_template(&mut reader).unwrap(),
            Template {
                delimiter: Some('"'),
                elements: vec![],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 3)),
            }
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("\"a#\"");
        assert_eq!(
            quoted_template(&mut reader).unwrap(),
            Template {
                delimiter: Some('"'),
                elements: vec![TemplateElement::String {
                    value: "a#".to_string(),
                    source: "a#".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 5)),
            }
        );
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("\"{0}\"");
        assert_eq!(
            quoted_template(&mut reader).unwrap(),
            Template {
                delimiter: Some('"'),
                elements: vec![TemplateElement::String {
                    value: "{0}".to_string(),
                    source: "{0}".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6)),
            }
        );
        assert_eq!(reader.cursor().index, 5);
    }

    #[test]
    fn test_quoted_template_with_quote() {
        // "\"hi\""
        let mut reader = Reader::new("\"\\\"hi\\\"\"");
        assert_eq!(
            quoted_template(&mut reader).unwrap(),
            Template {
                delimiter: Some('"'),
                elements: vec![TemplateElement::String {
                    value: "\"hi\"".to_string(),
                    source: "\\\"hi\\\"".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 9)),
            }
        );
        assert_eq!(reader.cursor().index, 8);
    }

    #[test]
    fn test_quoted_template_error_missing_closing_quote() {
        let mut reader = Reader::new("\"not found");
        let error = quoted_template(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 11
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_quoted_string() {
        let mut reader = Reader::new("\"\"");
        assert_eq!(quoted_oneline_string(&mut reader).unwrap(), "");
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("\"Hello\"");
        assert_eq!(quoted_oneline_string(&mut reader).unwrap(), "Hello");
        assert_eq!(reader.cursor().index, 7);
    }

    #[test]
    fn test_backtick_template() {
        let mut reader = Reader::new("``");
        assert_eq!(
            backtick_template(&mut reader).unwrap(),
            Template {
                delimiter: Some('`'),
                elements: vec![],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 3)),
            }
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("`foo#`");
        assert_eq!(
            backtick_template(&mut reader).unwrap(),
            Template {
                delimiter: Some('`'),
                elements: vec![TemplateElement::String {
                    value: "foo#".to_string(),
                    source: "foo#".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 7)),
            }
        );
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("`{0}`");
        assert_eq!(
            backtick_template(&mut reader).unwrap(),
            Template {
                delimiter: Some('`'),
                elements: vec![TemplateElement::String {
                    value: "{0}".to_string(),
                    source: "{0}".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6)),
            }
        );
        assert_eq!(reader.cursor().index, 5);
    }

    #[test]
    fn test_backtick_template_with_backtick() {
        // `\`hi\``
        let mut reader = Reader::new("`\\`hi\\``");
        assert_eq!(
            backtick_template(&mut reader).unwrap(),
            Template {
                delimiter: Some('`'),
                elements: vec![TemplateElement::String {
                    value: "`hi`".to_string(),
                    source: "\\`hi\\`".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 9)),
            }
        );
        assert_eq!(reader.cursor().index, 8);
    }

    #[test]
    fn test_backtick_template_error_missing_closing_backtick() {
        let mut reader = Reader::new("`not found");
        let error = backtick_template(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 11
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_any_char() {
        let mut reader = Reader::new("a");
        assert_eq!(any_char(&[], &mut reader).unwrap(), ('a', "a".to_string()));
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new(" ");
        assert_eq!(any_char(&[], &mut reader).unwrap(), (' ', " ".to_string()));
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("\\t");
        assert_eq!(
            any_char(&[], &mut reader).unwrap(),
            ('\t', "\\t".to_string())
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("#");
        assert_eq!(any_char(&[], &mut reader).unwrap(), ('#', "#".to_string()));
        assert_eq!(reader.cursor().index, 1);
    }

    #[test]
    fn test_any_char_quote() {
        let mut reader = Reader::new("\\\"");
        assert_eq!(
            any_char(&[], &mut reader).unwrap(),
            ('"', "\\\"".to_string())
        );
        assert_eq!(reader.cursor().index, 2);
    }

    #[test]
    fn test_any_char_error() {
        let mut reader = Reader::new("");
        let error = any_char(&[], &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("#");
        let error = any_char(&['#'], &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("\t");
        let error = any_char(&[], &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);
    }

    #[test]
    fn test_escape_char() {
        let mut reader = Reader::new("\\n");
        assert_eq!(escape_char(&mut reader).unwrap(), '\n');
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("\\u{0a}");
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
        let mut reader = Reader::new("{000a}");
        assert_eq!(unicode(&mut reader).unwrap(), '\n');
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("{E9}");
        assert_eq!(unicode(&mut reader).unwrap(), 'é');
        assert_eq!(reader.cursor().index, 4);
    }

    #[test]
    fn test_hex_value() {
        let mut reader = Reader::new("20x");
        assert_eq!(hex_value(&mut reader).unwrap(), 32);

        let mut reader = Reader::new("x");
        let error = hex_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::HexDigit);
        assert!(!error.recoverable);
    }

    #[test]
    fn test_quoted_template_benchmark() {
        // benchmark tests not in stable toolchain yet
        // Simply log duration for the time-being
        let mut reader = Reader::new(
            format!(
                "\"Hello World!\"{}",
                (0..10_000_000).map(|_| "X").collect::<String>()
            )
            .as_str(),
        );

        let now = SystemTime::now();
        assert!(quoted_template(&mut reader).is_ok());
        assert_eq!(reader.cursor().index, 14);
        eprintln!("duration= {}", now.elapsed().unwrap().as_nanos());
    }
}
