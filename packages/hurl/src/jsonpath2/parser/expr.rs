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
use crate::jsonpath2::{parser::selector, ChildSegment, DescendantSegment, JsonPathExpr, Segment};
use hurl_core::reader::Reader;

pub fn parse(reader: &mut Reader) -> ParseResult<JsonPathExpr> {
    literal("$", reader)?;
    let mut segments = vec![];
    while !reader.is_eof() {
        let segment = segment(reader)?;
        segments.push(segment);
    }
    Ok(JsonPathExpr::new(segments))
}

fn segment(reader: &mut Reader) -> ParseResult<Segment> {
    let is_descendant_segment = try_literal("..", reader);
    literal("[", reader)?;
    let first_selector = selector::parse(reader)?;
    let selectors = vec![first_selector];
    // TODO: select more than one selector
    literal("]", reader)?;
    let segment = if is_descendant_segment {
        Segment::Descendant(DescendantSegment::new(selectors))
    } else {
        Segment::Child(ChildSegment::new(selectors))
    };
    Ok(segment)
}

#[cfg(test)]
mod tests {

    use super::super::{ParseError, ParseErrorKind};
    use crate::jsonpath2::{NameSelector, Selector};
    use hurl_core::reader::{CharPos, Pos, Reader};

    use super::*;

    #[test]
    pub fn test_empty() {
        let mut reader = Reader::new("");
        assert_eq!(
            parse(&mut reader).unwrap_err(),
            ParseError::new(Pos::new(1, 1), ParseErrorKind::Expecting("$".to_string()))
        );
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    pub fn test_root_identifier() {
        let mut reader = Reader::new("$");

        assert_eq!(parse(&mut reader).unwrap(), JsonPathExpr::new(vec![]));
        assert_eq!(reader.cursor().index, CharPos(1));
    }

    #[test]
    pub fn test_child_segment() {
        let mut reader = Reader::new("$['store']");

        assert_eq!(
            parse(&mut reader).unwrap(),
            JsonPathExpr::new(vec![Segment::Child(ChildSegment::new(vec![
                Selector::Name(NameSelector::new("store".to_string()))
            ]))])
        );
        assert_eq!(reader.cursor().index, CharPos(10));
    }
}
