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

use super::base64;
use super::combinators::*;
use super::error::*;
use super::filename;
use super::reader::Reader;
use super::string::*;
use super::template;
use super::ParseResult;

pub fn space(reader: &mut Reader) -> ParseResult<'static, Whitespace> {
    let start = reader.state.clone();
    match reader.read() {
        None => Err(Error {
            pos: start.pos,
            recoverable: true,
            inner: ParseError::Space {},
        }),
        Some(c) => {
            if c == ' ' || c == '\t' {
                Ok(Whitespace {
                    value: c.to_string(),
                    source_info: SourceInfo::init(
                        start.pos.line,
                        start.pos.column,
                        reader.state.pos.line,
                        reader.state.pos.column,
                    ),
                })
            } else {
                Err(Error {
                    pos: start.pos,
                    recoverable: true,
                    inner: ParseError::Space {},
                })
            }
        }
    }
}

pub fn one_or_more_spaces<'a>(reader: &mut Reader) -> ParseResult<'a, Whitespace> {
    let start = reader.state.clone();
    match one_or_more(space, reader) {
        Ok(v) => {
            let s = v.iter().map(|x| x.value.clone()).collect();
            Ok(Whitespace {
                value: s,
                source_info: SourceInfo::init(
                    start.pos.line,
                    start.pos.column,
                    reader.state.pos.line,
                    reader.state.pos.column,
                ),
            })
        }
        Err(e) => Err(e),
    }
}

pub fn zero_or_more_spaces<'a>(reader: &mut Reader) -> ParseResult<'a, Whitespace> {
    let start = reader.state.clone();
    match zero_or_more(space, reader) {
        //Ok(v) => return Ok(v.join("")),
        Ok(v) => {
            let s = v.iter().map(|x| x.value.clone()).collect();
            Ok(Whitespace {
                value: s,
                source_info: SourceInfo::init(
                    start.pos.line,
                    start.pos.column,
                    reader.state.pos.line,
                    reader.state.pos.column,
                ),
            })
        }
        Err(e) => Err(e),
    }
}

pub fn line_terminator(reader: &mut Reader) -> ParseResult<'static, LineTerminator> {
    // let start = p.state.clone();
    let space0 = zero_or_more_spaces(reader)?;
    let comment = optional(comment, reader)?;
    let nl = if reader.is_eof() {
        Whitespace {
            value: "".to_string(),
            source_info: SourceInfo::init(
                reader.state.pos.line,
                reader.state.pos.column,
                reader.state.pos.line,
                reader.state.pos.column,
            ),
        }
    } else {
        match newline(reader) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error {
                    pos: e.pos,
                    recoverable: false,
                    inner: ParseError::Expecting {
                        value: String::from("line_terminator"),
                    },
                });
            }
        }
    };

    Ok(LineTerminator {
        space0,
        comment,
        newline: nl,
    })
}

pub fn optional_line_terminators(reader: &mut Reader) -> ParseResult<'static, Vec<LineTerminator>> {
    zero_or_more(|p2| recover(line_terminator, p2), reader)
}

pub fn comment(reader: &mut Reader) -> ParseResult<'static, Comment> {
    try_literal("#", reader)?;
    let mut value = "".to_string();
    loop {
        if reader.is_eof() {
            break;
        }
        let save_state = reader.state.clone();
        match newline(reader) {
            Ok(_) => {
                reader.state = save_state;
                break;
            }
            _ => {
                reader.state = save_state;
                if let Some(c) = reader.read() {
                    value.push(c)
                }
            }
        }
    }
    Ok(Comment { value })
}

pub fn literal(s: &str, reader: &mut Reader) -> ParseResult<'static, ()> {
    // does not return a value
    // non recoverable parser
    // => use combinator recover to make it recoverable

    let start = reader.state.clone();
    for c in s.chars() {
        let _state = reader.state.clone();
        match reader.read() {
            None => {
                return Err(Error {
                    pos: start.pos,
                    recoverable: false,
                    inner: ParseError::Expecting {
                        value: s.to_string(),
                    },
                });
            }
            Some(x) => {
                if x != c {
                    return Err(Error {
                        pos: start.pos,
                        recoverable: false,
                        inner: ParseError::Expecting {
                            value: s.to_string(),
                        },
                    });
                } else {
                    continue;
                }
            }
        }
    }
    Ok(())
}

