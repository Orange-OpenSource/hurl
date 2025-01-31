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
use crate::ast::{CookieAttribute, CookieAttributeName, CookiePath};
use crate::combinator::optional;
use crate::parser::primitives::{literal, try_literal, zero_or_more_spaces};
use crate::parser::{string, ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

pub fn cookiepath(reader: &mut Reader) -> ParseResult<CookiePath> {
    let start = reader.cursor().pos;

    // We create a specialized reader for the templated, error and created structures are
    // relative tho the main reader.
    let s = reader.read_while(|c| c != '[');
    let mut template_reader = Reader::with_pos(s.as_str(), start);
    let name = string::unquoted_template(&mut template_reader)?;
    let attribute = optional(cookiepath_attribute, reader)?;
    Ok(CookiePath { name, attribute })
}

fn cookiepath_attribute(reader: &mut Reader) -> ParseResult<CookieAttribute> {
    try_literal("[", reader)?;
    let space0 = zero_or_more_spaces(reader)?;
    let name = cookiepath_attribute_name(reader)?;
    let space1 = zero_or_more_spaces(reader)?;
    literal("]", reader)?;
    Ok(CookieAttribute {
        space0,
        name,
        space1,
    })
}

fn cookiepath_attribute_name(reader: &mut Reader) -> ParseResult<CookieAttributeName> {
    let start = reader.cursor().pos;
    let s = reader.read_while(|c| c.is_alphabetic() || c == '-');
    match s.to_lowercase().as_str() {
        "value" => Ok(CookieAttributeName::Value(s)),
        "expires" => Ok(CookieAttributeName::Expires(s)),
        "max-age" => Ok(CookieAttributeName::MaxAge(s)),
        "domain" => Ok(CookieAttributeName::Domain(s)),
        "path" => Ok(CookieAttributeName::Path(s)),
        "secure" => Ok(CookieAttributeName::Secure(s)),
        "httponly" => Ok(CookieAttributeName::HttpOnly(s)),
        "samesite" => Ok(CookieAttributeName::SameSite(s)),
        _ => Err(ParseError::new(
            start,
            false,
            ParseErrorKind::InvalidCookieAttribute,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        Expr, ExprKind, Placeholder, SourceInfo, Template, TemplateElement, Variable, Whitespace,
    };
    use crate::reader::Pos;
    use crate::typing::ToSource;

    #[test]
    fn cookiepath_simple() {
        let mut reader = Reader::new("cookie1");
        assert_eq!(
            cookiepath(&mut reader).unwrap(),
            CookiePath {
                name: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "cookie1".to_string(),
                        source: "cookie1".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 8)),
                },
                attribute: None,
            }
        );
        assert_eq!(reader.cursor().index, 7);
    }

    #[test]
    fn cookiepath_with_attribute() {
        let mut reader = Reader::new("cookie1[Domain]");
        assert_eq!(
            cookiepath(&mut reader).unwrap(),
            CookiePath {
                name: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "cookie1".to_string(),
                        source: "cookie1".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 8)),
                },
                attribute: Some(CookieAttribute {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 9)),
                    },
                    name: CookieAttributeName::Domain("Domain".to_string()),
                    space1: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 15), Pos::new(1, 15)),
                    },
                }),
            }
        );
        assert_eq!(reader.cursor().index, 15);
    }

    #[test]
    fn cookiepath_with_template() {
        let mut reader = Reader::new("{{name}}[Domain]");
        assert_eq!(
            cookiepath(&mut reader).unwrap(),
            CookiePath {
                name: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::Placeholder(Placeholder {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 3)),
                        },
                        expr: Expr {
                            kind: ExprKind::Variable(Variable {
                                name: "name".to_string(),
                                source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 7)),
                            }),
                            source_info: SourceInfo::new(Pos::new(1, 3), Pos::new(1, 7)),
                        },
                        space1: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 7)),
                        },
                    })],
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 9)),
                },
                attribute: Some(CookieAttribute {
                    space0: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(1, 10)),
                    },
                    name: CookieAttributeName::Domain("Domain".to_string()),
                    space1: Whitespace {
                        value: String::new(),
                        source_info: SourceInfo::new(Pos::new(1, 16), Pos::new(1, 16)),
                    },
                }),
            }
        );
        assert_eq!(reader.cursor().index, 16);
    }

    #[test]
    fn cookiepath_error() {
        let mut reader = Reader::new("cookie1[");
        let error = cookiepath(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 9 });
        assert_eq!(error.kind, ParseErrorKind::InvalidCookieAttribute);
        assert!(!error.recoverable);

        let mut reader = Reader::new("cookie1[{{field]");
        let error = cookiepath(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 9 });
        assert_eq!(error.kind, ParseErrorKind::InvalidCookieAttribute);
        assert!(!error.recoverable);

        // Check that errors are well reported with a buffer that have already read data.
        let mut reader = Reader::new("xxxx{{cookie[Domain]");
        _ = reader.read_while(|c| c == 'x');

        let error = cookiepath(&mut reader).err().unwrap();
        assert_eq!(
            error.pos,
            Pos {
                line: 1,
                column: 13
            }
        );
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "}}".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_cookiepath_attribute_name() {
        let mut reader = Reader::new("Domain");
        assert_eq!(
            cookiepath_attribute_name(&mut reader).unwrap(),
            CookieAttributeName::Domain("Domain".to_string())
        );
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("domain");
        assert_eq!(
            cookiepath_attribute_name(&mut reader).unwrap(),
            CookieAttributeName::Domain("domain".to_string())
        );
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("unknown");
        let error = cookiepath_attribute_name(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.kind, ParseErrorKind::InvalidCookieAttribute);
    }
}
