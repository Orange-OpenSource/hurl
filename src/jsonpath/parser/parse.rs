/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
use super::combinators::*;
use super::error::{Error, ParseError};
use super::ParseResult;
use super::primitives::*;
use super::reader::Reader;
use super::super::ast::*;

pub fn parse(s: &str) -> Result<Query, Error> {
    let mut reader = Reader::init(s);
    query(&mut reader)
}

fn query(reader: &mut Reader) -> ParseResult<Query> {
    literal("$", reader)?;


    let selectors = zero_or_more(selector, reader)?;
    if !reader.is_eof() {
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: false,
            inner: ParseError::Expecting { value: "eof".to_string() },
        });
    }
    Ok(Query {
        selectors
    })
}

fn selector(reader: &mut Reader) -> ParseResult<Selector> {
    choice(vec![
        selector_recursive_key,
        selector_array_index,
        selector_object_key_bracket,
        selector_object_key,
        selector_filter,
    ], reader)
}

fn selector_array_index(reader: &mut Reader) -> Result<Selector, Error> {
    try_literal("[", reader)?;
    let i = match natural(reader) {
        Err(e) => return Err(Error {
            pos: e.pos,
            recoverable: true,
            inner: e.inner,
        }),
        Ok(v) => v
    };
    literal("]", reader)?;
    Ok(Selector::ArrayIndex(i))
}

fn selector_filter(reader: &mut Reader) -> Result<Selector, Error> {
    try_literal("[?(", reader)?;
    let pred = predicate(reader)?;
    literal(")]", reader)?;
    Ok(Selector::Filter(pred))
}

fn selector_object_key_bracket(reader: &mut Reader) -> Result<Selector, Error> {
    try_literal("[", reader)?;
    match string_value(reader) {
        Err(_) => Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: true,
            inner: ParseError::Expecting { value: "value string".to_string() },
        }),
        Ok(v) => {
            literal("]", reader)?;
            Ok(Selector::NameChild(v))
        }
    }
}

fn selector_object_key(reader: &mut Reader) -> Result<Selector, Error> {
    let end_delim = if reader.try_literal("['") {
        "']".to_string()
    } else if reader.try_literal(".") {
        "".to_string()
    } else {
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: true,
            inner: ParseError::Expecting { value: "[ or .".to_string() },
        });
    };

    let s = reader.read_while(|c| c.is_alphanumeric());
    if s.is_empty() {
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: false,
            inner: ParseError::Expecting { value: "empty value".to_string() },
        });
    }
    if !end_delim.is_empty() {
        literal(end_delim.as_str(), reader)?;
    }

    Ok(Selector::NameChild(s))
}

fn selector_recursive_key(reader: &mut Reader) -> Result<Selector, Error> {
    try_literal("..", reader)?;
    let k = key_name(reader)?;
    Ok(Selector::RecursiveKey(k))
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
    literal("@.", reader)?;  // assume key value for the time being
    let key = key_name(reader)?;
    let func = predicate_func(reader)?;
    Ok(Predicate { key, func })
}

