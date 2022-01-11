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
// https://cburgmer.github.io/json-path-comparison/
// https://goessner.net/articles/JsonPath/
// https://jsonpath.com/

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    pub selectors: Vec<Selector>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector {
    Wildcard,
    NameChild(String),
    ArrayIndex(Vec<usize>), // one or more indexes (separated by comma)
    ArraySlice(Slice),
    ArrayWildcard,
    Filter(Predicate),
    RecursiveWildcard,
    RecursiveKey(String),
}

// For the time-being
// use simple slice start:end (without the step)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slice {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Predicate {
    pub key: Vec<String>,
    pub func: PredicateFunc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PredicateFunc {
    KeyExist {},
    EqualString(String),
    Equal(Number),
    GreaterThan(Number),
    GreaterThanOrEqual(Number),
    LessThan(Number),
    LessThanOrEqual(Number),
}

// Number
// - without rounding
// - Equalable
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Number {
    pub int: i64,
    pub decimal: u64,
}

impl Number {
    pub fn to_f64(&self) -> f64 {
        self.int as f64 + self.decimal as f64 / 1_000_000_000_000_000_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_number() {
        assert!((Number { int: 1, decimal: 0 }.to_f64() - 1.0).abs() < 0.0000001);
    }
}
