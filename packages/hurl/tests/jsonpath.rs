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
extern crate hurl;

use std::fs::read_to_string;

use serde_json;
use serde_json::json;

use hurl::jsonpath;

fn test_ok(s: &str, value: serde_json::Value) -> Vec<serde_json::Value> {
    return match jsonpath::parse(s) {
        Ok(expr) => expr.eval(value),
        Err(e) => panic!("{:?}", e),
    };
}

#[test]
fn test_bookstore_path() {
    let no_result: Vec<serde_json::Value> = vec![];
    let s = read_to_string("tests/bookstore.json").expect("could not read string from file");
    let value: serde_json::Value =
        serde_json::from_str(s.as_str()).expect("could not parse json file");

    assert_eq!(
        test_ok("$.store.book[0].title", value.clone()),
        vec![json!("Sayings of the Century")]
    );
    assert_eq!(
        test_ok("$.store.book.[0].title", value.clone()),
        vec![json!("Sayings of the Century")]
    );
    assert_eq!(
        test_ok("$.store.book[0].title", value.clone()),
        vec![json!("Sayings of the Century")]
    );
    assert_eq!(
        test_ok("$.store.book[?(@.price<10)].title", value.clone()),
        vec![json!("Sayings of the Century"), json!("Moby Dick")]
    );
    assert_eq!(
        test_ok("$.store.book[?(@.price < 10)].title", value.clone()),
        vec![json!("Sayings of the Century"), json!("Moby Dick")]
    );

    assert_eq!(
        test_ok("$..book[2]", value.clone()),
        vec![json!({
          "category": "fiction",
          "author": "Herman Melville",
          "title": "Moby Dick",
          "isbn": "0-553-21311-3",
          "price": 8.99
        })]
    );
    assert_eq!(
        test_ok("$..author", value.clone()),
        vec![
            json!("Nigel Rees"),
            json!("Evelyn Waugh"),
            json!("Herman Melville"),
            json!("J. R. R. Tolkien")
        ]
    );
    assert_eq!(
        test_ok("$.store.book[?(@.price>100)]", value.clone()),
        no_result.clone()
    );
}

#[test]
fn test_array() {
    let array = json!([0, 1, 2, 3]);
    assert_eq!(test_ok("$[2]", array.clone()), vec![json!(2)]);

    let array = json!([{"name": "Bob"},{"name": "Bill"}]);
    assert_eq!(test_ok("$[0].name", array.clone()), vec![json!("Bob")]);
}
