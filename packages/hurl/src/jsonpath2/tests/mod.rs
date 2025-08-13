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
use crate::jsonpath2::{self};

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

fn book0_value() -> serde_json::Value {
    json!(  { "category": "reference",
      "author": "Nigel Rees",
      "title": "Sayings of the Century",
      "price": 8.95
    })
}

#[test]
fn root_identifier() {
    let expr = jsonpath2::parse("$").unwrap();
    assert_eq!(expr.eval(&store_value()), vec![store_value()]);
}

#[test]
fn child_child_segment() {
    let expr = jsonpath2::parse("$['book']").unwrap();
    assert_eq!(expr.eval(&store_value()), vec![book_value()]);

    let expr = jsonpath2::parse("$['book'][0]").unwrap();
    assert_eq!(expr.eval(&store_value()), vec![book0_value()]);

    let expr = jsonpath2::parse("$['book'][0]['author']").unwrap();
    assert_eq!(expr.eval(&store_value()), vec![json!("Nigel Rees")]);
}
