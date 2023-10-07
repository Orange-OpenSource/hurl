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
use crate::parser::filter::filters;
use crate::parser::predicate::predicate;
use crate::parser::primitives::*;
use crate::parser::query::query;
use crate::parser::reader::Reader;
use crate::parser::string::*;
use crate::parser::{filename, key_string, ParseResult};

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
        "Options" => section_value_options(reader)?,
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

fn response_section(reader: &mut Reader) -> ParseResult<Section> {
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

fn section_name(reader: &mut Reader) -> ParseResult<String> {
    let pos = reader.state.pos.clone();
    try_literal("[", reader)?;
    let name = reader.read_while(|c| c.is_alphanumeric());
    if name.is_empty() {
        // Could be the empty json array for the body
        return Err(Error {
            pos,
            recoverable: true,
            inner: ParseError::Expecting {
                value: "a valid section name".to_string(),
            },
        });
    }
    try_literal("]", reader)?;
    Ok(name)
}

fn section_value_query_params(reader: &mut Reader) -> ParseResult<SectionValue> {
    let items = zero_or_more(key_value, reader)?;
    Ok(SectionValue::QueryParams(items))
}

fn section_value_basic_auth(reader: &mut Reader) -> ParseResult<SectionValue> {
    let v = optional(key_value, reader)?;
    Ok(SectionValue::BasicAuth(v))
}

fn section_value_form_params(reader: &mut Reader) -> ParseResult<SectionValue> {
    let items = zero_or_more(key_value, reader)?;
    Ok(SectionValue::FormParams(items))
}

fn section_value_multipart_form_data(reader: &mut Reader) -> ParseResult<SectionValue> {
    let items = zero_or_more(multipart_param, reader)?;
    Ok(SectionValue::MultipartFormData(items))
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
    let options = zero_or_more(option, reader)?;
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

fn file_param(reader: &mut Reader) -> ParseResult<FileParam> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let key = recover(key_string::parse, reader)?;
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

fn file_value(reader: &mut Reader) -> ParseResult<FileValue> {
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
                value: String::new(),
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

fn file_content_type(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let mut buf = String::new();
    let mut spaces = String::new();
    let mut save = reader.state.clone();
    while let Some(c) = reader.read() {
        if c.is_alphanumeric() || c == '/' || c == ';' || c == '=' || c == '-' {
            buf.push_str(spaces.as_str());
            spaces = String::new();
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
            inner: ParseError::FileContentType,
        });
    }
    Ok(buf)
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
    let line_terminator0 = line_terminator(reader)?;
    Ok(Capture {
        line_terminators,
        space0,
        name,
        space1,
        space2,
        query: q,
        filters,
        line_terminator0,
    })
}

fn assert(reader: &mut Reader) -> ParseResult<Assert> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let query0 = query(reader)?;
    let filters = filters(reader)?;
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
        filters,
        space1,
        predicate: predicate0,
        line_terminator0,
    })
}

fn option(reader: &mut Reader) -> ParseResult<EntryOption> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let pos = reader.state.pos.clone();
    let option = reader.read_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.');
    let space1 = zero_or_more_spaces(reader)?;
    try_literal(":", reader)?;
    let space2 = zero_or_more_spaces(reader)?;
    let kind = match option.as_str() {
        "aws-sigv4" => option_aws_sigv4(reader)?,
        "cacert" => option_cacert(reader)?,
        "cert" => option_cert(reader)?,
        "compressed" => option_compressed(reader)?,
        "connect-to" => option_connect_to(reader)?,
        "delay" => option_delay(reader)?,
        "insecure" => option_insecure(reader)?,
        "http1.0" => option_http_10(reader)?,
        "http1.1" => option_http_11(reader)?,
        "http2" => option_http_2(reader)?,
        "key" => option_key(reader)?,
        "location" => option_follow_location(reader)?,
        "max-redirs" => option_max_redirect(reader)?,
        "path-as-is" => option_path_as_is(reader)?,
        "proxy" => option_proxy(reader)?,
        "resolve" => option_resolve(reader)?,
        "retry" => option_retry(reader)?,
        "retry-interval" => option_retry_interval(reader)?,
        "variable" => option_variable(reader)?,
        "verbose" => option_verbose(reader)?,
        "very-verbose" => option_very_verbose(reader)?,
        _ => {
            return Err(Error {
                pos,
                recoverable: true,
                inner: ParseError::InvalidOption,
            });
        }
    };
    let line_terminator0 = line_terminator(reader)?;

    Ok(EntryOption {
        line_terminators,
        space0,
        space1,
        space2,
        kind,
        line_terminator0,
    })
}

