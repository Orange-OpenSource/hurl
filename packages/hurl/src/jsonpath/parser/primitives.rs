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
use hurl_core::reader::Reader;

use crate::jsonpath::ast::Number;
use crate::jsonpath::parser::error::{ParseError, ParseErrorKind, ParseResult};

pub fn natural(reader: &mut Reader) -> ParseResult<usize> {
    let start = reader.cursor();

    if reader.is_eof() {
        let kind = ParseErrorKind::Expecting("natural".to_string());
        let error = ParseError::new(start.pos, true, kind);
        return Err(error);
    }
    let first_digit = reader.read().unwrap();
    if !first_digit.is_ascii_digit() {
        let kind = ParseErrorKind::Expecting("natural".to_string());
        let error = ParseError::new(start.pos, true, kind);
        return Err(error);
    }

    let save = reader.cursor();
    let s = reader.read_while(|c| c.is_ascii_digit());

    // if the first digit is zero, you should not have any more digits
    if first_digit == '0' && !s.is_empty() {
        let kind = ParseErrorKind::Expecting("natural".to_string());
        let error = ParseError::new(save.pos, false, kind);
        return Err(error);
    }
    Ok(format!("{first_digit}{s}").parse().unwrap())
}

pub fn integer(reader: &mut Reader) -> ParseResult<i64> {
    let sign = if reader.peek() == Some('-') {
        _ = reader.read();
        -1
    } else {
        1
    };
    let nat = natural(reader)?;
    Ok(sign * (nat as i64))
}

pub fn number(reader: &mut Reader) -> ParseResult<Number> {
    let int = integer(reader)?;

    let decimal = if reader.peek() == Some('.') {
        _ = reader.read();
        if reader.is_eof() {
            let kind = ParseErrorKind::Expecting("natural".to_string());
            let error = ParseError::new(reader.cursor().pos, false, kind);
            return Err(error);
        }

        let s = reader.read_while(|c| c.is_ascii_digit());
        if s.is_empty() {
            let kind = ParseErrorKind::Expecting("natural".to_string());
            let error = ParseError::new(reader.cursor().pos, false, kind);
            return Err(error);
        }
        format!("{s:0<18}").parse().unwrap()
    } else {
        0
    };
    whitespace(reader);
    Ok(Number { int, decimal })
}

pub fn boolean(reader: &mut Reader) -> ParseResult<bool> {
    let token = reader.read_while(|c| c.is_alphabetic());

    // Match the token against the strings "true" and "false"
    let result = match token.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => {
            let kind = ParseErrorKind::Expecting("bool".to_string());
            let error = ParseError::new(reader.cursor().pos, true, kind);
            Err(error)
        }
    };
    whitespace(reader);
    result
}

