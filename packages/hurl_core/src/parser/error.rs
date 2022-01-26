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

    Method { name: String },
    Version {},
    Status {},
    Filename {},
    FileContentType {},
    Space {},
    RequestSectionName { name: String },
    ResponseSectionName { name: String },
    JsonpathExpr {},
    XPathExpr {},
    TemplateVariable {},
    Json {},
    Xml {},
    Predicate,
    PredicateValue,
    RegexExpr { message: String },

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
    UrlIllegalCharacter(char),
}

impl Error {
    pub fn recoverable(&self) -> Error {
        Error {
            pos: self.pos.clone(),
            recoverable: true,
            inner: self.inner.clone(),
        }
    }
    pub fn non_recoverable(&self) -> Error {
        Error {
            pos: self.pos.clone(),
            recoverable: false,
            inner: self.inner.clone(),
        }
    }
}
