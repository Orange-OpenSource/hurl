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
    parser::primitives::{string_literal, try_integer},
    IndexSelector, NameSelector, Selector,
};
use hurl_core::reader::Reader;

use super::ParseResult;

pub fn parse(reader: &mut Reader) -> ParseResult<Selector> {
    let initial_state = reader.cursor();
    if let Some(name_selector) = try_name_selector(reader)? {
        return Ok(Selector::Name(name_selector));
    } else if let Some(index_selector) = try_index_selector(reader)? {
        return Ok(Selector::Index(index_selector));
    }
    {}

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

/// Try to parse an index selector
fn try_index_selector(reader: &mut Reader) -> ParseResult<Option<IndexSelector>> {
    let value = try_integer(reader)?;
    Ok(value.map(IndexSelector::new))
}

#[cfg(test)]
mod tests {
    use hurl_core::reader::{CharPos, Reader};

    use super::*;

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
    pub fn test_index_selector() {
        let mut reader = Reader::new("1");
        assert_eq!(
            try_index_selector(&mut reader).unwrap().unwrap(),
            IndexSelector::new(1)
        );
        assert_eq!(reader.cursor().index, CharPos(1));
    }
}
