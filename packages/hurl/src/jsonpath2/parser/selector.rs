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

use super::{ParseError, ParseErrorKind};
use crate::jsonpath2::{
    parser::primitives::{literal, string_literal, try_integer, try_literal},
    ArraySliceSelector, IndexSelector, NameSelector, Selector, WildcardSelector,
};
use hurl_core::reader::Reader;

use super::ParseResult;

pub fn parse(reader: &mut Reader) -> ParseResult<Selector> {
    let initial_state = reader.cursor();
    if let Some(name_selector) = try_name_selector(reader)? {
        return Ok(Selector::Name(name_selector));
    } else if let Some(wildcard_selector) = try_wildcard_selector(reader) {
        return Ok(Selector::Wildcard(wildcard_selector));
    } else if let Some(index_selector) = try_index_selector(reader)? {
        return Ok(Selector::Index(index_selector));
    } else if let Some(array_slice_selector) = try_array_slice_selector(reader)? {
        return Ok(Selector::ArraySlice(array_slice_selector));
    }

    Err(ParseError::new(
        initial_state.pos,
        ParseErrorKind::Expecting("a selector".to_string()),
    ))
}

/// Try to parse a name selector
fn try_name_selector(reader: &mut Reader) -> ParseResult<Option<NameSelector>> {
    let value = string_literal(reader)?;
    Ok(value.map(NameSelector::new))
}

/// Try to parse a wildcard selector
/// Returns None if it can not be parse. It can not fail.
fn try_wildcard_selector(reader: &mut Reader) -> Option<WildcardSelector> {
    if try_literal("*", reader) {
        Some(WildcardSelector)
    } else {
        None
    }
}

/// Try to parse an index selector
fn try_index_selector(reader: &mut Reader) -> ParseResult<Option<IndexSelector>> {
    let value = try_integer(reader)?;
    Ok(value.map(IndexSelector::new))
}

/// Try to parse an array_slice_selector
fn try_array_slice_selector(reader: &mut Reader) -> ParseResult<Option<ArraySliceSelector>> {
    let start = try_integer(reader)?;
    literal(":", reader)?;
    let end = try_integer(reader)?;
    let step = if try_literal(":", reader) {
        try_integer(reader)?.unwrap_or(1)
    } else {
        1
    };
    Ok(Some(ArraySliceSelector::new(start, end, step)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hurl_core::reader::{CharPos, Reader};

    #[test]
    pub fn test_parse() {
        let mut reader = Reader::new("'store'");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Selector::Name(NameSelector::new("store".to_string()))
        );
        assert_eq!(reader.cursor().index, CharPos(7));
    }

    #[test]
    pub fn test_name_selector() {
        let mut reader = Reader::new("'store'");
        assert_eq!(
            try_name_selector(&mut reader).unwrap().unwrap(),
            NameSelector::new("store".to_string())
        );
        assert_eq!(reader.cursor().index, CharPos(7));
    }

    #[test]
    pub fn test_wildcard_selector() {
        let mut reader = Reader::new("*");
        assert_eq!(
            try_wildcard_selector(&mut reader).unwrap(),
            WildcardSelector
        );
        assert_eq!(reader.cursor().index, CharPos(1));
    }

    #[test]
    pub fn test_index_selector() {
        let mut reader = Reader::new("1");
        assert_eq!(
            try_index_selector(&mut reader).unwrap().unwrap(),
            IndexSelector::new(1)
        );
        assert_eq!(reader.cursor().index, CharPos(1));
    }

    #[test]
    pub fn test_array_slice_selector() {
        let mut reader = Reader::new("1:3");
        assert_eq!(
            try_array_slice_selector(&mut reader).unwrap().unwrap(),
            ArraySliceSelector::new(Some(1), Some(3), 1)
        );
        assert_eq!(reader.cursor().index, CharPos(3));

        let mut reader = Reader::new("5:");
        assert_eq!(
            try_array_slice_selector(&mut reader).unwrap().unwrap(),
            ArraySliceSelector::new(Some(5), None, 1)
        );
        assert_eq!(reader.cursor().index, CharPos(2));

        let mut reader = Reader::new("1:5:2");
        assert_eq!(
            try_array_slice_selector(&mut reader).unwrap().unwrap(),
            ArraySliceSelector::new(Some(1), Some(5), 2)
        );
        assert_eq!(reader.cursor().index, CharPos(5));

        let mut reader = Reader::new("1:5:-2");
        assert_eq!(
            try_array_slice_selector(&mut reader).unwrap().unwrap(),
            ArraySliceSelector::new(Some(1), Some(5), -2)
        );
        assert_eq!(reader.cursor().index, CharPos(6));

        let mut reader = Reader::new("::-1");
        assert_eq!(
            try_array_slice_selector(&mut reader).unwrap().unwrap(),
            ArraySliceSelector::new(None, None, -1)
        );
        assert_eq!(reader.cursor().index, CharPos(4));
    }
}
