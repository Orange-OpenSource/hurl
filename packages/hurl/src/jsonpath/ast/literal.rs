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

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Number(Number),
    Null,
    String(String),
}

impl Eq for Literal {}

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    BigInteger(String),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FiniteFloatError {
    Inf,
    NaN,
}

impl Number {
    pub fn try_finite_float(value: f64) -> Result<Self, FiniteFloatError> {
        if value.is_nan() {
            Err(FiniteFloatError::NaN)
        } else if value.is_infinite() {
            Err(FiniteFloatError::Inf)
        } else {
            Ok(Self::Float(value))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_try_finite_float() {
        assert_eq!(Number::try_finite_float(1.23).unwrap(), Number::Float(1.23));
        assert_eq!(
            Number::try_finite_float(f64::INFINITY).unwrap_err(),
            FiniteFloatError::Inf
        );
        assert_eq!(
            Number::try_finite_float(f64::NAN).unwrap_err(),
            FiniteFloatError::NaN
        );
    }
}
