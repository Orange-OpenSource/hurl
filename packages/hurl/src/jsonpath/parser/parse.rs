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
use hurl_core::combinator::{choice, zero_or_more};
use hurl_core::reader::Reader;

use crate::jsonpath::ast::{Predicate, PredicateFunc, Query, Selector, Slice};
use crate::jsonpath::parser::error::{ParseError, ParseErrorKind, ParseResult};
use crate::jsonpath::parser::primitives::{
    boolean, integer, key_name, key_path, literal, natural, number, string_value, try_literal,
    whitespace,
};

pub fn parse(s: &str) -> Result<Query, ParseError> {
    let mut reader = Reader::new(s);
    query(&mut reader)
}

fn query(reader: &mut Reader) -> ParseResult<Query> {
    literal("$", reader)?;

    let selectors = zero_or_more(selector, reader)?;
    if !reader.is_eof() {
        let kind = ParseErrorKind::Expecting("eof".to_string());
        let error = ParseError::new(reader.cursor().pos, false, kind);
        return Err(error);
    }
    Ok(Query { selectors })
}

fn selector(reader: &mut Reader) -> ParseResult<Selector> {
    choice(
        &[
            selector_filter,
            selector_wildcard,
            selector_recursive_wildcard,
            selector_recursive_key,
            selector_array_index_or_array_indices,
            selector_array_wildcard,
            selector_array_slice,
            selector_object_key_bracket,
            selector_object_key,
        ],
        reader,
    )
}

fn selector_array_index_or_array_indices(reader: &mut Reader) -> Result<Selector, ParseError> {
    let initial_state = reader.cursor();
    try_left_bracket(reader)?;
    let mut indexes = vec![];
    let i = match natural(reader) {
        Err(e) => {
            let error = ParseError::new(e.pos, true, e.kind);
            return Err(error);
        }
        Ok(v) => v,
    };
    indexes.push(i);
    loop {
        let start = reader.cursor();
        if try_literal(",", reader).is_ok() {
            let i = match natural(reader) {
                Err(e) => {
                    return Err(ParseError::new(e.pos, true, e.kind));
                }
                Ok(v) => v,
            };
            indexes.push(i);
        } else {
            reader.seek(start);
            break;
        }
    }
    // you will have a ':' for a slice
    // TODO: combine array index, indices and slice in the same function
    if let Err(e) = try_literal("]", reader) {
        reader.seek(initial_state);
        return Err(ParseError::new(reader.cursor().pos, true, e.kind));
    }
    let selector = if indexes.len() == 1 {
        Selector::ArrayIndex(*indexes.first().unwrap())
    } else {
        Selector::ArrayIndices(indexes)
    };
    Ok(selector)
}

fn selector_array_wildcard(reader: &mut Reader) -> Result<Selector, ParseError> {
    try_left_bracket(reader)?;
    try_literal("*", reader)?;
    literal("]", reader)?;
    Ok(Selector::ArrayWildcard)
}

fn selector_array_slice(reader: &mut Reader) -> Result<Selector, ParseError> {
    try_left_bracket(reader)?;
    let save = reader.cursor();
    let start = match integer(reader) {
        Err(_) => {
            reader.seek(save);
            None
        }
        Ok(v) => Some(v),
    };
    if try_literal(":", reader).is_err() {
        let kind = ParseErrorKind::Expecting(":".to_string());
        let error = ParseError::new(save.pos, true, kind);
        return Err(error);
    };
    let save = reader.cursor();
    let end = match integer(reader) {
        Err(_) => {
            reader.seek(save);
            None
        }
        Ok(v) => Some(v),
    };
    literal("]", reader)?;
    Ok(Selector::ArraySlice(Slice { start, end }))
}

fn selector_filter(reader: &mut Reader) -> Result<Selector, ParseError> {
    try_left_bracket(reader)?;
    try_literal("?(", reader)?;
    let pred = predicate(reader)?;
    literal(")]", reader)?;
    Ok(Selector::Filter(pred))
}

fn selector_object_key_bracket(reader: &mut Reader) -> Result<Selector, ParseError> {
    try_left_bracket(reader)?;
    match string_value(reader) {
        Err(_) => {
            let kind = ParseErrorKind::Expecting("value string".to_string());
            let error = ParseError::new(reader.cursor().pos, true, kind);
            Err(error)
        }
        Ok(v) => {
            literal("]", reader)?;
            Ok(Selector::NameChild(v))
        }
    }
}

