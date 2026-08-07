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
use crate::ast::{Function, I64, RandomIntArgs, RandomStringArgs, U64, Whitespace};
use crate::parser::number::{integer, natural};
use crate::parser::primitives::one_or_more_spaces;
use crate::parser::{ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

/// Parse a function
///
/// Functions parameters are separated by whitespaces, like filters parameters:
/// `randomInt 1 100`.
pub fn parse(reader: &mut Reader) -> ParseResult<Function> {
    let start = reader.cursor();
    let function_name = reader.read_while(|c| c.is_alphanumeric() || c == '_' || c == '-');
    match function_name.as_str() {
        "newDate" => Ok(Function::NewDate),
        "newUuid" => Ok(Function::NewUuid),
        "randomBool" => Ok(Function::RandomBool),
        "randomEmail" => Ok(Function::RandomEmail),
        "randomFirstName" => Ok(Function::RandomFirstName),
        "randomFullName" => Ok(Function::RandomFullName),
        "randomInt" => {
            let space0 = space_param(reader)?;
            let min = integer_param(reader)?;
            let space1 = space_param(reader)?;
            let start_max = reader.cursor();
            let max = integer_param(reader)?;
            // Both bounds are literals, so an empty range can be rejected right away rather than
            // at run time.
            if min.as_i64() > max.as_i64() {
                let kind = ParseErrorKind::Expecting {
                    value: format!("an integer greater than or equal to {min}"),
                };
                return Err(ParseError::new(start_max.pos, false, kind));
            }
            Ok(Function::RandomInt(Box::new(RandomIntArgs {
                space0,
                min,
                space1,
                max,
            })))
        }
        "randomLastName" => Ok(Function::RandomLastName),
        "randomString" => {
            let space0 = space_param(reader)?;
            let count = natural_param(reader)?;
            Ok(Function::RandomString(Box::new(RandomStringArgs {
                space0,
                count,
            })))
        }
        "randomWord" => Ok(Function::RandomWord),
        _ => Err(ParseError::new(
            start.pos,
            true,
            ParseErrorKind::Expecting {
                value: "function".to_string(),
            },
        )),
    }
}

/// Parses the whitespace preceding a function parameter.
///
/// The function name has already been consumed at this point, so the error is not recoverable:
/// we don't want the caller to backtrack and parse the function name as a variable.
fn space_param(reader: &mut Reader) -> ParseResult<Whitespace> {
    one_or_more_spaces(reader).map_err(|e| ParseError::new(e.pos, false, e.kind))
}

/// Parses an integer function parameter.
///
/// As [`space_param`], the error is not recoverable.
fn integer_param(reader: &mut Reader) -> ParseResult<I64> {
    integer(reader).map_err(|e| {
        let kind = ParseErrorKind::Expecting {
            value: "integer".to_string(),
        };
        ParseError::new(e.pos, false, kind)
    })
}

/// Parses a natural function parameter.
///
/// As [`space_param`], the error is not recoverable. Using a natural rather than an integer means
/// that negative values are rejected by the number parser itself.
fn natural_param(reader: &mut Reader) -> ParseResult<U64> {
    natural(reader).map_err(|e| {
        let kind = ParseErrorKind::Expecting {
            value: "natural".to_string(),
        };
        ParseError::new(e.pos, false, kind)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::SourceInfo;
    use crate::reader::Pos;
    use crate::types::ToSource;

    #[test]
    fn test_exist() {
        let mut reader = Reader::new("newUuid");
        assert_eq!(parse(&mut reader).unwrap(), Function::NewUuid);
    }

    #[test]
    fn test_not_exist() {
        let mut reader = Reader::new("name");
        let err = parse(&mut reader).unwrap_err();
        assert_eq!(err.pos, Pos::new(1, 1));
        assert!(err.recoverable);
    }

    #[test]
    fn test_random_no_param() {
        let mut reader = Reader::new("randomFirstName");
        assert_eq!(parse(&mut reader).unwrap(), Function::RandomFirstName);

        let mut reader = Reader::new("randomBool");
        assert_eq!(parse(&mut reader).unwrap(), Function::RandomBool);
    }

    #[test]
    fn test_random_int() {
        let mut reader = Reader::new("randomInt 1 100");
        let function = parse(&mut reader).unwrap();
        assert_eq!(
            function,
            Function::RandomInt(Box::new(RandomIntArgs {
                space0: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 10), Pos::new(1, 11)),
                },
                min: I64::new(1, "1".to_source()),
                space1: Whitespace {
                    value: " ".to_string(),
                    source_info: SourceInfo::new(Pos::new(1, 12), Pos::new(1, 13)),
                },
                max: I64::new(100, "100".to_source()),
            }))
        );
    }

    #[test]
    fn test_random_int_negative() {
        let mut reader = Reader::new("randomInt -10 -1");
        let function = parse(&mut reader).unwrap();
        assert_eq!(function.to_string(), "randomInt -10 -1");
    }

    #[test]
    fn test_random_string() {
        let mut reader = Reader::new("randomString 10");
        let function = parse(&mut reader).unwrap();
        assert_eq!(function.to_string(), "randomString 10");
    }

    /// Extra whitespaces between parameters are kept, so that a file can be rendered back to its
    /// exact source.
    #[test]
    fn test_random_int_keeps_spaces() {
        let mut reader = Reader::new("randomInt  1   100");
        let function = parse(&mut reader).unwrap();
        assert_eq!(function.to_string(), "randomInt  1   100");
    }

    /// Once the function name has been read, a malformed parameter is not recoverable: the caller
    /// must not fall back on parsing `randomInt` as a variable name.
    #[test]
    fn test_random_int_missing_param() {
        let mut reader = Reader::new("randomInt 1");
        let err = parse(&mut reader).unwrap_err();
        assert!(!err.recoverable);

        let mut reader = Reader::new("randomInt");
        let err = parse(&mut reader).unwrap_err();
        assert!(!err.recoverable);

        let mut reader = Reader::new("randomInt a b");
        let err = parse(&mut reader).unwrap_err();
        assert!(!err.recoverable);
        assert_eq!(
            err.kind,
            ParseErrorKind::Expecting {
                value: "integer".to_string()
            }
        );
    }

    /// Both bounds are literals, so an empty range is a parse error, not a run time one.
    #[test]
    fn test_random_int_empty_range() {
        let mut reader = Reader::new("randomInt 100 1");
        let err = parse(&mut reader).unwrap_err();
        assert!(!err.recoverable);
        assert_eq!(err.pos, Pos::new(1, 15));
        assert_eq!(
            err.kind,
            ParseErrorKind::Expecting {
                value: "an integer greater than or equal to 100".to_string()
            }
        );

        // A range of exactly one value is valid.
        let mut reader = Reader::new("randomInt 5 5");
        assert!(parse(&mut reader).is_ok());
    }

    /// `randomString` takes a natural, so a negative count cannot be expressed.
    #[test]
    fn test_random_string_negative_count() {
        let mut reader = Reader::new("randomString -1");
        let err = parse(&mut reader).unwrap_err();
        assert!(!err.recoverable);
        assert_eq!(
            err.kind,
            ParseErrorKind::Expecting {
                value: "natural".to_string()
            }
        );
    }
}
