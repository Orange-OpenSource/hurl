use serde_json::json;

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
mod cts;

use crate::jsonpath2::{self, eval::NodeList};

fn store_value() -> serde_json::Value {
    json!(
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
    )
}

fn book_value() -> serde_json::Value {
    json!(
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
     )
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

fn eval(value: &serde_json::Value, query: &str) -> NodeList {
    let expr = jsonpath2::parse(query).unwrap();
    expr.eval(value)
}

#[test]
fn root_identifier() {
    assert_eq!(eval(&store_value(), "$"), vec![store_value()]);
}

#[test]
fn child_segment() {
    assert_eq!(eval(&store_value(), "$['book']"), vec![book_value()]);
    assert_eq!(eval(&store_value(), "$.book"), vec![book_value()]);
    assert_eq!(eval(&store_value(), "$.book[0]"), vec![book0_value()]);
    assert_eq!(
        eval(&store_value(), "$.book[0].author"),
        vec![json!("Nigel Rees")]
    );
    assert_eq!(
        eval(&store_value(), "$.*"),
        vec![bicycle_value(), book_value()]
    );
    assert_eq!(
        eval(&store_value(), "$.book[:2]"),
        vec![book0_value(), book1_value()]
    );
    assert_eq!(
        eval(&store_value(), "$.book[?@.isbn]"),
        vec![book2_value(), book3_value()]
    );
    assert_eq!(
        eval(&store_value(), "$.book[0,1]"),
        vec![book0_value(), book1_value()]
    );
}

#[test]
fn descendant_segment() {
    assert_eq!(
        eval(&store_value(), "$..author"),
        vec![
            json!("Nigel Rees"),
            json!("Evelyn Waugh"),
            json!("Herman Melville"),
            json!("J. R. R. Tolkien")
        ]
    );
    assert_eq!(eval(&store_value(), "$..book[2]"), vec![book2_value()]);
    assert_eq!(eval(&store_value(), "$..book[-1]"), vec![book3_value()]);
    assert_eq!(
        eval(&store_value(), "$..book[:2]"),
        vec![book0_value(), book1_value()]
    );
}
