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
use crate::ast::Function;
use crate::parser::{ParseError, ParseErrorKind, ParseResult};
use crate::reader::Reader;

/// Parse a function
///
pub fn parse(reader: &mut Reader) -> ParseResult<Function> {
    let start = reader.cursor();
    let function_name = reader.read_while(|c| c.is_alphanumeric() || c == '_' || c == '-');
    match function_name.as_str() {
        "newDate" => Ok(Function::NewDate),
        "newUuid" => Ok(Function::NewUuid),
        _ => Err(ParseError::new(
            start.pos,
            true,
            ParseErrorKind::Expecting {
                value: "function".to_string(),
            },
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::Pos;

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
}
