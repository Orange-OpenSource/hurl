/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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

use crate::jsonpath::ast::function::argument::{
    NodesTypeArgument, RegexValueTypeArgument, ValueTypeArgument,
};
use crate::jsonpath::parser::function::functions;
use crate::jsonpath::parser::literal;
use crate::jsonpath::parser::literal::string::try_parse as try_string;
use crate::jsonpath::parser::query::try_filter_query;
use crate::jsonpath::parser::singular_query;
use crate::jsonpath::parser::{ParseError, ParseErrorKind, ParseResult};
use hurl_core::reader::Reader;
use regex::Regex;

/// Parse an argument with ValueType
pub fn value_type(reader: &mut Reader) -> ParseResult<ValueTypeArgument> {
    if let Some(v) = literal::try_parse(reader)? {
        Ok(ValueTypeArgument::Literal(v))
    } else if let Some(query) = singular_query::try_parse(reader)? {
        Ok(ValueTypeArgument::SingularQuery(query))
    } else if let Some(function) = functions::try_value_type_function(reader)? {
        Ok(ValueTypeArgument::Function(Box::new(function)))
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("a ValueType argument".to_string()),
        ))
    }
}

/// Parse an argument with regex ValueType
pub fn regex_value_type(reader: &mut Reader) -> ParseResult<RegexValueTypeArgument> {
    if let Some(v) = try_regex(reader)? {
        Ok(RegexValueTypeArgument::Literal(v))
    } else if let Some(query) = singular_query::try_parse(reader)? {
        Ok(RegexValueTypeArgument::SingularQuery(query))
    } else if let Some(function) = functions::try_value_type_function(reader)? {
        Ok(RegexValueTypeArgument::Function(Box::new(function)))
    } else {
        Err(ParseError::new(
            reader.cursor().pos,
            ParseErrorKind::Expecting("a RegexValueType argument".to_string()),
        ))
    }
}

/// Try to parse a regex
///
fn try_regex(reader: &mut Reader) -> ParseResult<Option<Regex>> {
    let saved_position = reader.cursor().pos;
    if let Some(s) = try_string(reader)? {
        match Regex::new(&s) {
            Ok(regex) => Ok(Some(regex)),
            Err(_) => Err(ParseError::new(
                saved_position,
                ParseErrorKind::Expecting("a valid regex".to_string()),
            )),
        }
    } else {
        Ok(None)
    }
}

/// Parse an argument with NodeType
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
    use crate::jsonpath::ast::literal::{Literal, Number};
    use crate::jsonpath::ast::query::{Query, RelativeQuery};
    use crate::jsonpath::ast::segment::{ChildSegment, Segment};
    use crate::jsonpath::ast::selector::{NameSelector, Selector, WildcardSelector};
    use hurl_core::reader::{CharPos, Pos};

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
    pub fn test_regex_value_type() {
        let mut reader = Reader::new("42");
        assert_eq!(
            regex_value_type(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 1),
                ParseErrorKind::Expecting("a RegexValueType argument".to_string())
            )
        );

        let mut reader = Reader::new("[]");
        assert_eq!(
            regex_value_type(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 1),
                ParseErrorKind::Expecting("a RegexValueType argument".to_string())
            )
        );

        let mut reader = Reader::new("\"[Bb]ob\"");
        assert_eq!(
            regex_value_type(&mut reader).unwrap(),
            RegexValueTypeArgument::Literal(Regex::new("[Bb]ob").unwrap())
        );
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

    #[test]
    pub fn test_try_regex() {
        let mut reader = Reader::new("1");
        assert!(try_regex(&mut reader).unwrap().is_none());

        let mut reader = Reader::new("'[]'");
        assert_eq!(
            try_regex(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 1),
                ParseErrorKind::Expecting("a valid regex".to_string())
            )
        );

        let mut reader = Reader::new("'[Bb]ob'");
        assert_eq!(
            try_regex(&mut reader).unwrap().unwrap().to_string(),
            Regex::new("[Bb]ob").unwrap().to_string()
        );
    }
}
