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
use crate::ast::Bytes;
use crate::combinator::choice;
use crate::parser::json::parse as parse_json;
use crate::parser::multiline::multiline_string;
use crate::parser::string::backtick_template;
use crate::parser::{primitives, xml, ParseResult};
use crate::reader::Reader;

pub fn bytes(reader: &mut Reader) -> ParseResult<Bytes> {
    choice(
        &[
            multiline_string_bytes,
            string_bytes,
            json_bytes,
            xml_bytes,
            base64_bytes,
            hex_bytes,
            file_bytes,
        ],
        reader,
    )
}

fn xml_bytes(reader: &mut Reader) -> ParseResult<Bytes> {
    match xml::parse(reader) {
        Err(e) => Err(e),
        Ok(value) => Ok(Bytes::Xml(value)),
    }
}

fn json_bytes(reader: &mut Reader) -> ParseResult<Bytes> {
    match parse_json(reader) {
        Err(e) => Err(e),
        Ok(value) => Ok(Bytes::Json(value)),
    }
}

fn file_bytes(reader: &mut Reader) -> ParseResult<Bytes> {
    primitives::file(reader).map(Bytes::File)
}

fn base64_bytes(reader: &mut Reader) -> ParseResult<Bytes> {
    primitives::base64(reader).map(Bytes::Base64)
}

fn hex_bytes(reader: &mut Reader) -> ParseResult<Bytes> {
    primitives::hex(reader).map(Bytes::Hex)
}

pub fn multiline_string_bytes(reader: &mut Reader) -> ParseResult<Bytes> {
    multiline_string(reader).map(Bytes::MultilineString)
}

fn string_bytes(reader: &mut Reader) -> ParseResult<Bytes> {
    backtick_template(reader).map(Bytes::OnelineString)
}

#[cfg(test)]
mod tests {
    use super::super::error::*;
    use super::*;
    use crate::ast::{JsonListElement, JsonValue, SourceInfo, Template, TemplateElement};
    use crate::reader::Pos;
    use crate::typing::ToSource;

    #[test]
    fn test_bytes_json() {
        let mut reader = Reader::new("[1,2,3] ");
        assert_eq!(
            bytes(&mut reader).unwrap(),
            Bytes::Json(JsonValue::List {
                space0: String::new(),
                elements: vec![
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("1".to_string()),
                        space1: String::new(),
                    },
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("2".to_string()),
                        space1: String::new(),
                    },
                    JsonListElement {
                        space0: String::new(),
                        value: JsonValue::Number("3".to_string()),
                        space1: String::new(),
                    },
                ],
            })
        );
        assert_eq!(reader.cursor().index, 7);

        let mut reader = Reader::new("{ } ");
        assert_eq!(
            bytes(&mut reader).unwrap(),
            Bytes::Json(JsonValue::Object {
                space0: " ".to_string(),
                elements: vec![],
            })
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("true");
        assert_eq!(
            bytes(&mut reader).unwrap(),
            Bytes::Json(JsonValue::Boolean(true))
        );
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("\"\" x");
        assert_eq!(
            bytes(&mut reader).unwrap(),
            Bytes::Json(JsonValue::String(Template {
                delimiter: Some('"'),
                elements: vec![],
                source_info: SourceInfo::new(Pos::new(1, 2), Pos::new(1, 2)),
            }))
        );
        assert_eq!(reader.cursor().index, 2);
    }

    #[test]
    fn test_bytes_xml() {
        let mut reader = Reader::new("<a/>");
        assert_eq!(
            bytes(&mut reader).unwrap(),
            Bytes::Xml(String::from("<a/>"))
        );
    }

    #[test]
    fn test_bytes_json_error() {
        let mut reader = Reader::new("{ x ");
        let error = bytes(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 3 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "\"".to_string()
            },
        );
    }

    #[test]
    fn test_bytes_multilines_error() {
        let mut reader = Reader::new("```\nxxx ");
        let error = bytes(&mut reader).err().unwrap();
        assert_eq!(error.pos, Pos { line: 2, column: 5 });
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: "```".to_string()
            }
        );
    }

    #[test]
    fn test_bytes_eof() {
        let mut reader = Reader::new("");
        let error = bytes(&mut reader).err().unwrap();
        assert_eq!(
            error.kind,
            ParseErrorKind::Expecting {
                value: String::from("file")
            }
        );
        assert!(error.recoverable);
    }

    #[test]
    fn test_json_bytes() {
        let mut reader = Reader::new("100");
        assert_eq!(
            json_bytes(&mut reader).unwrap(),
            Bytes::Json(JsonValue::Number("100".to_string()))
        );
    }

    #[test]
    fn test_bytes_string() {
        let mut reader = Reader::new("`foo`  ");
        assert_eq!(
            bytes(&mut reader).unwrap(),
            Bytes::OnelineString(Template {
                delimiter: Some('`'),
                elements: vec![TemplateElement::String {
                    value: "foo".to_string(),
                    source: "foo".to_source()
                }],
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 6))
            })
        );
        assert_eq!(reader.cursor().index, 5);
    }
}
