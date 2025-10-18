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
    Assert, Capture, Cookie, FilenameParam, FilenameValue, MultipartParam, Section, SectionValue,
    SourceInfo, Whitespace,
};
use crate::combinator::{optional, recover, zero_or_more};
use crate::parser::filter::filters;
use crate::parser::predicate::predicate;
use crate::parser::primitives::{
    key_value, line_terminator, literal, one_or_more_spaces, optional_line_terminators,
    try_literal, zero_or_more_spaces,
};
use crate::parser::query::query;
use crate::parser::string::unquoted_template;
use crate::parser::{filename, key_string, option, ParseError, ParseErrorKind, ParseResult};
use crate::reader::{Pos, Reader};

pub fn request_sections(reader: &mut Reader) -> ParseResult<Vec<Section>> {
    let sections = zero_or_more(request_section, reader)?;
    Ok(sections)
}

pub fn response_sections(reader: &mut Reader) -> ParseResult<Vec<Section>> {
    let sections = zero_or_more(response_section, reader)?;
    Ok(sections)
}

fn request_section(reader: &mut Reader) -> ParseResult<Section> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let start = reader.cursor();
    let name = section_name(reader)?;
    let source_info = SourceInfo::new(start.pos, reader.cursor().pos);

    let line_terminator0 = line_terminator(reader)?;
    let value = match name.as_str() {
        "Query" => section_value_query_params(reader, true)?,
        "QueryStringParams" => section_value_query_params(reader, false)?,
        "BasicAuth" => section_value_basic_auth(reader)?,
        "Form" => section_value_form_params(reader, true)?,
        "FormParams" => section_value_form_params(reader, false)?,
        "Multipart" => section_value_multipart_form_data(reader, true)?,
        "MultipartFormData" => section_value_multipart_form_data(reader, false)?,
        "Cookies" => section_value_cookies(reader)?,
        "Options" => section_value_options(reader)?,
        _ => {
            let kind = ParseErrorKind::RequestSectionName { name: name.clone() };
            let pos = Pos::new(start.pos.line, start.pos.column + 1);
            return Err(ParseError::new(pos, false, kind));
        }
    };

    Ok(Section {
        line_terminators,
        space0,
        line_terminator0,
        value,
        source_info,
    })
}

fn response_section(reader: &mut Reader) -> ParseResult<Section> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let start = reader.cursor();
    let name = section_name(reader)?;
    let end = reader.cursor();
    let source_info = SourceInfo::new(start.pos, end.pos);

    let line_terminator0 = line_terminator(reader)?;
    let value = match name.as_str() {
        "Captures" => section_value_captures(reader)?,
        "Asserts" => section_value_asserts(reader)?,
        _ => {
            let kind = ParseErrorKind::ResponseSectionName { name: name.clone() };
            let pos = Pos::new(start.pos.line, start.pos.column + 1);
            return Err(ParseError::new(pos, false, kind));
        }
    };

    Ok(Section {
        line_terminators,
        space0,
        line_terminator0,
        value,
        source_info,
    })
}

fn section_name(reader: &mut Reader) -> ParseResult<String> {
    let pos = reader.cursor().pos;
    try_literal("[", reader)?;
    let name = reader.read_while(|c| c.is_alphanumeric());
    if name.is_empty() {
        // Could be the empty json array for the body
        let kind = ParseErrorKind::Expecting {
            value: "a valid section name".to_string(),
        };
        return Err(ParseError::new(pos, true, kind));
    }
    try_literal("]", reader)?;
    Ok(name)
}

fn section_value_query_params(reader: &mut Reader, short: bool) -> ParseResult<SectionValue> {
    let items = zero_or_more(key_value, reader)?;
    Ok(SectionValue::QueryParams(items, short))
}

fn section_value_basic_auth(reader: &mut Reader) -> ParseResult<SectionValue> {
    let v = optional(key_value, reader)?;
    Ok(SectionValue::BasicAuth(v))
}

