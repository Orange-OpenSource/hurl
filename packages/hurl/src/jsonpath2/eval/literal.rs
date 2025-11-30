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

use crate::jsonpath2::ast::literal::{Literal, Number};

impl Literal {
    pub fn eval(&self) -> serde_json::Value {
        match self {
            Literal::String(s) => serde_json::Value::String(s.clone()),
            Literal::Number(n) => n.eval(),
            Literal::Bool(b) => serde_json::Value::Bool(*b),
            Literal::Null => serde_json::Value::Null,
        }
    }
}

impl Number {
    pub fn eval(&self) -> serde_json::Value {
        match self {
            Number::Integer(n) => serde_json::Number::from_i128(*n as i128).unwrap().into(),
            Number::Float(n) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::jsonpath2::ast::literal::{Literal, Number};

    #[test]
    pub fn test_number() {
        // Using different float values
        let value1 = Literal::Number(Number::Float(110.0)).eval();
        let value2 = Literal::Number(Number::Float(110.00000000000001)).eval();
        assert!(value1 != value2);
    }
}
