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
use crate::parser::combinators::*;
use crate::parser::error::*;
use crate::parser::number::{integer, natural, number};
use crate::parser::primitives::*;
use crate::parser::reader::Reader;
use crate::parser::string::*;
use crate::parser::{expr, filename, filename_password, ParseResult};
use crate::typing::Retry;

/// Parse an option in an `[Options]` section.
pub fn parse(reader: &mut Reader) -> ParseResult<EntryOption> {
    let line_terminators = optional_line_terminators(reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let start = reader.state.pos;
    // We accept '_' even if there is no option name with this character. We do this to be able to
    // enter the parsing of the option name and to have better error description (ex: 'max-redirs'
    // vs 'max_redirs').
    let option =
        reader.read_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.' || *c == '_');
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
        "http3" => option_http_3(reader)?,
        "ipv4" => option_ipv4(reader)?,
        "ipv6" => option_ipv6(reader)?,
        "key" => option_key(reader)?,
        "location" => option_follow_location(reader)?,
        "location-trusted" => option_follow_location_trusted(reader)?,
        "max-redirs" => option_max_redirect(reader)?,
        "netrc" => option_netrc(reader)?,
        "netrc-file" => option_netrc_file(reader)?,
        "netrc-optional" => option_netrc_optional(reader)?,
        "output" => option_output(reader)?,
        "path-as-is" => option_path_as_is(reader)?,
        "proxy" => option_proxy(reader)?,
        "resolve" => option_resolve(reader)?,
        "retry" => option_retry(reader)?,
        "retry-interval" => option_retry_interval(reader)?,
        "skip" => option_skip(reader)?,
        "unix-socket" => option_unix_socket(reader)?,
        "user" => option_user(reader)?,
        "variable" => option_variable(reader)?,
        "verbose" => option_verbose(reader)?,
        "very-verbose" => option_very_verbose(reader)?,
        _ => {
            return Err(ParseError::new(
                start,
                false,
                ParseErrorKind::InvalidOption(option.to_string()),
            ))
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
    let value = unquoted_template(reader)?;
    Ok(OptionKind::AwsSigV4(value))
}

fn option_cacert(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::CaCertificate(value))
}

fn option_cert(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename_password::parse(reader)?;
    Ok(OptionKind::ClientCert(value))
}

fn option_compressed(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::Compressed(value))
}

fn option_connect_to(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = unquoted_template(reader)?;
    Ok(OptionKind::ConnectTo(value))
}

fn option_delay(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = natural_option(reader)?;
    Ok(OptionKind::Delay(value))
}

fn option_follow_location(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::FollowLocation(value))
}

fn option_follow_location_trusted(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::FollowLocationTrusted(value))
}

fn option_http_10(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::Http10(value))
}

fn option_http_11(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::Http11(value))
}

fn option_http_2(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::Http2(value))
}

fn option_http_3(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::Http3(value))
}

fn option_insecure(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::Insecure(value))
}

fn option_ipv4(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::IpV4(value))
}

fn option_ipv6(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::IpV6(value))
}

fn option_key(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::ClientKey(value))
}

fn option_max_redirect(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(natural_option, reader)?;
    Ok(OptionKind::MaxRedirect(value))
}

fn option_netrc(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::NetRc(value))
}

fn option_netrc_file(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = unquoted_template(reader)?;
    Ok(OptionKind::NetRcFile(value))
}

fn option_netrc_optional(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::NetRcOptional(value))
}

fn option_output(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = filename::parse(reader)?;
    Ok(OptionKind::Output(value))
}

fn option_path_as_is(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::PathAsIs(value))
}

fn option_proxy(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = unquoted_template(reader)?;
    Ok(OptionKind::Proxy(value))
}

fn option_resolve(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = unquoted_template(reader)?;
    Ok(OptionKind::Resolve(value))
}

fn option_retry(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = retry_option(reader)?;
    Ok(OptionKind::Retry(value))
}

fn option_retry_interval(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(natural_option, reader)?;
    Ok(OptionKind::RetryInterval(value))
}

fn option_skip(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::Skip(value))
}

fn option_user(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = unquoted_template(reader)?;
    Ok(OptionKind::User(value))
}