fn option_aws_sigv4(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = aws_sigv4(reader)?;
    Ok(OptionKind::AwsSigV4(value))
}

fn option_cacert(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::CaCertificate(value))
}

fn option_cert(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::ClientCert(value))
}

fn option_compressed(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Compressed(value))
}

fn option_connect_to(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = connect_to(reader)?;
    Ok(OptionKind::ConnectTo(value))
}

fn option_delay(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(natural, reader)?;
    Ok(OptionKind::Delay(value))
}

fn option_follow_location(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::FollowLocation(value))
}

fn option_http_10(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Http10(value))
}

fn option_http_11(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Http11(value))
}

fn option_http_2(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Http2(value))
}

fn option_insecure(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Insecure(value))
}

fn option_key(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::ClientKey(value))
}

fn option_max_redirect(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(natural, reader)?;
    // FIXME: try to not unwrap redirect value
    // and returns an error if not possible
    let value = usize::try_from(value).unwrap();
    Ok(OptionKind::MaxRedirect(value))
}

fn option_path_as_is(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::PathAsIs(value))
}

fn option_proxy(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = proxy(reader)?;
    Ok(OptionKind::Proxy(value))
}

fn option_resolve(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = resolve(reader)?;
    Ok(OptionKind::Resolve(value))
}

fn option_retry(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = retry(reader)?;
    Ok(OptionKind::Retry(value))
}

fn option_retry_interval(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(natural, reader)?;
    Ok(OptionKind::RetryInterval(value))
}

fn option_variable(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = variable_definition(reader)?;
    Ok(OptionKind::Variable(value))
}

fn option_verbose(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::Verbose(value))
}

fn option_very_verbose(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean, reader)?;
    Ok(OptionKind::VeryVerbose(value))
}

fn aws_sigv4(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let provider = reader.read_while(|c| c.is_alphanumeric() || *c == ':' || *c == '-');
    if provider.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "aws-sigv4 provider".to_string(),
            },
        });
    }
    Ok(provider)
}

fn proxy(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let name = reader
        .read_while(|c| c.is_alphanumeric() || *c == ':' || *c == '.' || *c == '[' || *c == ']');
    if name.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "proxy name".to_string(),
            },
        });
    }
    Ok(name)
}

fn resolve(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let name = reader.read_while(|c| c.is_alphanumeric() || *c == ':' || *c == '.');
    if name.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "resolve".to_string(),
            },
        });
    }
    if !name.contains(':') {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "HOST:PORT:ADDR".to_string(),
            },
        });
    }
    Ok(name)
}

fn connect_to(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let name = reader.read_while(|c| c.is_alphanumeric() || *c == ':' || *c == '.');
    if name.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "connect-to".to_string(),
            },
        });
    }
    if !name.contains(':') {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "HOST1:PORT1:HOST2:PORT2".to_string(),
            },
        });
    }
    Ok(name)
}

fn retry(reader: &mut Reader) -> ParseResult<Retry> {
    let pos = reader.state.pos.clone();
    let value = nonrecover(integer, reader)?;
    if value == -1 {
        Ok(Retry::Infinite)
    } else if value == 0 {
        Ok(Retry::None)
    } else if value > 0 {
        Ok(Retry::Finite(value as usize))
    } else {
        Err(Error {
            pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "Expecting a retry value".to_string(),
            },
        })
    }
}
fn variable_definition(reader: &mut Reader) -> ParseResult<VariableDefinition> {
    let name = variable_name(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    literal("=", reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    let value = variable_value(reader)?;
    Ok(VariableDefinition {
        name,
        space0,
        space1,
        value,
    })
}

fn variable_name(reader: &mut Reader) -> ParseResult<String> {
    let start = reader.state.clone();
    let name = reader.read_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-');
    if name.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Expecting {
                value: "variable name".to_string(),
            },
        });
    }
    Ok(name)
}

