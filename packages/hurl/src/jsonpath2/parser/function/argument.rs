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

use crate::jsonpath2::ast::function::argument::{NodesTypeArgument, ValueTypeArgument};
use crate::jsonpath2::parser::literal;
use crate::jsonpath2::parser::query::try_filter_query;
use crate::jsonpath2::parser::singular_query;
use crate::jsonpath2::parser::{ParseError, ParseErrorKind, ParseResult};
use hurl_core::reader::Reader;

/// Parse an argument with ValueType
#[allow(dead_code)]
pub fn value_type(reader: &mut Reader) -> ParseResult<ValueTypeArgument> {
    if let Some(v) = literal::try_parse(reader)? {
        Ok(ValueTypeArgument::Literal(v))
    } else if let Some(query) = singular_query::try_parse(reader)? {
        Ok(ValueTypeArgument::SingularQuery(query))
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("a ValueType argument".to_string()),
        ))
    }
}

/// Parse an argument with NodeType
#[allow(dead_code)]
pub fn nodes_type(reader: &mut Reader) -> ParseResult<NodesTypeArgument> {
    if let Some(filter_query) = try_filter_query(reader)? {
        Ok(NodesTypeArgument::FilterQuery(filter_query))
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("a NodesType argument".to_string()),
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::jsonpath2::ast::literal::{Literal, Number};
    use crate::jsonpath2::ast::query::{Query, RelativeQuery};
    use crate::jsonpath2::ast::segment::{ChildSegment, Segment};
    use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
    use hurl_core::reader::CharPos;

    #[test]
    pub fn test_value_type() {
        let mut reader = Reader::new("42");
        assert_eq!(
            value_type(&mut reader).unwrap(),
            ValueTypeArgument::Literal(Literal::Number(Number::Integer(42)))
        );
        assert_eq!(reader.cursor().index, CharPos(2));
    }

    #[test]
    pub fn test_nodes_type() {
        let mut reader = Reader::new("@.*.author");
        assert_eq!(
            nodes_type(&mut reader).unwrap(),
            NodesTypeArgument::FilterQuery(Query::RelativeQuery(RelativeQuery::new(vec![
                Segment::Child(ChildSegment::new(vec![Selector::Wildcard(
                    WildcardSelector
                )])),
                Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                    "author".to_string()
                )),]))
            ])))
        );
        assert_eq!(reader.cursor().index, CharPos(10));
    }
}
