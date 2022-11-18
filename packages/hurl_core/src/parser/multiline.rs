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
use crate::parser::primitives::*;
use crate::parser::{template, Error, ParseError};

pub fn multiline_string(reader: &mut Reader) -> ParseResult<'static, MultilineString> {
    try_literal("```", reader)?;
    let save = reader.state.clone();

    match choice(
        &[
            base64_lang,
            hex_lang,
            json_lang,
            xml_lang,
            graphql_lang,
            undefined_lang,
        ],
        reader,
    ) {
        Ok(lang) => {
            let value = multiline_string_value(reader)?;
            Ok(MultilineString {
                value,
                lang: Some(lang),
            })
        }
        Err(_) => {
            reader.state = save;
            let value = oneline_string_value(reader)?;
            Ok(MultilineString { value, lang: None })
        }
    }
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
        quotes: false,
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
        quotes: false,
        elements,
        source_info: SourceInfo { start, end },
    })
}

fn undefined_lang(reader: &mut Reader) -> ParseResult<'static, Lang> {
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    Ok(Lang {
        value: None,
        space,
        newline,
    })
}

fn base64_lang(reader: &mut Reader) -> ParseResult<'static, Lang> {
    try_literal("base64", reader)?;
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    Ok(Lang {
        value: Some(LangValue::Base64),
        space,
        newline,
    })
}

fn hex_lang(reader: &mut Reader) -> ParseResult<'static, Lang> {
    try_literal("hex", reader)?;
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    Ok(Lang {
        value: Some(LangValue::Hex),
        space,
        newline,
    })
}

fn json_lang(reader: &mut Reader) -> ParseResult<'static, Lang> {
    try_literal("json", reader)?;
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    Ok(Lang {
        value: Some(LangValue::Json),
        space,
        newline,
    })
}

fn xml_lang(reader: &mut Reader) -> ParseResult<'static, Lang> {
    try_literal("xml", reader)?;
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    Ok(Lang {
        value: Some(LangValue::Xml),
        space,
        newline,
    })
}

