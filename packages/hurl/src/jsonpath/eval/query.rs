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

use crate::jsonpath::ast::Query;
use crate::jsonpath::JsonpathResult;

impl Query {
    /// Eval a JSONPath `Query` for a `serde_json::Value` input.
    /// It returns an Option<`JsonResultPath`>.
    pub fn eval(&self, value: &serde_json::Value) -> Option<JsonpathResult> {
        let mut result = JsonpathResult::SingleEntry(value.clone());
        for selector in &self.selectors {
            match result.clone() {
                JsonpathResult::SingleEntry(value) => {
                    result = selector.eval(&value)?;
                }
                JsonpathResult::Collection(values) => {
                    let mut elements = vec![];
                    for value in values {
                        match selector.eval(&value)? {
                            JsonpathResult::SingleEntry(new_value) => {
                                elements.push(new_value);
                            }
                            JsonpathResult::Collection(mut new_values) => {
                                elements.append(&mut new_values);
                            }
                        }
                        result = JsonpathResult::Collection(elements.clone());
                    }
                }
            }
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::jsonpath::ast::{Number, Predicate, PredicateFunc, Query, Selector};
    use crate::jsonpath::JsonpathResult;

    pub fn json_root() -> serde_json::Value {
        json!({ "store": json_store() })
    }

    pub fn json_store() -> serde_json::Value {
        json!({
            "book": json_books(),
            "bicycle": [
            ]
        })
    }

    pub fn json_books() -> serde_json::Value {
        json!([
            json_first_book(),
            json_second_book(),
            json_third_book(),
            json_fourth_book()
        ])
    }

    pub fn json_first_book() -> serde_json::Value {
        json!({
            "category": "reference",
            "published": false,
            "author": "Nigel Rees",
            "title": "Sayings of the Century",
            "price": 8.95
        })
    }

    pub fn json_second_book() -> serde_json::Value {
        json!({
            "category": "fiction",
            "published": false,
            "author": "Evelyn Waugh",
            "title": "Sword of Honour",
            "price": 12.99
        })
    }

    pub fn json_third_book() -> serde_json::Value {
        json!({
            "category": "fiction",
            "published": true,
            "author": "Herman Melville",
            "title": "Moby Dick",
            "isbn": "0-553-21311-3",
            "price": 8.99
        })
    }

    pub fn json_fourth_book() -> serde_json::Value {
        json!({
            "category": "fiction",
            "published": false,
            "author": "J. R. R. Tolkien",
            "title": "The Lord of the Rings",
            "isbn": "0-395-19395-8",
            "price": 22.99
        })
    }

    #[test]
    pub fn test_query() {
        assert_eq!(
            Query { selectors: vec![] }.eval(&json_root()).unwrap(),
            JsonpathResult::SingleEntry(json_root())
        );

        assert_eq!(
            Query {
                selectors: vec![Selector::NameChild("store".to_string())]
            }
            .eval(&json_root())
            .unwrap(),
            JsonpathResult::SingleEntry(json_store())
        );

        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::ArrayIndex(0),
                Selector::NameChild("title".to_string()),
            ],
        };
        assert_eq!(
            query.eval(&json_root()).unwrap(),
            JsonpathResult::SingleEntry(json!("Sayings of the Century"))
        );

        // $.store.book[?(@.price<10)].title
        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::Filter(Predicate {
                    key: vec!["price".to_string()],
                    func: PredicateFunc::LessThan(Number {
                        int: 10,
                        decimal: 0,
                    }),
                }),
                Selector::NameChild("title".to_string()),
            ],
        };
        assert_eq!(
            query.eval(&json_root()).unwrap(),
            JsonpathResult::Collection(vec![json!("Sayings of the Century"), json!("Moby Dick")])
        );

        // $.store.book[?(@.published==true)].title
        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::Filter(Predicate {
                    key: vec!["published".to_string()],
                    func: PredicateFunc::EqualBool(true),
                }),
                Selector::NameChild("title".to_string()),
            ],
        };
        assert_eq!(
            query.eval(&json_root()).unwrap(),
            JsonpathResult::Collection(vec![json!("Moby Dick")])
        );

        // $.store.book[?(@.published==false)].title
        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::Filter(Predicate {
                    key: vec!["published".to_string()],
                    func: PredicateFunc::EqualBool(false),
                }),
                Selector::NameChild("title".to_string()),
            ],
        };
        assert_eq!(
            query.eval(&json_root()).unwrap(),
            JsonpathResult::Collection(vec![
                json!("Sayings of the Century"),
                json!("Sword of Honour"),
                json!("The Lord of the Rings")
            ])
        );

        // $..author
        let query = Query {
            selectors: vec![Selector::RecursiveKey("author".to_string())],
        };
        assert_eq!(
            query.eval(&json_root()).unwrap(),
            JsonpathResult::Collection(vec![
                json!("Nigel Rees"),
                json!("Evelyn Waugh"),
                json!("Herman Melville"),
                json!("J. R. R. Tolkien")
            ])
        );

        // $.store.book[*].author
        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::ArrayWildcard,
                Selector::NameChild("author".to_string()),
            ],
        };
        assert_eq!(
            query.eval(&json_root()).unwrap(),
            JsonpathResult::Collection(vec![
                json!("Nigel Rees"),
                json!("Evelyn Waugh"),
                json!("Herman Melville"),
                json!("J. R. R. Tolkien")
            ])
        );
    }
}
