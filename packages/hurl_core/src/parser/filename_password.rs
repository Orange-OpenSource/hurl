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

/// Parse a filename with an optional password
///
/// This is very similar to the filename parser.
/// There is still a small different due to the password delimiter ":"
/// If the ':' character is part of the filename, it must be escaped.
/// While in the standard filename parser, you can not escape this character at all.
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
                    let value = filename_password_content(reader)?;
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

fn filename_password_content(reader: &mut Reader) -> ParseResult<String> {
    let mut s = String::new();
    loop {
        match filename_password_escaped_char(reader) {
            Ok(c) => {
                // ':' is escaped so that is it not recognized as the password delimiter
                // This is not due to Hurl format
                if c == ':' {
                    s.push('\\');
                }
                s.push(c);
            }
            Err(e) => {
                if e.recoverable {
                    let s2 = filename_password_text(reader);
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

fn filename_password_text(reader: &mut Reader) -> String {
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

fn filename_password_escaped_char(reader: &mut Reader) -> ParseResult<char> {
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
        Some(':') => Ok(':'),
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
    fn test_filename_with_password() {
        let mut reader = Reader::new("file\\:123:pwd\\:\\#:");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "file\\:123:pwd\\:#:".to_string(),
                    source: "file\\:123:pwd\\:\\#:".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 19)),
            }
        );
        assert_eq!(reader.cursor().index, 18);
    }

    #[test]
    fn test_parse() {
        let mut reader = Reader::new("/etc/client-cert.pem #foo");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "/etc/client-cert.pem".to_string(),
                    source: "/etc/client-cert.pem".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 21)),
            }
        );
        assert_eq!(reader.cursor().index, 20);
    }
}
