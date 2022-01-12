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

use error::Error;
use reader::Reader;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

pub type ParseResult<'a, T> = std::result::Result<T, Error>;
pub type ParseFunc<'a, T> = fn(&mut Reader) -> ParseResult<'a, T>;

pub use self::parse::parse;

mod combinators;
mod error;
mod parse;
mod primitives;
mod reader;
