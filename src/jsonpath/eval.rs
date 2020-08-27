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
extern crate serde_json;

use super::ast::*;

pub type JsonpathResult = Vec<serde_json::Value>;


impl Query {
    pub fn eval(self, value: serde_json::Value) -> JsonpathResult {
        let mut results = vec![value];
        for selector in self.selectors {
            results = results.iter().flat_map(|value| selector.clone().eval(value.clone())).collect();
        }
        results
    }
}

impl Selector {
    pub fn eval(self, root: serde_json::Value) -> JsonpathResult {
        match self {
            Selector::NameChild(field) =>
                match root.get(field) {
                    None => vec![],
                    Some(value) => vec![value.clone()],
                }
            ,
            Selector::ArrayIndex(index) =>
                match root.get(index) {
                    None => vec![],
                    Some(value) => vec![value.clone()],
                }
            Selector::Filter(predicate) => {
                match root {
                    serde_json::Value::Array(elements) => {
                        elements
                            .iter()
                            .filter(|&e| predicate.eval(e.clone()))
                            .cloned()
                            .collect()
                    }
                    _ => vec![],
                }
            }
            Selector::RecursiveKey(key) => {
                let mut elements = vec![];
                match root {
                    serde_json::Value::Object(ref obj) => {
                        if let Some(elem) = obj.get(key.as_str()) {
                            elements.push(elem.clone());
                        }
                        for value in obj.values() {
                            eprintln!(">> value = {:?}", value);
                            for element in Selector::RecursiveKey(key.clone()).eval(value.clone()) {
                                elements.push(element);
                            };
                        }
                    }
                    serde_json::Value::Array(values) => {
                        for value in values {
                            for element in Selector::RecursiveKey(key.clone()).eval(value.clone()) {
                                elements.push(element);
                            };
                        }
                    }
                    _ => {}
                }
                elements
            }
        }
    }
}

impl Predicate {
    pub fn eval(&self, elem: serde_json::Value) -> bool {
        //eprintln!("eval elem {:?}", elem);
        match elem {
            serde_json::Value::Object(ref obj) => match (obj.get(self.key.as_str()), self.func.clone()) {
                (Some(serde_json::Value::Number(v)), PredicateFunc::Equal(ref num))
                => approx_eq!(f64, v.as_f64().unwrap(), num.to_f64(), ulps = 2), //v.as_f64().unwrap() == num.to_f64(),
                (Some(serde_json::Value::Number(v)), PredicateFunc::GreaterThan(ref num)) => v.as_f64().unwrap() > num.to_f64(),
                (Some(serde_json::Value::Number(v)), PredicateFunc::GreaterThanOrEqual(ref num)) => v.as_f64().unwrap() >= num.to_f64(),
                (Some(serde_json::Value::Number(v)), PredicateFunc::LessThan(ref num)) => v.as_f64().unwrap() < num.to_f64(),
                (Some(serde_json::Value::Number(v)), PredicateFunc::LessThanOrEqual(ref num)) => v.as_f64().unwrap() <= num.to_f64(),
                (Some(serde_json::Value::String(v)), PredicateFunc::EqualString(ref s)) => v == s,
                _ => false,
            },
            _ => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    pub fn json_root() -> serde_json::Value {
        json!({
        "store": json_store()
    })
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
              json_first_book(), json_second_book(), json_third_book(), json_fourth_book()
        ])
    }

    pub fn json_first_book() -> serde_json::Value {
        json!({
        "category": "reference",
        "author": "Nigel Rees",
        "title": "Sayings of the Century",
        "price": 8.95
          })
    }

    pub fn json_second_book() -> serde_json::Value {
        json!(              { "category": "fiction",
                "author": "Evelyn Waugh",
                "title": "Sword of Honour",
                "price": 12.99
              })
    }

    pub fn json_third_book() -> serde_json::Value {
        json!(              { "category": "fiction",
                            "author": "Herman Melville",
                            "title": "Moby Dick",
                            "isbn": "0-553-21311-3",
                            "price": 8.99
                        })
    }

