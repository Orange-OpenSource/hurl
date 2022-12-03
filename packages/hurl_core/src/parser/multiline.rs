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
use super::combinators::*;
use super::reader::Reader;
use super::ParseResult;
use crate::ast::*;
use crate::parser::json::object_value;
use crate::parser::primitives::*;
use crate::parser::{template, Error, ParseError};

pub fn multiline_string(reader: &mut Reader) -> ParseResult<'static, MultilineString> {
    try_literal("```", reader)?;
    let save = reader.state.clone();

    match choice(&[json_text, xml_text, graphql, plain_text], reader) {
        Ok(multi) => Ok(multi),
        Err(err) => {
            // FIXME: how to parse
            //
            // ```graphql_inline```
            //
            // => this one is non recoverable but should be parsed as TextOneline
            //
            // ```graphql
            // {
            //   me
            // }
            // variables
            // ```
            //
            // => this one is non recoverable should trigger an GraphQL variables error
            if let ParseError::GraphQlVariables = err.inner {
                return Err(err);
            }
            reader.state = save;
            let value = oneline_string_value(reader)?;
            Ok(MultilineString::OneLineText(value))
        }
    }
}

fn text(lang: &str, reader: &mut Reader) -> ParseResult<'static, Text> {
    try_literal(lang, reader)?;
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    let value = multiline_string_value(reader)?;
    Ok(Text {
        space,
        newline,
        value,
    })
}

fn json_text(reader: &mut Reader) -> ParseResult<'static, MultilineString> {
    let text = text("json", reader)?;
    Ok(MultilineString::Json(text))
}

fn xml_text(reader: &mut Reader) -> ParseResult<'static, MultilineString> {
    let text = text("xml", reader)?;
    Ok(MultilineString::Xml(text))
}

fn graphql(reader: &mut Reader) -> ParseResult<'static, MultilineString> {
    try_literal("graphql", reader)?;
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;

    let mut chars = vec![];

    let start = reader.state.pos.clone();
    while !reader.remaining().starts_with("```") && !reader.is_eof() {
        let pos = reader.state.pos.clone();
        let c = reader.read().unwrap();
        chars.push((c, c.to_string(), pos));
        if c == '\n' {
            let end = reader.state.pos.clone();
            let variables = optional(graphql_variables, reader)?;
            match variables {
                None => continue,
                Some(variables) => {
                    literal("```", reader)?;

                    let encoded_string = template::EncodedString {
                        source_info: SourceInfo {
                            start: start.clone(),
                            end: end.clone(),
                        },
                        chars: chars.clone(),
                    };

                    let elements = template::templatize(encoded_string)?;
                    let template = Template {
                        delimiter: None,
                        elements,
                        source_info: SourceInfo { start, end },
                    };

                    return Ok(MultilineString::GraphQl(GraphQl {
                        space,
                        newline,
                        value: template,
                        variables: Some(variables),
                    }));
                }
            }
        }
    }
    let end = reader.state.pos.clone();
    literal("```", reader)?;

    let encoded_string = template::EncodedString {
        source_info: SourceInfo {
            start: start.clone(),
            end: end.clone(),
        },
        chars,
    };

    let elements = template::templatize(encoded_string)?;
    let template = Template {
        delimiter: None,
        elements,
        source_info: SourceInfo { start, end },
    };

    Ok(MultilineString::GraphQl(GraphQl {
        space,
        newline,
        value: template,
        variables: None,
    }))
}

fn whitespace(reader: &mut Reader) -> ParseResult<'static, Whitespace> {
    let start = reader.state.clone();
    match reader.read() {
        None => Err(Error {
            pos: start.pos,
            recoverable: true,
            inner: ParseError::Space {},
        }),
        Some(c) => {
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                Ok(Whitespace {
                    value: c.to_string(),
                    source_info: SourceInfo::new(
                        start.pos.line,
                        start.pos.column,
                        reader.state.pos.line,
                        reader.state.pos.column,
                    ),
                })
            } else {
                Err(Error {
                    pos: start.pos,
                    recoverable: true,
                    inner: ParseError::Space {},
                })
            }
        }
    }
}

fn zero_or_more_whitespaces<'a>(reader: &mut Reader) -> ParseResult<'a, Whitespace> {
    let start = reader.state.clone();
    match zero_or_more(whitespace, reader) {
        Ok(v) => {
            let s = v.iter().map(|x| x.value.clone()).collect();
            Ok(Whitespace {
                value: s,
                source_info: SourceInfo::new(
                    start.pos.line,
                    start.pos.column,
                    reader.state.pos.line,
                    reader.state.pos.column,
                ),
            })
        }
        Err(e) => Err(e),
    }
}

