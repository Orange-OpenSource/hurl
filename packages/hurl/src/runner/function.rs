/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
use chrono::Utc;
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::{FirstName, LastName, Name};
use hurl_core::ast::Function;
use uuid::Uuid;

use super::error::RunnerError;
use super::number::Number;
use super::value::Value;

/// Alphabet used by the `randomString` function.
const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Evaluates the function `function`, returns a [`Value`] on success or an [`RunnerError`] .
pub fn eval(function: &Function) -> Result<Value, RunnerError> {
    match &function {
        Function::NewDate => {
            let now = Utc::now();
            Ok(Value::Date(now))
        }
        Function::NewUuid => {
            let uuid = Uuid::new_v4();
            Ok(Value::String(uuid.to_string()))
        }
        Function::RandomBool => Ok(Value::Bool(rand::random::<bool>())),
        Function::RandomEmail => Ok(Value::String(SafeEmail().fake())),
        Function::RandomFirstName => Ok(Value::String(FirstName().fake())),
        Function::RandomFullName => Ok(Value::String(Name().fake())),
        Function::RandomInt(args) => {
            // The parser has already rejected `min` greater than `max`, so the range is never
            // empty and `random_range` can not panic.
            let value = rand::random_range(args.min.as_i64()..=args.max.as_i64());
            Ok(Value::Number(Number::Integer(value)))
        }
        Function::RandomLastName => Ok(Value::String(LastName().fake())),
        Function::RandomString(args) => {
            let value = (0..args.count.as_u64())
                .map(|_| ALPHANUMERIC[rand::random_range(0..ALPHANUMERIC.len())] as char)
                .collect::<String>();
            Ok(Value::String(value))
        }
        Function::RandomWord => Ok(Value::String(Word().fake())),
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{I64, RandomIntArgs, RandomStringArgs, SourceInfo, U64, Whitespace};
    use hurl_core::reader::Pos;
    use hurl_core::types::ToSource;

    use super::*;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: " ".to_string(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    #[test]
    fn eval_random_int_is_within_bounds() {
        let function = Function::RandomInt(Box::new(RandomIntArgs {
            space0: whitespace(),
            min: I64::new(1, "1".to_source()),
            space1: whitespace(),
            max: I64::new(10, "10".to_source()),
        }));
        for _ in 0..100 {
            let Value::Number(Number::Integer(value)) = eval(&function).unwrap() else {
                panic!("randomInt should return an integer");
            };
            assert!((1..=10).contains(&value));
        }
    }

    #[test]
    fn eval_random_int_accepts_negative_bounds() {
        let function = Function::RandomInt(Box::new(RandomIntArgs {
            space0: whitespace(),
            min: I64::new(-5, "-5".to_source()),
            space1: whitespace(),
            max: I64::new(-5, "-5".to_source()),
        }));
        let Value::Number(Number::Integer(value)) = eval(&function).unwrap() else {
            panic!("randomInt should return an integer");
        };
        assert_eq!(value, -5);
    }

    #[test]
    fn eval_random_string_has_requested_length() {
        for count in [0, 1, 32] {
            let function = Function::RandomString(Box::new(RandomStringArgs {
                space0: whitespace(),
                count: U64::new(count, count.to_string().to_source()),
            }));
            let Value::String(value) = eval(&function).unwrap() else {
                panic!("randomString should return a string");
            };
            assert_eq!(value.chars().count(), count as usize);
            assert!(value.chars().all(|c| c.is_ascii_alphanumeric()));
        }
    }

    #[test]
    fn eval_random_email_looks_like_an_email() {
        let Value::String(value) = eval(&Function::RandomEmail).unwrap() else {
            panic!("randomEmail should return a string");
        };
        assert_eq!(value.matches('@').count(), 1);
    }

    #[test]
    fn eval_random_names_are_not_empty() {
        for function in [
            Function::RandomFirstName,
            Function::RandomLastName,
            Function::RandomFullName,
            Function::RandomWord,
        ] {
            let Value::String(value) = eval(&function).unwrap() else {
                panic!("{function} should return a string");
            };
            assert!(!value.is_empty());
        }
    }
}