pub fn try_literal(s: &str, reader: &mut Reader) -> ParseResult<'static, ()> {
    // recoverable version which reset the cursor
    // meant to be combined with following action
    let save_state = reader.state.clone();
    match literal(s, reader) {
        Ok(_) => Ok(()),
        Err(e) => {
            reader.state = save_state;
            Err(Error {
                pos: e.pos,
                recoverable: true,
                inner: e.inner,
            })
        }
    }
}

// return the literal string
pub fn try_literals(s1: &str, s2: &str, reader: &mut Reader) -> ParseResult<'static, String> {
    let start = reader.state.clone();
    match literal(s1, reader) {
        Ok(_) => Ok(s1.to_string()),
        Err(_) => {
            reader.state = start.clone();
            match literal(s2, reader) {
                Ok(_) => Ok(s2.to_string()),
                Err(_) => {
                    reader.state = start.clone();
                    return Err(Error {
                        pos: start.pos,
                        recoverable: true,
                        inner: ParseError::Expecting {
                            value: format!("<{}> or <{}>", s1, s2),
                        },
                    });
                }
            }
        }
    }
}

pub fn newline(reader: &mut Reader) -> ParseResult<'static, Whitespace> {
    let start = reader.state.clone();
    match try_literal("\r\n", reader) {
        Ok(_) => Ok(Whitespace {
            value: "\r\n".to_string(),
            source_info: SourceInfo::init(
                start.pos.line,
                start.pos.column,
                reader.state.pos.line,
                reader.state.pos.column,
            ),
        }),
        Err(_) => match literal("\n", reader) {
            Ok(_) => Ok(Whitespace {
                value: "\n".to_string(),
                source_info: SourceInfo::init(
                    start.pos.line,
                    start.pos.column,
                    reader.state.pos.line,
                    reader.state.pos.column,
                ),
            }),
            Err(_) => Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::Expecting {
                    value: String::from("newline"),
                },
            }),
        },
    }
}

pub fn key_value(reader: &mut Reader) -> ParseResult<'static, KeyValue> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let key = recover(unquoted_string_key, reader)?;
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

pub fn hex(reader: &mut Reader) -> ParseResult<'static, Hex> {
    try_literal("hex", reader)?;
    literal(",", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let mut value: Vec<u8> = vec![];
    let start = reader.state.cursor;
    let mut current: i32 = -1;
    loop {
        let s = reader.state.clone();
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
                reader.state = s;
                break;
            }
        };
    }
    if current != -1 {
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: false,
            inner: ParseError::OddNumberOfHexDigits {},
        });
    }
    let encoded = reader.from(start);
    let space1 = zero_or_more_spaces(reader)?;
    literal(";", reader)?;

    Ok(Hex {
        space0,
        value,
        encoded,
        space1,
    })
}

pub fn regex(reader: &mut Reader) -> ParseResult<'static, Regex> {
    try_literal("/", reader)?;
    let start = reader.state.pos.clone();
    let mut s = String::from("");

    // Hurl escaping /
    // in order to avoid terminating the regex
    // eg. \a\b/
    //
    // Other escaped sequences such as \* are part of the regex expression
    // They are not part of the syntax of Hurl itself.
    loop {
        match reader.read() {
            None => {
                return Err(Error {
                    pos: reader.state.pos.clone(),
                    recoverable: false,
                    inner: ParseError::Eof {},
                })
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
                    let lines = s.split('\n').clone().collect::<Vec<&str>>();
                    let last_line = lines.last().expect("at least one line");
                    last_line
                        .strip_prefix("error: ")
                        .unwrap_or(last_line)
                        .to_string()
                }
                regex::Error::CompiledTooBig(_) => "Size limit exceeded".to_string(),
                _ => "unknown".to_string(),
            };
            Err(Error {
                pos: start,
                recoverable: false,
                inner: ParseError::RegexExpr { message },
            })
        }
    }
}

pub fn null(reader: &mut Reader) -> ParseResult<'static, ()> {
    try_literal("null", reader)
}

