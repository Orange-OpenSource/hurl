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

use super::combinators::*;
use super::error::*;
use super::filename;
use super::predicate::predicate;
use super::primitives::*;
use super::query::query;
use super::reader::Reader;
use super::string::*;
use super::ParseResult;

pub fn request_sections(reader: &mut Reader) -> ParseResult<'static, Vec<Section>> {
    let sections = zero_or_more(request_section, reader)?;
    Ok(sections)
}

pub fn response_sections(reader: &mut Reader) -> ParseResult<'static, Vec<Section>> {
    let sections = zero_or_more(response_section, reader)?;
    Ok(sections)
}

fn request_section(reader: &mut Reader) -> ParseResult<'static, Section> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let start = reader.state.clone();
    let name = section_name(reader)?;
    let source_info = SourceInfo {
        start: start.clone().pos,
        end: reader.state.clone().pos,
    };
    let line_terminator0 = line_terminator(reader)?;
    let value = match name.as_str() {
        "QueryStringParams" => section_value_query_params(reader)?,
        "BasicAuth" => section_value_basic_auth(reader)?,
        "FormParams" => section_value_form_params(reader)?,
        "MultipartFormData" => section_value_multipart_form_data(reader)?,
        "Cookies" => section_value_cookies(reader)?,
        _ => {
            return Err(Error {
                pos: Pos {
                    line: start.pos.line,
                    column: start.pos.column + 1,
                },
                recoverable: false,
                inner: ParseError::RequestSectionName { name: name.clone() },
            });
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

fn response_section(reader: &mut Reader) -> ParseResult<'static, Section> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let start = reader.state.clone();
    let name = section_name(reader)?;
    let source_info = SourceInfo {
        start: start.clone().pos,
        end: reader.state.clone().pos,
    };
    let line_terminator0 = line_terminator(reader)?;
    let value = match name.as_str() {
        "Captures" => section_value_captures(reader)?,
        "Asserts" => section_value_asserts(reader)?,
        _ => {
            return Err(Error {
                pos: Pos {
                    line: start.pos.line,
                    column: start.pos.column + 1,
                },
                recoverable: false,
                inner: ParseError::ResponseSectionName { name: name.clone() },
            });
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

fn section_name(reader: &mut Reader) -> ParseResult<'static, String> {
    try_literal("[", reader)?;
    let name = reader.read_while(|c| c.is_alphanumeric());
    try_literal("]", reader)?;
    Ok(name)
}

fn section_value_query_params(reader: &mut Reader) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(key_value, reader)?;
    Ok(SectionValue::QueryParams(items))
}

fn section_value_basic_auth(reader: &mut Reader) -> ParseResult<'static, SectionValue> {
    let kv = key_value(reader)?;
    Ok(SectionValue::BasicAuth(kv))
}

fn section_value_form_params(reader: &mut Reader) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(key_value, reader)?;
    Ok(SectionValue::FormParams(items))
}

fn section_value_multipart_form_data(reader: &mut Reader) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(multipart_param, reader)?;
    Ok(SectionValue::MultipartFormData(items))
}

fn section_value_cookies(reader: &mut Reader) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(cookie, reader)?;
    Ok(SectionValue::Cookies(items))
}

fn section_value_captures(reader: &mut Reader) -> ParseResult<'static, SectionValue> {
    let items = zero_or_more(capture, reader)?;
    Ok(SectionValue::Captures(items))
}

fn section_value_asserts(reader: &mut Reader) -> ParseResult<'static, SectionValue> {
    let asserts = zero_or_more(assert, reader)?;
    Ok(SectionValue::Asserts(asserts))
}

fn cookie(reader: &mut Reader) -> ParseResult<'static, Cookie> {
    // let start = reader.state.clone();
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let name = unquoted_string_key(reader)?;
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

fn multipart_param(reader: &mut Reader) -> ParseResult<'static, MultipartParam> {
    let save = reader.state.clone();
    match file_param(reader) {
        Ok(f) => Ok(MultipartParam::FileParam(f)),
        Err(e) => {
            if e.recoverable {
                reader.state = save;
                let param = key_value(reader)?;
                Ok(MultipartParam::Param(param))
            } else {
                Err(e)
            }
        }
    }
}

