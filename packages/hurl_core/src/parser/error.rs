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
use crate::ast::Pos;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub pos: Pos,
    pub recoverable: bool,
    pub inner: ParseError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    Expecting { value: String },

    Method {},
    Version {},
    Status {},
    Filename {},
    FileContentType {},
    Space {},
    SectionName { name: String },
    JsonpathExpr {},
    XPathExpr {},
    TemplateVariable {},
    Json {},
    Xml {},
    Predicate,
    PredicateValue,
    RegexExpr,

    Unexpected { character: String },
    Eof {},
    Url {},

    DuplicateSection,
    RequestSection,
    ResponseSection,

    HexDigit,
    Unicode,
    EscapeChar,

    InvalidCookieAttribute,
    OddNumberOfHexDigits,
}
