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

use super::bytes::*;
use super::combinators::*;
use super::error::*;
use super::primitives::*;
use super::reader::Reader;
use super::sections::*;
use super::url::url;
use super::ParseResult;

pub fn hurl_file(reader: &mut Reader) -> ParseResult<'static, HurlFile> {
    let entries = zero_or_more(entry, reader)?;
    let line_terminators = optional_line_terminators(reader)?;
    eof(reader)?;
    Ok(HurlFile {
        entries,
        line_terminators,
    })
}

fn entry(reader: &mut Reader) -> ParseResult<'static, Entry> {
    let req = request(reader)?;
    let resp = optional(response, reader)?;
    Ok(Entry {
        request: req,
        response: resp,
    })
}

fn request(reader: &mut Reader) -> ParseResult<'static, Request> {
    let start = reader.state.clone();
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let m = method(reader)?;
    let space1 = one_or_more_spaces(reader)?;
    let u = url(reader)?;

    let line_terminator0 = line_terminator(reader)?;
    let headers = zero_or_more(key_value, reader)?;
    let sections = request_sections(reader)?;
    let b = optional(body, reader)?;
    let source_info = SourceInfo::init(
        start.pos.line,
        start.pos.column,
        reader.state.pos.line,
        reader.state.pos.column,
    );

    // check duplicated section
    let mut section_names = vec![];
    for section in sections.clone() {
        if section_names.contains(&section.name().to_string()) {
            return Err(Error {
                pos: section.source_info.start,
                recoverable: false,
                inner: ParseError::DuplicateSection,
            });
        } else {
            section_names.push(section.name().to_string());
        }
    }

    Ok(Request {
        line_terminators,
        space0,
        method: m,
        space1,
        url: u,
        line_terminator0,
        headers,
        sections,
        body: b,
        source_info,
    })
}

fn response(reader: &mut Reader) -> ParseResult<'static, Response> {
    let start = reader.state.clone();
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let _version = version(reader)?;
    let space1 = one_or_more_spaces(reader)?;
    let _status = status(reader)?;
    let line_terminator0 = line_terminator(reader)?;
    let headers = zero_or_more(key_value, reader)?;
    let sections = response_sections(reader)?;
    let b = optional(body, reader)?;
    Ok(Response {
        line_terminators,
        space0,
        version: _version,
        space1,
        status: _status,
        line_terminator0,
        headers,
        sections,
        body: b,
        source_info: SourceInfo::init(
            start.pos.line,
            start.pos.column,
            reader.state.pos.line,
            reader.state.pos.column,
        ),
    })
}

fn method(reader: &mut Reader) -> ParseResult<'static, Method> {
    if reader.is_eof() {
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: true,
            inner: ParseError::Method {
                name: "<EOF>".to_string(),
            },
        });
    }
    let start = reader.state.clone();
    let name = reader.read_while(|c| c.is_alphanumeric());
    let available_methods = vec![
        ("GET", Method::Get),
        ("HEAD", Method::Head),
        ("POST", Method::Post),
        ("PUT", Method::Put),
        ("DELETE", Method::Delete),
        ("CONNECT", Method::Connect),
        ("OPTIONS", Method::Options),
        ("TRACE", Method::Trace),
        ("PATCH", Method::Patch),
    ];

    for (s, method) in available_methods {
        if name == s {
            return Ok(method);
        }
    }
    reader.state = start.clone();
    Err(Error {
        pos: start.pos,
        recoverable: false,
        inner: ParseError::Method { name },
    })
}

fn version(reader: &mut Reader) -> ParseResult<'static, Version> {
    try_literal("HTTP/", reader)?;
    let available_version = vec![
        ("1.0", VersionValue::Version1),
        ("1.1", VersionValue::Version11),
        ("2", VersionValue::Version2),
        ("*", VersionValue::VersionAny),
    ];
    let start = reader.state.clone();
    for (s, value) in available_version {
        if try_literal(s, reader).is_ok() {
            return Ok(Version {
                value,
                source_info: SourceInfo::init(
                    start.pos.line,
                    start.pos.column,
                    reader.state.pos.line,
                    reader.state.pos.column,
                ),
            });
        }
    }
    Err(Error {
        pos: start.pos,
        recoverable: false,
        inner: ParseError::Version {},
    })
}

fn status(reader: &mut Reader) -> ParseResult<'static, Status> {
    let start = reader.state.pos.clone();
    let value = match try_literal("*", reader) {
        Ok(_) => StatusValue::Any,
        Err(_) => match natural(reader) {
            Ok(value) => StatusValue::Specific(value),
            Err(_) => {
                return Err(Error {
                    pos: start,
                    recoverable: false,
                    inner: ParseError::Status {},
                })
            }
        },
    };
    let end = reader.state.pos.clone();
    Ok(Status {
        value,
        source_info: SourceInfo { start, end },
    })
}