fn file_param(reader: &mut Reader) -> ParseResult<'static, FileParam> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let key = recover(unquoted_string_key, reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    recover(|reader1| literal(":", reader1), reader)?;
    let space2 = zero_or_more_spaces(reader)?;
    let value = file_value(reader)?;
    let line_terminator0 = line_terminator(reader)?;
    Ok(FileParam {
        line_terminators,
        space0,
        key,
        space1,
        space2,
        value,
        line_terminator0,
    })
}

fn file_value(reader: &mut Reader) -> ParseResult<'static, FileValue> {
    try_literal("file,", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let f = filename::parse(reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    literal(";", reader)?;
    let save = reader.state.clone();
    let (space2, content_type) = match line_terminator(reader) {
        Ok(_) => {
            reader.state = save.clone();
            let space2 = Whitespace {
                value: "".to_string(),
                source_info: SourceInfo {
                    start: save.pos.clone(),
                    end: save.pos,
                },
            };
            (space2, None)
        }
        Err(_) => {
            reader.state = save;
            let space2 = zero_or_more_spaces(reader)?;
            let content_type = file_content_type(reader)?;
            (space2, Some(content_type))
        }
    };

    Ok(FileValue {
        space0,
        filename: f,
        space1,
        space2,
        content_type,
    })
}

fn file_content_type(reader: &mut Reader) -> ParseResult<'static, String> {
    let start = reader.state.clone();
    let mut buf = "".to_string();
    let mut spaces = "".to_string();
    let mut save = reader.state.clone();
    while let Some(c) = reader.read() {
        if c.is_alphanumeric() || c == '/' || c == ';' || c == '=' || c == '-' {
            buf.push_str(spaces.as_str());
            spaces = "".to_string();
            buf.push(c);
            save = reader.state.clone();
        } else if c == ' ' {
            spaces.push(' ');
        } else {
            break;
        }
    }

    reader.state = save;
    if buf.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::FileContentType {},
        });
    }
    Ok(buf)
}

fn capture(reader: &mut Reader) -> ParseResult<'static, Capture> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let name = unquoted_string_key(reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    recover(|p1| literal(":", p1), reader)?;
    let space2 = zero_or_more_spaces(reader)?;
    let q = query(reader)?;

    let line_terminator0 = line_terminator(reader)?;
    Ok(Capture {
        line_terminators,
        space0,
        name,
        space1,
        space2,
        query: q,
        line_terminator0,
    })
}

