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
use super::{ParseFunc, ParseResult};

pub fn optional<'a, T>(f: ParseFunc<'a, T>, reader: &mut Reader) -> ParseResult<'a, Option<T>> {
    let start = reader.state.clone();
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

pub fn recover<'a, T>(f: ParseFunc<'a, T>, reader: &mut Reader) -> ParseResult<'a, T> {
    // make an error recoverable
    // but does not reset cursor
    match f(reader) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error {
            pos: e.pos,
            recoverable: true,
            inner: e.inner,
        }),
    }
}

pub fn nonrecover<'a, T>(f: ParseFunc<'a, T>, reader: &mut Reader) -> ParseResult<'a, T> {
    //let start = p.state.clone();
    match f(reader) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error {
            pos: e.pos,
            recoverable: false,
            inner: e.inner,
        }),
    }
}

pub fn zero_or_more<'a, T>(f: ParseFunc<'a, T>, reader: &mut Reader) -> ParseResult<'a, Vec<T>> {
    let _start = reader.state.clone();

    let mut v: Vec<T> = Vec::new();
    loop {
        let initial_state = reader.state.clone();
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

pub fn one_or_more<'a, T>(f: ParseFunc<'a, T>, reader: &mut Reader) -> ParseResult<'a, Vec<T>> {
    let _initial_state = reader.state.clone();
    match f(reader) {
        Ok(first) => {
            let mut v = vec![first];
            loop {
                let initial_state = reader.state.clone();
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
        Err(Error { pos, inner, .. }) => {
            // if zero occurence => should fail?
            Err(Error {
                pos,
                recoverable: false,
                inner,
            })
        }
    }
}

pub fn choice<'a, T>(fs: Vec<ParseFunc<'a, T>>, reader: &mut Reader) -> ParseResult<'a, T> {
    // return the last error when no default error is specified
    // tipically this should be recoverable
    match fs.get(0) {
        None => panic!("You can call choice with an empty vector of choice"),
        Some(f) => {
            let start = reader.state.clone();
            if fs.len() == 1 {
                f(reader)
            } else {
                match f(reader) {
                    Err(Error {
                        recoverable: true, ..
                    }) => {
                        reader.state = start;
                        choice(fs.clone().into_iter().skip(1).collect(), reader)
                    }
                    x => x,
                }
            }
        }
    }
}
