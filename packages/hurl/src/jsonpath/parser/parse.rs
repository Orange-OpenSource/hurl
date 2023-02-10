/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use super::super::ast::*;
use super::combinators::*;
use super::error::{Error, ParseError};
use super::primitives::*;
use super::reader::Reader;
use super::ParseResult;

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
            inner: ParseError::Expecting {
                value: "eof".to_string(),
            },
        });
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
            selector_array_index,
            selector_array_wildcard,
            selector_array_slice,
            selector_object_key_bracket,
            selector_object_key,
        ],
        reader,
    )
}

fn selector_array_index(reader: &mut Reader) -> Result<Selector, Error> {
    try_left_bracket(reader)?;
    let mut indexes = vec![];
    let i = match natural(reader) {
        Err(e) => {
            return Err(Error {
                pos: e.pos,
                recoverable: true,
                inner: e.inner,
            })
        }
        Ok(v) => v,
    };
    indexes.push(i);
    loop {
        let state = reader.state.clone();
        if try_literal(",", reader).is_ok() {
            let i = match natural(reader) {
                Err(e) => {
                    return Err(Error {
                        pos: e.pos,
                        recoverable: true,
                        inner: e.inner,
                    })
                }
                Ok(v) => v,
            };
            indexes.push(i);
        } else {
            reader.state = state;
            break;
        }
    }
    literal("]", reader)?;
    Ok(Selector::ArrayIndex(indexes))
}

fn selector_array_wildcard(reader: &mut Reader) -> Result<Selector, Error> {
    try_left_bracket(reader)?;
    try_literal("*", reader)?;
    literal("]", reader)?;
    Ok(Selector::ArrayWildcard {})
}

fn selector_array_slice(reader: &mut Reader) -> Result<Selector, Error> {
    try_left_bracket(reader)?;
    let state = reader.state.clone();
    let start = match integer(reader) {
        Err(_) => {
            reader.state = state.clone();
            None
        }
        Ok(v) => Some(v),
    };
    if try_literal(":", reader).is_err() {
        return Err(Error {
            pos: state.pos,
            recoverable: true,
            inner: ParseError::Expecting {
                value: ":".to_string(),
            },
        });
    };
    let state = reader.state.clone();
    let end = match integer(reader) {
        Err(_) => {
            reader.state = state;
            None
        }
        Ok(v) => Some(v),
    };
    literal("]", reader)?;
    Ok(Selector::ArraySlice(Slice { start, end }))
}

fn selector_filter(reader: &mut Reader) -> Result<Selector, Error> {
    try_literal("[?(", reader)?;
    let pred = predicate(reader)?;
    literal(")]", reader)?;
    Ok(Selector::Filter(pred))
}

fn selector_object_key_bracket(reader: &mut Reader) -> Result<Selector, Error> {
    try_left_bracket(reader)?;
    match string_value(reader) {
        Err(_) => Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: true,
            inner: ParseError::Expecting {
                value: "value string".to_string(),
            },
        }),
        Ok(v) => {
            literal("]", reader)?;
            Ok(Selector::NameChild(v))
        }
    }
}

fn selector_object_key(reader: &mut Reader) -> Result<Selector, Error> {
    if !reader.try_literal(".") {
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: true,
            inner: ParseError::Expecting {
                value: "[ or .".to_string(),
            },
        });
    };

    let s = reader.read_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-');
    if s.is_empty() {
        return Err(Error {
            pos: reader.state.pos.clone(),
            recoverable: false,
            inner: ParseError::Expecting {
                value: "empty value".to_string(),
            },
        });
    }
    Ok(Selector::NameChild(s))
}

fn selector_wildcard(reader: &mut Reader) -> Result<Selector, Error> {
    try_literal(".*", reader)?;
    Ok(Selector::Wildcard {})
}

fn selector_recursive_wildcard(reader: &mut Reader) -> Result<Selector, Error> {
    try_literal("..*", reader)?;
    Ok(Selector::RecursiveWildcard {})
}

fn selector_recursive_key(reader: &mut Reader) -> Result<Selector, Error> {
    try_literal("..", reader)?;
    let k = key_name(reader)?;
    Ok(Selector::RecursiveKey(k))
}

