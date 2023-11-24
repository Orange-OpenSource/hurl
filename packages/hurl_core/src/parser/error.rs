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
    Version,
    Status,
    Filename,
    FileContentType,
    Space,
    RequestSectionName { name: String },
    ResponseSectionName { name: String },
    JsonPathExpr,
    XPathExpr,
    TemplateVariable,
    Json(JsonErrorVariant),
    Xml,
    Predicate,
    PredicateValue,
    RegexExpr { message: String },

    Eof,
    Url,

    DuplicateSection,
    RequestSection,
    ResponseSection,

    HexDigit,
    Unicode,
    EscapeChar,

    InvalidCookieAttribute,
    OddNumberOfHexDigits,
    UrlIllegalCharacter(char),
    UrlInvalidStart,
    InvalidOption,
    Multiline,
    GraphQlVariables,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonErrorVariant {
    TrailingComma,
    ExpectingElement,
    EmptyElement,
}

impl Error {
    pub fn recoverable(&self) -> Error {
        Error {
            pos: self.pos,
            recoverable: true,
            inner: self.inner.clone(),
        }
    }
    pub fn non_recoverable(&self) -> Error {
        Error {
            pos: self.pos,
            recoverable: false,
            inner: self.inner.clone(),
        }
    }
}
