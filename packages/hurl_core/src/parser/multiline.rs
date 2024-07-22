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
use crate::combinator::{choice, optional, zero_or_more};
use crate::parser::json::object_value;
use crate::parser::primitives::*;
use crate::parser::{template, ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

pub fn multiline_string(reader: &mut Reader) -> ParseResult<MultilineString> {
    try_literal("```", reader)?;
    let save = reader.cursor();

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
            if let ParseErrorKind::GraphQlVariables = err.kind {
                return Err(err);
            }
            reader.seek(save);
            let value = oneline_string_value(reader)?;
            Ok(MultilineString::OneLineText(value))
        }
    }
}

fn text(lang: &str, reader: &mut Reader) -> ParseResult<Text> {
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

fn json_text(reader: &mut Reader) -> ParseResult<MultilineString> {
    let text = text("json", reader)?;
    Ok(MultilineString::Json(text))
}

fn xml_text(reader: &mut Reader) -> ParseResult<MultilineString> {
    let text = text("xml", reader)?;
    Ok(MultilineString::Xml(text))
}

fn graphql(reader: &mut Reader) -> ParseResult<MultilineString> {
    try_literal("graphql", reader)?;
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
                    let template = Template {
                        delimiter: None,
                        elements,
                        source_info: SourceInfo::new(start.pos, end.pos),
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
    let end = reader.cursor();
    literal("```", reader)?;

    let encoded_string = template::EncodedString {
        source_info: SourceInfo::new(start.pos, end.pos),
        chars,
    };

    let elements = template::templatize(encoded_string)?;
    let template = Template {
        delimiter: None,
        elements,
        source_info: SourceInfo::new(start.pos, end.pos),
    };

    Ok(MultilineString::GraphQl(GraphQl {
        space,
        newline,
        value: template,
        variables: None,
    }))
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

fn plain_text(reader: &mut Reader) -> ParseResult<MultilineString> {
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    let value = multiline_string_value(reader)?;
    Ok(MultilineString::Text(Text {
        space,
        newline,
        value,
    }))
}

fn multiline_string_value(reader: &mut Reader) -> ParseResult<Template> {
    let mut chars = vec![];

    let start = reader.cursor();
    while reader.peek_n(3) != "```" && !reader.is_eof() {
        let pos = reader.cursor().pos;
        let c = reader.read().unwrap();
        chars.push((c, c.to_string(), pos));
    }
    let end = reader.cursor();
    literal("```", reader)?;

    let encoded_string = template::EncodedString {
        source_info: SourceInfo::new(start.pos, end.pos),
        chars,
    };

    let elements = template::templatize(encoded_string)?;

    Ok(Template {
        delimiter: None,
        elements,
        source_info: SourceInfo::new(start.pos, end.pos),
    })
}

fn oneline_string_value(reader: &mut Reader) -> ParseResult<Template> {
    let mut chars = vec![];

    let start = reader.cursor();
    while reader.peek_n(3) != "```" && !reader.is_eof() {
        let pos = reader.cursor().pos;
        let c = reader.read().unwrap();
        if c == '\n' {
            return Err(ParseError::new(start.pos, false, ParseErrorKind::Multiline));
        }
        chars.push((c, c.to_string(), pos));
    }
    let end = reader.cursor();
    literal("```", reader)?;

    let encoded_string = template::EncodedString {
        source_info: SourceInfo::new(start.pos, end.pos),
        chars,
    };

    let elements = template::templatize(encoded_string)?;

    Ok(Template {
        delimiter: None,
        elements,
        source_info: SourceInfo::new(start.pos, end.pos),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::Pos;

    #[test]
    fn test_multiline_string_text() {
        let mut reader = Reader::new("```\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                },
            })
        );

        let mut reader = Reader::new("```         \nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: "         ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 13)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 13), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_json() {
        let mut reader = Reader::new("```json\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Json(Text {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(1, 8)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 8), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_graphql() {
        let mut reader = Reader::new("```graphql\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::GraphQl(GraphQl {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 11)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                },
                variables: None,
            })
        );

        let mut reader = Reader::new("```graphql      \nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::GraphQl(GraphQl {
                space: Whitespace {
                    value: "      ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 17)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 17), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                },
                variables: None,
            })
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
        let mut reader = Reader::new("``````");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::OneLineText(Template {
                delimiter: None,
                elements: vec![],
                source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
            })
        );

        let mut reader = Reader::new("```\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 1)),
                },
            })
        );
        let mut reader = Reader::new("```\r\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\r\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(2, 1)),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_hello() {
        let mut reader = Reader::new("```Hello World!```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::OneLineText(Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "Hello World!".to_string(),
                    encoded: "Hello World!".to_string(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 16)),
            })
        );
    }

    #[test]
    fn test_multiline_string_base64_prefix() {
        let mut reader = Reader::new("```base64_inline```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::OneLineText(Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "base64_inline".to_string(),
                    encoded: "base64_inline".to_string(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 17)),
            })
        );
    }

    #[test]
    fn test_multiline_string_csv() {
        let mut reader = Reader::new("```\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(5, 1)),
                },
            })
        );
    }

    #[test]
    fn test_multiline_string_one_empty_line() {
        let mut reader = Reader::new("```\n\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\n".to_string(),
                        encoded: "\n".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(3, 1)),
                },
            })
        );

        // One cr
        let mut reader = Reader::new("```\n\r\n````");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString::Text(Text {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(1, 4)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 4), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "\r\n".to_string(),
                        encoded: "\r\n".to_string(),
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(3, 1)),
                },
            })
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
        assert_eq!(error.pos, Pos { line: 1, column: 4 });
        assert_eq!(error.kind, ParseErrorKind::Multiline);
        assert!(!error.recoverable);

        let mut reader = Reader::new("```xxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "```".to_string()
            }
        );
        assert!(!error.recoverable);
    }

    #[test]
    fn test_multiline_string_value() {
        let mut reader = Reader::new("```");
        assert_eq!(
            multiline_string_value(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            }
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("hello```");
        assert_eq!(
            multiline_string_value(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "hello".to_string(),
                    encoded: "hello".to_string(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6)),
            }
        );
        assert_eq!(reader.cursor().index, 8);
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
            MultilineString::GraphQl(GraphQl {
                space: Whitespace {
                    value: String::new(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(1, 11)),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 11), Pos::new(2, 1)),
                },
                value: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "query Human($name: String!) {\n  human(name: $name) {\n    name\n    height(unit: FOOT)\n}\n\n".to_string(),
                        encoded:
                            "query Human($name: String!) {\n  human(name: $name) {\n    name\n    height(unit: FOOT)\n}\n\n".to_string()
                    }],
                    source_info: SourceInfo::new(Pos::new(2, 1), Pos::new(8, 1)),
                },
                variables: Some(GraphQlVariables {
                    space: Whitespace {
                        value: " ".to_string(),
                        source_info: SourceInfo::new(Pos::new(8, 10), Pos::new(8, 11)),
                    },
                    value: JsonValue::Object {
                        space0: "\n  ".to_string(),
                        elements: vec![JsonObjectElement {
                            space0: String::new(),
                            name: Template {
                                delimiter: Some('"'),
                                elements: vec![
                                    TemplateElement::String {
                                        value: "name".to_string(),
                                        encoded: "name".to_string()
                                    }
                                ],
                                source_info: SourceInfo::new(Pos::new(9, 4), Pos::new(9, 8))
                            },
                            space1: String::new(),
                            space2: " ".to_string(),
                            value: JsonValue::String(Template {
                                delimiter: Some('"'),
                                elements: vec![
                                    TemplateElement::String {
                                        value: "Han Solo".to_string(),
                                        encoded: "Han Solo".to_string()
                                    }
                                ],
                                source_info: SourceInfo::new(Pos::new(9, 12), Pos::new(9, 20))
                            }),
                            space3: "\n".to_string()
                        }]
                    },
                    whitespace: Whitespace {
                        value: "\n".to_string(),
                        source_info: SourceInfo::new(Pos::new(10, 2), Pos::new(11, 1))
                    }
                })
            })
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