fn assert(reader: &mut Reader) -> ParseResult<'static, Assert> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let query0 = query(reader)?;
    let space1 = one_or_more_spaces(reader)?;
    let predicate0 = predicate(reader)?;

    // Specifics for jsonpath //
    // jsonpath always return a list
    // the equals predicate will be used as "firstEquals"
    // you also need the firstStartsWith => not really orthogonal!!
    /* let predicate0 = Predicate {
            not: predicate0.clone().not,
            space0: predicate0.clone().space0,
            predicate_func: PredicateFunc {
                source_info: predicate0.clone().predicate_func.source_info,
                value: if query0.clone().is_jsonpath() {
                    match predicate0.clone().predicate_func.value {
                        PredicateFuncValue::EqualBool { space0, value } => PredicateFuncValue::FirstEqualBool { space0, value },
                        PredicateFuncValue::EqualInt { space0, value } => PredicateFuncValue::FirstEqualInt { space0, value },
                        PredicateFuncValue::EqualString { space0, value } => PredicateFuncValue::FirstEqualString { space0, value },
                        PredicateFuncValue::CountEqual { space0, value } => PredicateFuncValue::FirstCountEqual { space0, value },
                        PredicateFuncValue::StartWith { space0, value } => PredicateFuncValue::FirstStartWith { space0, value },
                        _ => predicate0.clone().predicate_func.value
                    }
                } else {
                    predicate0.clone().predicate_func.value
                },
            },
        };
    */
    let line_terminator0 = line_terminator(reader)?;
    Ok(Assert {
        line_terminators,
        space0,
        query: query0,
        space1,
        predicate: predicate0,
        line_terminator0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;

    #[test]
    fn test_section_name() {
        let mut reader = Reader::init("[SectionA]");
        assert_eq!(section_name(&mut reader).unwrap(), String::from("SectionA"));
    }

    #[test]
    fn test_asserts_section() {
        let mut reader =
            Reader::init("[Asserts]\nheader \"Location\" equals \"https://google.fr\"\n");

        assert_eq!(
            response_section(&mut reader).unwrap(),
            Section {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 1, 1, 1),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 10, 1, 10),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::from("\n"),
                        source_info: SourceInfo::init(1, 10, 2, 1),
                    },
                },
                value: SectionValue::Asserts(vec![Assert {
                    line_terminators: vec![],
                    space0: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(2, 1, 2, 1),
                    },
                    query: Query {
                        source_info: SourceInfo::init(2, 1, 2, 18),
                        value: QueryValue::Header {
                            space0: Whitespace {
                                value: String::from(" "),
                                source_info: SourceInfo::init(2, 7, 2, 8),
                            },
                            name: Template {
                                quotes: true,
                                elements: vec![TemplateElement::String {
                                    value: "Location".to_string(),
                                    encoded: "Location".to_string(),
                                }],
                                source_info: SourceInfo::init(2, 8, 2, 18),
                            },
                        },
                        subquery: None,
                    },
                    space1: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::init(2, 18, 2, 19),
                    },
                    predicate: Predicate {
                        not: false,
                        space0: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(2, 19, 2, 19),
                        },
                        predicate_func: PredicateFunc {
                            source_info: SourceInfo::init(2, 19, 2, 45),
                            value: PredicateFuncValue::Equal {
                                space0: Whitespace {
                                    value: String::from(" "),
                                    source_info: SourceInfo::init(2, 25, 2, 26),
                                },
                                value: PredicateValue::String(Template {
                                    quotes: true,
                                    elements: vec![TemplateElement::String {
                                        value: "https://google.fr".to_string(),
                                        encoded: "https://google.fr".to_string(),
                                    }],
                                    source_info: SourceInfo::init(2, 26, 2, 45),
                                }),
                                operator: false,
                            },
                        },
                    },
                    line_terminator0: LineTerminator {
                        space0: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(2, 45, 2, 45),
                        },
                        comment: None,
                        newline: Whitespace {
                            value: String::from("\n"),
                            source_info: SourceInfo::init(2, 45, 3, 1),
                        },
                    },
                }]),
                source_info: SourceInfo::init(1, 1, 1, 10),
            }
        );
    }

    #[test]
    fn test_asserts_section_error() {
        let mut reader =
            Reader::init("x[Assertsx]\nheader Location equals \"https://google.fr\"\n");
        let error = response_section(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("[")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("[Assertsx]\nheader Location equals \"https://google.fr\"\n");
        let error = response_section(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert_eq!(
            error.inner,
            ParseError::ResponseSectionName {
                name: String::from("Assertsx")
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_cookie() {
        let mut reader = Reader::init("Foo: Bar");
        let c = cookie(&mut reader).unwrap();
        assert_eq!(c.name.value, String::from("Foo"));
        assert_eq!(
            c.value,
            Template {
                quotes: false,
                elements: vec![TemplateElement::String {
                    value: "Bar".to_string(),
                    encoded: "Bar".to_string()
                }],
                source_info: SourceInfo::init(1, 6, 1, 9)
            }
        );
    }

    #[test]
    fn test_cookie_error() {
        let mut reader = Reader::init("Foo: {{Bar");
        let error = cookie(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 11
            }
        );
        assert!(!error.recoverable);
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "}}".to_string()
            }
        );
    }

    #[test]
    fn test_file_value() {
        let mut reader = Reader::init("file,hello.txt;");
        assert_eq!(
            file_value(&mut reader).unwrap(),
            FileValue {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 6, 1, 6),
                },
                filename: Filename {
                    value: "hello.txt".to_string(),
                    source_info: SourceInfo::init(1, 6, 1, 15),
                },
                space1: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 15, 1, 15),
                },
                space2: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 16, 1, 16),
                },
                content_type: None,
            }
        );
        let mut reader = Reader::init("file,hello.txt; text/html");
        assert_eq!(
            file_value(&mut reader).unwrap(),
            FileValue {
                space0: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 6, 1, 6),
                },
                filename: Filename {
                    value: "hello.txt".to_string(),
                    source_info: SourceInfo::init(1, 6, 1, 15),
                },
                space1: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::init(1, 15, 1, 15),
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::init(1, 16, 1, 17),
                },
                content_type: Some("text/html".to_string()),
            }
        );
    }

    #[test]
    fn test_file_content_type() {
        let mut reader = Reader::init("text/html");
        assert_eq!(
            file_content_type(&mut reader).unwrap(),
            "text/html".to_string()
        );
        assert_eq!(reader.state.cursor, 9);

        let mut reader = Reader::init("text/plain; charset=us-ascii");
        assert_eq!(
            file_content_type(&mut reader).unwrap(),
            "text/plain; charset=us-ascii".to_string()
        );
        assert_eq!(reader.state.cursor, 28);

        let mut reader = Reader::init("text/html # comment");
        assert_eq!(
            file_content_type(&mut reader).unwrap(),
            "text/html".to_string()
        );
        assert_eq!(reader.state.cursor, 9);
    }

    #[test]
    fn test_capture() {
        let mut reader = Reader::init("url: header \"Location\"");
        let capture0 = capture(&mut reader).unwrap();

        assert_eq!(
            capture0.name,
            EncodedString {
                quotes: false,
                value: String::from("url"),
                encoded: String::from("url"),
                source_info: SourceInfo::init(1, 1, 1, 4),
            }
        );
        assert_eq!(
            capture0.query,
            Query {
                source_info: SourceInfo::init(1, 6, 1, 23),
                value: QueryValue::Header {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::init(1, 12, 1, 13),
                    },
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "Location".to_string(),
                            encoded: "Location".to_string(),
                        }],
                        source_info: SourceInfo::init(1, 13, 1, 23),
                    },
                },
                subquery: None,
            }
        );
    }

    #[test]
    fn test_capture_with_subquery() {
        let mut reader = Reader::init("token: header \"Location\" regex \"token=(.*)\"");
        let capture0 = capture(&mut reader).unwrap();

        assert_eq!(
            capture0.query,
            Query {
                source_info: SourceInfo::init(1, 8, 1, 25),
                value: QueryValue::Header {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::init(1, 14, 1, 15),
                    },
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "Location".to_string(),
                            encoded: "Location".to_string(),
                        }],
                        source_info: SourceInfo::init(1, 15, 1, 25),
                    },
                },

                subquery: Some((
                    Whitespace {
                        value: " ".to_string(),
                        source_info: SourceInfo::init(1, 25, 1, 26),
                    },
                    Subquery {
                        source_info: SourceInfo::init(1, 26, 1, 44),
                        value: SubqueryValue::Regex {
                            space0: Whitespace {
                                value: " ".to_string(),
                                source_info: SourceInfo::init(1, 31, 1, 32),
                            },
                            value: RegexValue::Template(Template {
                                quotes: true,
                                elements: vec![TemplateElement::String {
                                    value: "token=(.*)".to_string(),
                                    encoded: "token=(.*)".to_string(),
                                }],
                                source_info: SourceInfo::init(1, 32, 1, 44),
                            }),
                        },
                    }
                )),
            }
        );
        assert_eq!(reader.state.cursor, 43);
    }

    #[test]
    fn test_capture_with_subquery_error() {
        let mut reader = Reader::init("token: header \"Location\" regex ");
        let error = capture(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 32
            }
        );
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "\" or /".to_string()
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::init("token: header \"Location\" xxx");
        let error = capture(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 26,
            }
        );
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "line_terminator".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_assert() {
        let mut reader = Reader::init("header \"Location\" equals \"https://google.fr\"");
        let assert0 = assert(&mut reader).unwrap();

        assert_eq!(
            assert0.query,
            Query {
                source_info: SourceInfo::init(1, 1, 1, 18),
                value: QueryValue::Header {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::init(1, 7, 1, 8),
                    },
                    name: Template {
                        quotes: true,
                        elements: vec![TemplateElement::String {
                            value: "Location".to_string(),
                            encoded: "Location".to_string(),
                        }],
                        source_info: SourceInfo::init(1, 8, 1, 18),
                    },
                },
                subquery: None,
            }
        );
    }

    #[test]
    fn test_assert_jsonpath() {
        let mut reader = Reader::init("jsonpath \"$.errors\" equals 5");

        assert_eq!(
            assert(&mut reader).unwrap().predicate,
            Predicate {
                not: false,
                space0: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::init(1, 21, 1, 21),
                },
                predicate_func: PredicateFunc {
                    source_info: SourceInfo::init(1, 21, 1, 29),
                    value: PredicateFuncValue::Equal {
                        space0: Whitespace {
                            value: String::from(" "),
                            source_info: SourceInfo::init(1, 27, 1, 28),
                        },
                        value: PredicateValue::Integer(5),
                        operator: false,
                    },
                },
            }
        );
    }
}