fn graphql_variables(reader: &mut Reader) -> ParseResult<'static, GraphQlVariables> {
    try_literal("variables", reader)?;
    let space = zero_or_more_spaces(reader)?;
    let start = reader.state.clone();
    let object = object_value(reader);
    let value = match object {
        Ok(obj) => obj,
        Err(_) => {
            return Err(Error {
                pos: start.pos,
                recoverable: false,
                inner: ParseError::GraphQlVariables,
            });
        }
    };
    let whitespace = zero_or_more_whitespaces(reader)?;
    Ok(GraphQlVariables {
        space,
        value,
        whitespace,
    })
}

fn plain_text(reader: &mut Reader) -> ParseResult<'static, MultilineString> {
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    let value = multiline_string_value(reader)?;
    Ok(MultilineString::Text(Text {
        space,
        newline,
        value,
    }))
}

fn multiline_string_value(reader: &mut Reader) -> ParseResult<'static, Template> {
    let mut chars = vec![];

    let start = reader.state.pos.clone();
    while !reader.remaining().starts_with("```") && !reader.is_eof() {
        let pos = reader.state.pos.clone();
        let c = reader.read().unwrap();
        chars.push((c, c.to_string(), pos));
    }
    let end = reader.state.pos.clone();
    literal("```", reader)?;

    let encoded_string = template::EncodedString {
        source_info: SourceInfo {
            start: start.clone(),
            end: end.clone(),
        },
        chars,
    };

    let elements = template::templatize(encoded_string)?;

    Ok(Template {
        delimiter: None,
        elements,
        source_info: SourceInfo { start, end },
    })
}

