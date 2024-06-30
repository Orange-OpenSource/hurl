/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use crate::parser::error::*;
use crate::parser::primitives::try_literal;
use crate::parser::ParseResult;
use crate::reader::Reader;

pub fn natural(reader: &mut Reader) -> ParseResult<u64> {
    let start = reader.cursor;

    if reader.is_eof() {
        let kind = ParseErrorKind::Expecting {
            value: String::from("natural"),
        };
        return Err(ParseError::new(start.pos, true, kind));
    }
    let first_digit = reader.read().unwrap();
    if !first_digit.is_ascii_digit() {
        let kind = ParseErrorKind::Expecting {
            value: String::from("natural"),
        };
        return Err(ParseError::new(start.pos, true, kind));
    }

    let save = reader.cursor;
    let s = reader.read_while(|c| c.is_ascii_digit());

    // if the first digit is zero, you should not have any more digits
    if first_digit == '0' && !s.is_empty() {
        let kind = ParseErrorKind::Expecting {
            value: String::from("natural"),
        };
        return Err(ParseError::new(save.pos, false, kind));
    }
    match format!("{first_digit}{s}").parse() {
        Ok(value) => Ok(value),
        Err(_) => {
            let kind = ParseErrorKind::Expecting {
                value: String::from("natural"),
            };
            Err(ParseError::new(save.pos, false, kind))
        }
    }
}

pub fn integer(reader: &mut Reader) -> ParseResult<i64> {
    let sign = match try_literal("-", reader) {
        Err(_) => 1,
        Ok(_) => -1,
    };
    let nat = natural(reader)?;
    Ok(sign * (nat as i64))
}

pub fn number(reader: &mut Reader) -> ParseResult<Number> {
    let start = reader.cursor;
    let sign = match try_literal("-", reader) {
        Err(_) => "",
        Ok(_) => "-",
    };
    let integer_digits = reader.read_while(|c| c.is_ascii_digit());
    if integer_digits.is_empty() {
        let kind = ParseErrorKind::Expecting {
            value: "number".to_string(),
        };
        return Err(ParseError::new(reader.cursor.pos, true, kind));

        // if the first digit is zero, you should not have any more digits
    } else if integer_digits.len() > 1 && integer_digits.starts_with('0') {
        let save = reader.cursor;
        let kind = ParseErrorKind::Expecting {
            value: String::from("natural"),
        };
        return Err(ParseError::new(save.pos, false, kind));
    }

    // Float
    if try_literal(".", reader).is_ok() {
        let save = reader.cursor;
        let decimal_digits = reader.read_while(|c| c.is_ascii_digit());
        if decimal_digits.is_empty() {
            let kind = ParseErrorKind::Expecting {
                value: String::from("decimal digits"),
            };
            return Err(ParseError::new(save.pos, false, kind));
        }
        match format!("{sign}{integer_digits}.{decimal_digits}").parse() {
            Ok(value) => {
                let encoded = reader.peek_back(start.offset);
                Ok(Number::Float(Float { value, encoded }))
            }
            Err(_) => {
                let kind = ParseErrorKind::Expecting {
                    value: String::from("float"),
                };
                Err(ParseError::new(start.pos, false, kind))
            }
        }

    // Integer or BigInteger
    } else {
        match format!("{sign}{integer_digits}").parse() {
            Ok(value) => Ok(Number::Integer(value)),
            Err(_) => Ok(Number::BigInteger(integer_digits)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::Pos;

    #[test]
    fn test_natural() {
        let mut reader = Reader::new("0");
        assert_eq!(natural(&mut reader).unwrap(), 0);
        assert_eq!(reader.cursor.offset, 1);

        let mut reader = Reader::new("10x");
        assert_eq!(natural(&mut reader).unwrap(), 10);
        assert_eq!(reader.cursor.offset, 2);
    }

    #[test]
    fn test_natural_error() {
        let mut reader = Reader::new("");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("natural")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("01");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("natural")
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::new("x");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("natural")
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_integer() {
        let mut reader = Reader::new("0");
        assert_eq!(integer(&mut reader).unwrap(), 0);
        assert_eq!(reader.cursor.offset, 1);

        let mut reader = Reader::new("-1");
        assert_eq!(integer(&mut reader).unwrap(), -1);
        assert_eq!(reader.cursor.offset, 2);

        let mut reader = Reader::new("0");
        assert_eq!(number(&mut reader).unwrap(), Number::Integer(0));
        assert_eq!(reader.cursor.offset, 1);

        let mut reader = Reader::new("10x");
        assert_eq!(number(&mut reader).unwrap(), Number::Integer(10));
        assert_eq!(reader.cursor.offset, 2);

        let mut reader = Reader::new("-10x");
        assert_eq!(number(&mut reader).unwrap(), Number::Integer(-10));
        assert_eq!(reader.cursor.offset, 3);
    }

    #[test]
    fn test_float() {
        let mut reader = Reader::new("1.0");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: 1.0,
                encoded: "1.0".to_string()
            })
        );
        assert_eq!(reader.cursor.offset, 3);

        let mut reader = Reader::new("-1.0");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: -1.0,
                encoded: "-1.0".to_string()
            })
        );
        assert_eq!(reader.cursor.offset, 4);

        let mut reader = Reader::new("1.1");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: 1.1,
                encoded: "1.1".to_string()
            })
        );
        assert_eq!(reader.cursor.offset, 3);

        let mut reader = Reader::new("1.100");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: 1.1,
                encoded: "1.100".to_string()
            })
        );
        assert_eq!(reader.cursor.offset, 5);

        let mut reader = Reader::new("1.01");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: 1.01,
                encoded: "1.01".to_string()
            })
        );
        assert_eq!(reader.cursor.offset, 4);

        let mut reader = Reader::new("1.010");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: 1.01,
                encoded: "1.010".to_string()
            })
        );
        assert_eq!(reader.cursor.offset, 5);

        // provide more digits than necessary
        let mut reader = Reader::new("-0.3333333333333333333");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: -0.3333333333333333,
                encoded: "-0.3333333333333333333".to_string()
            })
        );
        assert_eq!(reader.cursor.offset, 22);

        let mut reader = Reader::new("1000000000000000000000.5");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: 1000000000000000000000.0,
                encoded: "1000000000000000000000.5".to_string()
            })
        );
        assert_eq!(reader.cursor.offset, 24);
    }

    #[test]
    pub fn test_number_error() {
        let mut reader = Reader::new("");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("number")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("-");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("number")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::new("x");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("number")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("1.");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("decimal digits")
            }
        );
        assert!(!error.recoverable);
    }
}