fn variable_value(reader: &mut Reader) -> ParseResult<VariableValue> {
    choice(
        &[
            |p1| match null(p1) {
                Ok(()) => Ok(VariableValue::Null),
                Err(e) => Err(e),
            },
            |p1| match boolean(p1) {
                Ok(value) => Ok(VariableValue::Bool(value)),
                Err(e) => Err(e),
            },
            |p1| match float(p1) {
                Ok(value) => Ok(VariableValue::Float(value)),
                Err(e) => Err(e),
            },
            |p1| match integer(p1) {
                Ok(value) => Ok(VariableValue::Integer(value)),
                Err(e) => Err(e),
            },
            |p1| match quoted_template(p1) {
                Ok(value) => Ok(VariableValue::String(value)),
                Err(e) => Err(e),
            },
            |p1| match unquoted_template(p1) {
                Ok(value) => Ok(VariableValue::String(value)),
                Err(e) => Err(e),
            },
        ],
        reader,
    )
    .map_err(|e| Error {
        pos: e.pos,
        recoverable: false,
        inner: ParseError::Expecting {
            value: "variable value".to_string(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;

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
                    source_info: SourceInfo::new(1, 1, 1, 1),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(1, 10, 1, 10),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::from("\n"),
                        source_info: SourceInfo::new(1, 10, 2, 1),
                    },
                },
                value: SectionValue::Asserts(vec![Assert {
                    line_terminators: vec![],
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(2, 1, 2, 1),
                    },
                    query: Query {
                        source_info: SourceInfo::new(2, 1, 2, 18),
                        value: QueryValue::Header {
                            space0: Whitespace {
                                value: String::from(" "),
                                source_info: SourceInfo::new(2, 7, 2, 8),
                            },
                            name: Template {
                                delimiter: Some('"'),
                                elements: vec![TemplateElement::String {
                                    value: "Location".to_string(),
                                    encoded: "Location".to_string(),
                                }],
                                source_info: SourceInfo::new(2, 8, 2, 18),
                            },
                        },
                    },
                    filters: vec![],
                    space1: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::new(2, 18, 2, 19),
                    },
                    predicate: Predicate {
                        not: false,
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(2, 19, 2, 19),
                        },
                        predicate_func: PredicateFunc {
                            source_info: SourceInfo::new(2, 19, 2, 41),
                            value: PredicateFuncValue::Equal {
                                space0: Whitespace {
                                    value: String::from(" "),
                                    source_info: SourceInfo::new(2, 21, 2, 22),
                                },
                                value: PredicateValue::String(Template {
                                    delimiter: Some('"'),
                                    elements: vec![TemplateElement::String {
                                        value: "https://google.fr".to_string(),
                                        encoded: "https://google.fr".to_string(),
                                    }],
                                    source_info: SourceInfo::new(2, 22, 2, 41),
                                }),
                                operator: true,
                            },
                        },
                    },
                    line_terminator0: LineTerminator {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(2, 41, 2, 41),
                        },
                        comment: None,
                        newline: Whitespace {
                            value: String::from("\n"),
                            source_info: SourceInfo::new(2, 41, 3, 1),
                        },
                    },
                }]),
                source_info: SourceInfo::new(1, 1, 1, 10),
            }
        );
    }

    #[test]
    fn test_asserts_section_error() {
        let mut reader = Reader::new("x[Assertsx]\nheader Location == \"https://google.fr\"\n");
        let error = response_section(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("[")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("[Assertsx]\nheader Location == \"https://google.fr\"\n");
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
        let mut reader = Reader::new("Foo: Bar");
        let c = cookie(&mut reader).unwrap();
        assert_eq!(c.name.to_string(), String::from("Foo"));
        assert_eq!(
            c.value,
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "Bar".to_string(),
                    encoded: "Bar".to_string(),
                }],
                source_info: SourceInfo::new(1, 6, 1, 9),
            }
        );
    }

    #[test]
    fn test_option_insecure() {
        let mut reader = Reader::new("insecure: true");
        let option = option(&mut reader).unwrap();
        assert_eq!(
            option,
            EntryOption {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 1 },
                        end: Pos { line: 1, column: 1 },
                    },
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 9 },
                        end: Pos { line: 1, column: 9 },
                    },
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo {
                        start: Pos {
                            line: 1,
                            column: 10,
                        },
                        end: Pos {
                            line: 1,
                            column: 11,
                        },
                    },
                },
                kind: OptionKind::Insecure(true),
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo {
                            start: Pos {
                                line: 1,
                                column: 15,
                            },
                            end: Pos {
                                line: 1,
                                column: 15,
                            },
                        },
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo {
                            start: Pos {
                                line: 1,
                                column: 15,
                            },
                            end: Pos {
                                line: 1,
                                column: 15,
                            },
                        },
                    },
                },
            }
        );
    }

    #[test]
    fn test_option_insecure_error() {
        let mut reader = Reader::new("insecure: error");
        let error = option(&mut reader).err().unwrap();
        assert!(!error.recoverable)
    }

    #[test]
    fn test_option_cacert() {
        let mut reader = Reader::new("cacert: /home/foo/cert.pem");
        let option = option(&mut reader).unwrap();
        assert_eq!(
            option,
            EntryOption {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 1 },
                        end: Pos { line: 1, column: 1 },
                    },
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 7 },
                        end: Pos { line: 1, column: 7 },
                    },
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 8 },
                        end: Pos { line: 1, column: 9 },
                    },
                },
                kind: OptionKind::CaCertificate(Filename {
                    value: "/home/foo/cert.pem".to_string(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 9 },
                        end: Pos {
                            line: 1,
                            column: 27,
                        },
                    },
                }),
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo {
                            start: Pos {
                                line: 1,
                                column: 27,
                            },
                            end: Pos {
                                line: 1,
                                column: 27,
                            },
                        },
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo {
                            start: Pos {
                                line: 1,
                                column: 27,
                            },
                            end: Pos {
                                line: 1,
                                column: 27,
                            },
                        },
                    },
                },
            }
        );
    }

    #[test]
    fn test_option_cacert_error() {
        let mut reader = Reader::new("cacert: ###");
        let error = option(&mut reader).err().unwrap();
        assert!(!error.recoverable)
    }

    #[test]
    fn test_variable_definition() {
        let mut reader = Reader::new("a=1");
        assert_eq!(
            variable_definition(&mut reader).unwrap(),
            VariableDefinition {
                name: "a".to_string(),
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 2 },
                        end: Pos { line: 1, column: 2 },
                    },
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo {
                        start: Pos { line: 1, column: 3 },
                        end: Pos { line: 1, column: 3 },
                    },
                },
                value: VariableValue::Integer(1),
            }
        );
    }

    #[test]
    fn test_variable_value() {
        let mut reader = Reader::new("null");
        assert_eq!(variable_value(&mut reader).unwrap(), VariableValue::Null);

        let mut reader = Reader::new("true");
        assert_eq!(
            variable_value(&mut reader).unwrap(),
            VariableValue::Bool(true)
        );

        let mut reader = Reader::new("1");
        assert_eq!(
            variable_value(&mut reader).unwrap(),
            VariableValue::Integer(1)
        );

        let mut reader = Reader::new("toto");
        assert_eq!(
            variable_value(&mut reader).unwrap(),
            VariableValue::String(Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "toto".to_string(),
                    encoded: "toto".to_string(),
                }],
                source_info: SourceInfo {
                    start: Pos { line: 1, column: 1 },
                    end: Pos { line: 1, column: 5 },
                },
            })
        );
        let mut reader = Reader::new("\"123\"");
        assert_eq!(
            variable_value(&mut reader).unwrap(),
            VariableValue::String(Template {
                delimiter: Some('"'),
                elements: vec![TemplateElement::String {
                    value: "123".to_string(),
                    encoded: "123".to_string(),
                }],
                source_info: SourceInfo {
                    start: Pos { line: 1, column: 1 },
                    end: Pos { line: 1, column: 6 },
                },
            })
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
            error.inner,
            ParseError::Expecting {
                value: "}}".to_string()
            }
        );
    }

    #[test]
    fn test_file_value() {
        let mut reader = Reader::new("file,hello.txt;");
        assert_eq!(
            file_value(&mut reader).unwrap(),
            FileValue {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(1, 6, 1, 6),
                },
                filename: Filename {
                    value: "hello.txt".to_string(),
                    source_info: SourceInfo::new(1, 6, 1, 15),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(1, 15, 1, 15),
                },
                space2: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(1, 16, 1, 16),
                },
                content_type: None,
            }
        );
        let mut reader = Reader::new("file,hello.txt; text/html");
        assert_eq!(
            file_value(&mut reader).unwrap(),
            FileValue {
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(1, 6, 1, 6),
                },
                filename: Filename {
                    value: "hello.txt".to_string(),
                    source_info: SourceInfo::new(1, 6, 1, 15),
                },
                space1: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(1, 15, 1, 15),
                },
                space2: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(1, 16, 1, 17),
                },
                content_type: Some("text/html".to_string()),
            }
        );
    }

    #[test]
    fn test_file_content_type() {
        let mut reader = Reader::new("text/html");
        assert_eq!(
            file_content_type(&mut reader).unwrap(),
            "text/html".to_string()
        );
        assert_eq!(reader.state.cursor, 9);

        let mut reader = Reader::new("text/plain; charset=us-ascii");
        assert_eq!(
            file_content_type(&mut reader).unwrap(),
            "text/plain; charset=us-ascii".to_string()
        );
        assert_eq!(reader.state.cursor, 28);

        let mut reader = Reader::new("text/html # comment");
        assert_eq!(
            file_content_type(&mut reader).unwrap(),
            "text/html".to_string()
        );
        assert_eq!(reader.state.cursor, 9);
    }

    #[test]
    fn test_capture() {
        let mut reader = Reader::new("url: header \"Location\"");
        let capture0 = capture(&mut reader).unwrap();

        assert_eq!(
            capture0.name,
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "url".to_string(),
                    encoded: "url".to_string(),
                }],
                source_info: SourceInfo::new(1, 1, 1, 4),
            },
        );
        assert_eq!(
            capture0.query,
            Query {
                source_info: SourceInfo::new(1, 6, 1, 23),
                value: QueryValue::Header {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::new(1, 12, 1, 13),
                    },
                    name: Template {
                        delimiter: Some('"'),
                        elements: vec![TemplateElement::String {
                            value: "Location".to_string(),
                            encoded: "Location".to_string(),
                        }],
                        source_info: SourceInfo::new(1, 13, 1, 23),
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
                source_info: SourceInfo::new(1, 8, 1, 25),
                value: QueryValue::Header {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::new(1, 14, 1, 15),
                    },
                    name: Template {
                        delimiter: Some('"'),
                        elements: vec![TemplateElement::String {
                            value: "Location".to_string(),
                            encoded: "Location".to_string(),
                        }],
                        source_info: SourceInfo::new(1, 15, 1, 25),
                    },
                },
            }
        );
        assert_eq!(reader.state.cursor, 43);
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
            error.inner,
            ParseError::Expecting {
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
            error.inner,
            ParseError::Expecting {
                value: "line_terminator".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_assert() {
        let mut reader = Reader::new("header \"Location\" == \"https://google.fr\"");
        let assert0 = assert(&mut reader).unwrap();

        assert_eq!(
            assert0.query,
            Query {
                source_info: SourceInfo::new(1, 1, 1, 18),
                value: QueryValue::Header {
                    space0: Whitespace {
                        value: String::from(" "),
                        source_info: SourceInfo::new(1, 7, 1, 8),
                    },
                    name: Template {
                        delimiter: Some('"'),
                        elements: vec![TemplateElement::String {
                            value: "Location".to_string(),
                            encoded: "Location".to_string(),
                        }],
                        source_info: SourceInfo::new(1, 8, 1, 18),
                    },
                },
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
                    source_info: SourceInfo::new(1, 21, 1, 21),
                },
                predicate_func: PredicateFunc {
                    source_info: SourceInfo::new(1, 21, 1, 25),
                    value: PredicateFuncValue::Equal {
                        space0: Whitespace {
                            value: String::from(" "),
                            source_info: SourceInfo::new(1, 23, 1, 24),
                        },
                        value: PredicateValue::Integer(5),
                        operator: true,
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
                    source_info: SourceInfo::new(1, 1, 1, 1),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(1, 12, 1, 12),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::from("\n"),
                        source_info: SourceInfo::new(1, 12, 2, 1),
                    },
                },
                value: SectionValue::BasicAuth(Some(KeyValue {
                    line_terminators: vec![],
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(2, 1, 2, 1)
                    },
                    key: Template {
                        delimiter: None,
                        elements: vec![TemplateElement::String {
                            value: "user".to_string(),
                            encoded: "user".to_string()
                        }],
                        source_info: SourceInfo::new(2, 1, 2, 5),
                    },
                    space1: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(2, 5, 2, 5)
                    },
                    space2: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(2, 6, 2, 6)
                    },
                    value: Template {
                        delimiter: None,
                        elements: vec![TemplateElement::String {
                            value: "password".to_string(),
                            encoded: "password".to_string()
                        }],
                        source_info: SourceInfo::new(2, 6, 2, 14),
                    },
                    line_terminator0: LineTerminator {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(2, 14, 2, 14)
                        },
                        comment: None,
                        newline: Whitespace {
                            value: "\n".to_string(),
                            source_info: SourceInfo::new(2, 14, 3, 1)
                        },
                    },
                })),
                source_info: SourceInfo::new(1, 1, 1, 12),
            }
        );
        assert_eq!(reader.state.pos, Pos { line: 3, column: 1 });

        let mut reader = Reader::new("[BasicAuth]\nHTTP 200\n");
        assert_eq!(
            request_section(&mut reader).unwrap(),
            Section {
                line_terminators: vec![],
                space0: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(1, 1, 1, 1),
                },
                line_terminator0: LineTerminator {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(1, 12, 1, 12),
                    },
                    comment: None,
                    newline: Whitespace {
                        value: String::from("\n"),
                        source_info: SourceInfo::new(1, 12, 2, 1),
                    },
                },
                value: SectionValue::BasicAuth(None),
                source_info: SourceInfo::new(1, 1, 1, 12),
            }
        );
        assert_eq!(reader.state.pos, Pos { line: 2, column: 1 });
    }
}
