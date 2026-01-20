/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
    GraphQl, GraphQlVariables, MultilineString, MultilineStringKind, SourceInfo, Template,
    TemplateElement, Whitespace,
};
use crate::combinator::{choice, optional, zero_or_more};
use crate::parser::json::object_value;
use crate::parser::primitives::{literal, newline, try_literal, zero_or_more_spaces};
use crate::parser::{template, ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;
use crate::typing::ToSource;

pub fn multiline_string(reader: &mut Reader) -> ParseResult<MultilineString> {
    try_literal("```", reader)?;

    choice(
        &[json_text, xml_text, graphql, raw_text, plain_text],
        reader,
    )
}

/// Parses a multiline text bloc.
///
/// Plain text can have a language hint `lang` (like <code>&#96;&#96;&#96;json</code>,
/// or <code>&#96;&#96;&#96;xml</code>) to give some semantic to the string bloc. All multiline
/// variants don't escape chars (`\n` is a literal two chars `\` and `n`) and evaluate variables
/// (`raw` multiline is the only exception that doesn't evaluate variables).
///
/// ## Example of a text bloc without language hint
///
/// ~~~hurl
/// GET https://foo.com
/// ```
/// one
/// two
/// three
/// ```
/// ~~~
///
/// ## Example of a text bloc with JSON language hint
///
/// ~~~hurl
/// GET https://foo.com
/// ```json
/// {
///   "name: "bob"
/// }
/// ```
/// ~~~
///
///
fn text(
    lang: Option<&str>,
    templatized: bool,
    reader: &mut Reader,
) -> ParseResult<(Whitespace, Whitespace, Template)> {
    if let Some(lang) = lang {
        try_literal(lang, reader)?;
    }
    let space = zero_or_more_spaces(reader)?;

    // We check that we have a newline here, and raise error if we have a language hint.
    // The language hints that we supported (`json`, `raw` etc...) have been processed by
    // earlier parse functions.
    let start = reader.cursor();
    let hint = language_hint(reader)?;
    if !hint.is_empty() {
        return Err(ParseError {
            pos: start.pos,
            recoverable: false,
            kind: ParseErrorKind::MultilineLanguageHint(hint),
        });
    }

    let newline = newline(reader)?;
    let value = multiline_string_value(templatized, reader)?;
    Ok((space, newline, value))
}

/// Parses a plain text multilines block.
///
/// ## Example
///
/// ~~~hurl
/// GET https://foo.com
/// ```
/// toto
/// ```
/// ~~~
fn plain_text(reader: &mut Reader) -> ParseResult<MultilineString> {
    let (space, newline, text) = text(None, true, reader)?;
    let kind = MultilineStringKind::Text(text);
    Ok(MultilineString {
        space,
        newline,
        kind,
    })
}

/// Parses a plain text multilines block.
///
/// Contrary to [`plain_text`], this function doesn't templatize the text (i.e. the inline block
/// `{{my_variable}}` are not transformed to template elements, and remains simple text.
///
/// ## Example
///
/// ~~~hurl
/// GET https://foo.com
/// ```
/// toto
/// ```
/// ~~~
fn raw_text(reader: &mut Reader) -> ParseResult<MultilineString> {
    let (space, newline, text) = text(Some("raw"), false, reader)?;
    let kind = MultilineStringKind::Raw(text);
    Ok(MultilineString {
        space,
        newline,
        kind,
    })
}

/// Parses a JSON multilines block.
///
/// ## Example
///
/// ~~~hurl
/// GET https://foo.com
/// ```json
/// {"foo":"bar"}
/// ```
/// ~~~
fn json_text(reader: &mut Reader) -> ParseResult<MultilineString> {
    let (space, newline, text) = text(Some("json"), true, reader)?;
    let kind = MultilineStringKind::Json(text);
    Ok(MultilineString {
        space,
        newline,
        kind,
    })
}

/// Parses an XML multilines block.
///
/// ## Example
///
/// ~~~hurl
/// GET https://foo.com
/// ```xml
/// <?xml version="1.0"?>
///     <book id="bk101">
///         <author>Gambardella, Matthew</author>
///     </book>
/// </xml>
/// ```
/// ~~~
fn xml_text(reader: &mut Reader) -> ParseResult<MultilineString> {
    let (space, newline, text) = text(Some("xml"), true, reader)?;
    let kind = MultilineStringKind::Xml(text);
    Ok(MultilineString {
        space,
        newline,
        kind,
    })
}

/// Parses a GraphQL multilines block.
///
/// ## Example
///
/// ~~~hurl
/// GET https://foo.com
/// ```graphql
/// query getCity($city: String) {
///     cities(name: $city) {
///         population
///         weather {
///             temperature
///             precipitation
///         }
///     }
/// }
/// ```
/// ~~~
fn graphql(reader: &mut Reader) -> ParseResult<MultilineString> {
    try_literal("graphql", reader)?;
    drop(try_literal(",", reader));
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;

    let mut chars = vec![];

    let start = reader.cursor();
    while reader.peek_n(3) != "```" && !reader.is_eof() {
        let pos = reader.cursor().pos;
        let c = reader.read().unwrap();
        chars.push((c, c.to_string(), pos));
        if c == '\n' {
            let end = reader.cursor();
            let variables = optional(graphql_variables, reader)?;
            match variables {
                None => continue,
                Some(variables) => {
                    literal("```", reader)?;

                    let encoded_string = template::EncodedString {
                        source_info: SourceInfo::new(start.pos, end.pos),
                        chars: chars.clone(),
                    };

                    let elements = template::templatize(encoded_string)?;
                    let template =
                        Template::new(None, elements, SourceInfo::new(start.pos, end.pos));
                    let kind = MultilineStringKind::GraphQl(GraphQl {
                        value: template,
                        variables: Some(variables),
                    });
                    return Ok(MultilineString {
                        space,
                        newline,
                        kind,
                    });
                }
            }
        }
    }
    let end = reader.cursor();
    literal("```", reader)?;

    let encoded_string = template::EncodedString {
        source_info: SourceInfo::new(start.pos, end.pos),
        chars,
    };

    let elements = template::templatize(encoded_string)?;
    let template = Template::new(None, elements, SourceInfo::new(start.pos, end.pos));
    let kind = MultilineStringKind::GraphQl(GraphQl {
        value: template,
        variables: None,
    });
    Ok(MultilineString {
        space,
        newline,
        kind,
    })
}

fn whitespace(reader: &mut Reader) -> ParseResult<Whitespace> {
    let start = reader.cursor();
    match reader.read() {
        None => Err(ParseError::new(start.pos, true, ParseErrorKind::Space)),
        Some(c) => {
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                Ok(Whitespace {
                    value: c.to_string(),
                    source_info: SourceInfo::new(start.pos, reader.cursor().pos),
                })
            } else {
                Err(ParseError::new(start.pos, true, ParseErrorKind::Space))
            }
        }
    }
}

