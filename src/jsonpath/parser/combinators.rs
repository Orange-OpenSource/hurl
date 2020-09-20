/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

pub fn optional<'a, T>(f: ParseFunc<'a, T>, p: &mut Reader) -> ParseResult<'a, Option<T>> {
    let start = p.state.clone();
    match f(p) {
        Ok(r) => Ok(Some(r)),
        Err(e) => {
            if e.recoverable {
                p.state = start;
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

// make an error recoverable
// but does not reset cursor
pub fn recover<'a, T>(f: ParseFunc<'a, T>, p: &mut Reader) -> ParseResult<'a, T> {
    //   let start = p.state.clone();
    match f(p) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error {
            pos: e.pos,
            recoverable: true,
            inner: e.inner,
        }),
    }
}

pub fn nonrecover<'a, T>(f: ParseFunc<'a, T>, p: &mut Reader) -> ParseResult<'a, T> {
    //let start = p.state.clone();
    match f(p) {
        Ok(r) => Ok(r),
        Err(e) => Err(Error {
            pos: e.pos,
            recoverable: false,
            inner: e.inner,
        }),
    }
}

pub fn zero_or_more<'a, T>(f: ParseFunc<'a, T>, p: &mut Reader) -> ParseResult<'a, Vec<T>> {
    let _start = p.state.clone();

    let mut v: Vec<T> = Vec::new();
    loop {
        let initial_state = p.state.clone();
        if p.is_eof() {
            return Ok(v);
        }

        match f(p) {
            Ok(r) => {
                v.push(r);
            }
            Err(e) => {
                if e.recoverable {
                    p.state.pos = initial_state.pos;
                    p.state.cursor = initial_state.cursor;
                    return Ok(v);
                } else {
                    return Err(e);
                };
            }
        }
    }
}

pub fn one_or_more<'a, T>(f: ParseFunc<'a, T>, reader: &mut Reader) -> ParseResult<'a, Vec<T>> {
    let _initial_state = reader.state.clone();
    match f(reader) {
        Ok(r) => {
            let mut v = vec![r];
            loop {
                let initial_state = reader.state.clone();
                match f(reader) {
                    Ok(r) => {
                        v.push(r);
                    }
                    Err(e) => {
                        if e.recoverable {
                            reader.state.pos = initial_state.pos;
                            reader.state.cursor = initial_state.cursor;
                            return Ok(v);
                        } else {
                            return Err(e);
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

// return the last error when no default error is specified
// tipically this should be recoverable
pub fn choice<'a, T>(fs: Vec<ParseFunc<'a, T>>, p: &mut Reader) -> ParseResult<'a, T> {
    match fs.get(0) {
        None => panic!("You can call choice with an empty vector of choice"),
        Some(f) => {
            let start = p.state.clone();
            if fs.len() == 1 {
                f(p)
            } else {
                match f(p) {
                    Err(Error {
                        recoverable: true, ..
                    }) => {
                        p.state = start;
                        choice(fs.clone().into_iter().skip(1).collect(), p)
                    }
                    x => x,
                }
            }
        }
    }
}

pub fn peek<T>(f: ParseFunc<T>, p: Reader) -> ParseResult<T> {
    let start = p.state.clone();
    let mut p = p;
    match f(&mut p) {
        Ok(r) => {
            p.state = start;
            Ok(r)
        }
        Err(e) => {
            p.state = start;
            Err(Error {
                pos: e.pos,
                recoverable: false,
                inner: e.inner,
            })
        }
    }
}
