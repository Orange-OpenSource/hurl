/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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

use hurl::jsonpath;
use serde_json::json;
use std::fs::read_to_string;

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
    let expr = jsonpath::parse("$.store.book[*].author").unwrap();
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
    let expr = jsonpath::parse("$..author").unwrap();
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
    let expr = jsonpath::parse("$.store.*").unwrap();
    // Attention, there is no ordering on object keys with serde_json
    // But you expect that order stays the same
    // that's why bicycle and boot are inverted
    assert_eq!(
        expr.eval(bookstore_value()),
        vec![bicycle_value(), book_value(),]
    );

    // the price of everything in the store.
    let expr = jsonpath::parse("$.store..price").unwrap();
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
    let expr = jsonpath::parse("$..book[2]").unwrap();
    assert_eq!(expr.eval(bookstore_value()), vec![book2_value()]);

    // the last book in order
    // The following expression is not supported
    // (@.length-1)
    // use python-like indexing instead
    let expr = jsonpath::parse("$..book[-1:]").unwrap();
    assert_eq!(expr.eval(bookstore_value()), vec![book3_value()]);

    // the first two books
    let expr = jsonpath::parse("$..book[0,1]").unwrap();
    assert_eq!(
        expr.eval(bookstore_value()),
        vec![book0_value(), book1_value()]
    );
    let expr = jsonpath::parse("$..book[:2]").unwrap();
    assert_eq!(
        expr.eval(bookstore_value()),
        vec![book0_value(), book1_value()]
    );

    // filter all books with isbn number
    let expr = jsonpath::parse("$..book[?(@.isbn)]").unwrap();
    assert_eq!(
        expr.eval(bookstore_value()),
        vec![book2_value(), book3_value(),]
    );

    // filter all books cheapier than 10
    let expr = jsonpath::parse("$..book[?(@.price<10)]").unwrap();
    assert_eq!(
        expr.eval(bookstore_value()),
        vec![book0_value(), book2_value(),]
    );

    // All members of JSON structure
    let expr = jsonpath::parse("$..*").unwrap();
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
    let expr = jsonpath::parse("$.store.book[?(@.price>100)]").unwrap();
    assert_eq!(expr.eval(bookstore_value()), no_result);

    // find all authors for reference book
    let expr = jsonpath::parse("$..book[?(@.category=='reference')].author").unwrap();
    assert_eq!(expr.eval(bookstore_value()), vec![json!("Nigel Rees")]);
}

#[test]
fn test_array() {
    let array = json!([0, 1, 2, 3]);
    let expr = jsonpath::parse("$[2]").unwrap();
    assert_eq!(expr.eval(array), vec![json!(2)]);

    let expr = jsonpath::parse("$[0].name").unwrap();
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
    let expr = jsonpath::parse("$['-']").unwrap();
    assert_eq!(expr.eval(obj.clone()), vec![json!("hyphen")]);

    let expr = jsonpath::parse("$['_']").unwrap();
    assert_eq!(expr.eval(obj.clone()), vec![json!("underscore")]);

    let expr = jsonpath::parse("$['*']").unwrap();
    assert_eq!(expr.eval(obj.clone()), vec![json!("asterisk")]);

    let expr = jsonpath::parse("$['\\'']").unwrap();
    assert_eq!(expr.eval(obj.clone()), vec![json!("single_quote")]);

    let expr = jsonpath::parse("$['\"']").unwrap();
    assert_eq!(expr.eval(obj.clone()), vec![json!("double_quote")]);

    let expr = jsonpath::parse("$['✈']").unwrap();
    assert_eq!(expr.eval(obj.clone()), vec![json!("plane")]);

    // Dot notation
    let expr = jsonpath::parse("$._").unwrap();
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

    let expr = jsonpath::parse("$.*").unwrap();
    let results = expr.eval(obj.clone());
    let mut results = results
        .iter()
        .map(|e| e.as_str().unwrap())
        .collect::<Vec<&str>>();
    results.sort_unstable();
    assert_eq!(results, values);

    let expr = jsonpath::parse("$[*]").unwrap();
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
    let expr = jsonpath::parse("$.fruit[?(@.price.US==200)].name").unwrap();
    assert_eq!(expr.eval(fruit_prices_value()), vec![json!("grape")]);

    let expr = jsonpath::parse("$.fruit[?(@.pricex.US==200)].name").unwrap();
    assert!(expr.eval(fruit_prices_value()).is_empty());
}

#[test]
fn test_parsing_error() {
    // not supported yet
    assert!(jsonpath::parse("$..book[(@.length-1)]").is_err());
}
