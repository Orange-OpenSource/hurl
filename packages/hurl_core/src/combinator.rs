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
use crate::reader::Reader;

/// Represent a parser error. This type of error ca be recoverable or not and
/// implements conversion to recoverable / non-recoverable instance.
pub trait ParseError {
    /// Is this error recoverable or not?
    fn is_recoverable(&self) -> bool;

    /// Transforms this error to a recoverable one.
    fn to_recoverable(self) -> Self;

    /// Transforms this error to a non-recoverable one.
    fn to_non_recoverable(self) -> Self;
}

/// A parser func.
pub type ParseFunc<T, E> = fn(&mut Reader) -> Result<T, E>;

/// Try to consume one instances of the provided parser.
pub fn optional<T, E>(f: ParseFunc<T, E>, reader: &mut Reader) -> Result<Option<T>, E>
where
    E: ParseError,
{
    let start = reader.cursor();
    match f(reader) {
        Ok(r) => Ok(Some(r)),
        Err(e) => {
            if e.is_recoverable() {
                reader.seek(start);
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

/// Makes an error recoverable but does not reset cursor.
pub fn recover<T, E>(f: ParseFunc<T, E>, reader: &mut Reader) -> Result<T, E>
where
    E: ParseError,
{
    match f(reader) {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_recoverable()),
    }
}

/// Makes an error non recoverable.
pub fn non_recover<T, E>(f: ParseFunc<T, E>, reader: &mut Reader) -> Result<T, E>
where
    E: ParseError,
{
    match f(reader) {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_non_recoverable()),
    }
}

/// Consumes zero or more instances of the provided parser.
pub fn zero_or_more<T, E>(f: ParseFunc<T, E>, reader: &mut Reader) -> Result<Vec<T>, E>
where
    E: ParseError,
{
    let mut v = Vec::new();
    loop {
        let initial_state = reader.cursor();
        if reader.is_eof() {
            return Ok(v);
        }

        match f(reader) {
            Ok(r) => {
                v.push(r);
            }
            Err(e) => {
                return if e.is_recoverable() {
                    reader.seek(initial_state);
                    Ok(v)
                } else {
                    Err(e)
                };
            }
        }
    }
}

/// Consumes one or more instances of the provided parser.
pub fn one_or_more<T, E>(f: ParseFunc<T, E>, reader: &mut Reader) -> Result<Vec<T>, E>
where
    E: ParseError,
{
    match f(reader) {
        Ok(first) => {
            let mut v = vec![first];
            loop {
                let initial_state = reader.cursor();
                match f(reader) {
                    Ok(r) => {
                        v.push(r);
                    }
                    Err(e) => {
                        return if e.is_recoverable() {
                            reader.seek(initial_state);
                            Ok(v)
                        } else {
                            Err(e)
                        };
                    }
                }
            }
        }
        // if zero occurrence => should fail?
        Err(e) => Err(e.to_non_recoverable()),
    }
}

/// Tries to apply the list of parser until one of them succeeds.
/// Typically, this should be recoverable
pub fn choice<T, E>(fs: &[ParseFunc<T, E>], reader: &mut Reader) -> Result<T, E>
where
    E: ParseError,
{
    for (pos, f) in fs.iter().enumerate() {
        let start = reader.cursor();
        if pos == fs.len() - 1 {
            return f(reader);
        }
        match f(reader) {
            Err(err) if err.is_recoverable() => {
                reader.seek(start);
                continue;
            }
            x => return x,
        }
    }
    unreachable!("You can't call choice with an empty vector of choice")
}