fn oneline_string_value(reader: &mut Reader) -> ParseResult<'static, Template> {
    let mut chars = vec![];

    let start = reader.state.pos.clone();
    while !reader.remaining().starts_with("```") && !reader.is_eof() {
        let pos = reader.state.pos.clone();
        let c = reader.read().unwrap();
        if c == '\n' {
            return Err(Error {
                pos: start,
                recoverable: false,
                inner: ParseError::Multiline,
            });
        }
        chars.push((c, c.to_string(), pos));
    }
    let end = reader.state.pos.clone();
    literal("```", reader)?;

    let encoded_string = template::EncodedString {
        source_info: SourceInfo {
            start: start.clone(),
            end: end.clone(),
        },
        chars,
    };

    let elements = template::templatize(encoded_string)?;

    Ok(Template {
        delimiter: None,
        elements,
        source_info: SourceInfo { start, end },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiline_string_text() {
        let mut reader = Reader::init("```\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 4, 1, 4),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 4, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 5, 1),
                },
            })
        );

        let mut reader = Reader::init("```         \nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: "         ".to_string(),
                    source_info: SourceInfo::new(1, 4, 1, 13),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 13, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 5, 1),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_json() {
        let mut reader = Reader::init("```json\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Json(Text {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 8, 1, 8),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 8, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 5, 1),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_graphql() {
        let mut reader = Reader::init("```graphql\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::GraphQl(GraphQl {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 11, 1, 11),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 11, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 5, 1),
                },
                variables: None,
            })
        );

        let mut reader = Reader::init("```graphql      \nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::GraphQl(GraphQl {
                space: Whitespace {
                    value: "      ".to_string(),
                    source_info: SourceInfo::new(1, 11, 1, 17),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 17, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 5, 1),
                },
                variables: None,
            })
        );
    }

    #[test]
    fn test_multiline_string_failed() {
        let datas = [
            "```hexaaa\nline1\nline2\nline3\n```",
            "```aaa\nline1\nline2\nline3\n```",
        ];

        for text in datas.iter() {
            let mut reader = Reader::init(text);
            assert!(multiline_string(&mut reader).is_err())
        }
    }

    #[test]
    fn test_multiline_string_empty() {
        let mut reader = Reader::init("``````");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::OneLineText(Template {
                delimiter: None,
                elements: vec![],
                source_info: SourceInfo::new(1, 4, 1, 4),
            })
        );

        let mut reader = Reader::init("```\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 4, 1, 4),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 4, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![],
                    source_info: SourceInfo::new(2, 1, 2, 1),
                },
            })
        );
        let mut reader = Reader::init("```\r\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 4, 1, 4),
                },
                newline: Whitespace {
                    value: "\r\n".to_string(),
                    source_info: SourceInfo::new(1, 4, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![],
                    source_info: SourceInfo::new(2, 1, 2, 1),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_hello() {
        let mut reader = Reader::init("```Hello World!```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::OneLineText(Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "Hello World!".to_string(),
                    encoded: "Hello World!".to_string(),
                }],
                source_info: SourceInfo::new(1, 4, 1, 16),
            })
        );
    }

    #[test]
    fn test_multiline_string_base64_prefix() {
        let mut reader = Reader::init("```base64_inline```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::OneLineText(Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "base64_inline".to_string(),
                    encoded: "base64_inline".to_string(),
                }],
                source_info: SourceInfo::new(1, 4, 1, 17),
            })
        );
    }

    #[test]
    fn test_multiline_string_csv() {
        let mut reader = Reader::init("```\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 4, 1, 4),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 4, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 5, 1),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_one_empty_line() {
        let mut reader = Reader::init("```\n\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 4, 1, 4),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 4, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\n".to_string(),
                        encoded: "\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 3, 1),
                },
            })
        );

        // One cr
        let mut reader = Reader::init("```\n\r\n````");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 4, 1, 4),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 4, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\r\n".to_string(),
                        encoded: "\r\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 3, 1),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_error() {
        let mut reader = Reader::init("xxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "```".to_string()
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("```\nxxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 4 });
        assert_eq!(error.inner, ParseError::Multiline);
        assert!(!error.recoverable);

        let mut reader = Reader::init("```xxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: "```".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_multiline_string_value() {
        let mut reader = Reader::init("```");
        assert_eq!(
            multiline_string_value(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![],
                source_info: SourceInfo::new(1, 1, 1, 1),
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("hello```");
        assert_eq!(
            multiline_string_value(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "hello".to_string(),
                    encoded: "hello".to_string(),
                }],
                source_info: SourceInfo::new(1, 1, 1, 6),
            }
        );
        assert_eq!(reader.state.cursor, 8);
    }

    #[test]
    fn test_multiline_string_graphql_with_variables() {
        let mut reader = Reader::init(
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
            MultilineString::GraphQl(GraphQl {
                space: Whitespace {
                    value: "".to_string(),
                    source_info: SourceInfo::new(1, 11, 1, 11),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 11, 2, 1),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "query Human($name: String!) {\n  human(name: $name) {\n    name\n    height(unit: FOOT)\n}\n\n".to_string(),
                        encoded:
                            "query Human($name: String!) {\n  human(name: $name) {\n    name\n    height(unit: FOOT)\n}\n\n".to_string()
                    }],
                    source_info: SourceInfo::new(2, 1, 8, 1),
                },
                variables: Some(GraphQlVariables {
                    space: Whitespace {
                        value: " ".to_string(),
                        source_info: SourceInfo::new(8, 10, 8, 11),
                    },
                    value: JsonValue::Object {
                        space0: "\n  ".to_string(),
                        elements: vec![JsonObjectElement {
                            space0: "".to_string(),
                            name: Template {
                                delimiter: Some('"'),
                                elements: vec![
                                    TemplateElement::String {
                                        value: "name".to_string(),
                                        encoded: "name".to_string()
                                    }
                                ],
                                source_info: SourceInfo::new(9, 4, 9, 8)
                            },
                            space1: "".to_string(),
                            space2: " ".to_string(),
                            value: JsonValue::String(Template {
                                delimiter: Some('"'),
                                elements: vec![
                                    TemplateElement::String {
                                        value: "Han Solo".to_string(),
                                        encoded: "Han Solo".to_string()
                                    }
                                ],
                                source_info: SourceInfo::new(9, 12, 9, 20)
                            }),
                            space3: "\n".to_string()
                        }]
                    },
                    whitespace: Whitespace {
                        value: "\n".to_string(),
                        source_info: SourceInfo::new(10, 2, 11, 1)
                    }
                })
            })
        )
    }

    #[test]
    fn test_multiline_string_graphql_with_variables_error() {
        let mut reader = Reader::init(
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
            Error {
                pos: Pos {
                    line: 8,
                    column: 10
                },
                recoverable: false,
                inner: ParseError::GraphQlVariables
            }
        )
    }

    #[test]
    fn test_multiline_string_graphql_with_variables_not_an_object() {
        let mut reader = Reader::init(
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
            Error {
                pos: Pos {
                    line: 8,
                    column: 11
                },
                recoverable: false,
                inner: ParseError::GraphQlVariables
            }
        )
    }
}
