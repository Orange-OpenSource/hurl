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
use std::str::FromStr;

use crate::parser::number::natural;
use crate::parser::{ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;
use crate::typing::{Duration, DurationUnit};

pub fn duration(reader: &mut Reader) -> ParseResult<Duration> {
    let value = natural(reader)?;
    let unit = duration_unit(reader)?;
    Ok(Duration::new(value, unit))
}

fn duration_unit(reader: &mut Reader) -> ParseResult<Option<DurationUnit>> {
    let pos = reader.cursor().pos;
    let s = reader.read_while(|c| c.is_ascii_alphabetic());
    if s.is_empty() {
        Ok(None)
    } else {
        match DurationUnit::from_str(&s) {
            Ok(unit) => Ok(Some(unit)),
            Err(_) => Err(ParseError {
                pos,
                kind: ParseErrorKind::InvalidDurationUnit(s),
                recoverable: false,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::U64;
    use crate::reader::Pos;
    use crate::typing::{DurationUnit, ToSource};

    #[test]
    fn test_duration_unit() {
        let mut reader = Reader::new("");
        assert!(duration_unit(&mut reader).unwrap().is_none());
        let mut reader = Reader::new("\n");
        assert!(duration_unit(&mut reader).unwrap().is_none());
        let mut reader = Reader::new("s\n");
        assert_eq!(
            duration_unit(&mut reader).unwrap().unwrap(),
            DurationUnit::Second
        );
        let mut reader = Reader::new("ms\n");
        assert_eq!(
            duration_unit(&mut reader).unwrap().unwrap(),
            DurationUnit::MilliSecond
        );
        let mut reader = Reader::new("m\n");
        assert_eq!(
            duration_unit(&mut reader).unwrap().unwrap(),
            DurationUnit::Minute
        );
    }

    #[test]
    fn test_duration_unit_error() {
        let mut reader = Reader::new("mms\n");
        let error = duration_unit(&mut reader).unwrap_err();
        assert_eq!(error.pos, Pos::new(1, 1));
        assert_eq!(
            error.kind,
            ParseErrorKind::InvalidDurationUnit("mms".to_string())
        );
    }

    #[test]
    fn test_duration() {
        let mut reader = Reader::new("10");
        assert_eq!(
            duration(&mut reader).unwrap(),
            Duration::new(U64::new(10, "10".to_source()), None)
        );
        let mut reader = Reader::new("10s");
        assert_eq!(
            duration(&mut reader).unwrap(),
            Duration::new(U64::new(10, "10".to_source()), Some(DurationUnit::Second))
        );
        let mut reader = Reader::new("10000ms");
        assert_eq!(
            duration(&mut reader).unwrap(),
            Duration::new(
                U64::new(10000, "10000".to_source()),
                Some(DurationUnit::MilliSecond)
            )
        );
    }

    #[test]
    fn test_duration_error() {
        let mut reader = Reader::new("10mms\n");
        let error = duration(&mut reader).unwrap_err();
        assert_eq!(error.pos, Pos::new(1, 3));
        assert_eq!(
            error.kind,
            ParseErrorKind::InvalidDurationUnit("mms".to_string())
        );
    }
}