fn selector_object_key(reader: &mut Reader) -> Result<Selector, ParseError> {
    if reader.peek() != Some('.') {
        let kind = ParseErrorKind::Expecting("[ or .".to_string());
        let error = ParseError::new(reader.cursor().pos, true, kind);
        return Err(error);
    };
    _ = reader.read();

    let s = reader.read_while(|c| c.is_alphanumeric() || c == '_' || c == '-');
    if s.is_empty() {
        let kind = ParseErrorKind::Expecting("empty value".to_string());
        let error = ParseError::new(reader.cursor().pos, false, kind);
        return Err(error);
    }
    Ok(Selector::NameChild(s))
}

fn selector_wildcard(reader: &mut Reader) -> Result<Selector, ParseError> {
    try_literal(".*", reader)?;
    Ok(Selector::Wildcard)
}

fn selector_recursive_wildcard(reader: &mut Reader) -> Result<Selector, ParseError> {
    try_literal("..*", reader)?;
    Ok(Selector::RecursiveWildcard)
}

fn selector_recursive_key(reader: &mut Reader) -> Result<Selector, ParseError> {
    try_literal("..", reader)?;
    let k = key_name(reader)?;
    Ok(Selector::RecursiveKey(k))
}

fn try_left_bracket(reader: &mut Reader) -> Result<(), ParseError> {
    let start = reader.cursor();
    if literal(".[", reader).is_err() {
        reader.seek(start);
        try_literal("[", reader)?;
    }
    Ok(())
}

fn predicate(reader: &mut Reader) -> ParseResult<Predicate> {
    // predicate always on key?
    // TODO parsing key-value
    // ?(@.key=='value')
    // @<3 => assume number => plan it in your ast => ValueEqualInt should be used for that
    // KeyValueEqualInt

    // @.key          Exist(Key)
    // @.key==value   Equal(Key,Value)
    // @.key>=value   GreaterThanOrEqual(Key, Value)
    literal("@.", reader)?; // assume key value for the time being
    let key = key_path(reader)?;
    let save = reader.cursor();
    let func = match predicate_func(reader) {
        Ok(f) => f,
        Err(_) => {
            reader.seek(save);
            PredicateFunc::KeyExist
        }
    };
    Ok(Predicate { key, func })
}

fn predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    choice(
        &[
            equal_number_predicate_func,
            greater_than_predicate_func,
            greater_than_or_equal_predicate_func,
            less_than_predicate_func,
            less_than_or_equal_predicate_func,
            equal_boolean_predicate_func,
            equal_string_predicate_func,
            notequal_string_predicate_func,
            notequal_number_func,
        ],
        reader,
    )
}

fn equal_number_predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal("==", reader)?;
    whitespace(reader);
    let num = number(reader)?;
    Ok(PredicateFunc::Equal(num))
}

fn equal_boolean_predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal("==", reader)?;
    whitespace(reader);
    let boolean = boolean(reader)?;
    Ok(PredicateFunc::EqualBool(boolean))
}

fn greater_than_predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal(">", reader)?;
    whitespace(reader);
    let num = number(reader)?;
    Ok(PredicateFunc::GreaterThan(num))
}

fn greater_than_or_equal_predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal(">=", reader)?;
    whitespace(reader);
    let num = number(reader)?;
    Ok(PredicateFunc::GreaterThanOrEqual(num))
}

fn less_than_predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal("<", reader)?;
    whitespace(reader);
    let num = number(reader)?;
    Ok(PredicateFunc::LessThan(num))
}

fn less_than_or_equal_predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal("<=", reader)?;
    whitespace(reader);
    let num = number(reader)?;
    Ok(PredicateFunc::LessThanOrEqual(num))
}

fn equal_string_predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal("==", reader)?;
    whitespace(reader);
    let s = string_value(reader)?;
    Ok(PredicateFunc::EqualString(s))
}

fn notequal_string_predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal("!=", reader)?;
    whitespace(reader);
    let s = string_value(reader)?;
    Ok(PredicateFunc::NotEqualString(s))
}

fn notequal_number_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    try_literal("!=", reader)?;
    whitespace(reader);
    let num = number(reader)?;
    Ok(PredicateFunc::NotEqual(num))
}

#[cfg(test)]
mod tests {
    use hurl_core::reader::Pos;

    // tests from https://cburgmer.github.io/json-path-comparison
    use super::*;
    use crate::jsonpath::ast::Number;

