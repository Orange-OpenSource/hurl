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

mod comparison;
mod error;
mod literal;
mod primitives;
mod query;
mod segments;
mod selectors;
mod singular_query;

use crate::jsonpath2::ast::JsonPathQuery;
pub use error::{ParseError, ParseErrorKind};
use hurl_core::reader::Reader;

pub type ParseResult<T> = Result<T, ParseError>;

#[allow(dead_code)]
pub fn parse(s: &str) -> ParseResult<JsonPathQuery> {
    let mut reader = Reader::new(s);
    query::parse(&mut reader)
}
