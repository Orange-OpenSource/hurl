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

use hurl_core::reader::Reader;

use super::{primitives::literal, ParseResult};
use crate::jsonpath2::JsonPathExpr;

pub fn parse(reader: &mut Reader) -> ParseResult<JsonPathExpr> {
    literal("$", reader)?;
    let segments = vec![];
    Ok(JsonPathExpr::new(segments))
}

#[cfg(test)]
mod tests {
    use crate::jsonpath2::parser::{ParseError, ParseErrorKind};
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
}
