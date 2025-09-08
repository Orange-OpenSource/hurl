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

use super::{primitives::literal, primitives::try_literal, ParseResult};
use crate::jsonpath2::{parser::selector, ChildSegment, DescendantSegment, Segment};
use hurl_core::reader::Reader;

pub fn parse(reader: &mut Reader) -> ParseResult<Vec<Segment>> {
    let mut segments = vec![];
    while let Some(segment) = try_segment(reader)? {
        segments.push(segment);
    }
    Ok(segments)
}

pub fn try_segment(reader: &mut Reader) -> ParseResult<Option<Segment>> {
    let is_descendant_segment = if try_literal("..[", reader) {
        true
    } else if try_literal("[", reader) {
        false
    } else {
        return Ok(None);
    };
    let first_selector = selector::parse(reader)?;
    let selectors = vec![first_selector];
    // TODO: select more than one selector
    literal("]", reader)?;
    let segment = if is_descendant_segment {
        Segment::Descendant(DescendantSegment::new(selectors))
    } else {
        Segment::Child(ChildSegment::new(selectors))
    };
    Ok(Some(segment))
}

#[cfg(test)]
mod tests {

    use crate::jsonpath2::{NameSelector, Selector};
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
}
