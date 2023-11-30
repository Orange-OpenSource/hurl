/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use crate::parser::combinators::*;
use crate::parser::error::*;
use crate::parser::primitives::try_literal;
use crate::parser::reader::Reader;

use crate::parser::ParseResult;

pub fn natural(reader: &mut Reader) -> ParseResult<u64> {
    let start = reader.state;

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

    let save = reader.state;
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
    match format!("{first_digit}{s}").parse() {
        Ok(value) => Ok(value),
        Err(_) => Err(Error {
            pos: save.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: String::from("natural"),
            },
        }),
    }
}

pub fn number(reader: &mut Reader) -> ParseResult<Number> {
    choice(
        &[
            |p1| match float(p1) {
                Ok(value) => Ok(Number::Float(value)),
                Err(e) => Err(e.recoverable()),
            },
            |p1| match integer(p1) {
                Ok(value) => Ok(Number::Integer(value)),
                Err(e) => Err(e.recoverable()),
            },
            |p1| match string_number(p1) {
                Ok(value) => Ok(Number::String(value)),
                Err(e) => Err(e),
            },
        ],
        reader,
    )
    .map_err(|e| Error {
        pos: e.pos,
        recoverable: true,
        inner: if e.recoverable {
            ParseError::Expecting {
                value: "number".to_string(),
            }
        } else {
            e.inner
        },
    })
}

pub fn integer(reader: &mut Reader) -> ParseResult<i64> {
    let sign = match try_literal("-", reader) {
        Err(_) => 1,
        Ok(_) => -1,
    };
    let nat = natural(reader)?;
    Ok(sign * (nat as i64))
}

pub fn float(reader: &mut Reader) -> ParseResult<Float> {
    // non recoverable after the dot
    // an integer is parsed ok as float => no like a computer language
    let start = reader.state;
    let sign = match try_literal("-", reader) {
        Err(_) => "",
        Ok(_) => "-",
    };
    let nat = natural(reader)?;
    try_literal(".", reader)?;

    if reader.is_eof() {
        return Err(Error {
            pos: reader.state.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: String::from("natural"),
            },
        });
    }

    let s = reader.read_while(|c| c.is_ascii_digit());
    if s.is_empty() {
        return Err(Error {
            pos: reader.state.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: String::from("natural"),
            },
        });
    }
    match format!("{sign}{nat}.{s}").parse() {
        Ok(value) => {
            let encoded = reader.peek_back(start.cursor);
            Ok(Float { value, encoded })
        }
        Err(_) => Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: String::from("float"),
            },
        }),
    }
}

pub fn string_number(reader: &mut Reader) -> ParseResult<String> {
    let sign = match try_literal("-", reader) {
        Err(_) => "",
        Ok(_) => "-",
    };
    let integer = reader.read_while(|c| c.is_ascii_digit());
    if integer.is_empty() {
        return Err(Error {
            pos: reader.state.pos,
            recoverable: true,
            inner: ParseError::Expecting {
                value: "number".to_string(),
            },
        });
    }
    let decimal = if try_literal(".", reader).is_ok() {
        let s = reader.read_while(|c| c.is_ascii_digit());
        if s.is_empty() {
            return Err(Error {
                pos: reader.state.pos,
                recoverable: false,
                inner: ParseError::Expecting {
                    value: "decimals".to_string(),
                },
            });
        }
        format!(".{s}")
    } else {
        String::new()
    };
    Ok(format!("{sign}{integer}{decimal}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;

    #[test]
    fn test_natural() {
        let mut reader = Reader::new("0");
        assert_eq!(natural(&mut reader).unwrap(), 0);
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::new("0.");
        assert_eq!(natural(&mut reader).unwrap(), 0);
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::new("10x");
        assert_eq!(natural(&mut reader).unwrap(), 10);
        assert_eq!(reader.state.cursor, 2);
    }

    #[test]
    fn test_natural_error() {
        let mut reader = Reader::new("");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("01");
        let error = natural(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::new("x");
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
    pub fn test_number() {
        let mut reader = Reader::new("1");
        assert_eq!(number(&mut reader).unwrap(), Number::Integer(1));

        let mut reader = Reader::new("1.1");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::Float(Float {
                value: 1.1,
                encoded: "1.1".to_string()
            })
        );

        let mut reader = Reader::new("1000000000000000000000");
        assert_eq!(
            number(&mut reader).unwrap(),
            Number::String("1000000000000000000000".to_string())
        );
    }

    #[test]
    pub fn test_number_error() {
        let mut reader = Reader::new("x");
        let error = number(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("number")
            }
        );
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
        let mut reader = Reader::new("1.0");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.0,
                encoded: "1.0".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::new("-1.0");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: -1.0,
                encoded: "-1.0".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::new("1.1");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.1,
                encoded: "1.1".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::new("1.100");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.1,
                encoded: "1.100".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::new("1.01");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.01,
                encoded: "1.01".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::new("1.010");
        assert_eq!(
            float(&mut reader).unwrap(),
            Float {
                value: 1.01,
                encoded: "1.010".to_string()
            }
        );
        assert_eq!(reader.state.cursor, 5);

        // provide more digits than necessary
        let mut reader = Reader::new("-0.3333333333333333333");
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
        let mut reader = Reader::new("");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);

        let mut reader = Reader::new("-");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::new("1");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from(".")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::new("1x");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from(".")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(error.recoverable);

        let mut reader = Reader::new("1.");
        let error = float(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("natural")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert!(!error.recoverable);

        let mut reader = Reader::new("1.x");
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
    pub fn test_string_number() {
        let mut reader = Reader::new("1");
        assert_eq!(string_number(&mut reader).unwrap(), "1");

        let mut reader = Reader::new("1000000000000000000000");
        assert_eq!(
            string_number(&mut reader).unwrap(),
            "1000000000000000000000"
        );
    }

    #[test]
    fn test_string_number_error() {
        let mut reader = Reader::new("1.x");
        let error = string_number(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("decimals")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert!(!error.recoverable);

        let mut reader = Reader::new("{{");
        let error = string_number(&mut reader).err().unwrap();
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("number")
            }
        );
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert!(error.recoverable);
    }
}