fn body(reader: &mut Reader) -> ParseResult<'static, Body> {
    //  let start = reader.state.clone();
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let value = bytes(reader)?;
    let line_terminator0 = line_terminator(reader)?;
    Ok(Body {
        line_terminators,
        space0,
        value,
        line_terminator0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hurl_file() {
        let mut reader = Reader::init("GET http://google.fr");
        let hurl_file = hurl_file(&mut reader).unwrap();
        assert_eq!(hurl_file.entries.len(), 1);
    }

    #[test]
    fn test_entry() {
        let mut reader = Reader::init("GET http://google.fr");
        //println!("{:?}", entry(&mut reader));
        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method::Get);
        assert_eq!(reader.state.cursor, 20);
    }

    #[test]
    fn test_several_entry() {
        let mut reader = Reader::init("GET http://google.fr\nGET http://google.fr");

        let e = entry(&mut reader).unwrap();
        //println!("{:?}", e);
        assert_eq!(e.request.method, Method::Get);
        assert_eq!(reader.state.cursor, 21);
        assert_eq!(reader.state.pos.line, 2);

        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method::Get);
        assert_eq!(reader.state.cursor, 41);
        assert_eq!(reader.state.pos.line, 2);

        let mut reader =
            Reader::init("GET http://google.fr # comment1\nGET http://google.fr # comment2");

        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method::Get);
        assert_eq!(reader.state.cursor, 32);
        assert_eq!(reader.state.pos.line, 2);

        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method::Get);
        assert_eq!(reader.state.cursor, 63);
        assert_eq!(reader.state.pos.line, 2);
    }

    #[test]
    fn test_entry_with_response() {
        let mut reader = Reader::init("GET http://google.fr\nHTTP/1.1 200");
        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method::Get);
        assert_eq!(e.response.unwrap().status.value, StatusValue::Specific(200));
    }

    #[test]
    fn test_request() {
        let mut reader = Reader::init("GET http://google.fr");
        let default_request = Request {
            line_terminators: vec![],
            space0: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::init(1, 1, 1, 1),
            },
            method: Method::Get,
            space1: Whitespace {
                value: " ".to_string(),
                source_info: SourceInfo::init(1, 4, 1, 5),
            },
            url: Template {
                elements: vec![TemplateElement::String {
                    value: String::from("http://google.fr"),
                    encoded: String::from("http://google.fr"),
                }],
                quotes: false,
                source_info: SourceInfo::init(1, 5, 1, 21),
            },
            line_terminator0: LineTerminator {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 21, 1, 21),
                },
                comment: None,
                newline: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 21, 1, 21),
                },
            },
            headers: vec![],
            sections: vec![],
            body: None,
            source_info: SourceInfo::init(1, 1, 1, 21),
        };
        assert_eq!(request(&mut reader), Ok(default_request));

        let mut reader = Reader::init("GET  http://google.fr # comment");
        let default_request = Request {
            line_terminators: vec![],
            space0: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::init(1, 1, 1, 1),
            },
            method: Method::Get,
            space1: Whitespace {
                value: "  ".to_string(),
                source_info: SourceInfo::init(1, 4, 1, 6),
            },
            url: Template {
                elements: vec![TemplateElement::String {
                    value: String::from("http://google.fr"),
                    encoded: String::from("http://google.fr"),
                }],
                quotes: false,
                source_info: SourceInfo::init(1, 6, 1, 22),
            },
            line_terminator0: LineTerminator {
                space0: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::init(1, 22, 1, 23),
                },
                comment: Some(Comment {
                    value: " comment".to_string(),
                }),
                newline: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 32, 1, 32),
                },
            },
            headers: vec![],
            sections: vec![],
            body: None,
            source_info: SourceInfo::init(1, 1, 1, 32),
        };
        assert_eq!(request(&mut reader), Ok(default_request));

        let mut reader = Reader::init("GET http://google.fr\nGET http://google.fr");
        let r = request(&mut reader);
        assert_eq!(r.unwrap().method, Method::Get);
        assert_eq!(reader.state.cursor, 21);
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method::Get);
    }

    #[test]
    fn test_request_multilines() {
        // GET http://google.fr
        // ```
        // Hello World!
        // ```
        let mut reader = Reader::init("GET http://google.fr\n```\nHello World!\n```");
        let req = request(&mut reader).unwrap();
        assert_eq!(
            req.body.unwrap(),
            Body {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(2, 1, 2, 1),
                },
                value: Bytes::RawString(RawString {
                    newline: Whitespace {
                        value: "\n".to_string(),
                        source_info: SourceInfo::init(2, 4, 3, 1),
                    },
                    value: Template {
                        elements: vec![TemplateElement::String {
                            value: String::from("Hello World!\n"),
                            encoded: String::from("Hello World!\n"),
                        }],
                        quotes: false,
                        source_info: SourceInfo::init(3, 1, 4, 1),
                    },
                }),
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(4, 4, 4, 4),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::init(4, 4, 4, 4),
                    },
                },
            }
        );
    }

    #[test]
    fn test_request_post_json() {
        let mut reader = Reader::init("POST http://localhost:8000/post-json-array\n[1,2,3]");
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method::Post);
        assert_eq!(
            r.body.unwrap().value,
            Bytes::Json {
                value: JsonValue::List {
                    space0: "".to_string(),
                    elements: vec![
                        JsonListElement {
                            space0: "".to_string(),
                            value: JsonValue::Number("1".to_string()),
                            space1: "".to_string(),
                        },
                        JsonListElement {
                            space0: "".to_string(),
                            value: JsonValue::Number("2".to_string()),
                            space1: "".to_string(),
                        },
                        JsonListElement {
                            space0: "".to_string(),
                            value: JsonValue::Number("3".to_string()),
                            space1: "".to_string(),
                        },
                    ],
                }
            }
        );

        let mut reader = Reader::init("POST http://localhost:8000/post-json-string\n\"Hello\"");
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method::Post);
        assert_eq!(
            r.body.unwrap().value,
            Bytes::Json {
                value: JsonValue::String(Template {
                    quotes: true,
                    elements: vec![TemplateElement::String {
                        value: "Hello".to_string(),
                        encoded: "Hello".to_string(),
                    }],
                    source_info: SourceInfo::init(2, 2, 2, 7),
                })
            }
        );

        let mut reader = Reader::init("POST http://localhost:8000/post-json-number\n100");
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method::Post);
        assert_eq!(
            r.body.unwrap().value,
            Bytes::Json {
                value: JsonValue::Number("100".to_string())
            }
        );
    }

    #[test]
    fn test_request_error() {
        let mut reader = Reader::init("xxx");
        let error = request(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
    }

    #[test]
    fn test_response() {
        let mut reader = Reader::init("HTTP/1.1 200");
        //println!("{:?}", response(&mut reader));
        let r = response(&mut reader).unwrap();

        assert_eq!(r.version.value, VersionValue::Version11);
        assert_eq!(r.status.value, StatusValue::Specific(200));
    }

    #[test]
    fn test_method() {
        let mut reader = Reader::init("xxx ");
        let error = method(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(reader.state.cursor, 0);

        let mut reader = Reader::init("");
        let error = method(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(reader.state.cursor, 0);

        let mut reader = Reader::init("GET ");
        assert_eq!(Ok(Method::Get), method(&mut reader));
        assert_eq!(reader.state.cursor, 3);
    }

    #[test]
    fn test_version() {
        let mut reader = Reader::init("HTTP/1.1 200");
        assert_eq!(version(&mut reader).unwrap().value, VersionValue::Version11);

        let mut reader = Reader::init("HTTP/1. 200");
        let error = version(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 6 });
    }

    #[test]
    fn test_status() {
        let mut reader = Reader::init("*");
        let s = status(&mut reader).unwrap();
        assert_eq!(s.value, StatusValue::Any);

        let mut reader = Reader::init("200");
        let s = status(&mut reader).unwrap();
        assert_eq!(s.value, StatusValue::Specific(200));

        let mut reader = Reader::init("xxx");
        let result = status(&mut reader);
        assert!(result.is_err());
        // assert!(result.err().unwrap().pos, Pos { line: 1, column: 1 });
    }

    #[test]
    fn test_body_json() {
        let mut reader = Reader::init("[1,2,3] ");
        let b = body(&mut reader).unwrap();
        assert_eq!(b.line_terminators.len(), 0);
        assert_eq!(
            b.value,
            Bytes::Json {
                value: JsonValue::List {
                    space0: "".to_string(),
                    elements: vec![
                        JsonListElement {
                            space0: "".to_string(),
                            value: JsonValue::Number("1".to_string()),
                            space1: "".to_string(),
                        },
                        JsonListElement {
                            space0: "".to_string(),
                            value: JsonValue::Number("2".to_string()),
                            space1: "".to_string(),
                        },
                        JsonListElement {
                            space0: "".to_string(),
                            value: JsonValue::Number("3".to_string()),
                            space1: "".to_string(),
                        },
                    ],
                }
            }
        );
        assert_eq!(reader.state.cursor, 8);

        let mut reader = Reader::init("{}");
        let b = body(&mut reader).unwrap();
        assert_eq!(b.line_terminators.len(), 0);
        assert_eq!(
            b.value,
            Bytes::Json {
                value: JsonValue::Object {
                    space0: "".to_string(),
                    elements: vec![],
                }
            }
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("# comment\n {} # comment\nxxx");
        let b = body(&mut reader).unwrap();
        assert_eq!(b.line_terminators.len(), 1);
        assert_eq!(
            b.value,
            Bytes::Json {
                value: JsonValue::Object {
                    space0: "".to_string(),
                    elements: vec![],
                }
            }
        );
        assert_eq!(reader.state.cursor, 24);

        let mut reader = Reader::init("{x");
        let error = body(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(!error.recoverable);
    }
}
