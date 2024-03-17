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
use crate::parser::error::*;
use crate::parser::primitives::try_literal;
use crate::parser::reader::Reader;
use crate::parser::template::template;
use crate::parser::{string, ParseResult};

pub fn parse(reader: &mut Reader) -> ParseResult<Template> {
    let start = reader.state;

    let mut elements = vec![];
    loop {
        let start = reader.state;
        match template(reader) {
            Ok(expr) => {
                let element = TemplateElement::Expression(expr);
                elements.push(element);
            }
            Err(e) => {
                if e.recoverable {
                    let value = filename_content(reader)?;
                    if value.is_empty() {
                        break;
                    }
                    let encoded: String = reader.buffer[start.cursor..reader.state.cursor]
                        .iter()
                        .collect();
                    let element = TemplateElement::String { value, encoded };
                    elements.push(element);
                } else {
                    return Err(e);
                }
            }
        }
    }
    if elements.is_empty() {
        let inner = ParseError::Filename;
        return Err(Error::new(start.pos, false, inner));
    }
    if let Some(TemplateElement::String { encoded, .. }) = elements.first() {
        if encoded.starts_with('[') {
            let inner = ParseError::Expecting {
                value: "filename".to_string(),
            };
            return Err(Error::new(start.pos, false, inner));
        }
    }

    let end = reader.state;
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
        let save = reader.state;
        match reader.read() {
            None => break,
            Some(c) => {
                if c.is_alphanumeric()
                    || c == '_'
                    || c == '-'
                    || c == '.'
                    || c == '['
                    || c == ']'
                    || c == '@'
                    || c == '$'
                    || c == '/'
                    || c == ':'
                {
                    s.push(c);
                } else {
                    reader.state = save;
                    break;
                }
            }
        }
    }

    s
}

fn filename_escaped_char(reader: &mut Reader) -> ParseResult<char> {
    try_literal("\\", reader)?;
    let start = reader.state;
    match reader.read() {
        Some(';') => Ok(';'),
        Some('#') => Ok('#'),
        Some('[') => Ok('['),
        Some(' ') => Ok(' '),
        Some(']') => Ok(']'),
        Some(':') => Ok(':'),
        Some('\\') => Ok('\\'),
        Some('/') => Ok('/'),
        Some('b') => Ok('\x08'),
        Some('f') => Ok('\x0c'),
        Some('n') => Ok('\n'),
        Some('r') => Ok('\r'),
        Some('t') => Ok('\t'),
        Some('u') => string::unicode(reader),
        _ => Err(Error::new(start.pos, false, ParseError::EscapeChar)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Pos;

    #[test]
    fn test_filename() {
        let mut reader = Reader::new("data/data.bin");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "data/data.bin".to_string(),
                    encoded: "data/data.bin".to_string()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 14)),
            }
        );
        assert_eq!(reader.state.cursor, 13);

        let mut reader = Reader::new("data.bin");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                //value: String::from("data.bin"),
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "data.bin".to_string(),
                    encoded: "data.bin".to_string()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 9)),
            }
        );
        assert_eq!(reader.state.cursor, 8);
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
                    encoded: "file\\ with\\ spaces".to_string()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19)),
            }
        );
        assert_eq!(reader.state.cursor, 18);
    }

    #[test]
    fn test_filename_error() {
        let mut reader = Reader::new("???");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.inner, ParseError::Filename);
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
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
                        encoded: "foo_".to_string(),
                    },
                    TemplateElement::Expression(Expr {
                        space0: Whitespace {
                            value: "".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 7)),
                        },
                        variable: Variable {
                            name: "bar".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                        },
                        space1: Whitespace {
                            value: "".to_string(),
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
                        encoded: "foo_".to_string(),
                    },
                    TemplateElement::Expression(Expr {
                        space0: Whitespace {
                            value: "".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 7)),
                        },
                        variable: Variable {
                            name: "bar".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 7), Pos::new(1, 10)),
                        },
                        space1: Whitespace {
                            value: "".to_string(),
                            source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(1, 10)),
                        },
                    }),
                    TemplateElement::String {
                        value: "_baz".to_string(),
                        encoded: "_baz".to_string(),
                    },
                ],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 16)),
            }
        );
    }
}
