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

use super::ast::*;
use float_cmp::approx_eq;

pub type JsonpathResult = Vec<serde_json::Value>;

impl Query {
    pub fn eval(self, value: serde_json::Value) -> JsonpathResult {
        let mut results = vec![value];
        for selector in self.selectors {
            results = results
                .iter()
                .flat_map(|value| selector.clone().eval(value.clone()))
                .collect();
        }
        results
    }
}

impl Selector {
    pub fn eval(self, root: serde_json::Value) -> JsonpathResult {
        match self {
            Selector::Wildcard | Selector::ArrayWildcard => {
                let mut elements = vec![];
                if let serde_json::Value::Array(values) = root {
                    for value in values {
                        elements.push(value);
                    }
                } else if let serde_json::Value::Object(key_values) = root {
                    for value in key_values.values() {
                        elements.push(value.clone());
                    }
                }
                elements
            }
            Selector::NameChild(field) => match root.get(field) {
                None => vec![],
                Some(value) => vec![value.clone()],
            },
            Selector::ArrayIndex(indexes) => {
                let mut values = vec![];
                for index in indexes {
                    if let Some(value) = root.get(index) {
                        values.push(value.clone())
                    }
                }
                values
            }
            Selector::ArraySlice(Slice { start, end }) => {
                let mut elements = vec![];
                if let serde_json::Value::Array(values) = root {
                    for (i, value) in values.iter().enumerate() {
                        if let Some(n) = start {
                            let n = if n < 0 { values.len() as i64 + n } else { n };
                            if (i as i64) < n {
                                continue;
                            }
                        }
                        if let Some(n) = end {
                            let n = if n < 0 { values.len() as i64 + n } else { n };
                            if (i as i64) >= n {
                                continue;
                            }
                        }
                        elements.push(value.clone());
                    }
                }
                elements
            }
            Selector::Filter(predicate) => match root {
                serde_json::Value::Array(elements) => elements
                    .iter()
                    .filter(|&e| predicate.eval(e.clone()))
                    .cloned()
                    .collect(),
                _ => vec![],
            },
            Selector::RecursiveKey(key) => {
                let mut elements = vec![];
                match root {
                    serde_json::Value::Object(ref obj) => {
                        if let Some(elem) = obj.get(key.as_str()) {
                            elements.push(elem.clone());
                        }
                        for value in obj.values() {
                            for element in Selector::RecursiveKey(key.clone()).eval(value.clone()) {
                                elements.push(element);
                            }
                        }
                    }
                    serde_json::Value::Array(values) => {
                        for value in values {
                            for element in Selector::RecursiveKey(key.clone()).eval(value.clone()) {
                                elements.push(element);
                            }
                        }
                    }
                    _ => {}
                }
                elements
            }
            Selector::RecursiveWildcard => {
                let mut elements = vec![];
                match root {
                    serde_json::Value::Object(map) => {
                        for elem in map.values() {
                            elements.push(elem.clone());
                            for element in Selector::RecursiveWildcard.eval(elem.clone()) {
                                elements.push(element);
                            }
                        }
                    }
                    serde_json::Value::Array(values) => {
                        for elem in values {
                            elements.push(elem.clone());
                            for element in Selector::RecursiveWildcard.eval(elem.clone()) {
                                elements.push(element);
                            }
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
        match elem {
            serde_json::Value::Object(_) => {
                if let Some(value) = extract_value(elem, self.key.clone()) {
                    match (value, self.func.clone()) {
                        (_, PredicateFunc::KeyExist {}) => true,
                        (serde_json::Value::Number(v), PredicateFunc::Equal(ref num)) => {
                            approx_eq!(f64, v.as_f64().unwrap(), num.to_f64(), ulps = 2)
                        } //v.as_f64().unwrap() == num.to_f64(),
                        (serde_json::Value::Number(v), PredicateFunc::GreaterThan(ref num)) => {
                            v.as_f64().unwrap() > num.to_f64()
                        }
                        (
                            serde_json::Value::Number(v),
                            PredicateFunc::GreaterThanOrEqual(ref num),
                        ) => v.as_f64().unwrap() >= num.to_f64(),
                        (serde_json::Value::Number(v), PredicateFunc::LessThan(ref num)) => {
                            v.as_f64().unwrap() < num.to_f64()
                        }
                        (serde_json::Value::Number(v), PredicateFunc::LessThanOrEqual(ref num)) => {
                            v.as_f64().unwrap() <= num.to_f64()
                        }
                        (serde_json::Value::String(v), PredicateFunc::EqualString(ref s)) => {
                            v == *s
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

fn extract_value(obj: serde_json::Value, key_path: Vec<String>) -> Option<serde_json::Value> {
    let mut path = key_path;
    let mut value = obj;
    loop {
        if path.is_empty() {
            break;
        }
        let key = path.remove(0);
        match value.get(key) {
            None => return None,
            Some(v) => value = v.clone(),
        }
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

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
            "author": "Nigel Rees",
            "title": "Sayings of the Century",
            "price": 8.95
        })
    }

    pub fn json_second_book() -> serde_json::Value {
        json!({
            "category": "fiction",
            "author": "Evelyn Waugh",
            "title": "Sword of Honour",
            "price": 12.99
        })
    }

    pub fn json_third_book() -> serde_json::Value {
        json!({
            "category": "fiction",
            "author": "Herman Melville",
            "title": "Moby Dick",
            "isbn": "0-553-21311-3",
            "price": 8.99
        })
    }

    pub fn json_fourth_book() -> serde_json::Value {
        json!({
            "category": "fiction",
            "author": "J. R. R. Tolkien",
            "title": "The Lord of the Rings",
            "isbn": "0-395-19395-8",
            "price": 22.99
        })
    }

    #[test]
    pub fn test_query() {
        assert_eq!(
            Query { selectors: vec![] }.eval(json_root()),
            vec![json_root()]
        );

        assert_eq!(
            Query {
                selectors: vec![Selector::NameChild("store".to_string())]
            }
            .eval(json_root()),
            vec![json_store()]
        );

        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::ArrayIndex(vec![0]),
                Selector::NameChild("title".to_string()),
            ],
        };
        assert_eq!(
            query.eval(json_root()),
            vec![json!("Sayings of the Century")]
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
            query.eval(json_root()),
            vec![json!("Sayings of the Century"), json!("Moby Dick")]
        );

        // $..author
        let query = Query {
            selectors: vec![Selector::RecursiveKey("author".to_string())],
        };
        assert_eq!(
            query.eval(json_root()),
            vec![
                json!("Nigel Rees"),
                json!("Evelyn Waugh"),
                json!("Herman Melville"),
                json!("J. R. R. Tolkien")
            ]
        );

        // $.store.book[*].author
        let query = Query {
            selectors: vec![
                Selector::NameChild("store".to_string()),
                Selector::NameChild("book".to_string()),
                Selector::ArrayWildcard {},
                Selector::NameChild("author".to_string()),
            ],
        };
        assert_eq!(
            query.eval(json_root()),
            vec![
                json!("Nigel Rees"),
                json!("Evelyn Waugh"),
                json!("Herman Melville"),
                json!("J. R. R. Tolkien")
            ]
        );
    }

    #[test]
    pub fn test_selector_array_index() {
        assert_eq!(
            Selector::ArrayIndex(vec![0]).eval(json_books()),
            vec![json_first_book()]
        );
        assert_eq!(
            Selector::ArrayIndex(vec![1, 2]).eval(json_books()),
            vec![json_second_book(), json_third_book()]
        );
    }

    #[test]
    pub fn test_selector_array_wildcard() {
        assert_eq!(
            Selector::ArrayWildcard {}.eval(json_books()),
            vec![
                json_first_book(),
                json_second_book(),
                json_third_book(),
                json_fourth_book()
            ]
        );
    }

    #[test]
    pub fn test_selector_array_slice() {
        assert_eq!(
            Selector::ArraySlice(Slice {
                start: None,
                end: Some(2),
            })
            .eval(json_books()),
            vec![json_first_book(), json_second_book(),]
        );
    }

    #[test]
    pub fn test_recursive_key() {
        assert_eq!(
            Selector::RecursiveKey("author".to_string()).eval(json_root()),
            vec![
                json!("Nigel Rees"),
                json!("Evelyn Waugh"),
                json!("Herman Melville"),
                json!("J. R. R. Tolkien")
            ]
        );
    }

    // tests from https://cburgmer.github.io/json-path-comparison
    #[test]
    pub fn test_array_index() {
        let value = json!(["first", "second", "third", "forth", "fifth"]);
        assert_eq!(
            Selector::ArrayIndex(vec![2]).eval(value),
            vec![json!("third")]
        );
    }

    #[test]
    pub fn test_predicate() {
        assert!(Predicate {
            key: vec!["key".to_string()],
            func: PredicateFunc::KeyExist {},
        }
        .eval(json!({"key": "value"})));
        assert!(Predicate {
            key: vec!["key".to_string()],
            func: PredicateFunc::EqualString("value".to_string()),
        }
        .eval(json!({"key": "value"})));

        assert!(!Predicate {
            key: vec!["key".to_string()],
            func: PredicateFunc::EqualString("value".to_string()),
        }
        .eval(json!({"key": "some"})));

        assert!(Predicate {
            key: vec!["key".to_string()],
            func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
        }
        .eval(json!({"key": 1})));

        assert!(!Predicate {
            key: vec!["key".to_string()],
            func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
        }
        .eval(json!({"key": 2})));

        assert!(!Predicate {
            key: vec!["key".to_string()],
            func: PredicateFunc::Equal(Number { int: 1, decimal: 0 }),
        }
        .eval(json!({"key": "1"})));

        assert!(Predicate {
            key: vec!["key".to_string()],
            func: PredicateFunc::LessThan(Number {
                int: 10,
                decimal: 0,
            }),
        }
        .eval(json!({"key": 1})));
    }

    #[test]
    pub fn test_extract_value() {
        assert_eq!(
            extract_value(json!({"key": 1}), vec!["key".to_string()]).unwrap(),
            json!(1)
        );
        assert!(extract_value(json!({"key": 1}), vec!["unknown".to_string()]).is_none());
        assert_eq!(
            extract_value(
                json!({"key1": {"key2": 1}}),
                vec!["key1".to_string(), "key2".to_string()]
            )
            .unwrap(),
            json!(1)
        );
    }
}