fn try_left_bracket(reader: &mut Reader) -> Result<(), Error> {
    let start = reader.state.clone();
    if literal(".[", reader).is_err() {
        reader.state = start;
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
    let state = reader.state.clone();
    let func = match predicate_func(reader) {
        Ok(f) => f,
        Err(_) => {
            reader.state = state;
            PredicateFunc::KeyExist {}
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
            equal_string_predicate_func,
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

    use super::super::Pos;
    use super::*;

    #[test]
    pub fn test_try_left_bracket() {
        let mut reader = Reader::init("xxx");
        let error = try_left_bracket(&mut reader).err().unwrap();
        assert!(error.recoverable);

        let mut reader = Reader::init("[xxx");
        assert!(try_left_bracket(&mut reader).is_ok());
        assert_eq!(reader.state.cursor, 1);

        let mut reader = Reader::init(".[xxx");
        assert!(try_left_bracket(&mut reader).is_ok());
        assert_eq!(reader.state.cursor, 2);
    }

    #[test]
    pub fn test_query() {
        let expected_query = Query {
            selectors: vec![Selector::ArrayIndex(vec![2])],
        };
        assert_eq!(query(&mut Reader::init("$[2]")).unwrap(), expected_query);

        let expected_query = Query {
            selectors: vec![Selector::NameChild("key".to_string())],
        };
        assert_eq!(
            query(&mut Reader::init("$['key']")).unwrap(),
            expected_query
        );
        assert_eq!(query(&mut Reader::init("$.key")).unwrap(), expected_query);

        let expected_query = Query {
            selectors: vec![Selector::NameChild("profile-id".to_string())],
        };
        assert_eq!(
            query(&mut Reader::init("$['profile-id']")).unwrap(),
            expected_query
        );
        assert_eq!(
            query(&mut Reader::init("$.profile-id")).unwrap(),
            expected_query
        );

        let expected_query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::ArrayIndex(vec![0]),
                Selector::NameChild("title".to_string()),
            ],
        };
        assert_eq!(
            query(&mut Reader::init("$.store.book[0].title")).unwrap(),
            expected_query
        );
        assert_eq!(
            query(&mut Reader::init("$['store']['book'][0]['title']")).unwrap(),
            expected_query
        );

        let expected_query = Query {
            selectors: vec![
                Selector::RecursiveKey("book".to_string()),
                Selector::ArrayIndex(vec![2]),
            ],
        };
        assert_eq!(
            query(&mut Reader::init("$..book[2]")).unwrap(),
            expected_query
        );
    }

    #[test]
    pub fn test_query_error() {
        let error = query(&mut Reader::init("?$.store")).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 1 });

        let error = query(&mut Reader::init("$.store?")).err().unwrap();
        assert_eq!(error.pos, Pos { line: 1, column: 8 });
    }

    #[test]
    pub fn test_selector_filter() {
        // Filter exist value
        let mut reader = Reader::init("[?(@.isbn)]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::Filter(Predicate {
                key: vec!["isbn".to_string()],
                func: PredicateFunc::KeyExist {},
            })
        );
        assert_eq!(reader.state.cursor, 11);

        // Filter equal on string with single quotes
        let mut reader = Reader::init("[?(@.key=='value')]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::Filter(Predicate {
                key: vec!["key".to_string()],
                func: PredicateFunc::EqualString("value".to_string()),
            })
        );
        assert_eq!(reader.state.cursor, 19);

        let mut reader = Reader::init("[?(@.price<10)]");
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
        assert_eq!(reader.state.cursor, 15);
    }

    #[test]
    pub fn test_selector_recursive() {
        let mut reader = Reader::init("..book");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::RecursiveKey("book".to_string())
        );
        assert_eq!(reader.state.cursor, 6);
    }

    #[test]
    pub fn test_selector_array_index() {
        let mut reader = Reader::init("[2]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArrayIndex(vec![2])
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("[0,1]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArrayIndex(vec![0, 1])
        );
        assert_eq!(reader.state.cursor, 5);

        // you don't need to keep the exact string
        // this is not part of the AST
        let mut reader = Reader::init(".[2]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArrayIndex(vec![2])
        );
        assert_eq!(reader.state.cursor, 4);
    }

    #[test]
    pub fn test_selector_wildcard() {
        let mut reader = Reader::init("[*]");
        assert_eq!(selector(&mut reader).unwrap(), Selector::ArrayWildcard {});
        assert_eq!(reader.state.cursor, 3);

        // you don't need to keep the exact string
        // this is not part of the AST
        let mut reader = Reader::init(".[*]");
        assert_eq!(selector(&mut reader).unwrap(), Selector::ArrayWildcard {});
        assert_eq!(reader.state.cursor, 4);
    }

    #[test]
    pub fn test_selector_array_slice() {
        let mut reader = Reader::init("[-1:]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArraySlice(Slice {
                start: Some(-1),
                end: None
            })
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("[:2]");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::ArraySlice(Slice {
                start: None,
                end: Some(2)
            })
        );
        assert_eq!(reader.state.cursor, 4);
    }

    #[test]
    pub fn test_key_bracket_selector() {
        let mut reader = Reader::init("['key']");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key".to_string())
        );
        assert_eq!(reader.state.cursor, 7);

        let mut reader = Reader::init(".['key']");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key".to_string())
        );
        assert_eq!(reader.state.cursor, 8);

        let mut reader = Reader::init("['key1']");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key1".to_string())
        );
        assert_eq!(reader.state.cursor, 8);
    }

    #[test]
    pub fn test_selector_key_dot_notation() {
        let mut reader = Reader::init(".key");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key".to_string())
        );
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init(".key1");
        assert_eq!(
            selector(&mut reader).unwrap(),
            Selector::NameChild("key1".to_string())
        );
        assert_eq!(reader.state.cursor, 5);
    }

    #[test]
    pub fn test_predicate() {
        // Key exists
        assert_eq!(
            predicate(&mut Reader::init("@.isbn")).unwrap(),
            Predicate {
                key: vec!["isbn".to_string()],
                func: PredicateFunc::KeyExist {},
            }
        );

        // Filter equal on string with single quotes
        assert_eq!(
            predicate(&mut Reader::init("@.key=='value'")).unwrap(),
            Predicate {
                key: vec!["key".to_string()],
                func: PredicateFunc::EqualString("value".to_string()),
            }
        );

        // Filter equal on int
        assert_eq!(
            predicate(&mut Reader::init("@.key==1")).unwrap(),
            Predicate {
                key: vec!["key".to_string()],
                func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
            }
        );

        // Filter equal on int for key in object
        assert_eq!(
            predicate(&mut Reader::init("@.obj.key==1")).unwrap(),
            Predicate {
                key: vec!["obj".to_string(), "key".to_string()],
                func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
            }
        );

        // Filter less than int
        assert_eq!(
            predicate(&mut Reader::init("@.price<10")).unwrap(),
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
        let mut reader = Reader::init("==2");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::Equal(Number { int: 2, decimal: 0 })
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("==2.1");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::Equal(Number {
                int: 2,
                decimal: 100_000_000_000_000_000
            })
        );
        assert_eq!(reader.state.cursor, 5);

        let mut reader = Reader::init("== 2.1 ");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::Equal(Number {
                int: 2,
                decimal: 100_000_000_000_000_000
            })
        );
        assert_eq!(reader.state.cursor, 7);

        let mut reader = Reader::init("=='hello'");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::EqualString("hello".to_string())
        );
        assert_eq!(reader.state.cursor, 9);

        let mut reader = Reader::init(">5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::GreaterThan(Number { int: 5, decimal: 0 })
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init(">=5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::GreaterThanOrEqual(Number { int: 5, decimal: 0 })
        );
        assert_eq!(reader.state.cursor, 3);

        let mut reader = Reader::init("<5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::LessThan(Number { int: 5, decimal: 0 })
        );
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("<=5");
        assert_eq!(
            predicate_func(&mut reader).unwrap(),
            PredicateFunc::LessThanOrEqual(Number { int: 5, decimal: 0 })
        );
        assert_eq!(reader.state.cursor, 3);
    }
}


