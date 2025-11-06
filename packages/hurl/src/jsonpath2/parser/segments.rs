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

use super::primitives::match_str;
use super::ParseResult;
use crate::jsonpath2::ast::segment::{ChildSegment, DescendantSegment, Segment};
use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
use crate::jsonpath2::parser::primitives::expect_str;
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

///  Try to parse a segment
fn try_segment(reader: &mut Reader) -> ParseResult<Option<Segment>> {
    if let Some(descendant_segment) = try_descendant_segment(reader)? {
        return Ok(Some(Segment::Descendant(descendant_segment)));
    } else if let Some(child_segment) = try_child_segment(reader)? {
        return Ok(Some(Segment::Child(child_segment)));
    }
    Ok(None)
}

/// Try to parse a child segment
fn try_child_segment(reader: &mut Reader) -> ParseResult<Option<ChildSegment>> {
    if let Some(selectors) = try_bracketed_selection(reader)? {
        Ok(Some(ChildSegment::new(selectors)))
    } else if match_str(".", reader) {
        let save_state = reader.cursor();
        if match_str("*", reader) {
            Ok(Some(ChildSegment::new(vec![Selector::Wildcard(
                WildcardSelector,
            )])))
        } else if let Ok(name) = member_name_shorthand(reader) {
            Ok(Some(ChildSegment::new(vec![Selector::Name(
                NameSelector::new(name),
            )])))
        } else {
            Err(ParseError::new(
                save_state.pos,
                ParseErrorKind::Expecting(
                    "a wildcard-selector or member-name shorthand".to_string(),
                ),
            ))
        }
    } else {
        Ok(None)
    }
}

// Try to parse a descendant segment
fn try_descendant_segment(reader: &mut Reader) -> ParseResult<Option<DescendantSegment>> {
    if match_str("..", reader) {
        let save_state = reader.cursor();
        if let Some(selectors) = try_bracketed_selection(reader)? {
            Ok(Some(DescendantSegment::new(selectors)))
        } else if match_str("*", reader) {
            Ok(Some(DescendantSegment::new(vec![Selector::Wildcard(
                WildcardSelector,
            )])))
        } else if let Ok(name) = member_name_shorthand(reader) {
            Ok(Some(DescendantSegment::new(vec![Selector::Name(
                NameSelector::new(name),
            )])))
        } else {
            Err(ParseError::new(
                save_state.pos,
                ParseErrorKind::Expecting(
                    "a bracketed-selection, wildcard-selector or member-name shorthand".to_string(),
                ),
            ))
        }
    } else {
        Ok(None)
    }
}

fn try_bracketed_selection(reader: &mut Reader) -> ParseResult<Option<Vec<Selector>>> {
    if match_str("[", reader) {
        let selectors = selectors::parse(reader)?;
        expect_str("]", reader)?;
        Ok(Some(selectors))
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

    use crate::jsonpath2::ast::selector::{
        IndexSelector, NameSelector, Selector, WildcardSelector,
    };
    use hurl_core::reader::{CharPos, Pos, Reader};

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
    pub fn test_segment() {
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
    pub fn test_child_segment() {
        let mut reader = Reader::new("['store']");
        assert_eq!(
            try_child_segment(&mut reader).unwrap().unwrap(),
            ChildSegment::new(vec![Selector::Name(NameSelector::new("store".to_string()))])
        );
        assert_eq!(reader.cursor().index, CharPos(9));
    }

    #[test]
    pub fn test_child_segment_error() {
        let mut reader = Reader::new(".1");
        assert_eq!(
            try_child_segment(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 2),
                ParseErrorKind::Expecting(
                    "a wildcard-selector or member-name shorthand".to_string()
                )
            )
        );
    }

    #[test]
    pub fn test_descendant_segment() {
        let mut reader = Reader::new("..[1]");
        assert_eq!(
            try_descendant_segment(&mut reader).unwrap().unwrap(),
            DescendantSegment::new(vec![Selector::Index(IndexSelector::new(1))])
        );
        assert_eq!(reader.cursor().index, CharPos(5));

        let mut reader = Reader::new("..[1]");
        assert_eq!(
            try_segment(&mut reader).unwrap().unwrap(),
            Segment::Descendant(DescendantSegment::new(vec![Selector::Index(
                IndexSelector::new(1)
            )]))
        );
        assert_eq!(reader.cursor().index, CharPos(5));
    }

    #[test]
    pub fn test_descendant_segment_error() {
        let mut reader = Reader::new("..1");
        assert_eq!(
            try_descendant_segment(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 3),
                ParseErrorKind::Expecting(
                    "a bracketed-selection, wildcard-selector or member-name shorthand".to_string()
                )
            )
        );
    }

    #[test]
    pub fn test_bracketed_selection() {
        let mut reader = Reader::new("[1,'store']");
        assert_eq!(
            try_bracketed_selection(&mut reader).unwrap().unwrap(),
            vec![
                Selector::Index(IndexSelector::new(1)),
                Selector::Name(NameSelector::new("store".to_string()))
            ]
        );
        assert_eq!(reader.cursor().index, CharPos(11));
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
