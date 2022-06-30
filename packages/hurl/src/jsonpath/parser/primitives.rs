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
use super::super::ast::*;
use super::error::{Error, ParseError};
use super::ParseResult;
use super::Reader;

pub fn natural(reader: &mut Reader) -> ParseResult<'static, usize> {
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
    let sign = if reader.try_literal("-") { -1 } else { 1 };
    let nat = natural(reader)?;
    Ok(sign * (nat as i64))
}

pub fn number(reader: &mut Reader) -> ParseResult<'static, Number> {
    let int = integer(reader)?;

    let decimal = if reader.try_literal(".") {
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
        format!("{:0<18}", s).parse().unwrap()
    } else {
        0
    };
    whitespace(reader);
    Ok(Number { int, decimal })
}

pub fn string_value(reader: &mut Reader) -> Result<String, Error> {
    try_literal("'", reader)?;
    let mut s = "".to_string();
    loop {
        match reader.read() {
            None => {
                return Err(Error {
                    pos: reader.state.pos.clone(),
                    recoverable: false,
                    inner: ParseError::Expecting {
                        value: String::from("'"),
                    },
                })
            }
            Some('\'') => break,
            Some('\\') => {
                // only single quote can be escaped
                match reader.read() {
                    Some('\'') => {
                        s.push('\'');
                    }
                    _ => {
                        return Err(Error {
                            pos: reader.state.pos.clone(),
                            recoverable: false,
                            inner: ParseError::Expecting {
                                value: String::from("'"),
                            },
                        })
                    }
                }
            }
            Some(c) => {
                s.push(c);
            }
        }
    }
    whitespace(reader);
    Ok(s)
}

pub fn key_name(reader: &mut Reader) -> Result<String, Error> {
    // test python or javascript
    // subset that can used for dot notation
    // The key must not be empty and must not start with a digit

    let first_char = match reader.read() {
        Some(c) => {
            if c.is_alphabetic() || c == '_' {
                c
            } else {
                return Err(Error {
                    pos: reader.state.pos.clone(),
                    recoverable: false,
                    inner: ParseError::Expecting {
                        value: "key".to_string(),
                    },
                });
            }
        }
        None => {
            return Err(Error {
                pos: reader.state.pos.clone(),
                recoverable: false,
                inner: ParseError::Expecting {
                    value: "key".to_string(),
                },
            })
        }
    };
    let s = reader.read_while(|c| c.is_alphanumeric() || *c == '_');
    whitespace(reader);
    Ok(format!("{}{}", first_char, s))
}

// key1.key2.key3
pub fn key_path(reader: &mut Reader) -> Result<Vec<String>, Error> {
    let root = key_name(reader)?;
    let mut path = vec![root];
    while let Some('.') = reader.peek() {
        reader.read();
        let key = key_name(reader)?;
        path.push(key);
    }
    Ok(path)
}

pub fn literal(s: &str, reader: &mut Reader) -> ParseResult<'static, ()> {
    // does not return a value
    // non recoverable reader
    // => use combinator recover to make it recoverable
    let start = reader.state.clone();
    if reader.clone().is_eof() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: s.to_string(),
            },
        });
    }
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
    whitespace(reader);
    Ok(())
}

pub fn try_literal(s: &str, p: &mut Reader) -> ParseResult<'static, ()> {
    match literal(s, p) {
        Ok(_) => Ok(()),
        Err(Error { pos, inner, .. }) => Err(Error {
            pos,
            recoverable: true,
            inner,
        }),
    }
}

pub fn whitespace(reader: &mut Reader) {
    while reader.peek() == Some(' ') {
        reader.read();
    }
}

#[cfg(test)]
mod tests {
    use super::super::Pos;
    use super::*;

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
    fn test_number() {
        let mut reader = Reader::init("1");
        assert_eq!(number(&mut reader).unwrap(), Number { int: 1, decimal: 0 });
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init("1.0");
        assert_eq!(number(&mut reader).unwrap(), Number { int: 1, decimal: 0 });
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("-1.0");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: -1,
                decimal: 0
            }
        );
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("1.1");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 1,
                decimal: 100_000_000_000_000_000
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("1.100");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 1,
                decimal: 100_000_000_000_000_000
            }
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("1.01");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 1,
                decimal: 10_000_000_000_000_000
            }
        );
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("1.010");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 1,
                decimal: 10_000_000_000_000_000
            }
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("-0.333333333333333333");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 0,
                decimal: 333_333_333_333_333_333
            }
        );
        assert_eq!(reader.state.cursor, 21);
    }

    #[test]
    fn test_number_error() {
        let mut reader = Reader::init("");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::init("-");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::init("1.");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert!(!error.recoverable);

        let mut reader = Reader::init("1.x");
        let error = number(&mut reader).err().unwrap();
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
    fn test_string_value() {
        let mut reader = Reader::init("'hello'");
        assert_eq!(string_value(&mut reader).unwrap(), "hello".to_string());

        let mut reader = Reader::init("'\\''");
        assert_eq!(string_value(&mut reader).unwrap(), "'".to_string());

        let mut reader = Reader::init("1");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("'")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::init("'hi");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("'")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 4 });
        assert!(!error.recoverable);
    }

    #[test]
    fn test_key_name() {
        let mut reader = Reader::init("id'");
        assert_eq!(key_name(&mut reader).unwrap(), "id".to_string());

        let mut reader = Reader::init("id123");
        assert_eq!(key_name(&mut reader).unwrap(), "id123".to_string());

        let mut reader = Reader::init(".");
        let error = key_name(&mut reader).err().unwrap();
        assert!(!error.recoverable);
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "key".to_string()
            }
        );

        let mut reader = Reader::init("1id");
        let error = key_name(&mut reader).err().unwrap();
        assert!(!error.recoverable);
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "key".to_string()
            }
        );
    }

    #[test]
    fn test_key_path() {
        let mut reader = Reader::init("id");
        assert_eq!(key_path(&mut reader).unwrap(), vec!["id".to_string()]);

        let mut reader = Reader::init("key1.key2");
        assert_eq!(
            key_path(&mut reader).unwrap(),
            vec!["key1".to_string(), "key2".to_string()]
        );
    }

    #[test]
    fn test_literal() {
        let mut reader = Reader::init("hello");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("hello ");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.state.cursor, 6);

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
}