#[cfg(test)]
mod tests_integration {

    use std::fs::read_to_string;
    use serde_json::json;
    use super::parse;

    fn bookstore_value() -> serde_json::Value {
        let s = read_to_string("tests/bookstore.json").expect("could not read string from file");
        serde_json::from_str(s.as_str()).expect("could not parse json file")
    }

    fn store_value() -> serde_json::Value {
        serde_json::from_str(
            r#"
             {
    "book": [
      {
        "category": "reference",
        "author": "Nigel Rees",
        "title": "Sayings of the Century",
        "price": 8.95
      },
      {
        "category": "fiction",
        "author": "Evelyn Waugh",
        "title": "Sword of Honour",
        "price": 12.99
      },
      {
        "category": "fiction",
        "author": "Herman Melville",
        "title": "Moby Dick",
        "isbn": "0-553-21311-3",
        "price": 8.99
      },
      {
        "category": "fiction",
        "author": "J. R. R. Tolkien",
        "title": "The Lord of the Rings",
        "isbn": "0-395-19395-8",
        "price": 22.99
      }
    ],
    "bicycle": {
      "color": "red",
      "price": 19.95
    }
  }
            "#,
        )
            .unwrap()
    }

    fn book_value() -> serde_json::Value {
        serde_json::from_str(
            r#"
             [
      {
        "category": "reference",
        "author": "Nigel Rees",
        "title": "Sayings of the Century",
        "price": 8.95
      },
      {
        "category": "fiction",
        "author": "Evelyn Waugh",
        "title": "Sword of Honour",
        "price": 12.99
      },
      {
        "category": "fiction",
        "author": "Herman Melville",
        "title": "Moby Dick",
        "isbn": "0-553-21311-3",
        "price": 8.99
      },
      {
        "category": "fiction",
        "author": "J. R. R. Tolkien",
        "title": "The Lord of the Rings",
        "isbn": "0-395-19395-8",
        "price": 22.99
      }
    ]
            "#,
        )
            .unwrap()
    }

