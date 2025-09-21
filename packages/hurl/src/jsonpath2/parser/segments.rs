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
/// Only parse singular segments by setting `only_singular` to true
pub fn parse(reader: &mut Reader, only_singular: bool) -> ParseResult<Vec<Segment>> {
    let mut segments = vec![];
    let mut current_pos = reader.cursor().pos;

    // Parsing singular segments
    // In the spec, it is defined as parsing only name and index segments
    // For reporting error, we will first parse any segment instead, and then check whether the segment is singular
    while let Some(segment) = try_segment(reader)? {
        if only_singular && !segment.is_singular() {
            return Err(ParseError::new(
                current_pos,
                ParseErrorKind::Expecting("singular segment".to_string()),
            ));
        }
        segments.push(segment);
        current_pos = reader.cursor().pos;
    }
    Ok(segments)
}

impl Segment {
    fn is_singular(&self) -> bool {
        if let Segment::Child(child_segment) = self {
            for selector in child_segment.selectors() {
                if !matches!(selector, Selector::Name(_)) && !matches!(selector, Selector::Index(_))
                {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
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
    let mut name = alpha(reader)?.to_string();
    name.push_str(&reader.read_while(|c| c.is_alphanumeric()));
    Ok(name)
}

fn alpha(reader: &mut Reader) -> ParseResult<char> {
    let pos = reader.cursor().pos;
    if let Some(c) = reader.read() {
        if c.is_alphabetic() {
            Ok(c)
        } else {
            let kind = ParseErrorKind::Expecting("a character".to_string());
            Err(ParseError::new(pos, kind))
        }
    } else {
        let kind = ParseErrorKind::Expecting("a character".to_string());
        Err(ParseError::new(pos, kind))
    }
}

#[cfg(test)]
mod tests {

    use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
    use hurl_core::reader::{CharPos, Pos, Reader};

    use super::*;

    #[test]
    pub fn test_segments() {
        let mut reader = Reader::new("['isbn']]");

        assert_eq!(
            parse(&mut reader, false).unwrap(),
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
    pub fn test_singular_segments() {
        let mut reader = Reader::new(".*");

        assert_eq!(
            parse(&mut reader, true).unwrap_err(),
            ParseError::new(
                Pos::new(1, 1),
                ParseErrorKind::Expecting("singular segment".to_string())
            )
        );
        assert_eq!(reader.cursor().index, CharPos(2));
    }

    #[test]
    pub fn test_is_singular() {
        assert!(
            Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                "name".to_string()
            ))]))
            .is_singular()
        );
        assert!(!Segment::Child(ChildSegment::new(vec![Selector::Wildcard(
            WildcardSelector
        )]))
        .is_singular());
    }
}