    pub fn json_fourth_book() -> serde_json::Value {
        json!({ "category": "fiction",
                "author": "J. R. R. Tolkien",
                "title": "The Lord of the Rings",
                "isbn": "0-395-19395-8",
                "price": 22.99
              })
    }

    #[test]
    pub fn test_query() {
        assert_eq!(Query { selectors: vec![] }.eval(json_root()), vec![json_root()]);

        assert_eq!(Query { selectors: vec![Selector::NameChild("store".to_string())] }.eval(json_root()),
                   vec![json_store()]
        );

        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::ArrayIndex(0),
                Selector::NameChild("title".to_string()),
            ]
        };
        assert_eq!(query.eval(json_root()),
                   vec![json!("Sayings of the Century")]
        );

        // $.store.book[?(@.price<10)].title
        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::Filter(Predicate {
                    key: "price".to_string(),
                    func: PredicateFunc::LessThan(Number { int: 10, decimal: 0 }),
                }),
                Selector::NameChild("title".to_string()),
            ]
        };
        assert_eq!(query.eval(json_root()),
                   vec![json!("Sayings of the Century"), json!("Moby Dick")]
        );

        // $..author
        let query = Query {
            selectors: vec![
                Selector::RecursiveKey("author".to_string())
            ]
        };
        assert_eq!(query.eval(json_root()),
                   vec![json!("Nigel Rees"), json!("Evelyn Waugh"), json!("Herman Melville"), json!("J. R. R. Tolkien")]
        );
    }

    #[test]
    pub fn test_bookstore() {
//    assert_eq!(Selector::NameChild("store".to_string()).eval(json_root()),
//               vec![json_store()]
//    );
//    assert_eq!(Selector::NameChild("book".to_string()).eval(json_store()),
//               vec![json_books()]
//    );
//
//    assert_eq!(Selector::ArrayIndex(0).eval(json_books()),
//               vec![json_first_book()]
//    );
//
//    assert_eq!(Selector::NameChild("title".to_string()).eval(json_first_book()),
//               vec![json!("Sayings of the Century")]
//    );
//
//    assert_eq!(Selector::ArrayIndex(0).eval(json_books()),
//               vec![json_first_book()]
//    );
//    assert_eq!(Selector::Filter(Predicate::KeyEqualStringValue("category".to_string(), "reference".to_string())).eval(json_books()),
//               vec![json_first_book()]
//    );
//
//    assert_eq!(Selector::Filter(Predicate::KeySmallerThanIntValue("price".to_string(), 10)).eval(json_books()),
//               vec![json_first_book(), json_third_book()]
//    );
//
//    assert_eq!(Selector::RecursiveKey("book".to_string()).eval(json_root()),
//              vec![json_books()]
//    );

        assert_eq!(Selector::RecursiveKey("author".to_string()).eval(json_root()),
                   vec![json!("Nigel Rees"), json!("Evelyn Waugh"), json!("Herman Melville"), json!("J. R. R. Tolkien")]
        );
    }

    // tests from https://cburgmer.github.io/json-path-comparison
    #[test]
    pub fn test_array_index() {
        let value = json!(["first", "second", "third", "forth", "fifth"]);
        assert_eq!(Selector::ArrayIndex(2).eval(value),
                   vec![json!("third")]
        );
    }

    #[test]
    pub fn test_predicate() {
        assert_eq!(Predicate {
            key: "key".to_string(),
            func: PredicateFunc::EqualString("value".to_string()),
        }.eval(json!({"key": "value"})), true);

        assert_eq!(Predicate {
            key: "key".to_string(),
            func: PredicateFunc::EqualString("value".to_string()),
        }.eval(json!({"key": "some"})), false);

        assert_eq!(Predicate {
            key: "key".to_string(),
            func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
        }.eval(json!({"key": 1})), true);

        assert_eq!(Predicate {
            key: "key".to_string(),
            func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
        }.eval(json!({"key": 2})), false);

        assert_eq!(Predicate {
            key: "key".to_string(),
            func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
        }.eval(json!({"key": "1"})), false);

        assert_eq!(Predicate {
            key: "key".to_string(),
            func: PredicateFunc::LessThan(Number { int: 10, decimal: 0 }),
        }.eval(json!({"key": 1})), true);
    }
}
