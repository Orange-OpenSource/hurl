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
use core::fmt;

use super::common::SourceInfo;
use super::json;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlFile {
    pub entries: Vec<Entry>,
    pub line_terminators: Vec<LineTerminator>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub request: Request,
    pub response: Option<Response>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub method: Method,
    pub space1: Whitespace,
    pub url: Template,
    pub line_terminator0: LineTerminator,
    pub headers: Vec<Header>,
    pub sections: Vec<Section>,
    pub body: Option<Body>,
    pub source_info: SourceInfo,
}

impl Request {
    pub fn querystring_params(self) -> Vec<KeyValue> {
        for section in self.sections {
            if let SectionValue::QueryParams(params) = section.value {
                return params;
            }
        }
        return vec![];
    }
    pub fn form_params(self) -> Vec<KeyValue> {
        for section in self.sections {
            if let SectionValue::FormParams(params) = section.value {
                return params;
            }
        }
        return vec![];
    }
    pub fn multipart_form_data(self) -> Vec<MultipartParam> {
        for section in self.sections {
            if let SectionValue::MultipartFormData(params) = section.value {
                return params;
            }
        }
        return vec![];
    }

    pub fn cookies(self) -> Vec<Cookie> {
        for section in self.sections {
            if let SectionValue::Cookies(cookies) = section.value {
                return cookies;
            }
        }
        return vec![];
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub line_terminators: Vec<LineTerminator>,
    pub version: Version,
    pub space0: Whitespace,
    pub status: Status,
    pub space1: Whitespace,
    pub line_terminator0: LineTerminator,
    pub headers: Vec<Header>,
    pub sections: Vec<Section>,
    pub body: Option<Body>,
    pub source_info: SourceInfo,
}

impl Response {
    pub fn captures(self) -> Vec<Capture> {
        for section in self.sections {
            if let SectionValue::Captures(captures) = section.value {
                return captures;
            }
        }
        return vec![];
    }
    pub fn asserts(self) -> Vec<Assert> {
        for section in self.sections {
            if let SectionValue::Asserts(asserts) = section.value {
                return asserts;
            }
        }
        return vec![];
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Method {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH"
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub value: u64,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub value: VersionValue,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionValue {
    Version1,
    Version11,
    Version2,
    VersionAny,
}

impl VersionValue {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            VersionValue::Version1 => "1.0",
            VersionValue::Version11 => "1.1",
            VersionValue::Version2 => "2",
            VersionValue::VersionAny => "*",
        }
    }
}

type Header = KeyValue;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub value: Bytes,
    pub line_terminator0: LineTerminator,
}

//
// Sections
//

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub line_terminator0: LineTerminator,
    pub value: SectionValue,
    pub source_info: SourceInfo,
}

