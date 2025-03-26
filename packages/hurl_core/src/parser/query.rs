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
use crate::ast::{CertificateAttributeName, Query, QueryValue, RegexValue, SourceInfo};
use crate::combinator::{choice, ParseError as ParseErrorTrait};
use crate::parser::cookiepath::cookiepath;
use crate::parser::primitives::{literal, one_or_more_spaces, regex, try_literal};
use crate::parser::string::{quoted_oneline_string, quoted_template};
use crate::parser::{ParseError, ParseErrorKind, ParseResult};
use crate::reader::{Pos, Reader};

pub fn query(reader: &mut Reader) -> ParseResult<Query> {
    let start = reader.cursor();
    let value = query_value(reader)?;
    let end = reader.cursor();
    Ok(Query {
        source_info: SourceInfo::new(start.pos, end.pos),
        value,
    })
}

fn query_value(reader: &mut Reader) -> ParseResult<QueryValue> {
    choice(
        &[
            status_query,
            version_query,
            url_query,
            header_query,
            cookie_query,
            body_query,
            xpath_query,
            jsonpath_query,
            regex_query,
            variable_query,
            duration_query,
            bytes_query,
            sha256_query,
            md5_query,
            certificate_query,
            ip_query,
            redirects_query,
        ],
        reader,
    )
}

fn status_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("status", reader)?;
    Ok(QueryValue::Status)
}

fn version_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("version", reader)?;
    Ok(QueryValue::Version)
}

fn url_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("url", reader)?;
    Ok(QueryValue::Url)
}

fn header_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("header", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let name = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(QueryValue::Header { space0, name })
}

fn cookie_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("cookie", reader)?;
    let space0 = one_or_more_spaces(reader)?;

    // Read the whole value of the coookie path and parse it with a specialized reader.
    let start = reader.cursor();
    let s = quoted_oneline_string(reader)?;
    // todo should work with an encodedString in order to support escape sequence
    // or decode escape sequence with the cookiepath parser

    // We will parse the cookiepath value without `"`.
    let pos = Pos::new(start.pos.line, start.pos.column + 1);
    let mut cookiepath_reader = Reader::with_pos(s.as_str(), pos);
    let expr = cookiepath(&mut cookiepath_reader)?;

    Ok(QueryValue::Cookie { space0, expr })
}

fn body_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("body", reader)?;
    Ok(QueryValue::Body)
}

fn xpath_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("xpath", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let expr = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(QueryValue::Xpath { space0, expr })
}

fn jsonpath_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("jsonpath", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    //let expr = jsonpath_expr(reader)?;
    //  let start = reader.state.pos.clone();
    let expr = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    //    let end = reader.state.pos.clone();
    //    let expr = Template {
    //        elements: template.elements.iter().map(|e| match e {
    //            TemplateElement::String { value, encoded } => HurlTemplateElement::Literal {
    //                value: HurlString2 { value: value.clone(), encoded: Some(encoded.clone()) }
    //            },
    //            TemplateElement::Expression(value) => HurlTemplateElement::Expression { value: value.clone() }
    //        }).collect(),
    //        quotes: true,
    //        source_info: SourceInfo { start, end },
    //    };

    Ok(QueryValue::Jsonpath { space0, expr })
}

fn regex_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("regex", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let value = regex_value(reader)?;
    Ok(QueryValue::Regex { space0, value })
}

pub fn regex_value(reader: &mut Reader) -> ParseResult<RegexValue> {
    choice(
        &[
            |p1| match quoted_template(p1) {
                Ok(value) => Ok(RegexValue::Template(value)),
                Err(e) => Err(e),
            },
            |p1| match regex(p1) {
                Ok(value) => Ok(RegexValue::Regex(value)),
                Err(e) => Err(e),
            },
        ],
        reader,
    )
    .map_err(|e| {
        let kind = ParseErrorKind::Expecting {
            value: "\" or /".to_string(),
        };
        ParseError::new(e.pos, false, kind)
    })
}

fn variable_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("variable", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let name = quoted_template(reader).map_err(|e| e.to_non_recoverable())?;
    Ok(QueryValue::Variable { space0, name })
}

fn duration_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("duration", reader)?;
    Ok(QueryValue::Duration)
}

fn bytes_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("bytes", reader)?;
    Ok(QueryValue::Bytes)
}

fn sha256_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("sha256", reader)?;
    Ok(QueryValue::Sha256)
}

fn md5_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("md5", reader)?;
    Ok(QueryValue::Md5)
}

fn certificate_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("certificate", reader)?;
    let space0 = one_or_more_spaces(reader)?;
    let field = certificate_field(reader)?;
    Ok(QueryValue::Certificate {
        space0,
        attribute_name: field,
    })
}

fn ip_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("ip", reader)?;
    Ok(QueryValue::Ip)
}

fn redirects_query(reader: &mut Reader) -> ParseResult<QueryValue> {
    try_literal("redirects", reader)?;
    Ok(QueryValue::Redirects)
}

