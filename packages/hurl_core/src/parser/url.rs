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
use crate::ast::*;

use super::error::*;
use super::expr;
use super::primitives::*;
use super::reader::Reader;
use super::ParseResult;

pub fn url(reader: &mut Reader) -> ParseResult<'static, Template> {
    // can not be json-encoded
    // can not be empty
    // but more restrictive: whitelist characters, not empty

    let start = reader.state.clone();
    let mut elements = vec![];
    let mut buffer = String::from("");

    if reader.is_eof() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Url {},
        });
    }

    loop {
        let save = reader.state.clone();
        match line_terminator(reader) {
            Ok(_) => {
                reader.state = save;
                break;
            }
            _ => reader.state = save.clone(),
        }

        match expr::parse(reader) {
            Ok(value) => {
                if !buffer.is_empty() {
                    elements.push(TemplateElement::String {
                        value: buffer.clone(),
                        encoded: buffer.clone(),
                    });
                    buffer = String::from("");
                }
                elements.push(TemplateElement::Expression(value));
            }
            Err(e) => {
                if !e.recoverable {
                    return Err(e);
                } else {
                    reader.state = save.clone();
                    match reader.read() {
                        None => break,
                        Some(c) => {
                            if c.is_alphanumeric()
                                || vec![
                                    ':', '/', '.', '-', '?', '=', '&', '_', '%', '*', ',', '@', '~',
                                ]
                                .contains(&c)
                            {
                                buffer.push(c);
                            } else {
                                reader.state = save;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    if !buffer.is_empty() {
        elements.push(TemplateElement::String {
            value: buffer.clone(),
            encoded: buffer,
        });
    }

    if elements.is_empty() {
        reader.state = start.clone();
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Url {},
        });
    }

    // url should be followed by a line terminator
    let save = reader.state.clone();
    if line_terminator(reader).is_err() {
        reader.state = save;
        let c = reader.peek().unwrap();
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: false,
            inner: ParseError::UrlIllegalCharacter(c),
        });
    }

    reader.state = save;
    Ok(Template {
        quotes: false,
        elements,
        source_info: SourceInfo {
            start: start.pos,
            end: reader.state.clone().pos,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let mut reader = Reader::init("http://google.fr # ");
        assert_eq!(
            url(&mut reader).unwrap(),
            Template {
                elements: vec![TemplateElement::String {
                    value: String::from("http://google.fr"),
                    encoded: String::from("http://google.fr"),
                }],
                quotes: false,
                source_info: SourceInfo::init(1, 1, 1, 17),
            }
        );
        assert_eq!(reader.state.cursor, 16);
    }

    #[test]
    fn test_url2() {
        let mut reader = Reader::init("http://localhost:8000/cookies/set-session-cookie2-valueA");
        assert_eq!(
            url(&mut reader).unwrap(),
            Template {
                elements: vec![TemplateElement::String {
                    value: String::from("http://localhost:8000/cookies/set-session-cookie2-valueA"),
                    encoded: String::from(
                        "http://localhost:8000/cookies/set-session-cookie2-valueA"
                    ),
                }],
                quotes: false,
                source_info: SourceInfo::init(1, 1, 1, 57),
            }
        );
        assert_eq!(reader.state.cursor, 56);
    }

    #[test]
    fn test_url_with_expression() {
        let mut reader = Reader::init("http://{{host}}.fr ");
        assert_eq!(
            url(&mut reader).unwrap(),
            Template {
                elements: vec![
                    TemplateElement::String {
                        value: String::from("http://"),
                        encoded: String::from("http://"),
                    },
                    TemplateElement::Expression(Expr {
                        space0: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 10, 1, 10),
                        },
                        variable: Variable {
                            name: String::from("host"),
                            source_info: SourceInfo::init(1, 10, 1, 14),
                        },
                        space1: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 14, 1, 14),
                        },
                    }),
                    TemplateElement::String {
                        value: String::from(".fr"),
                        encoded: String::from(".fr"),
                    },
                ],
                //encoded: None,
                quotes: false,
                source_info: SourceInfo::init(1, 1, 1, 19),
            }
        );
        assert_eq!(reader.state.cursor, 18);
    }

    #[test]
    fn test_url_error_variable() {
        let mut reader = Reader::init("http://{{host>}}.fr");
        let error = url(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 14,
            }
        );
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("}}")
            }
        );
        assert!(!error.recoverable);
        assert_eq!(reader.state.cursor, 14);
    }

    #[test]
    fn test_url_error_missing_delimiter() {
        let mut reader = Reader::init("http://{{host");
        let error = url(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 14,
            }
        );
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("}}")
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_url_error_empty() {
        let mut reader = Reader::init(" # eol");
        let error = url(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::Url {});
    }
}