fn option_unix_socket(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = unquoted_template(reader)?;
    Ok(OptionKind::UnixSocket(value))
}

fn option_variable(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = variable_definition(reader)?;
    Ok(OptionKind::Variable(value))
}

fn option_verbose(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::Verbose(value))
}

fn option_very_verbose(reader: &mut Reader) -> ParseResult<OptionKind> {
    let value = nonrecover(boolean_option, reader)?;
    Ok(OptionKind::VeryVerbose(value))
}

fn retry(reader: &mut Reader) -> ParseResult<Retry> {
    let pos = reader.state.pos;
    let value = nonrecover(integer, reader)?;
    if value == -1 {
        Ok(Retry::Infinite)
    } else if value >= 0 {
        Ok(Retry::Finite(value as usize))
    } else {
        let kind = ParseErrorKind::Expecting {
            value: "Expecting a retry value".to_string(),
        };
        Err(ParseError::new(pos, false, kind))
    }
}

fn boolean_option(reader: &mut Reader) -> ParseResult<BooleanOption> {
    let start = reader.state;
    match boolean(reader) {
        Ok(v) => Ok(BooleanOption::Literal(v)),
        Err(_) => {
            reader.state = start;
            let exp = expr::parse(reader).map_err(|e| {
                let kind = ParseErrorKind::Expecting {
                    value: "true|false".to_string(),
                };
                ParseError::new(e.pos, false, kind)
            })?;
            Ok(BooleanOption::Expression(exp))
        }
    }
}

fn natural_option(reader: &mut Reader) -> ParseResult<NaturalOption> {
    let start = reader.state;
    match natural(reader) {
        Ok(v) => Ok(NaturalOption::Literal(v)),
        Err(_) => {
            reader.state = start;
            let exp = expr::parse(reader).map_err(|e| {
                let kind = ParseErrorKind::Expecting {
                    value: "integer".to_string(),
                };
                ParseError::new(e.pos, false, kind)
            })?;
            Ok(NaturalOption::Expression(exp))
        }
    }
}

fn retry_option(reader: &mut Reader) -> ParseResult<RetryOption> {
    let start = reader.state;
    match retry(reader) {
        Ok(v) => Ok(RetryOption::Literal(v)),
        Err(_) => {
            reader.state = start;
            let exp = expr::parse(reader).map_err(|e| {
                let kind = ParseErrorKind::Expecting {
                    value: "integer".to_string(),
                };
                ParseError::new(e.pos, false, kind)
            })?;
            Ok(RetryOption::Expression(exp))
        }
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
    let start = reader.state;
    let name = reader.read_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-');
    if name.is_empty() {
        let kind = ParseErrorKind::Expecting {
            value: "variable name".to_string(),
        };
        return Err(ParseError::new(start.pos, false, kind));
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
            |p1| match number(p1) {
                Ok(value) => Ok(VariableValue::Number(value)),
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
    .map_err(|e| {
        let kind = ParseErrorKind::Expecting {
            value: "variable value".to_string(),
        };
        ParseError::new(e.pos, false, kind)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;

    #[test]
    fn test_option_insecure() {
        let mut reader = Reader::new("insecure: true");
        let option = parse(&mut reader).unwrap();
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
                kind: OptionKind::Insecure(BooleanOption::Literal(true)),
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
        let error = parse(&mut reader).err().unwrap();
        assert!(!error.recoverable);
    }

    #[test]
    fn test_option_cacert() {
        let mut reader = Reader::new("cacert: /home/foo/cert.pem");
        let option = parse(&mut reader).unwrap();
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
                kind: OptionKind::CaCertificate(Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "/home/foo/cert.pem".to_string(),
                        encoded: "/home/foo/cert.pem".to_string()
                    }],
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
        let error = parse(&mut reader).err().unwrap();
        assert!(!error.recoverable);
    }

    #[test]
    fn test_option_retry_error() {
        let mut reader = Reader::new("retry: ###");
        let error = parse(&mut reader).err().unwrap();
        assert!(!error.recoverable);
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "integer".to_string()
            }
        );
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
                value: VariableValue::Number(Number::Integer(1)),
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
            VariableValue::Number(Number::Integer(1))
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
}
