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
use crate::ast::HurlFile;

pub type ParseResult<'a, T> = Result<T, Error>;
pub type ParseFunc<'a, T> = fn(&mut Reader) -> ParseResult<'a, T>;

pub fn parse_hurl_file(s: &str) -> ParseResult<'static, HurlFile> {
    let mut reader = Reader::init(s);
    parsers::hurl_file(&mut reader)
}

pub use self::error::{Error, ParseError};
pub use self::json::boolean_value as parse_json_boolean;
pub use self::json::null_value as parse_json_null;
pub use self::json::number_value as parse_json_number;
pub use self::json::parse as parse_json;
pub use self::reader::Reader;
pub use self::template::templatize;

mod base64;
mod bytes;
mod combinators;
mod cookiepath;
mod error;
mod expr;
mod filename;
mod json;
mod parsers;
mod predicate;
mod predicate_value;
mod primitives;
mod query;
mod reader;
mod sections;
mod string;
mod subquery;
mod template;
mod url;
mod xml;