fn predicate_func(reader: &mut Reader) -> ParseResult<PredicateFunc> {
    choice(
        vec![
            equal_number_predicate_func,
            greater_than_predicate_func,
            greater_than_or_equal_predicate_func,
            less_than_predicate_func,
            less_than_or_equal_predicate_func,
            equal_string_predicate_func
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


#[cfg(test)]
mod tests {
    // tests from https://cburgmer.github.io/json-path-comparison

    use super::*;
    use super::super::Pos;

    #[test]
    pub fn test_query() {
        let expected_query = Query {
            selectors: vec![
                Selector::ArrayIndex(2)
            ]
        };
        assert_eq!(query(&mut Reader::init("$[2]")).unwrap(), expected_query);

        let expected_query = Query {
            selectors: vec![
                Selector::NameChild("key".to_string())
            ]
        };
        assert_eq!(query(&mut Reader::init("$['key']")).unwrap(), expected_query);
        assert_eq!(query(&mut Reader::init("$.key")).unwrap(), expected_query);

        let expected_query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::ArrayIndex(0),
                Selector::NameChild("title".to_string()),
            ]
        };
        assert_eq!(query(&mut Reader::init("$.store.book[0].title")).unwrap(), expected_query);
        assert_eq!(query(&mut Reader::init("$['store']['book'][0]['title']")).unwrap(), expected_query);


        let expected_query = Query {
            selectors: vec![
                Selector::RecursiveKey("book".to_string()),
                Selector::ArrayIndex(2),
            ]
        };
        assert_eq!(query(&mut Reader::init("$..book[2]")).unwrap(), expected_query);
    }

    #[test]
    pub fn test_query_error() {
        let error = query(&mut Reader::init("?$.store")).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });

        let error = query(&mut Reader::init("$.store?")).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
    }

    #[test]
    pub fn test_selector() {

        // Array index
        let mut reader = Reader::init("[2]");
        assert_eq!(selector(&mut reader).unwrap(), Selector::ArrayIndex(2));
        assert_eq!(reader.state.cursor, 3);

        // Key bracket notation
        let mut reader = Reader::init("['key']");
        assert_eq!(selector(&mut reader).unwrap(), Selector::NameChild("key".to_string()));
        assert_eq!(reader.state.cursor, 7);

        let mut reader = Reader::init("['key1']");
        assert_eq!(selector(&mut reader).unwrap(), Selector::NameChild("key1".to_string()));
        assert_eq!(reader.state.cursor, 8);

        // Key dot notation
        let mut reader = Reader::init(".key");
        assert_eq!(selector(&mut reader).unwrap(), Selector::NameChild("key".to_string()));
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init(".key1");
        assert_eq!(selector(&mut reader).unwrap(), Selector::NameChild("key1".to_string()));
        assert_eq!(reader.state.cursor, 5);

        // Filter equal on string with single quotes
        let mut reader = Reader::init("[?(@.key=='value')]");
        assert_eq!(selector(&mut reader).unwrap(), Selector::Filter(Predicate {
            key: "key".to_string(),
            func: PredicateFunc::EqualString("value".to_string()),
        }));
        assert_eq!(reader.state.cursor, 19);

        let mut reader = Reader::init("..book");
        assert_eq!(selector(&mut reader).unwrap(), Selector::RecursiveKey("book".to_string()));
        assert_eq!(reader.state.cursor, 6);
    }

    #[test]
    pub fn test_predicate() {

        // Filter equal on string with single quotes
        assert_eq!(predicate(&mut Reader::init("@.key=='value'")).unwrap(),
                   Predicate {
                       key: "key".to_string(),
                       func: PredicateFunc::EqualString("value".to_string()),
                   });

        // Filter equal on int
        assert_eq!(predicate(&mut Reader::init("@.key==1")).unwrap(),
                   Predicate {
                       key: "key".to_string(),
                       func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
                   });
    }

    #[test]
    pub fn test_predicate_func() {
        let mut reader = Reader::init("==2");
        assert_eq!(predicate_func(&mut reader).unwrap(), PredicateFunc::Equal(Number { int: 2, decimal: 0 }));
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("==2.1");
        assert_eq!(predicate_func(&mut reader).unwrap(), PredicateFunc::Equal(Number { int: 2, decimal: 100_000_000_000_000_000 }));
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("== 2.1 ");
        assert_eq!(predicate_func(&mut reader).unwrap(), PredicateFunc::Equal(Number { int: 2, decimal: 100_000_000_000_000_000 }));
        assert_eq!(reader.state.cursor, 7);

        let mut reader = Reader::init("=='hello'");
        assert_eq!(predicate_func(&mut reader).unwrap(), PredicateFunc::EqualString("hello".to_string()));
        assert_eq!(reader.state.cursor, 9);

        let mut reader = Reader::init(">5");
        assert_eq!(predicate_func(&mut reader).unwrap(), PredicateFunc::GreaterThan(Number { int: 5, decimal: 0 }));
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init(">=5");
        assert_eq!(predicate_func(&mut reader).unwrap(), PredicateFunc::GreaterThanOrEqual(Number { int: 5, decimal: 0 }));
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("<5");
        assert_eq!(predicate_func(&mut reader).unwrap(), PredicateFunc::LessThan(Number { int: 5, decimal: 0 }));
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("<=5");
        assert_eq!(predicate_func(&mut reader).unwrap(), PredicateFunc::LessThanOrEqual(Number { int: 5, decimal: 0 }));
        assert_eq!(reader.state.cursor, 3);
    }
}
