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

use super::{primitives::expect_str, primitives::match_str, ParseResult};
use crate::jsonpath2::ast::segment::{ChildSegment, DescendantSegment, Segment};
use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
use crate::jsonpath2::parser::{selectors, ParseError, ParseErrorKind};
use hurl_core::reader::Reader;

/// Parse segments
pub fn parse(reader: &mut Reader) -> ParseResult<Vec<Segment>> {
    let mut segments = vec![];
    while let Some(segment) = try_segment(reader)? {
        segments.push(segment);
    }
    Ok(segments)
}

fn try_segment(reader: &mut Reader) -> ParseResult<Option<Segment>> {
    if let Some(segment) = try_segment_shorthand(reader)? {
        return Ok(Some(segment));
    }

    let is_descendant_segment = if match_str("..[", reader) {
        true
    } else if match_str("[", reader) {
        false
    } else {
        return Ok(None);
    };
    let selectors = selectors::parse(reader)?;
    expect_str("]", reader)?;
    let segment = if is_descendant_segment {
        Segment::Descendant(DescendantSegment::new(selectors))
    } else {
        Segment::Child(ChildSegment::new(selectors))
    };
    Ok(Some(segment))
}

/// try to parse a shorthand notation
fn try_segment_shorthand(reader: &mut Reader) -> ParseResult<Option<Segment>> {
    if match_str(".*", reader) {
        Ok(Some(Segment::Child(ChildSegment::new(vec![
            Selector::Wildcard(WildcardSelector),
        ]))))
    } else if match_str("..*", reader) {
        Ok(Some(Segment::Descendant(DescendantSegment::new(vec![
            Selector::Wildcard(WildcardSelector),
        ]))))
    } else if match_str("..", reader) {
        let name = member_name_shorthand(reader)?;
        Ok(Some(Segment::Descendant(DescendantSegment::new(vec![
            Selector::Name(NameSelector::new(name)),
        ]))))
    } else if match_str(".", reader) {
        let name = member_name_shorthand(reader)?;
        Ok(Some(Segment::Child(ChildSegment::new(vec![
            Selector::Name(NameSelector::new(name)),
        ]))))
    } else {
        Ok(None)
    }
}

fn member_name_shorthand(reader: &mut Reader) -> ParseResult<String> {
    let mut value = if let Some(c) = name_first(reader) {
        c.to_string()
    } else {
        return Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("a member name".to_string()),
        ));
    };
    while let Some(c) = name_char(reader) {
        value.push(c);
    }

    Ok(value)
}

fn name_first(reader: &mut Reader) -> Option<char> {
    let save = reader.cursor();
    if let Some(c) = reader.read() {
        let unicode = c as u32;
        if c.is_alphabetic()
            || c == '_'
            || (0x80..=0xD7FF).contains(&unicode)
            || (0xE000..=0x0010_FFFF).contains(&unicode)
        {
            Some(c)
        } else {
            reader.seek(save);
            None
        }
    } else {
        None
    }
}

fn name_char(reader: &mut Reader) -> Option<char> {
    name_first(reader).or_else(|| digit(reader))
}

fn digit(reader: &mut Reader) -> Option<char> {
    let save = reader.cursor();
    if let Some(c) = reader.read() {
        if c.is_ascii_digit() {
            Some(c)
        } else {
            reader.seek(save);
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
    use hurl_core::reader::{CharPos, Reader};

    use super::*;

    #[test]
    pub fn test_segments() {
        let mut reader = Reader::new("['isbn']]");

        assert_eq!(
            parse(&mut reader).unwrap(),
            vec![Segment::Child(ChildSegment::new(vec![Selector::Name(
                NameSelector::new("isbn".to_string())
            )]))]
        );
        assert_eq!(reader.cursor().index, CharPos(8));
    }

    #[test]
    pub fn test_child_segment() {
        let mut reader = Reader::new("['store']");
        assert_eq!(
            try_segment(&mut reader).unwrap().unwrap(),
            Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                "store".to_string()
            ))]))
        );
        assert_eq!(reader.cursor().index, CharPos(9));

        let mut reader = Reader::new("['store']");
        assert_eq!(
            try_segment(&mut reader).unwrap().unwrap(),
            Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                "store".to_string()
            ))]))
        );
        assert_eq!(reader.cursor().index, CharPos(9));
    }

    #[test]
    pub fn test_shorthand_notation() {
        let mut reader = Reader::new(".book");
        assert_eq!(
            try_segment(&mut reader).unwrap().unwrap(),
            Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                "book".to_string()
            ))]))
        );
        assert_eq!(reader.cursor().index, CharPos(5));

        let mut reader = Reader::new(".☺");
        assert_eq!(
            try_segment(&mut reader).unwrap().unwrap(),
            Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                "☺".to_string()
            ))]))
        );
        assert_eq!(reader.cursor().index, CharPos(2));

        let mut reader = Reader::new("..book");
        assert_eq!(
            try_segment(&mut reader).unwrap().unwrap(),
            Segment::Descendant(DescendantSegment::new(vec![Selector::Name(
                NameSelector::new("book".to_string())
            )]))
        );
        assert_eq!(reader.cursor().index, CharPos(6));

        let mut reader = Reader::new(".*");
        assert_eq!(
            try_segment(&mut reader).unwrap().unwrap(),
            Segment::Child(ChildSegment::new(vec![Selector::Wildcard(
                WildcardSelector
            )]))
        );
        assert_eq!(reader.cursor().index, CharPos(2));

        let mut reader = Reader::new("..*");
        assert_eq!(
            try_segment(&mut reader).unwrap().unwrap(),
            Segment::Descendant(DescendantSegment::new(vec![Selector::Wildcard(
                WildcardSelector
            )]))
        );
        assert_eq!(reader.cursor().index, CharPos(3));
    }

    #[test]
    pub fn test_name_first() {
        let mut reader = Reader::new("a");
        assert_eq!(name_first(&mut reader).unwrap(), 'a');

        let mut reader = Reader::new("_");
        assert_eq!(name_first(&mut reader).unwrap(), '_');

        let mut reader = Reader::new("☺");
        assert_eq!(name_first(&mut reader).unwrap(), '☺');

        let mut reader = Reader::new("1");
        assert!(name_first(&mut reader).is_none());
    }
}