    fn bicycle_value() -> serde_json::Value {
        serde_json::from_str(
            r#"
{
      "color": "red",
      "price": 19.95
    }
    "#,
        )
            .unwrap()
    }

    fn book0_value() -> serde_json::Value {
        json!(  { "category": "reference",
      "author": "Nigel Rees",
      "title": "Sayings of the Century",
      "price": 8.95
    })
    }

    fn book1_value() -> serde_json::Value {
        json!( { "category": "fiction",
      "author": "Evelyn Waugh",
      "title": "Sword of Honour",
      "price": 12.99
    })
    }

    fn book2_value() -> serde_json::Value {
        json!( {
      "category": "fiction",
      "author": "Herman Melville",
      "title": "Moby Dick",
      "isbn": "0-553-21311-3",
      "price": 8.99
    })
    }

    fn book3_value() -> serde_json::Value {
        json!({ "category": "fiction",
      "author": "J. R. R. Tolkien",
      "title": "The Lord of the Rings",
      "isbn": "0-395-19395-8",
      "price": 22.99
    })
    }

    #[test]
    fn test_bookstore_path() {
        // examples from https://goessner.net/articles/JsonPath/

        //the authors of all books in the store
        let expr = parse("$.store.book[*].author").unwrap();
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![
                json!("Nigel Rees"),
                json!("Evelyn Waugh"),
                json!("Herman Melville"),
                json!("J. R. R. Tolkien")
            ]
        );