pub fn boolean(reader: &mut Reader) -> ParseResult<'static, bool> {
    let start = reader.state.clone();
    match try_literal("true", reader) {
        Ok(_) => Ok(true),
        Err(_) => match literal("false", reader) {
            Ok(_) => Ok(false),
            Err(_) => Err(Error {
                pos: start.pos,
                recoverable: true,
                inner: ParseError::Expecting {
                    value: String::from("true|false"),
                },
            }),
        },
    }
}

pub fn natural(reader: &mut Reader) -> ParseResult<'static, u64> {
    let start = reader.state.clone();

    if reader.is_eof() {
        return Err(Error {
            pos: start.pos,
            recoverable: true,
            inner: ParseError::Expecting {
                value: String::from("natural"),
            },
        });
    }
    let first_digit = reader.read().unwrap();
    if !first_digit.is_ascii_digit() {
        return Err(Error {
            pos: start.pos,
            recoverable: true,
            inner: ParseError::Expecting {
                value: String::from("natural"),
            },
        });
    }

    let save = reader.state.clone();
    let s = reader.read_while(|c| c.is_ascii_digit());

    // if the first digit is zero, you should not have any more digits
    if first_digit == '0' && !s.is_empty() {
        return Err(Error {
            pos: save.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: String::from("natural"),
            },
        });
    }
    Ok(format!("{}{}", first_digit, s).parse().unwrap())
}

pub fn integer(reader: &mut Reader) -> ParseResult<'static, i64> {
    let sign = match try_literal("-", reader) {
        Err(_) => 1,
        Ok(_) => -1,
    };
    let nat = natural(reader)?;
    Ok(sign * (nat as i64))
}

pub fn float(reader: &mut Reader) -> ParseResult<'static, Float> {
    // non recoverable after the dot
    // an integer is parsed ok as float => no like a computer language
    let start = reader.state.cursor;
    let sign = match try_literal("-", reader) {
        Err(_) => "",
        Ok(_) => "-",
    };
    let nat = natural(reader)?;
    try_literal(".", reader)?;

    if reader.is_eof() {
        return Err(Error {
            pos: reader.clone().state.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: String::from("natural"),
            },
        });
    }

    let s = reader.read_while(|c| c.is_ascii_digit());
    if s.is_empty() {
        return Err(Error {
            pos: reader.clone().state.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: String::from("natural"),
            },
        });
    }
    let value = format!("{}{}.{}", sign, nat, s).parse().unwrap();
    let encoded = reader.from(start);
    Ok(Float { value, encoded })
}

pub fn raw_string(reader: &mut Reader) -> ParseResult<'static, RawString> {
    // one value without newline or multiline mode
    // includes the last newline (consistent with bash EOL)
    try_literal("```", reader)?;
    let save = reader.state.clone();
    match newline(reader) {
        Ok(newline) => {
            let value = raw_string_value(reader)?;
            Ok(RawString { newline, value })
        }
        Err(_) => {
            reader.state = save;
            let newline = Whitespace {
                value: String::from(""),
                source_info: SourceInfo {
                    start: reader.state.clone().pos,
                    end: reader.state.clone().pos,
                },
            };
            let value = raw_string_value(reader)?;
            Ok(RawString { newline, value })
        }
    }
}

pub fn raw_string_value(reader: &mut Reader) -> ParseResult<'static, Template> {
    let mut chars = vec![];

    let start = reader.state.pos.clone();
    while !reader.remaining().starts_with("```") && !reader.is_eof() {
        let pos = reader.state.pos.clone();
        let c = reader.read().unwrap();
        chars.push((c, c.to_string(), pos));
    }
    let end = reader.state.pos.clone();
    literal("```", reader)?;

    let encoded_string = template::EncodedString {
        source_info: SourceInfo {
            start: start.clone(),
            end: end.clone(),
        },
        chars,
    };

    let elements = template::templatize(encoded_string)?;

    Ok(Template {
        quotes: false,
        elements,
        source_info: SourceInfo { start, end },
    })
}

pub(crate) fn file(reader: &mut Reader) -> ParseResult<'static, File> {
    let _start = reader.state.clone();
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

