/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use crate::parser::error::*;
use crate::parser::reader::Reader;
use crate::parser::{ParseFunc, ParseResult};

pub fn optional<T>(f: ParseFunc<T>, reader: &mut Reader) -> ParseResult<Option<T>> {
    let start = reader.state;
    match f(reader) {
        Ok(r) => Ok(Some(r)),
        Err(e) => {
            if e.recoverable {
                reader.state = start;
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

pub fn recover<T>(f: ParseFunc<T>, reader: &mut Reader) -> ParseResult<T> {
    // make an error recoverable
    // but does not reset cursor
    match f(reader) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error::new(e.pos, true, e.inner)),
    }
}

pub fn nonrecover<T>(f: ParseFunc<T>, reader: &mut Reader) -> ParseResult<T> {
    match f(reader) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error::new(e.pos, false, e.inner)),
    }
}

pub fn zero_or_more<T>(f: ParseFunc<T>, reader: &mut Reader) -> ParseResult<Vec<T>> {
    let _start = reader.state;

    let mut v: Vec<T> = Vec::new();
    loop {
        let initial_state = reader.state;
        if reader.is_eof() {
            return Ok(v);
        }

        match f(reader) {
            Ok(r) => {
                v.push(r);
            }
            Err(e) => {
                return if e.recoverable {
                    reader.state.pos = initial_state.pos;
                    reader.state.cursor = initial_state.cursor;
                    Ok(v)
                } else {
                    Err(e)
                };
            }
        }
    }
}

pub fn one_or_more<T>(f: ParseFunc<T>, reader: &mut Reader) -> ParseResult<Vec<T>> {
    let _initial_state = reader.state;
    match f(reader) {
        Ok(first) => {
            let mut v = vec![first];
            loop {
                let initial_state = reader.state;
                match f(reader) {
                    Ok(r) => {
                        v.push(r);
                    }
                    Err(e) => {
                        return if e.recoverable {
                            reader.state.pos = initial_state.pos;
                            reader.state.cursor = initial_state.cursor;
                            Ok(v)
                        } else {
                            Err(e)
                        };
                    }
                }
            }
        }
        // if zero occurrence => should fail?
        Err(Error { pos, inner, .. }) => Err(Error::new(pos, false, inner)),
    }
}

/// Tries to apply the list of parser functions `fs` until one of them succeeds.
/// Typically this should be recoverable
pub fn choice<T>(fs: &[ParseFunc<T>], reader: &mut Reader) -> ParseResult<T> {
    for (pos, f) in fs.iter().enumerate() {
        let start = reader.state;
        if pos == fs.len() - 1 {
            return f(reader);
        }
        match f(reader) {
            Err(Error {
                recoverable: true, ..
            }) => {
                reader.state = start;
                continue;
            }
            x => return x,
        }
    }
    panic!("You can't call choice with an empty vector of choice")
}