fn zero_or_more_whitespaces(reader: &mut Reader) -> ParseResult<Whitespace> {
    let start = reader.cursor();
    match zero_or_more(whitespace, reader) {
        Ok(v) => {
            let s = v.iter().map(|x| x.value.clone()).collect();
            Ok(Whitespace {
                value: s,
                source_info: SourceInfo::new(start.pos, reader.cursor().pos),
            })
        }
        Err(e) => Err(e),
    }
}

fn language_hint(reader: &mut Reader) -> ParseResult<String> {
    let hint = reader.read_while(|c| c != '\n' && c != '\r' && c != '`');
    Ok(hint)
}

fn graphql_variables(reader: &mut Reader) -> ParseResult<GraphQlVariables> {
    try_literal("variables", reader)?;
    let space = zero_or_more_spaces(reader)?;
    let start = reader.cursor();
    let object = object_value(reader);
    let value = match object {
        Ok(obj) => obj,
        Err(_) => {
            return Err(ParseError::new(
                start.pos,
                false,
                ParseErrorKind::GraphQlVariables,
            ))
        }
    };
    let whitespace = zero_or_more_whitespaces(reader)?;
    Ok(GraphQlVariables {
        space,
        value,
        whitespace,
    })
}

fn multiline_string_value(templatize: bool, reader: &mut Reader) -> ParseResult<Template> {
    let mut chars = vec![];

    let start = reader.cursor();
    while reader.peek_n(3) != "```" && !reader.is_eof() {
        let pos = reader.cursor().pos;
        let c = reader.read().unwrap();
        chars.push((c, c.to_string(), pos));
    }
    let end = reader.cursor();
    literal("```", reader)?;

    let source_info = SourceInfo::new(start.pos, end.pos);

    let elements = if templatize {
        let encoded_string = template::EncodedString { source_info, chars };
        template::templatize(encoded_string)?
    } else {
        let source = chars.iter().map(|(c, _, _)| c).collect::<String>();
        let template = TemplateElement::String {
            value: source.to_string(),
            source: source.to_source(),
        };
        vec![template]
    };
    let template = Template::new(None, elements, SourceInfo::new(start.pos, end.pos));
    Ok(template)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        Expr, ExprKind, JsonObjectElement, JsonValue, Placeholder, TemplateElement, Variable,
    };
    use crate::reader::{CharPos, Pos};
    use crate::types::ToSource;

    #[test]
    fn test_multiline_string_text() {
        let mut reader = Reader::new("```\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Text(Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        source: "line1\nline2\nline3\n".to_source(),
                    }],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1))
                )),
            }
        );

        let mut reader = Reader::new("```         \nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: "         ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 13)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 13), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Text(Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        source: "line1\nline2\nline3\n".to_source(),
                    }],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1))
                )),
            }
        );
    }

    #[test]
    fn test_multiline_string_text_with_variables() {
        let mut reader = Reader::new("```\nfoo\n{{var_1}} bar {{var2}}\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Text(Template::new(
                    None,
                    vec![
                        TemplateElement::String {
                            value: "foo\n".to_string(),
                            source: "foo\n".to_source()
                        },
                        TemplateElement::Placeholder(Placeholder {
                            space0: Whitespace {
                                value: String::new(),
                                source_info: SourceInfo::new(Pos::new(3, 3), Pos::new(3, 3)),
                            },
                            expr: Expr {
                                source_info: SourceInfo::new(Pos::new(3, 3), Pos::new(3, 8)),
                                kind: ExprKind::Variable(Variable {
                                    name: "var_1".to_string(),
                                    source_info: SourceInfo::new(Pos::new(3, 3), Pos::new(3, 8)),
                                }),
                            },
                            space1: Whitespace {
                                value: String::new(),
                                source_info: SourceInfo::new(Pos::new(3, 8), Pos::new(3, 8)),
                            },
                        }),
                        TemplateElement::String {
                            value: " bar ".to_string(),
                            source: " bar ".to_source()
                        },
                        TemplateElement::Placeholder(Placeholder {
                            space0: Whitespace {
                                value: String::new(),
                                source_info: SourceInfo::new(Pos::new(3, 17), Pos::new(3, 17)),
                            },
                            expr: Expr {
                                source_info: SourceInfo::new(Pos::new(3, 17), Pos::new(3, 21)),
                                kind: ExprKind::Variable(Variable {
                                    name: "var2".to_string(),
                                    source_info: SourceInfo::new(Pos::new(3, 17), Pos::new(3, 21)),
                                }),
                            },
                            space1: Whitespace {
                                value: String::new(),
                                source_info: SourceInfo::new(Pos::new(3, 21), Pos::new(3, 21)),
                            },
                        }),
                        TemplateElement::String {
                            value: "\nline3\n".to_string(),
                            source: "\nline3\n".to_source()
                        },
                    ],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1))
                )),
            }
        );
    }

    #[test]
    fn test_multiline_string_text_with_variables_error() {
        let mut reader = Reader::new("```\nline1\n{{bar\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(4, 1),
                false,
                ParseErrorKind::Expecting {
                    value: "}}".to_string()
                }
            )
        );
    }

    #[test]
    fn test_multiline_string_json() {
        let mut reader = Reader::new("```json\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 8)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Json(Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        source: "line1\nline2\nline3\n".to_source(),
                    }],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                )),
            }
        );

        // JSON multilines don't escape test (so Hurl Unicode literals are not supported)
        let mut reader = Reader::new(
            r#"```json
{
  "g_clef": "\u{1D11E}"
}
```"#,
        );
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 8)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Json(Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "{\n  \"g_clef\": \"\\u{1D11E}\"\n}\n".to_string(),
                        source: "{\n  \"g_clef\": \"\\u{1D11E}\"\n}\n".to_source(),
                    }],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                )),
            }
        );
    }

    #[test]
    fn test_multiline_string_graphql() {
        let mut reader = Reader::new("```graphql\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 11)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::GraphQl(GraphQl {
                    value: Template::new(
                        None,
                        vec![TemplateElement::String {
                            value: "line1\nline2\nline3\n".to_string(),
                            source: "line1\nline2\nline3\n".to_source(),
                        }],
                        SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1))
                    ),
                    variables: None,
                }),
            }
        );

        let mut reader = Reader::new("```graphql      \nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: "      ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 17)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 17), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::GraphQl(GraphQl {
                    value: Template::new(
                        None,
                        vec![TemplateElement::String {
                            value: "line1\nline2\nline3\n".to_string(),
                            source: "line1\nline2\nline3\n".to_source(),
                        }],
                        SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                    ),
                    variables: None,
                }),
            }
        );
    }

    #[test]
    fn test_multiline_string_failed() {
        let data = [
            "```hexaaa\nline1\nline2\nline3\n```",
            "```aaa\nline1\nline2\nline3\n```",
        ];

        for text in data.iter() {
            let mut reader = Reader::new(text);
            assert!(multiline_string(&mut reader).is_err());
        }
    }

    #[test]
    fn test_multiline_string_empty() {
        let mut reader = Reader::new("```\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Text(Template::new(
                    None,
                    vec![],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(2, 1))
                )),
            }
        );
        let mut reader = Reader::new("```\r\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\r\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Text(Template::new(
                    None,
                    vec![],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(2, 1)),
                )),
            }
        );
    }

    #[test]
    fn test_multiline_string_hello_error() {
        let mut reader = Reader::new("```Hello World!```");
        let error = multiline_string(&mut reader).unwrap_err();
        assert_eq!(error.pos, Pos::new(1, 4));
        assert_eq!(
            error.kind,
            ParseErrorKind::MultilineLanguageHint("Hello World!".to_string())
        );
    }

    #[test]
    fn test_multiline_string_csv() {
        let mut reader = Reader::new("```\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Text(Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        source: "line1\nline2\nline3\n".to_source(),
                    }],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                )),
            }
        );
    }

    #[test]
    fn test_multiline_string_one_empty_line() {
        let mut reader = Reader::new("```\n\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                kind: MultilineStringKind::Text(Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "\n".to_string(),
                        source: "\n".to_source(),
                    }],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(3, 1)),
                )),
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
            }
        );

        // One cr
        let mut reader = Reader::new("```\n\r\n````");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::Text(Template::new(
                    None,
                    vec![TemplateElement::String {
                        value: "\r\n".to_string(),
                        source: "\r\n".to_source(),
                    }],
                    SourceInfo::new(Pos::new(2, 1), Pos::new(3, 1)),
                )),
            }
        );
    }

    #[test]
    fn test_multiline_string_error() {
        let mut reader = Reader::new("xxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "```".to_string()
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::new("```\nxxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 2, column: 4 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "```".to_string()
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::new("```xxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 4 });
        assert_eq!(
            error.kind,
            ParseErrorKind::MultilineLanguageHint("xxx".to_string())
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_multiline_string_value() {
        let mut reader = Reader::new("```");
        assert_eq!(
            multiline_string_value(true, &mut reader).unwrap(),
            Template::new(
                None,
                vec![],
                SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1))
            )
        );
        assert_eq!(reader.cursor().index, CharPos(3));

        let mut reader = Reader::new("hello```");
        assert_eq!(
            multiline_string_value(true, &mut reader).unwrap(),
            Template::new(
                None,
                vec![TemplateElement::String {
                    value: "hello".to_string(),
                    source: "hello".to_source(),
                }],
                SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6))
            )
        );
        assert_eq!(reader.cursor().index, CharPos(8));
    }

    #[test]
    fn test_multiline_string_graphql_with_variables() {
        let mut reader = Reader::new(
            r#"```graphql
query Human($name: String!) {
  human(name: $name) {
    name
    height(unit: FOOT)
}

variables {
  "name": "Han Solo"
}
```"#,
        );

        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 11)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(2, 1)),
                },
                kind: MultilineStringKind::GraphQl(GraphQl {
                    value: Template::new(
                        None,
                        vec![TemplateElement::String {
                            value: "query Human($name: String!) {\n  human(name: $name) {\n    name\n    height(unit: FOOT)\n}\n\n".to_string(),
                            source:
                            "query Human($name: String!) {\n  human(name: $name) {\n    name\n    height(unit: FOOT)\n}\n\n".to_source()
                        }],
                        SourceInfo::new(Pos::new(2, 1), Pos::new(8, 1)),
                    ),
                    variables: Some(GraphQlVariables {
                        space: Whitespace {
                            value: " ".to_string(),
                            source_info: SourceInfo::new(Pos::new(8, 10), Pos::new(8, 11)),
                        },
                        value: JsonValue::Object {
                            space0: "\n  ".to_string(),
                            elements: vec![JsonObjectElement {
                                space0: String::new(),
                                name: Template::new(
                                    Some('"'),
                                    vec![
                                        TemplateElement::String {
                                            value: "name".to_string(),
                                            source: "name".to_source()
                                        }
                                    ],
                                    SourceInfo::new(Pos::new(9, 4), Pos::new(9, 8))
                                ),
                                space1: String::new(),
                                space2: " ".to_string(),
                                value: JsonValue::String(Template::new(
                                    Some('"'),
                                    vec![
                                        TemplateElement::String {
                                            value: "Han Solo".to_string(),
                                            source: "Han Solo".to_source()
                                        }
                                    ],
                                    SourceInfo::new(Pos::new(9, 12), Pos::new(9, 20))
                                )),
                                space3: "\n".to_string()
                            }]
                        },
                        whitespace: Whitespace {
                            value: "\n".to_string(),
                            source_info: SourceInfo::new(Pos::new(10, 2), Pos::new(11, 1))
                        }
                    })
                }),
            }
        );
    }

    #[test]
    fn test_multiline_string_graphql_with_variables_error() {
        let mut reader = Reader::new(
            r#"```graphql
query Human($name: String!) {
  human(name: $name) {
    name
    height(unit: FOOT)
}

variables
```"#,
        );

        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(
            error,
            ParseError::new(Pos::new(8, 10), false, ParseErrorKind::GraphQlVariables)
        );
    }

    #[test]
    fn test_multiline_string_graphql_with_variables_not_an_object() {
        let mut reader = Reader::new(
            r#"```graphql
query Human($name: String!) {
  human(name: $name) {
    name
    height(unit: FOOT)
}

variables [
  "one",
  "two",
  "three"
]
```"#,
        );

        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(
            error,
            ParseError::new(Pos::new(8, 11), false, ParseErrorKind::GraphQlVariables)
        );
    }
}