pub(crate) fn base64(reader: &mut Reader) -> ParseResult<'static, Base64> {
    // base64 => can have whitespace
    // support pqrser position
    let _start = reader.state.clone();
    try_literal("base64", reader)?;
    literal(",", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let save_state = reader.state.clone();
    let value = base64::parse(reader);
    let count = reader.state.cursor - save_state.cursor;
    reader.state = save_state;
    let encoded = reader.read_n(count);
    let space1 = zero_or_more_spaces(reader)?;
    literal(";", reader)?;
    Ok(Base64 {
        space0,
        value,
        encoded,
        space1,
    })
}

pub fn eof(reader: &mut Reader) -> ParseResult<'static, ()> {
    if reader.is_eof() {
        Ok(())
    } else {
        Err(Error {
            pos: reader.state.clone().pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: String::from("eof"),
            },
        })
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

pub fn hex_digit(reader: &mut Reader) -> ParseResult<'static, u32> {
    let start = reader.clone().state;
    match reader.read() {
        Some(c) => match hex_digit_value(c) {
            Some(v) => Ok(v),
            None => Err(Error {
                pos: start.pos,
                recoverable: true,
                inner: ParseError::HexDigit {},
            }),
        },
        None => Err(Error {
            pos: start.pos,
            recoverable: true,
            inner: ParseError::HexDigit {},
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Pos;

    use super::*;

    #[test]
    fn test_space() {
        let mut reader = Reader::init("x");
        let error = space(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init("  ");
        assert_eq!(
            space(&mut reader),
            Ok(Whitespace {
                value: " ".to_string(),
                source_info: SourceInfo::init(1, 1, 1, 2),
            }),
        );
        assert_eq!(reader.state.cursor, 1);
    }

    #[test]
    fn test_one_or_more_spaces() {
        let mut reader = Reader::init("  ");
        assert_eq!(
            one_or_more_spaces(&mut reader),
            Ok(Whitespace {
                value: "  ".to_string(),
                source_info: SourceInfo::init(1, 1, 1, 3),
            })
        );

        let mut reader = Reader::init("abc");
        let error = one_or_more_spaces(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
    }

    #[test]
    fn test_zero_or_more_spaces() {
        let mut reader = Reader::init("  ");
        assert_eq!(
            zero_or_more_spaces(&mut reader),
            Ok(Whitespace {
                value: "  ".to_string(),
                source_info: SourceInfo::init(1, 1, 1, 3),
            })
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("xxx");
        assert_eq!(
            zero_or_more_spaces(&mut reader),
            Ok(Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::init(1, 1, 1, 1),
            })
        );
        assert_eq!(reader.state.cursor, 0);

        let mut reader = Reader::init(" xxx");
        assert_eq!(
            zero_or_more_spaces(&mut reader),
            Ok(Whitespace {
                value: " ".to_string(),
                source_info: SourceInfo::init(1, 1, 1, 2),
            })
        );
        assert_eq!(reader.state.cursor, 1);
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

        let mut reader = Reader::init("#\n");
        assert_eq!(
            comment(&mut reader),
            Ok(Comment {
                value: "".to_string()
            })
        );

        let mut reader = Reader::init("# comment\n");
        assert_eq!(
            comment(&mut reader),
            Ok(Comment {
                value: " comment".to_string()
            })
        );
        assert_eq!(reader.state.cursor, 9);

        let mut reader = Reader::init("xxx");
        let error = comment(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);
    }

    #[test]
    fn test_literal() {
        let mut reader = Reader::init("hello");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("hello")
            }
        );
        assert_eq!(reader.state.cursor, 0);

        let mut reader = Reader::init("hi");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("hello")
            }
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("he");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("hello")
            }
        );
        assert_eq!(reader.state.cursor, 2);
    }

    #[test]
    fn test_new_line() {
        let mut reader = Reader::init("\n");
        assert_eq!(
            newline(&mut reader).unwrap(),
            Whitespace {
                value: String::from("\n"),
                source_info: SourceInfo::init(1, 1, 2, 1),
            }
        );
    }

    #[test]
    fn test_key_value() {
        let mut reader = Reader::init("message: hello {{name}}! # comment");
        assert_eq!(
            key_value(&mut reader).unwrap(),
            KeyValue {
                line_terminators: vec![],
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 1, 1, 1),
                },
                key: EncodedString {
                    quotes: false,
                    value: "message".to_string(),
                    encoded: "message".to_string(),
                    source_info: SourceInfo::init(1, 1, 1, 8),
                },
                space1: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 8, 1, 8),
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::init(1, 9, 1, 10),
                },
                value: Template {
                    quotes: false,
                    elements: vec![
                        TemplateElement::String {
                            value: "hello ".to_string(),
                            encoded: "hello ".to_string(),
                        },
                        TemplateElement::Expression(Expr {
                            space0: Whitespace {
                                value: "".to_string(),
                                source_info: SourceInfo::init(1, 18, 1, 18),
                            },
                            variable: Variable {
                                name: "name".to_string(),
                                source_info: SourceInfo::init(1, 18, 1, 22),
                            },
                            space1: Whitespace {
                                value: "".to_string(),
                                source_info: SourceInfo::init(1, 22, 1, 22),
                            },
                        }),
                        TemplateElement::String {
                            value: "!".to_string(),
                            encoded: "!".to_string(),
                        },
                    ],
                    source_info: SourceInfo::init(1, 10, 1, 25),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: " ".to_string(),
                        source_info: SourceInfo::init(1, 25, 1, 26),
                    },
                    comment: Some(Comment {
                        value: " comment".to_string()
                    }),
                    newline: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(1, 35, 1, 35),
                    },
                },
            }
        );
    }

    #[test]
    fn test_boolean() {
        let mut reader = Reader::init("true");
        assert!(boolean(&mut reader).unwrap());

        let mut reader = Reader::init("xxx");
        let error = boolean(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("true|false")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("trux");
        let error = boolean(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("true|false")
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_natural() {
        let mut reader = Reader::init("0");
        assert_eq!(natural(&mut reader).unwrap(), 0);
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init("0.");
        assert_eq!(natural(&mut reader).unwrap(), 0);
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init("10x");
        assert_eq!(natural(&mut reader).unwrap(), 10);
        assert_eq!(reader.state.cursor, 2);
    }

    #[test]
    fn test_natural_error() {
        let mut reader = Reader::init("");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("01");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::init("x");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    pub fn test_integer() {
        let mut reader = Reader::init("1");
        assert_eq!(integer(&mut reader).unwrap(), 1);

        let mut reader = Reader::init("1.1");
        assert_eq!(integer(&mut reader).unwrap(), 1);

        let mut reader = Reader::init("-1.1");
        assert_eq!(integer(&mut reader).unwrap(), -1);

        let mut reader = Reader::init("x");
        let error = integer(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_float() {
        let mut reader = Reader::init("1.0");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.0,
                encoded: "1.0".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("-1.0");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: -1.0,
                encoded: "-1.0".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("1.1");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.1,
                encoded: "1.1".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("1.100");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.1,
                encoded: "1.100".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("1.01");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.01,
                encoded: "1.01".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("1.010");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.01,
                encoded: "1.010".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 5);

        // provide more digits than necessary
        let mut reader = Reader::init("-0.3333333333333333333");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: -0.3333333333333333,
                encoded: "-0.3333333333333333333".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 22);
    }

    #[test]
    fn test_float_error() {
        let mut reader = Reader::init("");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::init("-");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::init("1");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from(".")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::init("1x");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from(".")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::init("1.");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert!(!error.recoverable);

        let mut reader = Reader::init("1.x");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert!(!error.recoverable);
    }

    #[test]
    fn test_raw_string_empty() {
        let mut reader = Reader::init("``````");
        assert_eq!(
            raw_string(&mut reader).unwrap(),
            RawString {
                newline: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 4, 1, 4),
                },
                value: Template {
                    quotes: false,
                    elements: vec![],
                    source_info: SourceInfo::init(1, 4, 1, 4),
                },
            }
        );

        let mut reader = Reader::init("```\n```");
        assert_eq!(
            raw_string(&mut reader).unwrap(),
            RawString {
                newline: Whitespace {
                    value: String::from("\n"),
                    source_info: SourceInfo::init(1, 4, 2, 1),
                },
                value: Template {
                    quotes: false,
                    elements: vec![],
                    source_info: SourceInfo::init(2, 1, 2, 1),
                },
            }
        );
        let mut reader = Reader::init("```\r\n```");
        assert_eq!(
            raw_string(&mut reader).unwrap(),
            RawString {
                newline: Whitespace {
                    value: String::from("\r\n"),
                    source_info: SourceInfo::init(1, 4, 2, 1),
                },
                value: Template {
                    quotes: false,
                    elements: vec![],
                    source_info: SourceInfo::init(2, 1, 2, 1),
                },
            }
        );
    }

    #[test]
    fn test_raw_string_hello() {
        let mut reader = Reader::init("```Hello World!```");
        assert_eq!(
            raw_string(&mut reader).unwrap(),
            RawString {
                newline: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 4, 1, 4),
                },
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "Hello World!".to_string(),
                        encoded: "Hello World!".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 4, 1, 16),
                },
            }
        );
        let mut reader = Reader::init("```Hello\nWorld!\n```");
        assert_eq!(
            raw_string(&mut reader).unwrap(),
            RawString {
                newline: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 4, 1, 4),
                },
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "Hello\nWorld!\n".to_string(),
                        encoded: "Hello\nWorld!\n".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 4, 3, 1),
                },
            }
        );
    }

    #[test]
    fn test_raw_string_csv() {
        let mut reader = Reader::init("```\nline1\nline2\nline3\n```");
        assert_eq!(
            raw_string(&mut reader).unwrap(),
            RawString {
                newline: Whitespace {
                    value: String::from("\n"),
                    source_info: SourceInfo::init(1, 4, 2, 1),
                },
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::init(2, 1, 5, 1),
                },
            }
        );
    }

    #[test]
    fn test_raw_string_one_emptyline() {
        // one newline
        // the value takes the value of the newline??
        let mut reader = Reader::init("```\n\n```");
        assert_eq!(
            raw_string(&mut reader).unwrap(),
            RawString {
                newline: Whitespace {
                    value: String::from("\n"),
                    source_info: SourceInfo::init(1, 4, 2, 1),
                },
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "\n".to_string(),
                        encoded: "\n".to_string(),
                    }],
                    source_info: SourceInfo::init(2, 1, 3, 1),
                },
            }
        );

        // one cr
        let mut reader = Reader::init("```\n\r\n````");
        assert_eq!(
            raw_string(&mut reader).unwrap(),
            RawString {
                newline: Whitespace {
                    value: String::from("\n"),
                    source_info: SourceInfo::init(1, 4, 2, 1),
                },
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "\r\n".to_string(),
                        encoded: "\r\n".to_string(),
                    }],
                    source_info: SourceInfo::init(2, 1, 3, 1),
                },
            }
        );
    }

    #[test]
    fn test_raw_string_error() {
        let mut reader = Reader::init("xxx");
        let error = raw_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("```")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("```\nxxx");
        let error = raw_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 2, column: 4 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("```")
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::init("```xxx");
        let error = raw_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("```")
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_raw_string_value() {
        let mut reader = Reader::init("```");
        assert_eq!(
            raw_string_value(&mut reader).unwrap(),
            Template {
                quotes: false,
                elements: vec![],
                source_info: SourceInfo::init(1, 1, 1, 1),
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("hello```");
        assert_eq!(
            raw_string_value(&mut reader).unwrap(),
            Template {
                quotes: false,
                elements: vec![TemplateElement::String {
                    value: "hello".to_string(),
                    encoded: "hello".to_string(),
                }],
                source_info: SourceInfo::init(1, 1, 1, 6),
            }
        );
        assert_eq!(reader.state.cursor, 8);
    }

    #[test]
    fn test_hex_digit() {
        let mut reader = Reader::init("0");
        assert_eq!(hex_digit(&mut reader).unwrap(), 0);

        let mut reader = Reader::init("x");
        let error = hex_digit(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::HexDigit {});
        assert!(error.recoverable);
    }

    #[test]
    fn test_hex() {
        let mut reader = Reader::init("hex, ff;");
        assert_eq!(
            hex(&mut reader).unwrap(),
            Hex {
                space0: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::init(1, 5, 1, 6),
                },
                value: vec![255],
                encoded: "ff".to_string(),
                space1: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 8, 1, 8),
                },
            }
        );

        let mut reader = Reader::init("hex,010203 ;");
        assert_eq!(
            hex(&mut reader).unwrap(),
            Hex {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 5, 1, 5),
                },
                value: vec![1, 2, 3],
                encoded: "010203".to_string(),
                space1: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::init(1, 11, 1, 12),
                },
            }
        );
    }

    #[test]
    fn test_hex_error() {
        let mut reader = Reader::init("hex,012;");
        let error = hex(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
        assert_eq!(error.inner, ParseError::OddNumberOfHexDigits {});
    }

    #[test]
    fn test_regex() {
        let mut reader = Reader::init(r#"/a{3}/"#);
        assert_eq!(
            regex(&mut reader).unwrap(),
            Regex {
                inner: regex::Regex::new(r#"a{3}"#).unwrap()
            }
        );

        let mut reader = Reader::init(r#"/a\/b/"#);
        assert_eq!(
            regex(&mut reader).unwrap(),
            Regex {
                inner: regex::Regex::new(r#"a/b"#).unwrap()
            }
        );

        let mut reader = Reader::init(r#"/a\.b/"#);
        assert_eq!(
            regex(&mut reader).unwrap(),
            Regex {
                inner: regex::Regex::new(r#"a\.b"#).unwrap()
            }
        );

        let mut reader = Reader::init(r#"/\d{4}-\d{2}-\d{2}/"#);
        assert_eq!(
            regex(&mut reader).unwrap(),
            Regex {
                inner: regex::Regex::new(r#"\d{4}-\d{2}-\d{2}"#).unwrap()
            }
        );
    }

    #[test]
    fn test_regex_error() {
        let mut reader = Reader::init("xxx");
        let error = regex(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::init("/xxx");
        let error = regex(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 5 });
        assert!(!error.recoverable);
        assert_eq!(error.inner, ParseError::Eof {});

        let mut reader = Reader::init("/x{a}/");
        let error = regex(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(!error.recoverable);
        assert_eq!(
            error.inner,
            ParseError::RegexExpr {
                message: "repetition quantifier expects a valid decimal".to_string()
            }
        );
    }

    #[test]
    fn test_file() {
        let mut reader = Reader::init("file,data.xml;");
        assert_eq!(
            file(&mut reader).unwrap(),
            File {
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 6, 1, 6),
                },
                filename: Filename {
                    value: String::from("data.xml"),
                    source_info: SourceInfo::init(1, 6, 1, 14),
                },
                space1: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 14, 1, 14),
                },
            }
        );

        let mut reader = Reader::init("file, filename1;");
        assert_eq!(
            file(&mut reader).unwrap(),
            File {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 6, 1, 7),
                },
                filename: Filename {
                    value: String::from("filename1"),
                    source_info: SourceInfo::init(1, 7, 1, 16),
                },
                space1: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 16, 1, 16),
                },
            }
        );

        let mut reader = Reader::init("file, tmp/filename1;");
        assert_eq!(
            file(&mut reader).unwrap(),
            File {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 6, 1, 7),
                },
                filename: Filename {
                    value: String::from("tmp/filename1"),
                    source_info: SourceInfo::init(1, 7, 1, 20),
                },
                space1: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 20, 1, 20),
                },
            }
        );

        let mut reader = Reader::init(r#"file, tmp/filename\ with\ spaces.txt;"#);
        assert_eq!(
            file(&mut reader).unwrap(),
            File {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::init(1, 6, 1, 7),
                },
                filename: Filename {
                    value: String::from("tmp/filename with spaces.txt"),
                    source_info: SourceInfo::init(1, 7, 1, 37),
                },
                space1: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 37, 1, 37),
                },
            }
        );
    }

    #[test]
    fn test_file_error() {
        let mut reader = Reader::init("fil; filename1;");
        let error = file(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::init("file, filename1");
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
            error.inner,
            ParseError::Expecting {
                value: String::from(";")
            }
        );

        let mut reader = Reader::init(r#"file, tmp/filename\ with\ unescaped .txt;"#);
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
            error.inner,
            ParseError::Expecting {
                value: String::from(";")
            }
        );
    }

    #[test]
    fn test_base64() {
        let mut reader = Reader::init("base64,  T WE=;xxx");
        assert_eq!(
            base64(&mut reader).unwrap(),
            Base64 {
                space0: Whitespace {
                    value: String::from("  "),
                    source_info: SourceInfo::init(1, 8, 1, 10),
                },
                value: vec![77, 97],
                encoded: String::from("T WE="),
                space1: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 15, 1, 15),
                },
            }
        );
        assert_eq!(reader.state.cursor, 15);
    }
}
