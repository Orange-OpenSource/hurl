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

use hurl_core::reader::Reader;

use crate::jsonpath2::ast::literal::Number;
use crate::jsonpath2::parser::primitives::match_str;
use crate::jsonpath2::parser::{ParseError, ParseErrorKind, ParseResult};

/// Try to parse a decimal integer
/// if it does not start with a minus sign or a digit
/// it returns `None` rather than a `ParseError`
pub fn try_integer(reader: &mut Reader) -> ParseResult<Option<i32>> {
    if match_str("0", reader) {
        return Ok(Some(0));
    }
    let negative = match_str("-", reader);
    let saved_pos = reader.cursor().pos;
    let s = reader.read_while(|c| c.is_ascii_digit());

    if s.is_empty() || s.starts_with('0') {
        if negative {
            let kind = ParseErrorKind::Expecting("strictly positive digit".to_string());
            return Err(ParseError::new(saved_pos, kind));
        } else {
            return Ok(None);
        }
    }
    let sign = if negative { -1 } else { 1 };
    Ok(Some(sign * s.parse::<i32>().unwrap()))
}

// Try to parse a number
pub fn try_number(reader: &mut Reader) -> ParseResult<Option<Number>> {
    let save = reader.cursor();

    let int = if match_str("-0", reader) {
        0
    } else if let Some(value) = try_integer(reader)? {
        value
    } else {
        reader.seek(save);
        return Ok(None);
    };

    let fraction = try_fraction(reader)?;
    let exponent = try_exponent(reader)?;

    // At least a fraction or an exponent must be present to make it a float
    // otherwise it is an integer
    if fraction.is_none() && exponent.is_none() {
        Ok(Some(Number::Integer(int)))
    } else {
        let mut value = int as f64;
        if let Some(frac) = fraction {
            value += frac;
        }
        if let Some(exp) = exponent {
            value *= 10f64.powi(exp);
        }
        Ok(Some(Number::Float(value)))
    }
}

fn try_fraction(reader: &mut Reader) -> ParseResult<Option<f64>> {
    if match_str(".", reader) {
        let digits = reader.read_while(|c| c.is_ascii_digit());
        if digits.is_empty() {
            let kind = ParseErrorKind::Expecting("digit after decimal point".to_string());
            return Err(ParseError::new(reader.cursor().pos, kind));
        }
        let frac_string = format!("0.{}", digits);
        Ok(Some(frac_string.parse::<f64>().unwrap()))
    } else {
        Ok(None)
    }
}

fn try_exponent(reader: &mut Reader) -> ParseResult<Option<i32>> {
    if match_str("e", reader) || match_str("E", reader) {
        let mut exp_string = String::new();
        if match_str("+", reader) {
            exp_string.push('+');
        } else if match_str("-", reader) {
            exp_string.push('-');
        }
        let digits = reader.read_while(|c| c.is_ascii_digit());
        if digits.is_empty() {
            let kind = ParseErrorKind::Expecting("digit in exponent".to_string());
            return Err(ParseError::new(reader.cursor().pos, kind));
        }
        exp_string.push_str(&digits);
        Ok(Some(exp_string.parse::<i32>().unwrap()))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use hurl_core::reader::{CharPos, Reader};

    #[test]
    fn test_number() {
        let mut reader = Reader::new("110");
        assert_eq!(
            try_number(&mut reader).unwrap().unwrap(),
            Number::Integer(110)
        );
        assert_eq!(reader.cursor().index, CharPos(3));

        let mut reader = Reader::new("110.0");
        assert_eq!(
            try_number(&mut reader).unwrap().unwrap(),
            Number::Float(110.0)
        );
        assert_eq!(reader.cursor().index, CharPos(5));

        let mut reader = Reader::new("1.1e2");
        assert_eq!(
            try_number(&mut reader).unwrap().unwrap(),
            Number::Float(110.00000000000001)
        );
        assert_eq!(reader.cursor().index, CharPos(5));
    }

    #[test]
    fn test_float() {
        let mut reader = Reader::new("1.0");
        assert_eq!(
            try_number(&mut reader).unwrap().unwrap(),
            Number::Float(1.0)
        );
        assert_eq!(reader.cursor().index, CharPos(3));

        let mut reader = Reader::new("1e2");
        assert_eq!(
            try_number(&mut reader).unwrap().unwrap(),
            Number::Float(100.0)
        );
        assert_eq!(reader.cursor().index, CharPos(3));
    }

    #[test]
    fn test_integer() {
        let mut reader = Reader::new("1");
        assert_eq!(try_integer(&mut reader).unwrap().unwrap(), 1);
        assert_eq!(reader.cursor().index, CharPos(1));
    }

    #[test]
    fn test_fraction() {
        let mut reader = Reader::new(".02");
        assert_eq!(try_fraction(&mut reader).unwrap().unwrap(), 0.02);
        assert_eq!(reader.cursor().index, CharPos(3));

        let mut reader = Reader::new("e+2");
        assert!(try_fraction(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }

    #[test]
    fn test_exponent() {
        let mut reader = Reader::new("e-2");
        assert_eq!(try_exponent(&mut reader).unwrap().unwrap(), -2);
        assert_eq!(reader.cursor().index, CharPos(3));

        let mut reader = Reader::new("E+2");
        assert_eq!(try_exponent(&mut reader).unwrap().unwrap(), 2);
        assert_eq!(reader.cursor().index, CharPos(3));

        let mut reader = Reader::new("]");
        assert!(try_exponent(&mut reader).unwrap().is_none());
        assert_eq!(reader.cursor().index, CharPos(0));
    }
}