pub fn string_value(reader: &mut Reader) -> Result<String, ParseError> {
    try_literal("'", reader)?;
    let mut s = String::new();
    loop {
        match reader.read() {
            None => {
                let kind = ParseErrorKind::Expecting("'".to_string());
                let error = ParseError::new(reader.cursor().pos, false, kind);
                return Err(error);
            }
            Some('\'') => break,
            Some('\\') => {
                // only single quote can be escaped
                match reader.read() {
                    Some('\'') => {
                        s.push('\'');
                    }
                    _ => {
                        let kind = ParseErrorKind::Expecting("'".to_string());
                        let error = ParseError::new(reader.cursor().pos, false, kind);
                        return Err(error);
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

pub fn key_name(reader: &mut Reader) -> Result<String, ParseError> {
    // test python or javascript
    // subset that can used for dot notation
    // The key must not be empty and must not start with a digit

    let first_char = match reader.read() {
        Some(c) => {
            if c.is_alphabetic() || c == '_' {
                c
            } else {
                let kind = ParseErrorKind::Expecting("key".to_string());
                let error = ParseError::new(reader.cursor().pos, false, kind);
                return Err(error);
            }
        }
        None => {
            let kind = ParseErrorKind::Expecting("key".to_string());
            let error = ParseError::new(reader.cursor().pos, false, kind);
            return Err(error);
        }
    };
    let s = reader.read_while(|c| c.is_alphanumeric() || c == '_');
    whitespace(reader);
    Ok(format!("{first_char}{s}"))
}

// key1.key2.key3
pub fn key_path(reader: &mut Reader) -> Result<Vec<String>, ParseError> {
    let root = key_name(reader)?;
    let mut path = vec![root];
    while let Some('.') = reader.peek() {
        reader.read();
        let key = key_name(reader)?;
        path.push(key);
    }
    Ok(path)
}

pub fn literal(s: &str, reader: &mut Reader) -> ParseResult<()> {
    // does not return a value
    // non recoverable reader
    // => use combinator recover to make it recoverable
    let start = reader.cursor();
    if reader.is_eof() {
        let kind = ParseErrorKind::Expecting(s.to_string());
        let error = ParseError::new(start.pos, false, kind);
        return Err(error);
    }
    for c in s.chars() {
        match reader.read() {
            None => {
                let kind = ParseErrorKind::Expecting(s.to_string());
                let error = ParseError::new(start.pos, false, kind);
                return Err(error);
            }
            Some(x) => {
                if x != c {
                    let kind = ParseErrorKind::Expecting(s.to_string());
                    let error = ParseError::new(start.pos, false, kind);
                    return Err(error);
                } else {
                    continue;
                }
            }
        }
    }
    whitespace(reader);
    Ok(())
}

pub fn try_literal(s: &str, p: &mut Reader) -> ParseResult<()> {
    match literal(s, p) {
        Ok(_) => Ok(()),
        Err(ParseError { pos, kind, .. }) => Err(ParseError {
            pos,
            recoverable: true,
            kind,
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
    use hurl_core::reader::Pos;

    use super::*;

    #[test]
    fn test_natural() {
        let mut reader = Reader::new("0");
        assert_eq!(natural(&mut reader).unwrap(), 0);
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("0.");
        assert_eq!(natural(&mut reader).unwrap(), 0);
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("10x");
        assert_eq!(natural(&mut reader).unwrap(), 10);
        assert_eq!(reader.cursor().index, 2);
    }

    #[test]
    fn test_natural_error() {
        let mut reader = Reader::new("");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::Expecting("natural".to_string()));
        assert!(error.recoverable);

        let mut reader = Reader::new("01");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert_eq!(error.kind, ParseErrorKind::Expecting("natural".to_string()));
        assert!(!error.recoverable);

        let mut reader = Reader::new("x");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::Expecting("natural".to_string()));
        assert!(error.recoverable);
    }

    #[test]
    pub fn test_integer() {
        let mut reader = Reader::new("1");
        assert_eq!(integer(&mut reader).unwrap(), 1);

        let mut reader = Reader::new("1.1");
        assert_eq!(integer(&mut reader).unwrap(), 1);

        let mut reader = Reader::new("-1.1");
        assert_eq!(integer(&mut reader).unwrap(), -1);

        let mut reader = Reader::new("x");
        let error = integer(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::Expecting("natural".to_string()));
        assert!(error.recoverable);
    }

    #[test]
    fn test_number() {
        let mut reader = Reader::new("1");
        assert_eq!(number(&mut reader).unwrap(), Number { int: 1, decimal: 0 });
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new("1.0");
        assert_eq!(number(&mut reader).unwrap(), Number { int: 1, decimal: 0 });
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("-1.0");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: -1,
                decimal: 0
            }
        );
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("1.1");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 1,
                decimal: 100_000_000_000_000_000
            }
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("1.100");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 1,
                decimal: 100_000_000_000_000_000
            }
        );
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("1.01");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 1,
                decimal: 10_000_000_000_000_000
            }
        );
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("1.010");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 1,
                decimal: 10_000_000_000_000_000
            }
        );
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("-0.333333333333333333");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number {
                int: 0,
                decimal: 333_333_333_333_333_333
            }
        );
        assert_eq!(reader.cursor().index, 21);
    }

    #[test]
    fn test_number_error() {
        let mut reader = Reader::new("");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(error.kind, ParseErrorKind::Expecting("natural".to_string()));
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("-");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(error.kind, ParseErrorKind::Expecting("natural".to_string()));
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::new("1.");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(error.kind, ParseErrorKind::Expecting("natural".to_string()));
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert!(!error.recoverable);

        let mut reader = Reader::new("1.x");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(error.kind, ParseErrorKind::Expecting("natural".to_string()));
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert!(!error.recoverable);
    }

    #[test]
    fn test_string_value() {
        let mut reader = Reader::new("'hello'");
        assert_eq!(string_value(&mut reader).unwrap(), "hello".to_string());

        let mut reader = Reader::new("'\\''");
        assert_eq!(string_value(&mut reader).unwrap(), "'".to_string());

        let mut reader = Reader::new("1");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.kind, ParseErrorKind::Expecting("'".to_string()));
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("'hi");
        let error = string_value(&mut reader).err().unwrap();
        assert_eq!(error.kind, ParseErrorKind::Expecting("'".to_string()));
        assert_eq!(error.pos, Pos { line: 1, column: 4 });
        assert!(!error.recoverable);
    }

    #[test]
    fn test_key_name() {
        let mut reader = Reader::new("id'");
        assert_eq!(key_name(&mut reader).unwrap(), "id".to_string());

        let mut reader = Reader::new("id123");
        assert_eq!(key_name(&mut reader).unwrap(), "id123".to_string());

        let mut reader = Reader::new(".");
        let error = key_name(&mut reader).err().unwrap();
        assert!(!error.recoverable);
        assert_eq!(error.kind, ParseErrorKind::Expecting("key".to_string()));

        let mut reader = Reader::new("1id");
        let error = key_name(&mut reader).err().unwrap();
        assert!(!error.recoverable);
        assert_eq!(error.kind, ParseErrorKind::Expecting("key".to_string()));
    }

    #[test]
    fn test_key_path() {
        let mut reader = Reader::new("id");
        assert_eq!(key_path(&mut reader).unwrap(), vec!["id".to_string()]);

        let mut reader = Reader::new("key1.key2");
        assert_eq!(
            key_path(&mut reader).unwrap(),
            vec!["key1".to_string(), "key2".to_string()]
        );
    }

    #[test]
    fn test_literal() {
        let mut reader = Reader::new("hello");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("hello ");
        assert_eq!(literal("hello", &mut reader), Ok(()));
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::Expecting("hello".to_string()));
        assert_eq!(reader.cursor().index, 0);

        let mut reader = Reader::new("hi");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::Expecting("hello".to_string()));
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("he");
        let error = literal("hello", &mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::Expecting("hello".to_string()));
        assert_eq!(reader.cursor().index, 2);
    }
}
