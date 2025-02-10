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
use crate::ast::{
    Base64, Comment, File, Hex, KeyValue, LineTerminator, Regex, SourceInfo, Whitespace,
};
use crate::combinator::{one_or_more, optional, recover, zero_or_more};
use crate::parser::string::unquoted_template;
use crate::parser::{base64, filename, key_string, ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;
use crate::typing::ToSource;

pub fn space(reader: &mut Reader) -> ParseResult<Whitespace> {
    let start = reader.cursor();
    match reader.read() {
        None => Err(ParseError::new(start.pos, true, ParseErrorKind::Space)),
        Some(c) => {
            if c == ' ' || c == '\t' {
                Ok(Whitespace {
                    value: c.to_string(),
                    source_info: SourceInfo::new(start.pos, reader.cursor().pos),
                })
            } else {
                Err(ParseError::new(start.pos, true, ParseErrorKind::Space))
            }
        }
    }
}

pub fn one_or_more_spaces(reader: &mut Reader) -> ParseResult<Whitespace> {
    let start = reader.cursor();
    match one_or_more(space, reader) {
        Ok(v) => {
            let s = v.iter().map(|x| x.value.clone()).collect();
            Ok(Whitespace {
                value: s,
                source_info: SourceInfo::new(start.pos, reader.cursor().pos),
            })
        }
        Err(e) => Err(e),
    }
}

pub fn zero_or_more_spaces(reader: &mut Reader) -> ParseResult<Whitespace> {
    let start = reader.cursor();
    match zero_or_more(space, reader) {
        //Ok(v) => return Ok(v.join("")),
        Ok(v) => {
            let s = v.iter().map(|x| x.value.clone()).collect();
            Ok(Whitespace {
                value: s,
                source_info: SourceInfo::new(start.pos, reader.cursor().pos),
            })
        }
        Err(e) => Err(e),
    }
}

pub fn line_terminator(reader: &mut Reader) -> ParseResult<LineTerminator> {
    let space0 = zero_or_more_spaces(reader)?;
    let comment = optional(comment, reader)?;
    let nl = if reader.is_eof() {
        Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(reader.cursor().pos, reader.cursor().pos),
        }
    } else {
        match newline(reader) {
            Ok(r) => r,
            Err(e) => {
                let kind = ParseErrorKind::Expecting {
                    value: String::from("line_terminator"),
                };
                return Err(ParseError::new(e.pos, false, kind));
            }
        }
    };

    Ok(LineTerminator {
        space0,
        comment,
        newline: nl,
    })
}

pub fn optional_line_terminators(reader: &mut Reader) -> ParseResult<Vec<LineTerminator>> {
    zero_or_more(|p2| recover(line_terminator, p2), reader)
}

pub fn comment(reader: &mut Reader) -> ParseResult<Comment> {
    try_literal("#", reader)?;
    let start = reader.cursor();
    let mut value = String::new();
    loop {
        if reader.is_eof() {
            break;
        }
        let save_state = reader.cursor();
        match newline(reader) {
            Ok(_) => {
                reader.seek(save_state);
                break;
            }
            _ => {
                reader.seek(save_state);
                if let Some(c) = reader.read() {
                    value.push(c);
                }
            }
        }
    }
    let end = reader.cursor();
    Ok(Comment {
        value,
        source_info: SourceInfo::new(start.pos, end.pos),
    })
}

/// Does not return a value, non recoverable parser. Use combinator recover to make it recoverable
pub fn literal(s: &str, reader: &mut Reader) -> ParseResult<()> {
    let start = reader.cursor();
    for c in s.chars() {
        match reader.read() {
            None => {
                let kind = ParseErrorKind::Expecting {
                    value: s.to_string(),
                };
                return Err(ParseError::new(start.pos, false, kind));
            }
            Some(x) => {
                if x != c {
                    let kind = ParseErrorKind::Expecting {
                        value: s.to_string(),
                    };
                    return Err(ParseError::new(start.pos, false, kind));
                } else {
                    continue;
                }
            }
        }
    }
    Ok(())
}

/// Recoverable version which reset the cursor, meant to be combined with following action.
pub fn try_literal(s: &str, reader: &mut Reader) -> ParseResult<()> {
    let save_state = reader.cursor();
    match literal(s, reader) {
        Ok(_) => Ok(()),
        Err(e) => {
            reader.seek(save_state);
            Err(ParseError::new(e.pos, true, e.kind))
        }
    }
}

