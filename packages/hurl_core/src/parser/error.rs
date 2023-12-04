/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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

/// Represents a parser error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub pos: Pos,
    pub recoverable: bool,
    pub inner: ParseError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    DuplicateSection,
    EscapeChar,
    Expecting { value: String },
    FileContentType,
    Filename,
    GraphQlVariables,
    HexDigit,
    InvalidCookieAttribute,
    InvalidOption,
    Json(JsonErrorVariant),
    JsonPathExpr,
    Method { name: String },
    Multiline,
    OddNumberOfHexDigits,
    Predicate,
    PredicateValue,
    RegexExpr { message: String },
    RequestSection,
    RequestSectionName { name: String },
    ResponseSection,
    ResponseSectionName { name: String },
    Space,
    Status,
    TemplateVariable,
    Unicode,
    Url,
    UrlIllegalCharacter(char),
    UrlInvalidStart,
    Version,
    XPathExpr,
    Xml,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonErrorVariant {
    TrailingComma,
    ExpectingElement,
    EmptyElement,
}

impl Error {
    /// Creates a new error for the position `pos`, of type `inner`.
    pub fn new(pos: Pos, recoverable: bool, inner: ParseError) -> Error {
        Error {
            pos,
            recoverable,
            inner,
        }
    }

    /// Makes a recoverable error.
    pub fn recoverable(&self) -> Error {
        Error {
            pos: self.pos,
            recoverable: true,
            inner: self.inner.clone(),
        }
    }

    /// Makes a non recoverable error.
    pub fn non_recoverable(&self) -> Error {
        Error {
            pos: self.pos,
            recoverable: false,
            inner: self.inner.clone(),
        }
    }
}
