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
use super::placeholder;
use crate::ast::{SourceInfo, Template, TemplateElement};
use crate::parser::primitives::try_literal;
use crate::parser::{string, ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;
use crate::typing::ToSource;

/// Parse a filename.
///
/// A few characters need to be escaped such as space
/// for example: file\ with\ space.txt
/// This is very similar to the behaviour in a standard shell.
///
pub fn parse(reader: &mut Reader) -> ParseResult<Template> {
    let start = reader.cursor();

    let mut elements = vec![];
    loop {
        let start = reader.cursor();
        match placeholder::parse(reader) {
            Ok(placeholder) => {
                let element = TemplateElement::Placeholder(placeholder);
                elements.push(element);
            }
            Err(e) => {
                if e.recoverable {
                    let value = filename_content(reader)?;
                    if value.is_empty() {
                        break;
                    }
                    let source = reader.read_from(start.index).to_source();
                    let element = TemplateElement::String { value, source };
                    elements.push(element);
                } else {
                    return Err(e);
                }
            }
        }
    }
    if elements.is_empty() {
        let kind = ParseErrorKind::Filename;
        return Err(ParseError::new(start.pos, false, kind));
    }
    if let Some(TemplateElement::String { source, .. }) = elements.first() {
        if source.starts_with('[') {
            let kind = ParseErrorKind::Expecting {
                value: "filename".to_string(),
            };
            return Err(ParseError::new(start.pos, false, kind));
        }
    }

    let end = reader.cursor();
    Ok(Template {
        delimiter: None,
        elements,
        source_info: SourceInfo {
            start: start.pos,
            end: end.pos,
        },
    })
}

fn filename_content(reader: &mut Reader) -> ParseResult<String> {
    let mut s = String::new();
    loop {
        match filename_escaped_char(reader) {
            Ok(c) => {
                s.push(c);
            }
            Err(e) => {
                if e.recoverable {
                    let s2 = filename_text(reader);
                    if s2.is_empty() {
                        break;
                    } else {
                        s.push_str(&s2);
                    }
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(s)
}

fn filename_text(reader: &mut Reader) -> String {
    let mut s = String::new();
    loop {
        let save = reader.cursor();
        match reader.read() {
            None => break,
            Some(c) => {
                if ['#', ';', '{', '}', ' ', '\n', '\\'].contains(&c) {
                    reader.seek(save);
                    break;
                } else {
                    s.push(c);
                }
            }
        }
    }
    s
}

fn filename_escaped_char(reader: &mut Reader) -> ParseResult<char> {
    try_literal("\\", reader)?;
    let start = reader.cursor();
    match reader.read() {
        Some('\\') => Ok('\\'),
        Some('b') => Ok('\x08'),
        Some('f') => Ok('\x0c'),
        Some('n') => Ok('\n'),
        Some('r') => Ok('\r'),
        Some('t') => Ok('\t'),
        Some('#') => Ok('#'),
        Some(';') => Ok(';'),
        Some(' ') => Ok(' '),
        Some('{') => Ok('{'),
        Some('}') => Ok('}'),
        Some('u') => string::unicode(reader),
        _ => Err(ParseError::new(
            start.pos,
            false,
            ParseErrorKind::EscapeChar,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expr, ExprKind, Placeholder, Variable, Whitespace};
    use crate::reader::Pos;

    #[test]
    fn test_filename() {
        let mut reader = Reader::new("data/data.bin");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "data/data.bin".to_string(),
                    source: "data/data.bin".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 14)),
            }
        );
        assert_eq!(reader.cursor().index, 13);

        let mut reader = Reader::new("data.bin");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                //value: String::from("data.bin"),
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "data.bin".to_string(),
                    source: "data.bin".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 9)),
            }
        );
        assert_eq!(reader.cursor().index, 8);
    }

    #[test]
    fn test_include_space() {
        let mut reader = Reader::new("file\\ with\\ spaces");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "file with spaces".to_string(),
                    source: "file\\ with\\ spaces".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19)),
            }
        );
        assert_eq!(reader.cursor().index, 18);
    }

    #[test]
    fn test_escaped_chars() {
        let mut reader = Reader::new("filename\\{"); // to the possible escaped chars
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "filename{".to_string(),
                    source: "filename\\{".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 11)),
            }
        );
        assert_eq!(reader.cursor().index, 10);
    }

    #[test]
    fn test_filename_error() {
        let mut reader = Reader::new("{");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.kind, ParseErrorKind::Filename);
        assert_eq!(error.pos, Pos { line: 1, column: 1 });

        let mut reader = Reader::new("\\:");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.kind, ParseErrorKind::EscapeChar);
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
    }

    #[test]
    fn test_filename_with_variables() {
        let mut reader = Reader::new("foo_{{bar}}");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![
                    TemplateElement::String {
                        value: "foo_".to_string(),
                        source: "foo_".to_source(),
                    },
                    TemplateElement::Placeholder(Placeholder {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 7)),
                        },
                        expr: Expr {
                            kind: ExprKind::Variable(Variable {
                                name: "bar".to_string(),
                                source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                            }),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                        },
                        space1: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(1, 10)),
                        },
                    })
                ],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 12)),
            }
        );

        let mut reader = Reader::new("foo_{{bar}}_baz");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![
                    TemplateElement::String {
                        value: "foo_".to_string(),
                        source: "foo_".to_source(),
                    },
                    TemplateElement::Placeholder(Placeholder {
                        space0: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 7)),
                        },
                        expr: Expr {
                            kind: ExprKind::Variable(Variable {
                                name: "bar".to_string(),
                                source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                            }),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                        },
                        space1: Whitespace {
                            value: String::new(),
                            source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(1, 10)),
                        },
                    }),
                    TemplateElement::String {
                        value: "_baz".to_string(),
                        source: "_baz".to_source(),
                    },
                ],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 16)),
            }
        );
    }
}
