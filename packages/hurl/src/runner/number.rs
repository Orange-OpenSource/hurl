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
use std::cmp::Ordering;
use std::fmt;

/// System types used in Hurl.
///
/// Values are used by queries, captures, asserts and predicates.
#[derive(Clone, Debug)]
pub enum Number {
    Float(f64),
    Integer(i64),
    BigInteger(String),
}

// You must implement it yourself because of the Float
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Float(v1), Number::Float(v2)) => (v1 - v2).abs() < f64::EPSILON,
            (Number::Integer(v1), Number::Integer(v2)) => v1 == v2,
            (Number::BigInteger(v1), Number::BigInteger(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl Eq for Number {}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Number::Float(f) => format_float(*f),
            Number::Integer(x) => x.to_string(),
            Number::BigInteger(s) => s.to_string(),
        };
        write!(f, "{value}")
    }
}

fn format_float(value: f64) -> String {
    if value.fract() < f64::EPSILON {
        format!("{value}.0")
    } else {
        value.to_string()
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::Integer(value)
    }
}

impl Number {
    pub fn cmp_value(&self, other: &Number) -> Ordering {
        match (self, other) {
            (Number::Integer(i1), Number::Integer(i2)) => i1.cmp(i2),
            (Number::Float(f1), Number::Float(f2)) => compare_float(*f1, *f2),
            (Number::Integer(i1), Number::Float(f2)) => compare_float(*i1 as f64, *f2),
            (Number::Float(f1), Number::Integer(i2)) => compare_float(*f1, *i2 as f64),
            (n1, n2) => compare_number_string(&n1.to_string(), &n2.to_string()),
        }
    }
}

fn compare_float(f1: f64, f2: f64) -> Ordering {
    if f1 > f2 {
        Ordering::Greater
    } else if f1 < f2 {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn compare_number_string(n1: &str, n2: &str) -> Ordering {
    let (neg1, i1, d1) = number_components(n1);
    let (neg2, i2, d2) = number_components(n2);
    if neg1 == neg2 {
        match i1.cmp(i2) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => d1.cmp(d2),
        }
    } else if neg1 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

// return triple (negative, integer, decimals)
fn number_components(s: &str) -> (bool, &str, &str) {
    match s.strip_prefix('-') {
        None => match s.find('.') {
            None => (false, s.trim_start_matches('0'), ""),
            Some(index) => (
                false,
                (s[..index].trim_start_matches('0')),
                (s[(index + 1)..].trim_end_matches('0')),
            ),
        },
        Some(s) => {
            let (_, integer, decimal) = number_components(s);
            (true, integer, decimal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(Number::Float(1.0).to_string(), "1.0".to_string());
        assert_eq!(Number::Float(1.1).to_string(), "1.1".to_string());
        assert_eq!(Number::from(1.0).to_string(), "1.0".to_string());
        assert_eq!(Number::from(1.1).to_string(), "1.1".to_string());
        assert_eq!(
            Number::BigInteger("1.1".to_string()).to_string(),
            "1.1".to_string()
        );
        assert_eq!(
            Number::BigInteger("1".to_string()).to_string(),
            "1".to_string()
        );
    }

    #[test]
    fn test_cmp_value() {
        let integer_zero = Number::from(0);
        let integer_minus_one = Number::from(-1);
        let integer_one = Number::from(1);
        let integer_two = Number::from(2);
        let integer_max = Number::from(i64::MAX);
        let integer_min = Number::from(i64::MIN);

        let float_zero = Number::from(0.0);
        let float_one = Number::from(1.0);
        let float_one_plus_epsilon = Number::from(1.000_000_000_000_000_100);
        let float_one_plus_plus_epsilon = Number::from(1.000_000_000_000_001);
        let float_two = Number::from(2.0);
        let float_min = Number::from(f64::MIN);
        let float_max = Number::from(f64::MAX);

        let number_one = Number::BigInteger("1".to_string());
        let number_two = Number::BigInteger("2".to_string());
        let number_two_with_decimal = Number::BigInteger("2.0".to_string());

        assert_eq!(integer_minus_one.cmp_value(&integer_zero), Ordering::Less);

        assert_eq!(integer_one.cmp_value(&integer_one), Ordering::Equal);
        assert_eq!(integer_one.cmp_value(&number_one), Ordering::Equal);
        assert_eq!(integer_one.cmp_value(&float_one), Ordering::Equal);
        assert_eq!(integer_one.cmp_value(&integer_zero), Ordering::Greater);
        assert_eq!(integer_one.cmp_value(&float_zero), Ordering::Greater);
        assert_eq!(integer_one.cmp_value(&integer_two), Ordering::Less);
        assert_eq!(integer_one.cmp_value(&float_two), Ordering::Less);
        assert_eq!(integer_one.cmp_value(&number_two), Ordering::Less);
        assert_eq!(
            integer_one.cmp_value(&number_two_with_decimal),
            Ordering::Less
        );

        assert_eq!(integer_min.cmp_value(&float_min), Ordering::Greater);
        assert_eq!(integer_max.cmp_value(&float_max), Ordering::Less);

        assert_eq!(float_one.cmp_value(&float_one), Ordering::Equal);
        assert_eq!(
            float_one.cmp_value(&float_one_plus_epsilon),
            Ordering::Equal
        );
        assert_eq!(
            float_one.cmp_value(&float_one_plus_plus_epsilon),
            Ordering::Less
        );

        // edge cases
        // the integer 9_007_199_254_740_993 can not be represented by f64
        // it will be casted to 9_007_199_254_740_992 for comparison
        assert_eq!(
            Number::from(9_007_199_254_740_992.0).cmp_value(&Number::from(9_007_199_254_740_993)),
            Ordering::Equal
        );
    }

    #[test]
    fn test_cmp_number_string() {
        assert_eq!(compare_number_string("1", "1"), Ordering::Equal);
        assert_eq!(compare_number_string("1", "1.0"), Ordering::Equal);
        assert_eq!(compare_number_string("1.000", "1.0"), Ordering::Equal);
        assert_eq!(compare_number_string("1", "2"), Ordering::Less);
        assert_eq!(compare_number_string("1.1", "2"), Ordering::Less);
        assert_eq!(compare_number_string("-001.1000", "-1.1"), Ordering::Equal);
    }

    #[test]
    fn test_number_components() {
        assert_eq!(number_components("1"), (false, "1", ""));
        assert_eq!(number_components("1.0"), (false, "1", ""));
        assert_eq!(number_components("01"), (false, "1", ""));

        assert_eq!(number_components("1.1"), (false, "1", "1"));
        assert_eq!(number_components("1.100"), (false, "1", "1"));

        assert_eq!(number_components("-1.1"), (true, "1", "1"));
        assert_eq!(number_components("-01.100"), (true, "1", "1"));
    }
}
