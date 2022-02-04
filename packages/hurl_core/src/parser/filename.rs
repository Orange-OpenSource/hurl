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

use super::error::*;
use super::reader::Reader;

use super::ParseResult;
use crate::ast::*;

pub fn parse(reader: &mut Reader) -> ParseResult<'static, Filename> {
    // This is an absolute file
    // that you have to write with a relative name
    // default root_dir is the hurl directory
    let start = reader.state.clone();
    let s = reader.read_while_escaping(|c| {
        c.is_alphanumeric() || *c == '.' || *c == '/' || *c == '_' || *c == '-'
    });
    if s.is_empty() {
        return Err(Error {
            pos: start.pos,
            recoverable: false,
            inner: ParseError::Filename {},
        });
    }

    Ok(Filename {
        value: s,
        source_info: SourceInfo {
            start: start.pos,
            end: reader.state.clone().pos,
        },
    })
}

#[cfg(test)]
mod tests {
    use crate::ast::Pos;

    use super::*;

    #[test]
    fn test_filename() {
        let mut reader = Reader::init("data/data.bin");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Filename {
                value: String::from("data/data.bin"),
                source_info: SourceInfo::init(1, 1, 1, 14),
            }
        );
        assert_eq!(reader.state.cursor, 13);

        let mut reader = Reader::init("data.bin");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Filename {
                value: String::from("data.bin"),
                source_info: SourceInfo::init(1, 1, 1, 9),
            }
        );
        assert_eq!(reader.state.cursor, 8);
    }

    #[test]
    fn test_include_space() {
        let mut reader = Reader::init("file\\ with\\ spaces");
        assert_eq!(
            parse(&mut reader).unwrap(),
            Filename {
                value: String::from("file with spaces"),
                source_info: SourceInfo::init(1, 1, 1, 19),
            }
        );
        assert_eq!(reader.state.cursor, 18);
    }

    #[test]
    fn test_filename_error() {
        let mut reader = Reader::init("???");
        let error = parse(&mut reader).err().unwrap();
        assert_eq!(error.inner, ParseError::Filename {});
        assert_eq!(error.pos, Pos { line: 1, column: 1 });
    }
}
