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
use crate::core::ast::HurlFile;
use error::Error;

mod base64;
mod bytes;
mod combinators;
mod cookiepath;
pub mod error;
mod expr;
pub mod json;
mod parsers;
mod predicate;
mod primitives;
mod query;
pub mod reader;
mod sections;
mod string;
mod template;
mod xml;

pub type ParseResult<'a, T> = std::result::Result<T, Error>;
pub type ParseFunc<'a, T> = fn(&mut reader::Reader) -> ParseResult<'a, T>;

pub fn parse_hurl_file(s: &str) -> ParseResult<'static, HurlFile> {
    let mut reader = reader::Reader::init(s);
    parsers::hurl_file(&mut reader)
}
