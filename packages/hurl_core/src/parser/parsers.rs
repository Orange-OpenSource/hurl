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
use crate::ast::VersionValue::VersionAny;
use crate::ast::{
    Body, Entry, HurlFile, Method, Request, Response, SourceInfo, Status, StatusValue, Version,
    VersionValue,
};
use crate::combinator::{optional, zero_or_more};
use crate::parser::bytes::bytes;
use crate::parser::primitives::{
    eof, key_value, line_terminator, one_or_more_spaces, optional_line_terminators, try_literal,
    zero_or_more_spaces,
};
use crate::parser::sections::{request_sections, response_sections};
use crate::parser::string::unquoted_template;
use crate::parser::{ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

pub fn hurl_file(reader: &mut Reader) -> ParseResult<HurlFile> {
    let entries = zero_or_more(entry, reader)?;
    let line_terminators = optional_line_terminators(reader)?;
    eof(reader)?;
    Ok(HurlFile {
        entries,
        line_terminators,
    })
}

fn entry(reader: &mut Reader) -> ParseResult<Entry> {
    let req = request(reader)?;
    let resp = optional(response, reader)?;
    Ok(Entry {
        request: req,
        response: resp,
    })
}

fn request(reader: &mut Reader) -> ParseResult<Request> {
    let start = reader.cursor();
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let method = method(reader)?;
    let space1 = one_or_more_spaces(reader)?;
    let url = unquoted_template(reader)?;
    let line_terminator0 = line_terminator(reader)?;
    let headers = zero_or_more(key_value, reader)?;
    let sections = request_sections(reader)?;
    let body = optional(body, reader)?;
    let source_info = SourceInfo::new(start.pos, reader.cursor().pos);

    // Check duplicated section
    let mut section_names = vec![];
    for section in &sections {
        if section_names.contains(&section.identifier()) {
            return Err(ParseError::new(
                section.source_info.start,
                false,
                ParseErrorKind::DuplicateSection,
            ));
        } else {
            section_names.push(section.identifier());
        }
    }

    Ok(Request {
        line_terminators,
        space0,
        method,
        space1,
        url,
        line_terminator0,
        headers,
        sections,
        body,
        source_info,
    })
}

fn response(reader: &mut Reader) -> ParseResult<Response> {
    let start = reader.cursor();
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let version = version(reader)?;
    let space1 = one_or_more_spaces(reader)?;
    let status = status(reader)?;
    let line_terminator0 = line_terminator(reader)?;
    let headers = zero_or_more(key_value, reader)?;
    let sections = response_sections(reader)?;
    let body = optional(body, reader)?;
    let source_info = SourceInfo::new(start.pos, reader.cursor().pos);

    Ok(Response {
        line_terminators,
        space0,
        version,
        space1,
        status,
        line_terminator0,
        headers,
        sections,
        body,
        source_info,
    })
}

fn method(reader: &mut Reader) -> ParseResult<Method> {
    if reader.is_eof() {
        let kind = ParseErrorKind::Method {
            name: "<EOF>".to_string(),
        };
        return Err(ParseError::new(reader.cursor().pos, true, kind));
    }
    let start = reader.cursor();
    let name = reader.read_while(|c| c.is_ascii_alphabetic());
    if name.is_empty() || name.to_uppercase() != name {
        let kind = ParseErrorKind::Method { name };
        Err(ParseError::new(start.pos, false, kind))
    } else {
        Ok(Method(name))
    }
}

fn version(reader: &mut Reader) -> ParseResult<Version> {
    let start = reader.cursor();
    try_literal("HTTP", reader)?;

    let next_c = reader.peek();
    match next_c {
        Some('/') => {
            let available_version = [
                ("/1.0", VersionValue::Version1),
                ("/1.1", VersionValue::Version11),
                ("/2", VersionValue::Version2),
                ("/3", VersionValue::Version3),
            ];
            for (s, value) in available_version.iter() {
                if try_literal(s, reader).is_ok() {
                    return Ok(Version {
                        value: value.clone(),
                        source_info: SourceInfo::new(start.pos, reader.cursor().pos),
                    });
                }
            }
            Err(ParseError::new(start.pos, false, ParseErrorKind::Version))
        }
        Some(' ') | Some('\t') => Ok(Version {
            value: VersionAny,
            source_info: SourceInfo::new(start.pos, reader.cursor().pos),
        }),
        _ => Err(ParseError::new(start.pos, false, ParseErrorKind::Version)),
    }
}

fn status(reader: &mut Reader) -> ParseResult<Status> {
    let start = reader.cursor();
    let value = match try_literal("*", reader) {
        Ok(_) => StatusValue::Any,
        Err(_) => {
            if reader.is_eof() {
                let kind = ParseErrorKind::Status;
                return Err(ParseError::new(start.pos, false, kind));
            }
            let s = reader.read_while(|c| c.is_ascii_digit());
            if s.is_empty() {
                let kind = ParseErrorKind::Status;
                return Err(ParseError::new(start.pos, false, kind));
            }
            match s.to_string().parse() {
                Ok(value) => StatusValue::Specific(value),
                Err(_) => {
                    let kind = ParseErrorKind::Status;
                    return Err(ParseError::new(start.pos, false, kind));
                }
            }
        }
    };
    let end = reader.cursor();
    Ok(Status {
        value,
        source_info: SourceInfo::new(start.pos, end.pos),
    })
}

fn body(reader: &mut Reader) -> ParseResult<Body> {
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
    use crate::ast::{
        Bytes, Comment, JsonListElement, JsonValue, LineTerminator, MultilineString,
        MultilineStringKind, Template, TemplateElement, Text, Whitespace,
    };
    use crate::reader::Pos;
    use crate::typing::ToSource;

    #[test]
    fn test_hurl_file() {
        let mut reader = Reader::new("GET http://google.fr");
        let hurl_file = hurl_file(&mut reader).unwrap();
        assert_eq!(hurl_file.entries.len(), 1);
    }

    #[test]
    fn test_entry() {
        let mut reader = Reader::new("GET http://google.fr");
        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method("GET".to_string()));
        assert_eq!(reader.cursor().index, 20);
    }

    #[test]
    fn test_several_entry() {
        let mut reader = Reader::new("GET http://google.fr\nGET http://google.fr");

        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method("GET".to_string()));
        assert_eq!(reader.cursor().index, 21);
        assert_eq!(reader.cursor().pos.line, 2);

        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method("GET".to_string()));
        assert_eq!(reader.cursor().index, 41);
        assert_eq!(reader.cursor().pos.line, 2);

        let mut reader =
            Reader::new("GET http://google.fr # comment1\nGET http://google.fr # comment2");

        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method("GET".to_string()));
        assert_eq!(reader.cursor().index, 32);
        assert_eq!(reader.cursor().pos.line, 2);

        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method("GET".to_string()));
        assert_eq!(reader.cursor().index, 63);
        assert_eq!(reader.cursor().pos.line, 2);
    }

    #[test]
    fn test_entry_with_response() {
        let mut reader = Reader::new("GET http://google.fr\nHTTP/1.1 200");
        let e = entry(&mut reader).unwrap();
        assert_eq!(e.request.method, Method("GET".to_string()));
        assert_eq!(e.response.unwrap().status.value, StatusValue::Specific(200));
    }

    #[test]
    fn test_request() {
        let mut reader = Reader::new("GET http://google.fr");
        let default_request = Request {
            line_terminators: vec![],
            space0: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            },
            method: Method("GET".to_string()),
            space1: Whitespace {
                value: " ".to_string(),
                source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 5)),
            },
            url: Template {
                elements: vec![TemplateElement::String {
                    value: "http://google.fr".to_string(),
                    source: "http://google.fr".to_source(),
                }],
                delimiter: None,
                source_info: SourceInfo::new(Pos::new(1, 5), Pos::new(1, 21)),
            },
            line_terminator0: LineTerminator {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 21), Pos::new(1, 21)),
                },
                comment: None,
                newline: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 21), Pos::new(1, 21)),
                },
            },
            headers: vec![],
            sections: vec![],
            body: None,
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 21)),
        };
        assert_eq!(request(&mut reader).unwrap(), default_request);
        assert_eq!(reader.cursor().index, 20);

        let mut reader = Reader::new("GET  http://google.fr # comment");
        let default_request = Request {
            line_terminators: vec![],
            space0: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            },
            method: Method("GET".to_string()),
            space1: Whitespace {
                value: "  ".to_string(),
                source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 6)),
            },
            url: Template {
                elements: vec![TemplateElement::String {
                    value: "http://google.fr".to_string(),
                    source: "http://google.fr".to_source(),
                }],
                delimiter: None,
                source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 22)),
            },
            line_terminator0: LineTerminator {
                space0: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 22), Pos::new(1, 23)),
                },
                comment: Some(Comment {
                    value: " comment".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 24), Pos::new(1, 32)),
                }),
                newline: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 32), Pos::new(1, 32)),
                },
            },
            headers: vec![],
            sections: vec![],
            body: None,
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 32)),
        };
        assert_eq!(request(&mut reader).unwrap(), default_request);
        assert_eq!(reader.cursor().index, 31);

        let mut reader = Reader::new("GET http://google.fr\nGET http://google.fr");
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method("GET".to_string()));
        assert_eq!(reader.cursor().index, 21);
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method("GET".to_string()));
    }

    #[test]
    fn test_request_multilines() {
        // GET http://google.fr
        // ```
        // Hello World!
        // ```
        let mut reader = Reader::new("GET http://google.fr\n```\nHello World!\n```");
        let req = request(&mut reader).unwrap();
        assert_eq!(
            req.body.unwrap(),
            Body {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 1)),
                },
                value: Bytes::MultilineString(MultilineString {
                    kind: MultilineStringKind::Text(Text {
                        space: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(2, 4), Pos::new(2, 4)),
                        },
                        newline: Whitespace {
                            source_info: SourceInfo::new(Pos::new(2, 4), Pos::new(3, 1)),
                            value: "\n".to_string(),
                        },
                        value: Template {
                            elements: vec![TemplateElement::String {
                                value: "Hello World!\n".to_string(),
                                source: "Hello World!\n".to_source(),
                            }],
                            delimiter: None,
                            source_info: SourceInfo::new(Pos::new(3, 1), Pos::new(4, 1)),
                        },
                    }),
                    attributes: vec![]
                }),
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(4, 4), Pos::new(4, 4)),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(4, 4), Pos::new(4, 4)),
                    },
                },
            }
        );
    }

    #[test]
    fn test_request_post_json() {
        let mut reader = Reader::new("POST http://localhost:8000/post-json-array\n[1,2,3]");
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method("POST".to_string()));
        assert_eq!(
            r.body.unwrap().value,
            Bytes::Json(JsonValue::List {
                space0: String::new(),
                elements: vec![
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("1".to_string()),
                        space1: String::new(),
                    },
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("2".to_string()),
                        space1: String::new(),
                    },
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("3".to_string()),
                        space1: String::new(),
                    },
                ],
            })
        );

        let mut reader = Reader::new("POST http://localhost:8000/post-json-string\n\"Hello\"");
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method("POST".to_string()));
        assert_eq!(
            r.body.unwrap().value,
            Bytes::Json(JsonValue::String(Template {
                delimiter: Some('"'),
                elements: vec![TemplateElement::String {
                    value: "Hello".to_string(),
                    source: "Hello".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(2, 2), Pos::new(2, 7)),
            }))
        );

        let mut reader = Reader::new("POST http://localhost:8000/post-json-number\n100");
        let r = request(&mut reader).unwrap();
        assert_eq!(r.method, Method("POST".to_string()));
        assert_eq!(
            r.body.unwrap().value,
            Bytes::Json(JsonValue::Number("100".to_string()))
        );
    }

    #[test]
    fn test_request_error() {
        let mut reader = Reader::new("xxx");
        let error = request(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
    }

    #[test]
    fn test_response() {
        let mut reader = Reader::new("HTTP/1.1 200");
        //println!("{:?}", response(&mut reader));
        let r = response(&mut reader).unwrap();

        assert_eq!(r.version.value, VersionValue::Version11);
        assert_eq!(r.status.value, StatusValue::Specific(200));
    }

    #[test]
    fn test_method() {
        let mut reader = Reader::new("xxx ");
        let error = method(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("");
        let error = method(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(reader.cursor().index, 0);

        let mut reader = Reader::new("GET ");
        assert_eq!(method(&mut reader).unwrap(), Method("GET".to_string()));
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("CUSTOM");
        assert_eq!(method(&mut reader).unwrap(), Method("CUSTOM".to_string()));
        assert_eq!(reader.cursor().index, 6);
    }

    #[test]
    fn test_version() {
        let mut reader = Reader::new("HTTP 200");
        assert_eq!(version(&mut reader).unwrap().value, VersionAny);
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("HTTP\t200");
        assert_eq!(version(&mut reader).unwrap().value, VersionAny);
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("HTTP/1.1 200");
        assert_eq!(version(&mut reader).unwrap().value, VersionValue::Version11);

        let mut reader = Reader::new("HTTP/1. 200");
        let error = version(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
    }

    #[test]
    fn test_status() {
        let mut reader = Reader::new("*");
        let s = status(&mut reader).unwrap();
        assert_eq!(s.value, StatusValue::Any);

        let mut reader = Reader::new("200");
        let s = status(&mut reader).unwrap();
        assert_eq!(s.value, StatusValue::Specific(200));

        let mut reader = Reader::new("xxx");
        let result = status(&mut reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_body_json() {
        let mut reader = Reader::new("[1,2,3] ");
        let b = body(&mut reader).unwrap();
        assert_eq!(b.line_terminators.len(), 0);
        assert_eq!(
            b.value,
            Bytes::Json(JsonValue::List {
                space0: String::new(),
                elements: vec![
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("1".to_string()),
                        space1: String::new(),
                    },
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("2".to_string()),
                        space1: String::new(),
                    },
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("3".to_string()),
                        space1: String::new(),
                    },
                ],
            })
        );
        assert_eq!(reader.cursor().index, 8);

        let mut reader = Reader::new("{}");
        let b = body(&mut reader).unwrap();
        assert_eq!(b.line_terminators.len(), 0);
        assert_eq!(
            b.value,
            Bytes::Json(JsonValue::Object {
                space0: String::new(),
                elements: vec![],
            })
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("# comment\n {} # comment\nxxx");
        let b = body(&mut reader).unwrap();
        assert_eq!(b.line_terminators.len(), 1);
        assert_eq!(
            b.value,
            Bytes::Json(JsonValue::Object {
                space0: String::new(),
                elements: vec![],
            })
        );
        assert_eq!(reader.cursor().index, 24);

        let mut reader = Reader::new("{x");
        let error = body(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert!(!error.recoverable);
    }
}