fn graphql_lang(reader: &mut Reader) -> ParseResult<'static, Lang> {
    try_literal("graphql", reader)?;
    let space = zero_or_more_spaces(reader)?;
    let newline = newline(reader)?;
    Ok(Lang {
        value: Some(LangValue::GraphQl),
        space,
        newline,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_multiline_string_type() {
        let mut reader = Reader::init("base64\nxxxx");
        assert_eq!(
            base64_lang(&mut reader).unwrap(),
            Lang {
                value: Some(LangValue::Base64),
                space: Whitespace {
                    value: String::from(""),
                    source_info: SourceInfo::new(1, 7, 1, 7),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 7, 2, 1)
                }
            }
        );
    }

    #[test]
    fn test_base64_multiline_string_type_with_padding() {
        let mut reader = Reader::init("base64      \nxxxx");
        assert_eq!(
            base64_lang(&mut reader).unwrap(),
            Lang {
                value: Some(LangValue::Base64),
                space: Whitespace {
                    value: String::from("      "),
                    source_info: SourceInfo::new(1, 7, 1, 13),
                },
                newline: Whitespace {
                    value: "\n".to_string(),
                    source_info: SourceInfo::new(1, 13, 2, 1)
                }
            }
        );
    }

    #[test]
    fn test_multiline_string_ok() {
        let datas = [
            (
                "```\nline1\nline2\nline3\n```",
                None,
                "line1\nline2\nline3\n",
            ),
            (
                "```         \nline1\nline2\nline3\n```",
                None,
                "line1\nline2\nline3\n",
            ),
            (
                "```base64\nline1\nline2\nline3\n```",
                Some(LangValue::Base64),
                "line1\nline2\nline3\n",
            ),
            (
                "```hex\nline1\nline2\nline3\n```",
                Some(LangValue::Hex),
                "line1\nline2\nline3\n",
            ),
            (
                "```json\nline1\nline2\nline3\n```",
                Some(LangValue::Json),
                "line1\nline2\nline3\n",
            ),
            (
                "```graphql\nline1\nline2\nline3\n```",
                Some(LangValue::GraphQl),
                "line1\nline2\nline3\n",
            ),
            (
                "```graphql     \nline1\nline2\nline3\n```",
                Some(LangValue::GraphQl),
                "line1\nline2\nline3\n",
            ),
        ];

        for (text, lang, value) in datas.iter() {
            let mut reader = Reader::init(text);
            let multiline = multiline_string(&mut reader).unwrap();
            assert_eq!(multiline.lang.unwrap().value, *lang);
            assert_eq!(multiline.value.elements[0].to_string(), value.to_string())
        }
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
            MultilineString {
                lang: None,
                value: Template {
                    quotes: false,
                    elements: vec![],
                    source_info: SourceInfo::new(1, 4, 1, 4),
                },
            }
        );

        let mut reader = Reader::init("```\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                lang: Some(Lang {
                    value: None,
                    space: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::new(1, 4, 1, 4)
                    },
                    newline: Whitespace {
                        value: "\n".to_string(),
                        source_info: SourceInfo::new(1, 4, 2, 1)
                    }
                }),
                value: Template {
                    quotes: false,
                    elements: vec![],
                    source_info: SourceInfo::new(2, 1, 2, 1),
                },
            }
        );
        let mut reader = Reader::init("```\r\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                lang: Some(Lang {
                    value: None,
                    space: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::new(1, 4, 1, 4)
                    },
                    newline: Whitespace {
                        value: "\r\n".to_string(),
                        source_info: SourceInfo::new(1, 4, 2, 1)
                    }
                }),
                value: Template {
                    quotes: false,
                    elements: vec![],
                    source_info: SourceInfo::new(2, 1, 2, 1),
                },
            }
        );
    }

    #[test]
    fn test_multiline_string_hello() {
        let mut reader = Reader::init("```Hello World!```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                lang: None,
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "Hello World!".to_string(),
                        encoded: "Hello World!".to_string(),
                    }],
                    source_info: SourceInfo::new(1, 4, 1, 16),
                },
            }
        );
    }

    #[test]
    fn test_multiline_string_base64_prefix() {
        let mut reader = Reader::init("```base64_inline```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                lang: None,
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "base64_inline".to_string(),
                        encoded: "base64_inline".to_string(),
                    }],
                    source_info: SourceInfo::new(1, 4, 1, 17),
                },
            }
        );
    }

    #[test]
    fn test_multiline_string_csv() {
        let mut reader = Reader::init("```\nline1\nline2\nline3\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                lang: Some(Lang {
                    value: None,
                    space: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::new(1, 4, 1, 4),
                    },
                    newline: Whitespace {
                        value: "\n".to_string(),
                        source_info: SourceInfo::new(1, 4, 2, 1),
                    }
                }),
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "line1\nline2\nline3\n".to_string(),
                        encoded: "line1\nline2\nline3\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 5, 1),
                },
            }
        );
    }

    #[test]
    fn test_multiline_string_one_empty_line() {
        // one newline
        // the value takes the value of the newline??
        let mut reader = Reader::init("```\n\n```");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                lang: Some(Lang {
                    value: None,
                    space: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::new(1, 4, 1, 4),
                    },
                    newline: Whitespace {
                        value: "\n".to_string(),
                        source_info: SourceInfo::new(1, 4, 2, 1),
                    }
                }),
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "\n".to_string(),
                        encoded: "\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 3, 1),
                },
            }
        );

        // one cr
        let mut reader = Reader::init("```\n\r\n````");
        assert_eq!(
            multiline_string(&mut reader).unwrap(),
            MultilineString {
                lang: Some(Lang {
                    value: None,
                    space: Whitespace {
                        value: "".to_string(),
                        source_info: SourceInfo::new(1, 4, 1, 4),
                    },
                    newline: Whitespace {
                        value: "\n".to_string(),
                        source_info: SourceInfo::new(1, 4, 2, 1),
                    }
                }),
                value: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "\r\n".to_string(),
                        encoded: "\r\n".to_string(),
                    }],
                    source_info: SourceInfo::new(2, 1, 3, 1),
                },
            }
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
                value: String::from("```")
            }
        );
        assert!(error.recoverable);

        let mut reader = Reader::init("```\nxxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 2, column: 4 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("```")
            }
        );
        assert!(!error.recoverable);

        let mut reader = Reader::init("```xxx");
        let error = multiline_string(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 7 });
        assert_eq!(
            error.inner,
            ParseError::Expecting {
                value: String::from("```")
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
                quotes: false,
                elements: vec![],
                source_info: SourceInfo::new(1, 1, 1, 1),
            }
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("hello```");
        assert_eq!(
            multiline_string_value(&mut reader).unwrap(),
            Template {
                quotes: false,
                elements: vec![TemplateElement::String {
                    value: "hello".to_string(),
                    encoded: "hello".to_string(),
                }],
                source_info: SourceInfo::new(1, 1, 1, 6),
            }
        );
        assert_eq!(reader.state.cursor, 8);
    }
}
