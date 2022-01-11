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
use super::primitives::*;
use super::reader::Reader;
use super::string::*;
use super::ParseResult;

/*

"cookiepath" expression
not described in the official grammar

cookie-query 	= "cookie" sp sp* """  cookiepath-expr  """
cookiepath-expr = cookie-query-name ( "[" sp* cookie-query-attribute sp* "]" )
cookiepath-name = <[a-zA-Z0-9{}]+>
cookiepath-attribute = Value | Expires | Max-Age | Domain | Path | Secure | HttpOnly | SameSite

*/

pub fn cookiepath(reader: &mut Reader) -> ParseResult<'static, CookiePath> {
    let start = reader.state.pos.clone();
    let s = reader.read_while(|c| *c != '[');
    let mut template_reader = Reader::init(s.as_str());
    template_reader.state.pos = start;
    let name = unquoted_template(&mut template_reader)?;
    let attribute = optional(cookiepath_attribute, reader)?;
    Ok(CookiePath { name, attribute })
}

fn cookiepath_attribute(reader: &mut Reader) -> ParseResult<'static, CookieAttribute> {
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

fn cookiepath_attribute_name(reader: &mut Reader) -> ParseResult<'static, CookieAttributeName> {
    let start = reader.state.pos.clone();
    let s = reader.read_while(|c| c.is_alphabetic() || *c == '-');
    match s.to_lowercase().as_str() {
        "value" => Ok(CookieAttributeName::Value(s)),
        "expires" => Ok(CookieAttributeName::Expires(s)),
        "max-age" => Ok(CookieAttributeName::MaxAge(s)),
        "domain" => Ok(CookieAttributeName::Domain(s)),
        "path" => Ok(CookieAttributeName::Path(s)),
        "secure" => Ok(CookieAttributeName::Secure(s)),
        "httponly" => Ok(CookieAttributeName::HttpOnly(s)),
        "samesite" => Ok(CookieAttributeName::SameSite(s)),
        _ => Err(Error {
            pos: start,
            recoverable: false,
            inner: ParseError::InvalidCookieAttribute {},
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Pos, SourceInfo};

    use super::*;

    #[test]
    fn test_cookiepath_simple() {
        let mut reader = Reader::init("cookie1");
        assert_eq!(
            cookiepath(&mut reader).unwrap(),
            CookiePath {
                name: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "cookie1".to_string(),
                        encoded: "cookie1".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 1, 1, 8),
                },
                attribute: None,
            }
        );
        assert_eq!(reader.state.cursor, 7);
    }

    #[test]
    fn test_cookiepath_with_attribute() {
        let mut reader = Reader::init("cookie1[Domain]");
        assert_eq!(
            cookiepath(&mut reader).unwrap(),
            CookiePath {
                name: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "cookie1".to_string(),
                        encoded: "cookie1".to_string(),
                    }],
                    source_info: SourceInfo::init(1, 1, 1, 8),
                },
                attribute: Some(CookieAttribute {
                    space0: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 9, 1, 9),
                    },
                    name: CookieAttributeName::Domain("Domain".to_string()),
                    space1: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 15, 1, 15),
                    },
                }),
            }
        );
        assert_eq!(reader.state.cursor, 15);
    }

    #[test]
    fn test_cookiepath_with_template() {
        let mut reader = Reader::init("{{name}}[Domain]");
        assert_eq!(
            cookiepath(&mut reader).unwrap(),
            CookiePath {
                name: Template {
                    quotes: false,
                    elements: vec![TemplateElement::Expression(Expr {
                        space0: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 3, 1, 3),
                        },
                        variable: Variable {
                            name: "name".to_string(),
                            source_info: SourceInfo::init(1, 3, 1, 7),
                        },
                        space1: Whitespace {
                            value: String::from(""),
                            source_info: SourceInfo::init(1, 7, 1, 7),
                        },
                    })],
                    source_info: SourceInfo::init(1, 1, 1, 9),
                },
                attribute: Some(CookieAttribute {
                    space0: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 10, 1, 10),
                    },
                    name: CookieAttributeName::Domain("Domain".to_string()),
                    space1: Whitespace {
                        value: String::from(""),
                        source_info: SourceInfo::init(1, 16, 1, 16),
                    },
                }),
            }
        );
        assert_eq!(reader.state.cursor, 16);
    }

    #[test]
    fn test_cookiepath_error() {
        let mut reader = Reader::init("cookie1[");
        let error = cookiepath(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 9 });
        assert_eq!(error.inner, ParseError::InvalidCookieAttribute {});
        assert!(!error.recoverable);

        let mut reader = Reader::init("cookie1[{{field]");
        let error = cookiepath(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 9 });
        assert_eq!(error.inner, ParseError::InvalidCookieAttribute {});
        assert!(!error.recoverable);
    }

    #[test]
    fn test_cookie_attribute_name() {
        let mut reader = Reader::init("Domain");
        assert_eq!(
            cookiepath_attribute_name(&mut reader).unwrap(),
            CookieAttributeName::Domain("Domain".to_string())
        );
        assert_eq!(reader.state.cursor, 6);

        let mut reader = Reader::init("domain");
        assert_eq!(
            cookiepath_attribute_name(&mut reader).unwrap(),
            CookieAttributeName::Domain("domain".to_string())
        );
        assert_eq!(reader.state.cursor, 6);

        let mut reader = Reader::init("unknown");
        let error = cookiepath_attribute_name(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(error.inner, ParseError::InvalidCookieAttribute {});
    }
}