fn section_value_form_params(reader: &mut Reader, short: bool) -> ParseResult<SectionValue> {
    let items = zero_or_more(key_value, reader)?;
    Ok(SectionValue::FormParams(items, short))
}

fn section_value_multipart_form_data(
    reader: &mut Reader,
    short: bool,
) -> ParseResult<SectionValue> {
    let items = zero_or_more(multipart_param, reader)?;
    Ok(SectionValue::MultipartFormData(items, short))
}

fn section_value_cookies(reader: &mut Reader) -> ParseResult<SectionValue> {
    let items = zero_or_more(cookie, reader)?;
    Ok(SectionValue::Cookies(items))
}

fn section_value_captures(reader: &mut Reader) -> ParseResult<SectionValue> {
    let items = zero_or_more(capture, reader)?;
    Ok(SectionValue::Captures(items))
}

fn section_value_asserts(reader: &mut Reader) -> ParseResult<SectionValue> {
    let asserts = zero_or_more(assert, reader)?;
    Ok(SectionValue::Asserts(asserts))
}

fn section_value_options(reader: &mut Reader) -> ParseResult<SectionValue> {
    let options = zero_or_more(option::parse, reader)?;
    Ok(SectionValue::Options(options))
}

fn cookie(reader: &mut Reader) -> ParseResult<Cookie> {
    // let start = reader.state.clone();
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let name = recover(key_string::parse, reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    recover(|p1| literal(":", p1), reader)?;
    let space2 = zero_or_more_spaces(reader)?;
    let value = unquoted_template(reader)?;
    let line_terminator0 = line_terminator(reader)?;
    Ok(Cookie {
        line_terminators,
        space0,
        name,
        space1,
        space2,
        value,
        line_terminator0,
    })
}

fn multipart_param(reader: &mut Reader) -> ParseResult<MultipartParam> {
    let save = reader.cursor();
    match file_param(reader) {
        Ok(f) => Ok(MultipartParam::FilenameParam(f)),
        Err(e) => {
            if e.recoverable {
                reader.seek(save);
                let param = key_value(reader)?;
                Ok(MultipartParam::Param(param))
            } else {
                Err(e)
            }
        }
    }
}

fn file_param(reader: &mut Reader) -> ParseResult<FilenameParam> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let key = recover(key_string::parse, reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    recover(|reader1| literal(":", reader1), reader)?;
    let space2 = zero_or_more_spaces(reader)?;
    let value = file_value(reader)?;
    let line_terminator0 = line_terminator(reader)?;
    Ok(FilenameParam {
        line_terminators,
        space0,
        key,
        space1,
        space2,
        value,
        line_terminator0,
    })
}

fn file_value(reader: &mut Reader) -> ParseResult<FilenameValue> {
    try_literal("file,", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let f = filename::parse(reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    literal(";", reader)?;
    let save = reader.cursor();
    let (space2, content_type) = match line_terminator(reader) {
        Ok(_) => {
            reader.seek(save);
            let space2 = Whitespace {
                value: String::new(),
                source_info: SourceInfo {
                    start: save.pos,
                    end: save.pos,
                },
            };
            (space2, None)
        }
        Err(_) => {
            reader.seek(save);
            let space2 = zero_or_more_spaces(reader)?;
            let start = reader.cursor();
            let Ok(content_type) = unquoted_template(reader) else {
                return Err(ParseError::new(
                    start.pos,
                    false,
                    ParseErrorKind::FileContentType,
                ));
            };
            (space2, Some(content_type))
        }
    };

    Ok(FilenameValue {
        space0,
        filename: f,
        space1,
        space2,
        content_type,
    })
}

fn capture(reader: &mut Reader) -> ParseResult<Capture> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let name = recover(key_string::parse, reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    recover(|p1| literal(":", p1), reader)?;
    let space2 = zero_or_more_spaces(reader)?;
    let q = query(reader)?;
    let filters = filters(reader)?;
    let (redacted, space3) = if let Some(ws) = optional(redacted, reader)? {
        (true, ws)
    } else {
        // No `redact` keywork, space3 is an empty whitespace and the optional remaining whitespace
        // will be consumed by the line terminator.
        let pos = reader.cursor().pos;
        let value = String::new();
        let source_info = SourceInfo::new(pos, pos);
        (false, Whitespace { value, source_info })
    };

    let line_terminator0 = line_terminator(reader)?;
    Ok(Capture {
        line_terminators,
        space0,
        name,
        space1,
        space2,
        query: q,
        filters,
        space3,
        redacted,
        line_terminator0,
    })
}

fn redacted(reader: &mut Reader) -> ParseResult<Whitespace> {
    let space = zero_or_more_spaces(reader)?;
    try_literal("redact", reader)?;
    Ok(space)
}

fn assert(reader: &mut Reader) -> ParseResult<Assert> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let query0 = query(reader)?;
    let filters = filters(reader)?;
    let space1 = one_or_more_spaces(reader)?;
    let predicate0 = predicate(reader)?;

    let line_terminator0 = line_terminator(reader)?;
    Ok(Assert {
        line_terminators,
        space0,
        query: query0,
        filters,
        space1,
        predicate: predicate0,
        line_terminator0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        KeyValue, LineTerminator, Number, Predicate, PredicateFunc, PredicateFuncValue,
        PredicateValue, Query, QueryValue, Template, TemplateElement, I64,
    };
    use crate::reader::CharPos;
    use crate::types::ToSource;

    #[test]
    fn test_section_name() {
        let mut reader = Reader::new("[SectionA]");
        assert_eq!(section_name(&mut reader).unwrap(), String::from("SectionA"));

        let mut reader = Reader::new("[]");
        assert!(section_name(&mut reader).err().unwrap().recoverable);
    }

    #[test]
    fn test_asserts_section() {
        let mut reader = Reader::new("[Asserts]\nheader \"Location\" == \"https://google.fr\"\n");

        assert_eq!(
            response_section(&mut reader).unwrap(),
            Section {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(1, 10)),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::from("\n"),
                        source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(2, 1)),
                    },
                },
                value: SectionValue::Asserts(vec![Assert {
                    line_terminators: vec![],
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 1)),
                    },
                    query: Query {
                        source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 18)),
                        value: QueryValue::Header {
                            space0: Whitespace {
                                value: String::from(" "),
                                source_info: SourceInfo::new(Pos::new(2, 7), Pos::new(2, 8)),
                            },
                            name: Template::new(
                                Some('"'),
                                vec![TemplateElement::String {
                                    value: "Location".to_string(),
                                    source: "Location".to_source(),
                                }],
                                SourceInfo::new(Pos::new(2, 8), Pos::new(2, 18)),
                            ),
                        },
                    },
                    filters: vec![],
                    space1: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::new(Pos::new(2, 18), Pos::new(2, 19)),
                    },
                    predicate: Predicate {
                        not: false,
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(2, 19), Pos::new(2, 19)),
                        },
                        predicate_func: PredicateFunc {
                            source_info: SourceInfo::new(Pos::new(2, 19), Pos::new(2, 41)),
                            value: PredicateFuncValue::Equal {
                                space0: Whitespace {
                                    value: String::from(" "),
                                    source_info: SourceInfo::new(Pos::new(2, 21), Pos::new(2, 22)),
                                },
                                value: PredicateValue::String(Template::new(
                                    Some('"'),
                                    vec![TemplateElement::String {
                                        value: "https://google.fr".to_string(),
                                        source: "https://google.fr".to_source(),
                                    }],
                                    SourceInfo::new(Pos::new(2, 22), Pos::new(2, 41))
                                )),
                            },
                        },
                    },
                    line_terminator0: LineTerminator {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(2, 41), Pos::new(2, 41)),
                        },
                        comment: None,
                        newline: Whitespace {
                            value: String::from("\n"),
                            source_info: SourceInfo::new(Pos::new(2, 41), Pos::new(3, 1)),
                        },
                    },
                }]),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)),
            }
        );
    }

    #[test]
    fn test_asserts_section_error() {
        let mut reader = Reader::new("x[Assertsx]\nheader Location == \"https://google.fr\"\n");
        let error = response_section(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("[")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("[Assertsx]\nheader Location == \"https://google.fr\"\n");
        let error = response_section(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert_eq!(
            error.kind,
            ParseErrorKind::ResponseSectionName {
                name: String::from("Assertsx")
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_cookie() {
        let mut reader = Reader::new("Foo: Bar");
        let c = cookie(&mut reader).unwrap();
        assert_eq!(c.name.to_string(), String::from("Foo"));
        assert_eq!(
            c.value,
            Template::new(
                None,
                vec![TemplateElement::String {
                    value: "Bar".to_string(),
                    source: "Bar".to_source(),
                }],
                SourceInfo::new(Pos::new(1, 6), Pos::new(1, 9))
            )
        );
    }

    #[test]
    fn test_cookie_error() {
        let mut reader = Reader::new("Foo: {{Bar");
        let error = cookie(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 11,
            }
        );
        assert!(!error.recoverable);
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "}}".to_string()
            }
        );
    }

    #[test]
    fn test_file_value() {
        let mut reader = Reader::new("file,hello.txt;");
        assert_eq!(
            file_value(&mut reader).unwrap(),
            FilenameValue {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 6)),
                },
                filename: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "hello.txt".to_string(),
                        source: "hello.txt".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 6), Pos::new(1, 15)),
                ),
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                },
                space2: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 16), Pos::new(1, 16)),
                },
                content_type: None,
            }
        );
        let mut reader = Reader::new("file,hello.txt; text/html");
        assert_eq!(
            file_value(&mut reader).unwrap(),
            FilenameValue {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 6)),
                },
                filename: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "hello.txt".to_string(),
                        source: "hello.txt".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 6), Pos::new(1, 15)),
                ),
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 16), Pos::new(1, 17)),
                },
                content_type: Some(Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "text/html".to_string(),
                        source: "text/html".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 17), Pos::new(1, 26)),
                )),
            }
        );
    }

    #[test]
    fn test_file_content_type() {
        let mut reader = Reader::new("file,hello.txt; text/html");
        let file_value = file_value(&mut reader).unwrap();
        let content_type = file_value.content_type.unwrap();
        assert_eq!(content_type.to_string(), "text/html".to_string());
        assert_eq!(reader.cursor().index, CharPos(25));

        let mut reader = Reader::new("file,------; text/plain; charset=us-ascii");
        let file_value = crate::parser::sections::file_value(&mut reader).unwrap();
        let content_type = file_value.content_type.unwrap();
        assert_eq!(
            content_type.to_string(),
            "text/plain; charset=us-ascii".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(41));

        let mut reader = Reader::new("file,******; text/html # comment");
        let file_value = crate::parser::sections::file_value(&mut reader).unwrap();
        let content_type = file_value.content_type.unwrap();
        assert_eq!(content_type.to_string(), "text/html".to_string());
        assert_eq!(reader.cursor().index, CharPos(22));

        let mut reader = Reader::new("file,{{some_file}}; application/vnd.openxmlformats-officedocument.wordprocessingml.document # comment");
        let file_value = crate::parser::sections::file_value(&mut reader).unwrap();
        let content_type = file_value.content_type.unwrap();
        assert_eq!(
            content_type.to_string(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(91));

        let mut reader = Reader::new("file,{{some_file}}; {{some_content_type}} # comment");
        let file_value = crate::parser::sections::file_value(&mut reader).unwrap();
        let content_type = file_value.content_type.unwrap();
        assert_eq!(
            content_type.to_string(),
            "{{some_content_type}}".to_string()
        );
        assert_eq!(reader.cursor().index, CharPos(41));
    }

    #[test]
    fn test_capture() {
        let mut reader = Reader::new("url: header \"Location\"");
        let capture0 = capture(&mut reader).unwrap();

        assert_eq!(
            capture0,
            Capture {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos::new(1, 1),
                        end: Pos::new(1, 1),
                    },
                },
                name: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "url".to_string(),
                        source: "url".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 1), Pos::new(1, 4))
                ),
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos::new(1, 4),
                        end: Pos::new(1, 4),
                    }
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo {
                        start: Pos::new(1, 5),
                        end: Pos::new(1, 6),
                    }
                },
                query: Query {
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 23)),
                    value: QueryValue::Header {
                        space0: Whitespace {
                            value: String::from(" "),
                            source_info: SourceInfo::new(Pos::new(1, 12), Pos::new(1, 13)),
                        },
                        name: Template::new(
                            Some('"'),
                            vec![TemplateElement::String {
                                value: "Location".to_string(),
                                source: "Location".to_source(),
                            }],
                            SourceInfo::new(Pos::new(1, 13), Pos::new(1, 23))
                        )
                    }
                },
                filters: vec![],
                space3: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 23), Pos::new(1, 23)),
                },
                redacted: false,
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 23), Pos::new(1, 23)),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 23), Pos::new(1, 23)),
                    },
                },
            }
        );

        let mut reader = Reader::new("url: header \"Token\"    redact");
        let capture0 = capture(&mut reader).unwrap();
        assert_eq!(
            capture0,
            Capture {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos::new(1, 1),
                        end: Pos::new(1, 1),
                    },
                },
                name: Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "url".to_string(),
                        source: "url".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 1), Pos::new(1, 4))
                ),
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos::new(1, 4),
                        end: Pos::new(1, 4),
                    }
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo {
                        start: Pos::new(1, 5),
                        end: Pos::new(1, 6),
                    }
                },
                query: Query {
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 20)),
                    value: QueryValue::Header {
                        space0: Whitespace {
                            value: String::from(" "),
                            source_info: SourceInfo::new(Pos::new(1, 12), Pos::new(1, 13)),
                        },
                        name: Template::new(
                            Some('"'),
                            vec![TemplateElement::String {
                                value: "Token".to_string(),
                                source: "Token".to_source(),
                            }],
                            SourceInfo::new(Pos::new(1, 13), Pos::new(1, 20))
                        )
                    }
                },
                filters: vec![],
                space3: Whitespace {
                    value: "    ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 20), Pos::new(1, 24)),
                },
                redacted: true,
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 30), Pos::new(1, 30)),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 30), Pos::new(1, 30)),
                    },
                },
            }
        );
    }

    #[test]
    fn test_capture_with_filter() {
        let mut reader = Reader::new("token: header \"Location\" regex \"token=(.*)\"");
        let capture0 = capture(&mut reader).unwrap();

        assert_eq!(
            capture0.query,
            Query {
                source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 25)),
                value: QueryValue::Header {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::new(Pos::new(1, 14), Pos::new(1, 15)),
                    },
                    name: Template::new(
                        Some('"'),
                        vec![TemplateElement::String {
                            value: "Location".to_string(),
                            source: "Location".to_source(),
                        }],
                        SourceInfo::new(Pos::new(1, 15), Pos::new(1, 25))
                    )
                }
            }
        );
        assert_eq!(reader.cursor().index, CharPos(43));
    }

    #[test]
    fn test_capture_with_filter_error() {
        let mut reader = Reader::new("token: header \"Location\" regex ");
        let error = capture(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 32,
            }
        );
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\" or /".to_string()
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::new("token: header \"Location\" xxx");
        let error = capture(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 26,
            }
        );
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "line_terminator".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_capture_with_comment() {
        let mut reader = Reader::new("name: jsonpath \"$.name\"          # name");
        let capture0 = capture(&mut reader).unwrap();
        assert!(capture0.filters.is_empty());
        assert!(capture0.space3.value.is_empty());
        assert_eq!(capture0.line_terminator0.space0.as_str(), "          ");
    }

    #[test]
    fn test_assert() {
        let mut reader = Reader::new("header \"Location\" == \"https://google.fr\"");
        let assert0 = assert(&mut reader).unwrap();

        assert_eq!(
            assert0.query,
            Query {
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 18)),
                value: QueryValue::Header {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 8)),
                    },
                    name: Template::new(
                        Some('"'),
                        vec![TemplateElement::String {
                            value: "Location".to_string(),
                            source: "Location".to_source(),
                        }],
                        SourceInfo::new(Pos::new(1, 8), Pos::new(1, 18)),
                    )
                }
            }
        );
    }

    #[test]
    fn test_assert_jsonpath() {
        let mut reader = Reader::new("jsonpath \"$.errors\" == 5");

        assert_eq!(
            assert(&mut reader).unwrap().predicate,
            Predicate {
                not: false,
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 21), Pos::new(1, 21)),
                },
                predicate_func: PredicateFunc {
                    source_info: SourceInfo::new(Pos::new(1, 21), Pos::new(1, 25)),
                    value: PredicateFuncValue::Equal {
                        space0: Whitespace {
                            value: String::from(" "),
                            source_info: SourceInfo::new(Pos::new(1, 23), Pos::new(1, 24)),
                        },
                        value: PredicateValue::Number(Number::Integer(I64::new(
                            5,
                            "5".to_source()
                        ))),
                    },
                },
            }
        );
    }

    #[test]
    fn test_basicauth_section() {
        let mut reader = Reader::new("[BasicAuth]\nuser:password\n\nHTTP 200\n");

        assert_eq!(
            request_section(&mut reader).unwrap(),
            Section {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 12), Pos::new(1, 12)),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::from("\n"),
                        source_info: SourceInfo::new(Pos::new(1, 12), Pos::new(2, 1)),
                    },
                },
                value: SectionValue::BasicAuth(Some(KeyValue {
                    line_terminators: vec![],
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 1))
                    },
                    key: Template::new(
                        None,
                        vec![TemplateElement::String {
                            value: "user".to_string(),
                            source: "user".to_source()
                        }],
                        SourceInfo::new(Pos::new(2, 1), Pos::new(2, 5)),
                    ),
                    space1: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(2, 5), Pos::new(2, 5))
                    },
                    space2: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(2, 6), Pos::new(2, 6))
                    },
                    value: Template::new(
                        None,
                        vec![TemplateElement::String {
                            value: "password".to_string(),
                            source: "password".to_source()
                        }],
                        SourceInfo::new(Pos::new(2, 6), Pos::new(2, 14)),
                    ),
                    line_terminator0: LineTerminator {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(2, 14), Pos::new(2, 14))
                        },
                        comment: None,
                        newline: Whitespace {
                            value: "\n".to_string(),
                            source_info: SourceInfo::new(Pos::new(2, 14), Pos::new(3, 1))
                        },
                    },
                })),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 12)),
            }
        );
        assert_eq!(reader.cursor().pos, Pos { line: 3, column: 1 });

        let mut reader = Reader::new("[BasicAuth]\nHTTP 200\n");
        assert_eq!(
            request_section(&mut reader).unwrap(),
            Section {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 12), Pos::new(1, 12)),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::from("\n"),
                        source_info: SourceInfo::new(Pos::new(1, 12), Pos::new(2, 1)),
                    },
                },
                value: SectionValue::BasicAuth(None),
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 12)),
            }
        );
        assert_eq!(reader.cursor().pos, Pos { line: 2, column: 1 });
    }
}
