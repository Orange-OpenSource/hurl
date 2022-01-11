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
use sxd_document::parser;

use crate::ast::Pos;

use super::error::*;
use super::reader::Reader;
use super::ParseResult;

pub fn parse(reader: &mut Reader) -> ParseResult<'static, String> {
    let mut buf = String::from("");
    let start = reader.state.clone();
    match reader.read() {
        Some('<') => buf.push('<'),
        _ => {
            return Err(Error {
                pos: Pos { line: 1, column: 1 },
                recoverable: true,
                inner: ParseError::Xml {},
            })
        }
    }

    loop {
        match reader.read() {
            None => {
                break;
            }
            Some(c) => {
                buf.push(c);
                if c == '>' && is_valid(buf.as_str()) {
                    return Ok(buf);
                }
            }
        }
    }
    Err(Error {
        pos: start.pos,
        recoverable: false,
        inner: ParseError::Xml {},
    })
}

fn is_valid(s: &str) -> bool {
    matches!(parser::parse(s), Ok(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_xml_brute_force_errors() {
        let mut reader = Reader::init("");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::Xml {});
        assert!(error.recoverable);

        let mut reader = Reader::init("x");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::Xml {});
        assert!(error.recoverable);

        let mut reader = Reader::init("<<");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::Xml {});
        assert!(!error.recoverable);

        let mut reader = Reader::init("<users><user /></users");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::Xml {});

        let mut reader = Reader::init("<users aa><user /></users");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::Xml {});
    }

    #[test]
    fn test_parsing_xml_brute_force() {
        let mut reader = Reader::init("<users><user /></users>");
        assert_eq!(
            parse(&mut reader).unwrap(),
            String::from("<users><user /></users>")
        );
        assert_eq!(reader.state.cursor, 23);

        let mut reader = Reader::init("<users><user /></users>xx");
        assert_eq!(
            parse(&mut reader).unwrap(),
            String::from("<users><user /></users>")
        );
        assert_eq!(reader.state.cursor, 23);
        assert_eq!(reader.remaining(), String::from("xx"));

        let mut reader = Reader::init("<?xml version=\"1.0\"?><users/>xxx");
        assert_eq!(
            parse(&mut reader).unwrap(),
            String::from("<?xml version=\"1.0\"?><users/>")
        );
        assert_eq!(reader.state.cursor, 29);
    }
}
