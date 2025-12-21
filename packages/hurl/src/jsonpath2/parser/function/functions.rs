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

use super::argument;
use crate::jsonpath2::ast::function::functions::{LogicalTypeFunction, ValueTypeFunction};
use crate::jsonpath2::parser::primitives::{expect_str, match_str, skip_whitespace};
use crate::jsonpath2::parser::ParseResult;
use hurl_core::reader::Reader;

/// Try to parse a function return a ValueType
#[allow(dead_code)]
pub fn try_value_type_function(reader: &mut Reader) -> ParseResult<Option<ValueTypeFunction>> {
    //let initial_state = reader.cursor();
    if match_str("length", reader) {
        expect_str("(", reader)?;
        let argument = argument::value_type(reader)?;
        expect_str(")", reader)?;
        Ok(Some(ValueTypeFunction::Length(argument)))
    } else if match_str("value", reader) {
        expect_str("(", reader)?;
        let argument = argument::nodes_type(reader)?;
        expect_str(")", reader)?;
        Ok(Some(ValueTypeFunction::Value(argument)))
    } else if match_str("count", reader) {
        expect_str("(", reader)?;
        let argument = argument::nodes_type(reader)?;
        expect_str(")", reader)?;
        Ok(Some(ValueTypeFunction::Count(argument)))
    } else {
        Ok(None)
    }
}

/// Try to parse a function return a LogicalType
#[allow(dead_code)]
pub fn try_logical_type_function(reader: &mut Reader) -> ParseResult<Option<LogicalTypeFunction>> {
    //let initial_state = reader.cursor();
    if match_str("match", reader) {
        expect_str("(", reader)?;
        let argument1 = argument::value_type(reader)?;
        skip_whitespace(reader);
        expect_str(",", reader)?;
        skip_whitespace(reader);
        let argument2 = argument::value_type(reader)?;
        expect_str(")", reader)?;
        Ok(Some(LogicalTypeFunction::Match(argument1, argument2)))
    } else if match_str("search", reader) {
        expect_str("(", reader)?;
        let argument1 = argument::value_type(reader)?;
        skip_whitespace(reader);
        expect_str(",", reader)?;
        skip_whitespace(reader);
        let argument2 = argument::value_type(reader)?;
        expect_str(")", reader)?;
        Ok(Some(LogicalTypeFunction::Search(argument1, argument2)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::jsonpath2::ast::function::argument::{NodesTypeArgument, ValueTypeArgument};
    use crate::jsonpath2::ast::literal::Literal;
    use crate::jsonpath2::ast::query::{AbsoluteQuery, Query, RelativeQuery};
    use crate::jsonpath2::ast::segment::{ChildSegment, DescendantSegment, Segment};
    use crate::jsonpath2::ast::selector::{NameSelector, Selector, WildcardSelector};
    use crate::jsonpath2::ast::singular_query::{
        RelativeSingularQuery, SingularQuery, SingularQuerySegment,
    };
    use crate::jsonpath2::parser::{ParseError, ParseErrorKind};
    use hurl_core::reader::{CharPos, Pos};
    //use serde_json::json;

    #[test]
    pub fn test_length_function() {
        let mut reader = Reader::new("length(\"abc\")");
        assert_eq!(
            try_value_type_function(&mut reader).unwrap().unwrap(),
            ValueTypeFunction::Length(ValueTypeArgument::Literal(Literal::String(
                "abc".to_string()
            )))
        );
        assert_eq!(reader.cursor().index, CharPos(13));

        let mut reader = Reader::new("length(value($..c))");
        assert_eq!(
            try_value_type_function(&mut reader).unwrap().unwrap(),
            ValueTypeFunction::Length(ValueTypeArgument::Function(Box::new(
                ValueTypeFunction::Value(NodesTypeArgument::FilterQuery(Query::AbsoluteQuery(
                    AbsoluteQuery::new(vec![Segment::Descendant(DescendantSegment::new(vec![
                        Selector::Name(NameSelector::new("c".to_string()))
                    ]))])
                )))
            )))
        );
        assert_eq!(reader.cursor().index, CharPos(19));
    }

    #[test]
    pub fn test_length_function_error() {
        // not well-typed since @.* is a non-singular query
        let mut reader = Reader::new("length(@.*)");
        assert_eq!(
            try_value_type_function(&mut reader).unwrap_err(),
            ParseError::new(Pos::new(1, 9), ParseErrorKind::Expecting(")".to_string()))
        );
    }

    #[test]
    pub fn test_count_function() {
        let mut reader = Reader::new("count(@.*.author)");
        assert_eq!(
            try_value_type_function(&mut reader).unwrap().unwrap(),
            ValueTypeFunction::Count(NodesTypeArgument::FilterQuery(Query::RelativeQuery(
                RelativeQuery::new(vec![
                    Segment::Child(ChildSegment::new(vec![Selector::Wildcard(
                        WildcardSelector
                    )])),
                    Segment::Child(ChildSegment::new(vec![Selector::Name(NameSelector::new(
                        "author".to_string()
                    )),]))
                ])
            )))
        );
        assert_eq!(reader.cursor().index, CharPos(17));
    }

    #[test]
    pub fn test_count_function_error() {
        // not well-typed since 1 is not a query or function expression
        let mut reader = Reader::new("count(1)");
        assert_eq!(
            try_value_type_function(&mut reader).unwrap_err(),
            ParseError::new(
                Pos::new(1, 7),
                ParseErrorKind::Expecting("a NodesType argument".to_string())
            )
        );
    }

    #[test]
    pub fn test_match_function() {
        let mut reader = Reader::new("match(@.date, \"1974-05-01\")");
        assert_eq!(
            try_logical_type_function(&mut reader).unwrap().unwrap(),
            LogicalTypeFunction::Match(
                ValueTypeArgument::SingularQuery(SingularQuery::Relative(
                    RelativeSingularQuery::new(vec![SingularQuerySegment::Name(
                        NameSelector::new("date".to_string())
                    )])
                )),
                ValueTypeArgument::Literal(Literal::String("1974-05-01".to_string()))
            )
        );
        assert_eq!(reader.cursor().index, CharPos(27));
    }

    #[test]
    pub fn test_search_function() {
        let mut reader = Reader::new("search(@.author, \"[BR]ob\")");
        assert_eq!(
            try_logical_type_function(&mut reader).unwrap().unwrap(),
            LogicalTypeFunction::Search(
                ValueTypeArgument::SingularQuery(SingularQuery::Relative(
                    RelativeSingularQuery::new(vec![SingularQuerySegment::Name(
                        NameSelector::new("author".to_string())
                    )])
                )),
                ValueTypeArgument::Literal(Literal::String("[BR]ob".to_string()))
            )
        );
        assert_eq!(reader.cursor().index, CharPos(26));
    }

    #[test]
    pub fn test_value_function() {
        let mut reader = Reader::new("value(@..color)");
        assert_eq!(
            try_value_type_function(&mut reader).unwrap().unwrap(),
            ValueTypeFunction::Value(NodesTypeArgument::FilterQuery(Query::RelativeQuery(
                RelativeQuery::new(vec![Segment::Descendant(DescendantSegment::new(vec![
                    Selector::Name(NameSelector::new("color".to_string()))
                ]))])
            )))
        );
        assert_eq!(reader.cursor().index, CharPos(15));
    }
}