pub fn newline(reader: &mut Reader) -> ParseResult<Whitespace> {
    let start = reader.cursor();
    match try_literal("\r\n", reader) {
        Ok(_) => Ok(Whitespace {
            value: "\r\n".to_string(),
            source_info: SourceInfo::new(start.pos, reader.cursor().pos),
        }),
        Err(_) => match literal("\n", reader) {
            Ok(_) => Ok(Whitespace {
                value: "\n".to_string(),
                source_info: SourceInfo::new(start.pos, reader.cursor().pos),
            }),
            Err(_) => {
                let kind = ParseErrorKind::Expecting {
                    value: String::from("newline"),
                };
                Err(ParseError::new(start.pos, false, kind))
            }
        },
    }
}

pub fn key_value(reader: &mut Reader) -> ParseResult<KeyValue> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let key = recover(key_string::parse, reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    recover(|reader1| literal(":", reader1), reader)?;
    let space2 = zero_or_more_spaces(reader)?;
    let value = unquoted_template(reader)?;
    let line_terminator0 = line_terminator(reader)?;
    Ok(KeyValue {
        line_terminators,
        space0,
        key,
        space1,
        space2,
        value,
        line_terminator0,
    })
}

pub fn hex(reader: &mut Reader) -> ParseResult<Hex> {
    try_literal("hex", reader)?;
    literal(",", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let mut value: Vec<u8> = vec![];
    let start = reader.cursor();
    let mut current: i32 = -1;
    loop {
        let s = reader.cursor();
        match hex_digit(reader) {
            Ok(d) => {
                if current != -1 {
                    value.push((current * 16 + d as i32) as u8);
                    current = -1;
                } else {
                    current = d as i32;
                }
            }
            Err(_) => {
                reader.seek(s);
                break;
            }
        };
    }
    if current != -1 {
        return Err(ParseError::new(
            reader.cursor().pos,
            false,
            ParseErrorKind::OddNumberOfHexDigits,
        ));
    }
    let source = reader.read_from(start.index).to_source();
    let space1 = zero_or_more_spaces(reader)?;
    literal(";", reader)?;

    Ok(Hex {
        space0,
        value,
        source,
        space1,
    })
}

pub fn regex(reader: &mut Reader) -> ParseResult<Regex> {
    try_literal("/", reader)?;
    let start = reader.cursor();
    let mut s = String::new();

    // Hurl escaping /
    // in order to avoid terminating the regex
    // eg. \a\b/
    //
    // Other escaped sequences such as \* are part of the regex expression
    // They are not part of the syntax of Hurl itself.
    loop {
        match reader.read() {
            None => {
                let kind = ParseErrorKind::RegexExpr {
                    message: "unexpected end of file".to_string(),
                };
                return Err(ParseError::new(reader.cursor().pos, false, kind));
            }
            Some('/') => break,
            Some('\\') => {
                if let Some('/') = reader.peek() {
                    reader.read();
                    s.push('/');
                } else {
                    s.push('\\');
                }
            }
            Some(c) => s.push(c),
        }
    }
    match regex::Regex::new(s.as_str()) {
        Ok(inner) => Ok(Regex { inner }),
        Err(e) => {
            let message = match e {
                regex::Error::Syntax(s) => {
                    // The regex syntax error from the crate returns a multiline String
                    // For example
                    //     regex parse error:
                    //         x{a}
                    //           ^
                    //     error: repetition quantifier expects a valid decimal
                    //
                    // To fit nicely in Hurl Error reporting, you need an error message string that does not spread on multiple lines
                    // You will assume that the error most relevant description is on the last line
                    let lines = s.split('\n').collect::<Vec<&str>>();
                    let last_line = lines.last().expect("at least one line");
                    last_line
                        .strip_prefix("error: ")
                        .unwrap_or(last_line)
                        .to_string()
                }
                regex::Error::CompiledTooBig(_) => "Size limit exceeded".to_string(),
                _ => "unknown".to_string(),
            };
            Err(ParseError::new(
                start.pos,
                false,
                ParseErrorKind::RegexExpr { message },
            ))
        }
    }
}

pub fn null(reader: &mut Reader) -> ParseResult<()> {
    try_literal("null", reader)
}

pub fn boolean(reader: &mut Reader) -> ParseResult<bool> {
    let start = reader.cursor();
    match try_literal("true", reader) {
        Ok(_) => Ok(true),
        Err(_) => match literal("false", reader) {
            Ok(_) => Ok(false),
            Err(_) => {
                let kind = ParseErrorKind::Expecting {
                    value: String::from("true|false"),
                };
                Err(ParseError::new(start.pos, true, kind))
            }
        },
    }
}

pub(crate) fn file(reader: &mut Reader) -> ParseResult<File> {
    let _start = reader.cursor();
    try_literal("file", reader)?;
    literal(",", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let f = filename::parse(reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    literal(";", reader)?;
    Ok(File {
        space0,
        filename: f,
        space1,
    })
}

pub(crate) fn base64(reader: &mut Reader) -> ParseResult<Base64> {
    // base64 => can have whitespace
    // support parser position
    try_literal("base64", reader)?;
    literal(",", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let save_state = reader.cursor();
    let value = base64::parse(reader);
    let count = reader.cursor().index - save_state.index;
    reader.seek(save_state);
    let source = reader.read_n(count).to_source();
    let space1 = zero_or_more_spaces(reader)?;
    literal(";", reader)?;
    Ok(Base64 {
        space0,
        value,
        source,
        space1,
    })
}

pub fn eof(reader: &mut Reader) -> ParseResult<()> {
    if reader.is_eof() {
        Ok(())
    } else {
        let kind = ParseErrorKind::Expecting {
            value: String::from("eof"),
        };
        Err(ParseError::new(reader.cursor().pos, false, kind))
    }
}

pub fn hex_digit_value(c: char) -> Option<u32> {
    match c.to_ascii_lowercase() {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' => Some(10),
        'b' => Some(11),
        'c' => Some(12),
        'd' => Some(13),
        'e' => Some(14),
        'f' => Some(15),
        _ => None,
    }
}

pub fn hex_digit(reader: &mut Reader) -> ParseResult<u32> {
    let start = reader.cursor();
    match reader.read() {
        Some(c) => match hex_digit_value(c) {
            Some(v) => Ok(v),
            None => Err(ParseError::new(start.pos, true, ParseErrorKind::HexDigit)),
        },
        None => Err(ParseError::new(start.pos, true, ParseErrorKind::HexDigit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expr, ExprKind, Placeholder, Template, TemplateElement, Variable};
    use crate::reader::Pos;
    use crate::typing::ToSource;

    #[test]
    fn test_space() {
        let mut reader = Reader::new("x");
        let error = space(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("  ");
        assert_eq!(
            space(&mut reader),
            Ok(Whitespace {
                value: " ".to_string(),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 2)),
            }),
        );
        assert_eq!(reader.cursor().index, 1);
    }

    #[test]
    fn test_one_or_more_spaces() {
        let mut reader = Reader::new("  ");
        assert_eq!(
            one_or_more_spaces(&mut reader),
            Ok(Whitespace {
                value: "  ".to_string(),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 3)),
            })
        );

        let mut reader = Reader::new("abc");
        let error = one_or_more_spaces(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
    }

    #[test]
    fn test_zero_or_more_spaces() {
        let mut reader = Reader::new("  ");
        assert_eq!(
            zero_or_more_spaces(&mut reader),
            Ok(Whitespace {
                value: "  ".to_string(),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 3)),
            })
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("xxx");
        assert_eq!(
            zero_or_more_spaces(&mut reader),
            Ok(Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            })
        );
        assert_eq!(reader.cursor().index, 0);

        let mut reader = Reader::new(" xxx");
        assert_eq!(
            zero_or_more_spaces(&mut reader),
            Ok(Whitespace {
                value: " ".to_string(),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 2)),
            })
        );
        assert_eq!(reader.cursor().index, 1);
    }

    #[test]
    fn test_comment() {
        //    let mut parser = Parser::init("# comment");
        //    assert_eq!(
        //        comment(&mut reader),
        //        Ok(Comment {
        //            value: " comment".to_string()
        //        })
        //    );
        //    assert_eq!(reader.state.cursor, 9);

        let mut reader = Reader::new("#\n");
        assert_eq!(
            comment(&mut reader),
            Ok(Comment {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(1, 2), Pos::new(1, 2)),
            })
        );

        let mut reader = Reader::new("# comment\n");
        assert_eq!(
            comment(&mut reader),
            Ok(Comment {
                value: " comment".to_string(),
                source_info: SourceInfo::new(Pos::new(1, 2), Pos::new(1, 10)),
            })
        );
        assert_eq!(reader.cursor().index, 9);

        let mut reader = Reader::new("xxx");
        let error = comment(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);
    }

    #[test]
    fn test_literal() {
        let mut reader = Reader::new("hello");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("hello")
            }
        );
        assert_eq!(reader.cursor().index, 0);

        let mut reader = Reader::new("hi");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("hello")
            }
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("he");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("hello")
            }
        );
        assert_eq!(reader.cursor().index, 2);
    }

    #[test]
    fn test_new_line() {
        let mut reader = Reader::new("\n");
        assert_eq!(
            newline(&mut reader).unwrap(),
            Whitespace {
                value: String::from("\n"),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(2, 1)),
            }
        );
    }

    #[test]
    fn test_key_value() {
        let mut reader = Reader::new("message: hello {{name}}! # comment");
        assert_eq!(
            key_value(&mut reader).unwrap(),
            KeyValue {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                key: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "message".to_string(),
                        source: "message".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 8)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 8)),
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 10)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![
                        TemplateElement::String {
                            value: "hello ".to_string(),
                            source: "hello ".to_source(),
                        },
                        TemplateElement::Placeholder(Placeholder {
                            space0: Whitespace {
                                value: String::new(),
                                source_info: SourceInfo::new(Pos::new(1, 18), Pos::new(1, 18)),
                            },
                            expr: Expr {
                                kind: ExprKind::Variable(Variable {
                                    name: "name".to_string(),
                                    source_info: SourceInfo::new(Pos::new(1, 18), Pos::new(1, 22)),
                                }),
                                source_info: SourceInfo::new(Pos::new(1, 18), Pos::new(1, 22)),
                            },
                            space1: Whitespace {
                                value: String::new(),
                                source_info: SourceInfo::new(Pos::new(1, 22), Pos::new(1, 22)),
                            },
                        }),
                        TemplateElement::String {
                            value: "!".to_string(),
                            source: "!".to_source(),
                        },
                    ],
                    source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(1, 25)),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: " ".to_string(),
                        source_info: SourceInfo::new(Pos::new(1, 25), Pos::new(1, 26)),
                    },
                    comment: Some(Comment {
                        value: " comment".to_string(),
                        source_info: SourceInfo::new(Pos::new(1, 27), Pos::new(1, 35)),
                    }),
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 35), Pos::new(1, 35)),
                    },
                },
            }
        );
    }

    #[test]
    fn test_key_value_template() {
        let mut reader = Reader::new("{{key}}: value");
        assert_eq!(
            key_value(&mut reader).unwrap(),
            KeyValue {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                key: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::Placeholder(Placeholder {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 3)),
                        },
                        expr: Expr {
                            kind: ExprKind::Variable(Variable {
                                name: "key".to_string(),
                                source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 6)),
                            }),
                            source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 6))
                        },
                        space1: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 6)),
                        },
                    })],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 8)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 8)),
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 10)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "value".to_string(),
                        source: "value".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(1, 15)),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                    },
                },
            }
        );
        assert_eq!(reader.cursor().index, 14);
    }

    #[test]
    fn test_key_value_recover() {
        let mut reader = Reader::new("{{key");
        let error = key_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 6 });
        assert!(error.recoverable);
        assert_eq!(reader.cursor().index, 5); // does not reset cursor

        let mut reader = Reader::new("GET ®http://google.fr");
        let error = key_value(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert!(error.recoverable);
        assert_eq!(reader.cursor().index, 5); // does not reset cursor
    }

    #[test]
    fn test_boolean() {
        let mut reader = Reader::new("true");
        assert!(boolean(&mut reader).unwrap());

        let mut reader = Reader::new("xxx");
        let error = boolean(&mut reader).err().unwrap();
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("true|false")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("trux");
        let error = boolean(&mut reader).err().unwrap();
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("true|false")
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_hex_digit() {
        let mut reader = Reader::new("0");
        assert_eq!(hex_digit(&mut reader).unwrap(), 0);

        let mut reader = Reader::new("x");
        let error = hex_digit(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::HexDigit);
        assert!(error.recoverable);
    }

    #[test]
    fn test_hex() {
        let mut reader = Reader::new("hex, ff;");
        assert_eq!(
            hex(&mut reader).unwrap(),
            Hex {
                space0: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 5), Pos::new(1, 6)),
                },
                value: vec![255],
                source: "ff".to_source(),
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 8)),
                },
            }
        );

        let mut reader = Reader::new("hex,010203 ;");
        assert_eq!(
            hex(&mut reader).unwrap(),
            Hex {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 5), Pos::new(1, 5)),
                },
                value: vec![1, 2, 3],
                source: "010203".to_source(),
                space1: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 12)),
                },
            }
        );
    }

    #[test]
    fn test_hex_error() {
        let mut reader = Reader::new("hex,012;");
        let error = hex(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
        assert_eq!(error.kind, ParseErrorKind::OddNumberOfHexDigits);
    }

    #[test]
    fn test_regex() {
        let mut reader = Reader::new(r#"/a{3}/"#);
        assert_eq!(
            regex(&mut reader).unwrap(),
            Regex {
                inner: regex::Regex::new(r#"a{3}"#).unwrap()
            }
        );

        let mut reader = Reader::new(r"/a\/b/");
        assert_eq!(
            regex(&mut reader).unwrap(),
            Regex {
                inner: regex::Regex::new(r#"a/b"#).unwrap()
            }
        );

        let mut reader = Reader::new(r"/a\.b/");
        assert_eq!(
            regex(&mut reader).unwrap(),
            Regex {
                inner: regex::Regex::new(r"a\.b").unwrap()
            }
        );

        let mut reader = Reader::new(r"/\d{4}-\d{2}-\d{2}/");
        assert_eq!(
            regex(&mut reader).unwrap(),
            Regex {
                inner: regex::Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap()
            }
        );
    }

    #[test]
    fn test_regex_error() {
        let mut reader = Reader::new("xxx");
        let error = regex(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("/xxx");
        let error = regex(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert!(!error.recoverable);
        assert_eq!(
            error.kind,
            ParseErrorKind::RegexExpr {
                message: "unexpected end of file".to_string()
            }
        );

        let mut reader = Reader::new("/x{a}/");
        let error = regex(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(!error.recoverable);
        assert_eq!(
            error.kind,
            ParseErrorKind::RegexExpr {
                message: "repetition quantifier expects a valid decimal".to_string()
            }
        );
    }

    #[test]
    fn test_file() {
        let mut reader = Reader::new("file,data.xml;");
        assert_eq!(
            file(&mut reader).unwrap(),
            File {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 6)),
                },
                filename: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "data.xml".to_string(),
                        source: "data.xml".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 14)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 14), Pos::new(1, 14)),
                },
            }
        );

        let mut reader = Reader::new("file, filename1;");
        assert_eq!(
            file(&mut reader).unwrap(),
            File {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 7)),
                },
                filename: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "filename1".to_string(),
                        source: "filename1".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 16)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 16), Pos::new(1, 16)),
                },
            }
        );

        let mut reader = Reader::new("file, tmp/filename1;");
        assert_eq!(
            file(&mut reader).unwrap(),
            File {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 7)),
                },
                filename: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "tmp/filename1".to_string(),
                        source: "tmp/filename1".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 20)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 20), Pos::new(1, 20)),
                },
            }
        );

        let mut reader = Reader::new(r"file, tmp/filename\ with\ spaces.txt;");
        assert_eq!(
            file(&mut reader).unwrap(),
            File {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 7)),
                },
                filename: Template {
                    elements: vec![TemplateElement::String {
                        value: "tmp/filename with spaces.txt".to_string(),
                        source: "tmp/filename\\ with\\ spaces.txt".to_source(),
                    }],
                    delimiter: None,
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 37)),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 37), Pos::new(1, 37)),
                },
            }
        );
    }

    #[test]
    fn test_file_error() {
        let mut reader = Reader::new("fil; filename1;");
        let error = file(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("file, filename1");
        let error = file(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 16,
            }
        );
        assert!(!error.recoverable);
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from(";")
            }
        );

        let mut reader = Reader::new(r"file, tmp/filename\ with\ unescaped .txt;");
        let error = file(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 37,
            }
        );
        assert!(!error.recoverable);
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from(";")
            }
        );
    }

    #[test]
    fn test_base64() {
        let mut reader = Reader::new("base64,  T WE=;xxx");
        assert_eq!(
            base64(&mut reader).unwrap(),
            Base64 {
                space0: Whitespace {
                    value: String::from("  "),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 10)),
                },
                value: vec![77, 97],
                source: "T WE=".to_source(),
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                },
            }
        );
        assert_eq!(reader.cursor().index, 15);
    }
}