        // all authors
        let expr = parse("$..author").unwrap();
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![
                json!("Nigel Rees"),
                json!("Evelyn Waugh"),
                json!("Herman Melville"),
                json!("J. R. R. Tolkien")
            ]
        );

        // all things in store, which are some books and a red bicycle.
        let expr = parse("$.store.*").unwrap();
        // Attention, there is no ordering on object keys with serde_json
        // But you expect that order stays the same
        // that's why bicycle and boot are inverted
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![bicycle_value(), book_value(),]
        );

        // the price of everything in the store.
        let expr = parse("$.store..price").unwrap();
        // Attention, there is no ordering on object keys with serde_json
        // But you expect that order stays the same
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![
                json!(19.95),
                json!(8.95),
                json!(12.99),
                json!(8.99),
                json!(22.99),
            ]
        );

        // the third book
        let expr = parse("$..book[2]").unwrap();
        assert_eq!(expr.eval(bookstore_value()), vec![book2_value()]);

        // the last book in order
        // The following expression is not supported
        // (@.length-1)
        // use python-like indexing instead
        let expr = parse("$..book[-1:]").unwrap();
        assert_eq!(expr.eval(bookstore_value()), vec![book3_value()]);

        // the first two books
        let expr = parse("$..book[0,1]").unwrap();
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![book0_value(), book1_value()]
        );
        let expr = parse("$..book[:2]").unwrap();
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![book0_value(), book1_value()]
        );

        // filter all books with isbn number
        let expr = parse("$..book[?(@.isbn)]").unwrap();
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![book2_value(), book3_value(),]
        );

        // filter all books cheapier than 10
        let expr = parse("$..book[?(@.price<10)]").unwrap();
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![book0_value(), book2_value(),]
        );

        // All members of JSON structure
        let expr = parse("$..*").unwrap();
        // Order is reproducible
        // but does not keep same order of json input!
        assert_eq!(
            expr.eval(bookstore_value()),
            vec![
                store_value(),
                bicycle_value(),
                json!("red"),
                json!(19.95),
                book_value(),
                book0_value(),
                json!("Nigel Rees"),
                json!("reference"),
                json!(8.95),
                json!("Sayings of the Century"),
                book1_value(),
                json!("Evelyn Waugh"),
                json!("fiction"),
                json!(12.99),
                json!("Sword of Honour"),
                book2_value(),
                json!("Herman Melville"),
                json!("fiction"),
                json!("0-553-21311-3"),
                json!(8.99),
                json!("Moby Dick"),
                book3_value(),
                json!("J. R. R. Tolkien"),
                json!("fiction"),
                json!("0-395-19395-8"),
                json!(22.99),
                json!("The Lord of the Rings"),
            ]
        );
    }

    #[test]
    fn test_bookstore_additional() {
        let no_result: Vec<serde_json::Value> = vec![];

        // Find books more expensive than 100
        let expr = parse("$.store.book[?(@.price>100)]").unwrap();
        assert_eq!(expr.eval(bookstore_value()), no_result);

        // find all authors for reference book
        let expr = parse("$..book[?(@.category=='reference')].author").unwrap();
        assert_eq!(expr.eval(bookstore_value()), vec![json!("Nigel Rees")]);
    }

    #[test]
    fn test_array() {
        let array = json!([0, 1, 2, 3]);
        let expr = parse("$[2]").unwrap();
        assert_eq!(expr.eval(array), vec![json!(2)]);

        let expr = parse("$[0].name").unwrap();
        let array = json!([{"name": "Bob"},{"name": "Bill"}]);
        assert_eq!(expr.eval(array), vec![json!("Bob")]);
    }

    #[test]
    fn test_key_access() {
        let obj = json!({
        "_": "underscore",
        "-": "hyphen",
        "*": "asterisk",
        "'": "single_quote",
        "\"": "double_quote",
        "✈": "plane"
    });

        // Bracket notation
        let expr = parse("$['-']").unwrap();
        assert_eq!(expr.eval(obj.clone()), vec![json!("hyphen")]);

        let expr = parse("$['_']").unwrap();
        assert_eq!(expr.eval(obj.clone()), vec![json!("underscore")]);

        let expr = parse("$['*']").unwrap();
        assert_eq!(expr.eval(obj.clone()), vec![json!("asterisk")]);

        let expr = parse("$['\\'']").unwrap();
        assert_eq!(expr.eval(obj.clone()), vec![json!("single_quote")]);

        let expr = parse("$['\"']").unwrap();
        assert_eq!(expr.eval(obj.clone()), vec![json!("double_quote")]);

        let expr = parse("$['✈']").unwrap();
        assert_eq!(expr.eval(obj.clone()), vec![json!("plane")]);

        // Dot notation
        let expr = parse("$._").unwrap();
        assert_eq!(expr.eval(obj.clone()), vec![json!("underscore")]);

        // Asterisk
        // return all elements
        // There is no ordering in JSON keys
        // You must compare with their string values sorted
        let values = vec![
            "asterisk",
            "double_quote",
            "hyphen",
            "plane",
            "single_quote",
            "underscore",
        ];

        let expr = parse("$.*").unwrap();
        let results = expr.eval(obj.clone());
        let mut results = results
            .iter()
            .map(|e| e.as_str().unwrap())
            .collect::<Vec<&str>>();
        results.sort_unstable();
        assert_eq!(results, values);

        let expr = parse("$[*]").unwrap();
        let results = expr.eval(obj);
        let mut results = results
            .iter()
            .map(|e| e.as_str().unwrap())
            .collect::<Vec<&str>>();
        results.sort_unstable();
        assert_eq!(results, values);
    }

    fn fruit_prices_value() -> serde_json::Value {
        serde_json::from_str(
            r#"
          {
    "fruit": [
        {
            "name": "apple",
            "price": {
                "US": 100,
                "UN": 110
            }
        },
        {
            "name": "grape",
            "price": {
                "US": 200,
                "UN": 150
            }
        }
    ]
}
            "#,
        )
            .unwrap()
    }

    #[test]
    fn test_filter_nested_object() {
        let expr = parse("$.fruit[?(@.price.US==200)].name").unwrap();
        assert_eq!(expr.eval(fruit_prices_value()), vec![json!("grape")]);

        let expr = parse("$.fruit[?(@.pricex.US==200)].name").unwrap();
        assert!(expr.eval(fruit_prices_value()).is_empty());
    }

    #[test]
    fn test_parsing_error() {
        // not supported yet
        assert!(parse("$..book[(@.length-1)]").is_err());
    }


}