    #[test]
    pub fn test_try_left_bracket() {
        let mut reader = Reader::new("xxx");
        let error = try_left_bracket(&mut reader).err().unwrap();
        assert!(error.recoverable);

        let mut reader = Reader::new("[xxx");
        assert!(try_left_bracket(&mut reader).is_ok());
        assert_eq!(reader.cursor().index, 1);

        let mut reader = Reader::new(".[xxx");
        assert!(try_left_bracket(&mut reader).is_ok());
        assert_eq!(reader.cursor().index, 2);
    }

    #[test]
    pub fn test_query() {
        let expected_query = Query {
            selectors: vec![Selector::ArrayIndex(2)],
        };
        assert_eq!(query(&mut Reader::new("$[2]")).unwrap(), expected_query);

        let expected_query = Query {
            selectors: vec![Selector::NameChild("key".to_string())],
        };
        assert_eq!(query(&mut Reader::new("$['key']")).unwrap(), expected_query);
        assert_eq!(query(&mut Reader::new("$.key")).unwrap(), expected_query);

        let expected_query = Query {
            selectors: vec![Selector::NameChild("profile-id".to_string())],
        };
        assert_eq!(
            query(&mut Reader::new("$['profile-id']")).unwrap(),
            expected_query
        );
        assert_eq!(
            query(&mut Reader::new("$.profile-id")).unwrap(),
            expected_query
        );

        let expected_query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::ArrayIndex(0),
                Selector::NameChild("title".to_string()),
            ],
        };
        assert_eq!(
            query(&mut Reader::new("$.store.book[0].title")).unwrap(),
            expected_query
        );
        assert_eq!(
            query(&mut Reader::new("$['store']['book'][0]['title']")).unwrap(),
            expected_query
        );

        let expected_query = Query {
            selectors: vec![
                Selector::RecursiveKey("book".to_string()),
                Selector::ArrayIndex(2),
            ],
        };
        assert_eq!(
            query(&mut Reader::new("$..book[2]")).unwrap(),
            expected_query
        );
    }

    #[test]
    pub fn test_query_error() {
        let error = query(&mut Reader::new("?$.store")).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });

        let error = query(&mut Reader::new("$.store?")).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
    }

    #[test]
    pub fn test_selector_filter() {
        // Filter exist value
        let mut reader = Reader::new("[?(@.isbn)]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::Filter(Predicate {
                key: vec!["isbn".to_string()],
                func: PredicateFunc::KeyExist,
            })
        );
        assert_eq!(reader.cursor().index, 11);

        // Filter equal on string with single quotes
        let mut reader = Reader::new("[?(@.key=='value')]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::Filter(Predicate {
                key: vec!["key".to_string()],
                func: PredicateFunc::EqualString("value".to_string()),
            })
        );
        assert_eq!(reader.cursor().index, 19);
        let mut reader = Reader::new(".[?(@.key=='value')]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::Filter(Predicate {
                key: vec!["key".to_string()],
                func: PredicateFunc::EqualString("value".to_string()),
            })
        );
        assert_eq!(reader.cursor().index, 20);

        let mut reader = Reader::new("[?(@.price<10)]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::Filter(Predicate {
                key: vec!["price".to_string()],
                func: PredicateFunc::LessThan(Number {
                    int: 10,
                    decimal: 0
                }),
            })
        );
        assert_eq!(reader.cursor().index, 15);
    }

    #[test]
    pub fn test_selector_recursive() {
        let mut reader = Reader::new("..book");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::RecursiveKey("book".to_string())
        );
        assert_eq!(reader.cursor().index, 6);
    }

    #[test]
    pub fn test_selector_array_index() {
        let mut reader = Reader::new("[2]");
        assert_eq!(selector(&mut reader).unwrap(), Selector::ArrayIndex(2));
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("[0,1]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArrayIndices(vec![0, 1])
        );
        assert_eq!(reader.cursor().index, 5);

        // you don't need to keep the exact string
        // this is not part of the AST
        let mut reader = Reader::new(".[2]");
        assert_eq!(selector(&mut reader).unwrap(), Selector::ArrayIndex(2));
        assert_eq!(reader.cursor().index, 4);
    }

    #[test]
    pub fn test_selector_wildcard() {
        let mut reader = Reader::new("[*]");
        assert_eq!(selector(&mut reader).unwrap(), Selector::ArrayWildcard);
        assert_eq!(reader.cursor().index, 3);

        // you don't need to keep the exact string
        // this is not part of the AST
        let mut reader = Reader::new(".[*]");
        assert_eq!(selector(&mut reader).unwrap(), Selector::ArrayWildcard);
        assert_eq!(reader.cursor().index, 4);
    }

    #[test]
    pub fn test_selector_array_slice() {
        let mut reader = Reader::new("[1:]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArraySlice(Slice {
                start: Some(1),
                end: None
            })
        );
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new("[-1:]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArraySlice(Slice {
                start: Some(-1),
                end: None
            })
        );
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("[:2]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArraySlice(Slice {
                start: None,
                end: Some(2)
            })
        );
        assert_eq!(reader.cursor().index, 4);
    }

    #[test]
    pub fn test_key_bracket_selector() {
        let mut reader = Reader::new("['key']");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key".to_string())
        );
        assert_eq!(reader.cursor().index, 7);

        let mut reader = Reader::new(".['key']");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key".to_string())
        );
        assert_eq!(reader.cursor().index, 8);

        let mut reader = Reader::new("['key1']");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key1".to_string())
        );
        assert_eq!(reader.cursor().index, 8);
    }

    #[test]
    pub fn test_selector_key_dot_notation() {
        let mut reader = Reader::new(".key");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key".to_string())
        );
        assert_eq!(reader.cursor().index, 4);

        let mut reader = Reader::new(".key1");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key1".to_string())
        );
        assert_eq!(reader.cursor().index, 5);
    }

    #[test]
    pub fn test_predicate() {
        // Key exists
        assert_eq!(
            predicate(&mut Reader::new("@.isbn")).unwrap(),
            Predicate {
                key: vec!["isbn".to_string()],
                func: PredicateFunc::KeyExist,
            }
        );

        // Filter equal on string with single quotes
        assert_eq!(
            predicate(&mut Reader::new("@.key=='value'")).unwrap(),
            Predicate {
                key: vec!["key".to_string()],
                func: PredicateFunc::EqualString("value".to_string()),
            }
        );

        // Filter equal on int
        assert_eq!(
            predicate(&mut Reader::new("@.key==1")).unwrap(),
            Predicate {
                key: vec!["key".to_string()],
                func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
            }
        );

        // Filter equal on int for key in object
        assert_eq!(
            predicate(&mut Reader::new("@.obj.key==1")).unwrap(),
            Predicate {
                key: vec!["obj".to_string(), "key".to_string()],
                func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
            }
        );

        // Filter less than int
        assert_eq!(
            predicate(&mut Reader::new("@.price<10")).unwrap(),
            Predicate {
                key: vec!["price".to_string()],
                func: PredicateFunc::LessThan(Number {
                    int: 10,
                    decimal: 0
                }),
            }
        );
    }

    #[test]
    pub fn test_predicate_func() {
        let mut reader = Reader::new("==true");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::EqualBool(true)
        );
        assert_eq!(reader.cursor().index, 6);

        let mut reader = Reader::new("==false");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::EqualBool(false)
        );
        assert_eq!(reader.cursor().index, 7);

        let mut reader = Reader::new("==2");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::Equal(Number { int: 2, decimal: 0 })
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("==2.1");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::Equal(Number {
                int: 2,
                decimal: 100_000_000_000_000_000
            })
        );
        assert_eq!(reader.cursor().index, 5);

        let mut reader = Reader::new("== 2.1 ");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::Equal(Number {
                int: 2,
                decimal: 100_000_000_000_000_000
            })
        );
        assert_eq!(reader.cursor().index, 7);

        let mut reader = Reader::new("=='hello'");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::EqualString("hello".to_string())
        );
        assert_eq!(reader.cursor().index, 9);

        let mut reader = Reader::new("!='hello'");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::NotEqualString("hello".to_string())
        );

        let mut reader = Reader::new("!=2");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::NotEqual(Number { int: 2, decimal: 0 })
        );

        let mut reader = Reader::new("!=2.5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::NotEqual(Number {
                int: 2,
                decimal: 500_000_000_000_000_000
            })
        );

        let mut reader = Reader::new(">5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::GreaterThan(Number { int: 5, decimal: 0 })
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new(">=5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::GreaterThanOrEqual(Number { int: 5, decimal: 0 })
        );
        assert_eq!(reader.cursor().index, 3);

        let mut reader = Reader::new("<5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::LessThan(Number { int: 5, decimal: 0 })
        );
        assert_eq!(reader.cursor().index, 2);

        let mut reader = Reader::new("<=5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::LessThanOrEqual(Number { int: 5, decimal: 0 })
        );
        assert_eq!(reader.cursor().index, 3);
    }
}