fn certificate_field(reader: &mut Reader) -> ParseResult<CertificateAttributeName> {
    literal("\"", reader)?;
    if try_literal(r#"Subject""#, reader).is_ok() {
        Ok(CertificateAttributeName::Subject)
    } else if try_literal(r#"Issuer""#, reader).is_ok() {
        Ok(CertificateAttributeName::Issuer)
    } else if try_literal(r#"Start-Date""#, reader).is_ok() {
        Ok(CertificateAttributeName::StartDate)
    } else if try_literal(r#"Expire-Date""#, reader).is_ok() {
        Ok(CertificateAttributeName::ExpireDate)
    } else if try_literal(r#"Serial-Number""#, reader).is_ok() {
        Ok(CertificateAttributeName::SerialNumber)
    } else {
        let value =
            "Field <Subject>, <Issuer>, <Start-Date>, <Expire-Date> or <Serial-Number>".to_string();
        let kind = ParseErrorKind::Expecting { value };
        let cur = reader.cursor();
        Err(ParseError::new(cur.pos, false, kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        CookieAttribute, CookieAttributeName, CookiePath, Filter, FilterValue, Template,
        TemplateElement, Whitespace,
    };
    use crate::parser::filter::filters;
    use crate::reader::Pos;
    use crate::typing::ToSource;

    #[test]
    fn test_query() {
        let mut reader = Reader::new("status");
        assert_eq!(
            query(&mut reader).unwrap(),
            Query {
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 7)),
                value: QueryValue::Status,
            }
        );
    }

    #[test]
    fn test_status_query() {
        let mut reader = Reader::new("status");
        assert_eq!(
            query(&mut reader).unwrap(),
            Query {
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 7)),
                value: QueryValue::Status,
            }
        );
    }

    #[test]
    fn test_header_query() {
        let mut reader = Reader::new("header \"Foo\"");
        assert_eq!(
            header_query(&mut reader).unwrap(),
            QueryValue::Header {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 8)),
                },
                name: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "Foo".to_string(),
                        source: "Foo".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 8), Pos::new(1, 13))
                )
            }
        );
    }

    #[test]
    fn test_cookie_query() {
        let mut reader = Reader::new("cookie \"Foo[Domain]\"");
        assert_eq!(
            cookie_query(&mut reader).unwrap(),
            QueryValue::Cookie {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 8)),
                },
                expr: CookiePath {
                    name: Template::new(
                        None,
                        vec![TemplateElement::String {
                            value: "Foo".to_string(),
                            source: "Foo".to_source(),
                        }],
                        SourceInfo::new(Pos::new(1, 9), Pos::new(1, 12))
                    ),
                    attribute: Some(CookieAttribute {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 13), Pos::new(1, 13)),
                        },
                        name: CookieAttributeName::Domain("Domain".to_string()),
                        space1: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 19), Pos::new(1, 19)),
                        },
                    }),
                },
            }
        );
        assert_eq!(reader.cursor().index, 20);

        // todo test with escape sequence
        //let mut reader = Reader::init("cookie \"cookie\u{31}\"");
    }

    #[test]
    fn test_xpath_query() {
        let mut reader = Reader::new("xpath \"normalize-space(//head/title)\"");
        assert_eq!(
            xpath_query(&mut reader).unwrap(),
            QueryValue::Xpath {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 7)),
                },
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "normalize-space(//head/title)".to_string(),
                        source: "normalize-space(//head/title)".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 7), Pos::new(1, 38))
                ),
            },
        );

        let mut reader = Reader::new("xpath \"normalize-space(//div[contains(concat(' ',normalize-space(@class),' '),' monthly-price ')])\"");
        assert_eq!(xpath_query(&mut reader).unwrap(), QueryValue::Xpath {
            space0: Whitespace { value: String::from(" "), source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 7)) },
            expr: Template::new(
                Some('"'),
                vec![
                    TemplateElement::String {
                        value: "normalize-space(//div[contains(concat(' ',normalize-space(@class),' '),' monthly-price ')])".to_string(),
                        source: "normalize-space(//div[contains(concat(' ',normalize-space(@class),' '),' monthly-price ')])".to_source(),
                    }
                ],
                SourceInfo::new(Pos::new(1, 7), Pos::new(1, 100))
            )
        });
    }

    #[test]
    fn test_jsonpath_query() {
        let mut reader = Reader::new("jsonpath \"$['statusCode']\"");
        assert_eq!(
            jsonpath_query(&mut reader).unwrap(),
            QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 10)),
                },
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "$['statusCode']".to_string(),
                        source: "$['statusCode']".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 10), Pos::new(1, 27))
                )
            }
        );
        let mut reader = Reader::new("jsonpath \"$.success\"");
        assert_eq!(
            jsonpath_query(&mut reader).unwrap(),
            QueryValue::Jsonpath {
                space0: Whitespace {
                    value: String::from(" "),
                    source_info: SourceInfo::new(Pos::new(1, 9), Pos::new(1, 10)),
                },
                expr: Template::new(
                    Some('"'),
                    vec![TemplateElement::String {
                        value: "$.success".to_string(),
                        source: "$.success".to_source(),
                    }],
                    SourceInfo::new(Pos::new(1, 10), Pos::new(1, 21))
                )
            }
        );
    }

    #[test]
    fn test_query_with_filters() {
        let mut reader = Reader::new("body urlDecode ");
        assert_eq!(
            query(&mut reader).unwrap(),
            Query {
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 5)),
                value: QueryValue::Body,
            }
        );
        assert_eq!(
            filters(&mut reader).unwrap(),
            vec![(
                Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 5), Pos::new(1, 6))
                },
                Filter {
                    source_info: SourceInfo::new(Pos::new(1, 6), Pos::new(1, 15)),
                    value: FilterValue::UrlDecode,
                }
            )]
        );
        assert_eq!(reader.cursor().index, 14);
    }
}