impl Section {
    pub fn name(&self) -> &str {
        match self.value {
            SectionValue::Asserts(_) => "Asserts",
            SectionValue::QueryParams(_) => "QueryStringParams",
            SectionValue::FormParams(_) => "FormParams",
            SectionValue::Cookies(_) => "Cookies",
            SectionValue::Captures(_) => "Captures",
            SectionValue::MultipartFormData(_) => "MultipartFormData"
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SectionValue {
    QueryParams(Vec<KeyValue>),
    FormParams(Vec<KeyValue>),
    MultipartFormData(Vec<MultipartParam>),
    Cookies(Vec<Cookie>),
    Captures(Vec<Capture>),
    Asserts(Vec<Assert>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: EncodedString,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: CookieValue,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CookieValue {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyValue {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub key: EncodedString,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: Template,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultipartParam {
    Param(KeyValue),
    FileParam(FileParam),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileParam {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub key: EncodedString,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: FileValue,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileValue {
    pub space0: Whitespace,
    pub filename: Filename,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub content_type: Option<String>,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capture {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: EncodedString,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub query: Query,
    pub space3: Whitespace,
    pub subquery: Option<Subquery>,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assert {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub query: Query,
    pub space1: Whitespace,
    pub predicate: Predicate,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    pub source_info: SourceInfo,
    pub value: QueryValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum QueryValue {
    Status {},
    Header {
        space0: Whitespace,
        name: Template,
    },
    Cookie {
        space0: Whitespace,
        expr: CookiePath,
    },
    Body {},
    Xpath {
        space0: Whitespace,
        expr: Template,
    },
    Jsonpath {
        space0: Whitespace,
        expr: Template,
    },
    Regex {
        space0: Whitespace,
        expr: Template,
    },
    Variable {
        space0: Whitespace,
        name: Template,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CookiePath {
    pub name: Template,
    pub attribute: Option<CookieAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CookieAttribute {
    pub space0: Whitespace,
    pub name: CookieAttributeName,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CookieAttributeName {
    Value(String),
    Expires(String),
    MaxAge(String),
    Domain(String),
    Path(String),
    Secure(String),
    HttpOnly(String),
    SameSite(String),
}


impl CookieAttributeName {
    pub fn value(&self) -> String {
        match self {
            CookieAttributeName::Value(value) |
            CookieAttributeName::Expires(value) |
            CookieAttributeName::MaxAge(value) |
            CookieAttributeName::Domain(value) |
            CookieAttributeName::Path(value) |
            CookieAttributeName::Secure(value) |
            CookieAttributeName::HttpOnly(value) |
            CookieAttributeName::SameSite(value)
            => value.to_string()
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subquery {
    pub source_info: SourceInfo,
    pub value: SubqueryValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubqueryValue {
    Regex {
        space0: Whitespace,
        expr: Template,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Predicate {
    pub not: bool,
    pub space0: Whitespace,
    pub predicate_func: PredicateFunc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Not {
    pub value: bool,
    pub space0: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PredicateFunc {
    pub source_info: SourceInfo,
    pub value: PredicateFuncValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum PredicateFuncValue {
    EqualString {
        space0: Whitespace,
        value: Template,
    },
    EqualInt {
        space0: Whitespace,
        value: i64,
    },
    EqualFloat {
        space0: Whitespace,
        value: Float,
    },
    EqualBool {
        space0: Whitespace,
        value: bool,
    },
    EqualNull {
        space0: Whitespace
    },
    EqualExpression {
        space0: Whitespace,
        value: Expr
    },
    CountEqual {
        space0: Whitespace,
        value: u64,
    },
    StartWith {
        space0: Whitespace,
        value: Template,
    },
    Contain {
        space0: Whitespace,
        value: Template,
    },
    IncludeString {
        space0: Whitespace,
        value: Template,
    },
    IncludeInt {
        space0: Whitespace,
        value: i64,
    },
    IncludeFloat {
        space0: Whitespace,
        value: Float,
    },
    IncludeBool {
        space0: Whitespace,
        value: bool,
    },
    IncludeNull {
        space0: Whitespace,
    },
    IncludeExpression {
        space0: Whitespace,
        value: Expr,
    },
    Match {
        space0: Whitespace,
        value: Template,
    },
    Exist {},
}

//
// Primitives
//

// Unique template type for your string
// quoted and unquoted

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    pub quotes: bool,
    pub elements: Vec<TemplateElement>,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemplateElement {
    String { value: String, encoded: String },
    Expression(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodedString {
    pub value: String,
    pub encoded: String,
    pub quotes: bool,
    pub source_info: SourceInfo,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Whitespace {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Filename {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Number {
    pub int: i64,
    pub decimal: u64,
}

// keep Number terminology for both Integer and Decimal Numbers
// different representation for the same float value
// 1.01 and 1.010

// TBC: Issue with equality for f64
// represent your float only with integers
// must be easily compared to the core float value
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Float {
    pub int: i64,
    pub decimal: u64,
    // use 18 digits
    pub decimal_digits: usize,   // number of digits
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let decimal_str: String = format!("{:018}", self.decimal).chars().take(self.decimal_digits).collect();
        write!(f, "{}.{}", self.int, decimal_str)
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineTerminator {
    pub space0: Whitespace,
    pub comment: Option<Comment>,
    pub newline: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bytes {
    Json {
        value: json::Value,
    },
    Xml {
        value: String,
    },
    RawString {
        newline0: Whitespace,
        value: Template,
    },
    Base64 {
        space0: Whitespace,
        value: Vec<u8>,
        encoded: String,
        space1: Whitespace,
    },
    File {
        space0: Whitespace,
        filename: Filename,
        space1: Whitespace,
    },
}

//
// template
//
// might include expression
// which can only simple ASCII (even in json value)
// optional delimiter/encoding

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub space0: Whitespace,
    pub variable: Variable,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub source_info: SourceInfo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float() {
        assert_eq!(Float { int: 1, decimal: 0, decimal_digits: 1 }.to_string(), "1.0");
        assert_eq!(Float { int: 1, decimal: 10_000_000_000_000_000, decimal_digits: 2 }.to_string(), "1.01");
        assert_eq!(Float { int: 1, decimal: 10_000_000_000_000_000, decimal_digits: 3 }.to_string(), "1.010");
        assert_eq!(Float { int: -1, decimal: 333_333_333_333_333_333, decimal_digits: 3 }.to_string(), "-1.333");
    }
}
