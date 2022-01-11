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
                return if e.recoverable {
                    p.state.pos = initial_state.pos;
                    p.state.cursor = initial_state.cursor;
                    Ok(v)
                } else {
                    Err(e)
                };
            }
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
