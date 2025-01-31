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

/// Parses a string into a [key string](https://hurl.dev/docs/grammar.html#key-string).
pub fn parse(reader: &mut Reader) -> ParseResult<Template> {
    let start = reader.cursor();
    let mut elements = vec![];
    loop {
        let save_state = reader.cursor();
        match placeholder::parse(reader) {
            Ok(placeholder) => {
                let element = TemplateElement::Placeholder(placeholder);
                elements.push(element);
            }
            Err(e) => {
                if e.recoverable {
                    reader.seek(save_state);
                    let value = key_string_content(reader)?;
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
        let kind = ParseErrorKind::Expecting {
            value: "key-string".to_string(),
        };
        return Err(ParseError::new(start.pos, false, kind));
    }
    if let Some(TemplateElement::String { source, .. }) = elements.first() {
        if source.starts_with('[') {
            let kind = ParseErrorKind::Expecting {
                value: "key-string".to_string(),
            };
            return Err(ParseError::new(start.pos, false, kind));
        }
    }

    let end = reader.cursor();
    Ok(Template {
        delimiter: None,
        elements,
        source_info: SourceInfo::new(start.pos, end.pos),
    })
}

fn key_string_content(reader: &mut Reader) -> ParseResult<String> {
    let mut s = String::new();
    loop {
        match key_string_escaped_char(reader) {
            Ok(c) => {
                s.push(c);
            }
            Err(e) => {
                if e.recoverable {
                    let s2 = key_string_text(reader);
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

fn key_string_text(reader: &mut Reader) -> String {
    let mut s = String::new();
    loop {
        let save = reader.cursor();
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
                {
                    s.push(c);
                } else {
                    reader.seek(save);
                    break;
                }
            }
        }
    }

    s
}

fn key_string_escaped_char(reader: &mut Reader) -> ParseResult<char> {
    try_literal("\\", reader)?;
    let start = reader.cursor();
    match reader.read() {
        Some('#') => Ok('#'),
        Some(':') => Ok(':'),
        Some('\\') => Ok('\\'),
        Some('/') => Ok('/'),
        Some('b') => Ok('\x08'),
        Some('f') => Ok('\x0c'),
        Some('n') => Ok('\n'),
        Some('r') => Ok('\r'),
        Some('t') => Ok('\t'),
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
    use crate::reader::Pos;

    #[test]
    fn test_key_string() {
        let mut reader = Reader::new("aaa\\: ");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "aaa:".to_string(),
                    source: "aaa\\:".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6)),
            }
        );
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("$top:");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "$top".to_string(),
                    source: "$top".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 5)),
            }
        );
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("key\\u{20}\\u{3a} :");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "key :".to_string(),
                    source: "key\\u{20}\\u{3a}".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 16)),
            }
        );
        assert_eq!(reader.cursor().index, 15);

        let mut reader = Reader::new("values\\u{5b}0\\u{5d} :");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "values[0]".to_string(),
                    source: "values\\u{5b}0\\u{5d}".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 20)),
            }
        );
        assert_eq!(reader.cursor().index, 19);

        let mut reader = Reader::new("values[0] :");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "values[0]".to_string(),
                    source: "values[0]".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 10)),
            }
        );
        assert_eq!(reader.cursor().index, 9);

        let mut reader = Reader::new("\\u{5b}0\\u{5d}");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "[0]".to_string(),
                    source: "\\u{5b}0\\u{5d}".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 14)),
            }
        );
        assert_eq!(reader.cursor().index, 13);
    }

    #[test]
    fn test_key_string_error() {
        let mut reader = Reader::new("");
        let error = parse(&mut reader).err().unwrap();
        assert!(!error.recoverable);
        assert_eq!(error.pos, Pos { line: 1, column: 1 });

        let mut reader = Reader::new("{{key");
        let error = parse(&mut reader).err().unwrap();
        assert!(!error.recoverable);
        assert_eq!(error.pos, Pos { line: 1, column: 6 });

        // key string can not start with a '[' (reserved for section)
        let mut reader = Reader::new("[0]:");
        let error = parse(&mut reader).err().unwrap();
        assert!(!error.recoverable);
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("\\l");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 2 });
        assert_eq!(error.kind, ParseErrorKind::EscapeChar);

        let mut reader = Reader::new(r#"{"id":1}"#);
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "key-string".to_string()
            }
        );
    }

    #[test]
    fn test_key_string_content() {
        let mut reader = Reader::new("aaa\\:");
        assert_eq!(key_string_content(&mut reader).unwrap(), "aaa:");
    }

    #[test]
    fn test_key_string_text() {
        let mut reader = Reader::new("aaa\\:");
        assert_eq!(key_string_text(&mut reader), "aaa");
        assert_eq!(reader.cursor().index, 3);
    }

    #[test]
    fn test_key_string_escaped_char() {
        let mut reader = Reader::new("\\u{0a}");
        assert_eq!(key_string_escaped_char(&mut reader).unwrap(), '\n');
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("\\:");
        assert_eq!(key_string_escaped_char(&mut reader).unwrap(), ':');
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("x");
        let error = key_string_escaped_char(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\\".to_string()
            }
        );
        assert!(error.recoverable);
        assert_eq!(reader.cursor().index, 0);
    }
}
