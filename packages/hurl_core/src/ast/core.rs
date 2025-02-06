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
use std::fmt;

use crate::ast::json;
use crate::reader::Pos;
use crate::typing::{Count, Duration, SourceString};

/// Represents Hurl AST root node.
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

impl Entry {
    /// Returns the source information for this entry.
    pub fn source_info(&self) -> SourceInfo {
        self.request.space0.source_info
    }
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
    /// Returns the query strings params for this request.
    ///
    /// See <https://developer.mozilla.org/en-US/docs/Web/API/URL/searchParams>.
    pub fn querystring_params(&self) -> &[KeyValue] {
        for section in &self.sections {
            if let SectionValue::QueryParams(params, _) = &section.value {
                return params;
            }
        }
        &[]
    }

    /// Returns the form params for this request.
    ///
    /// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST#url-encoded_form_submission>.
    pub fn form_params(&self) -> &[KeyValue] {
        for section in &self.sections {
            if let SectionValue::FormParams(params, _) = &section.value {
                return params;
            }
        }
        &[]
    }

    /// Returns the multipart form data for this request.
    ///
    /// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST#multipart_form_submission>.
    pub fn multipart_form_data(&self) -> &[MultipartParam] {
        for section in &self.sections {
            if let SectionValue::MultipartFormData(params, _) = &section.value {
                return params;
            }
        }
        &[]
    }

    /// Returns the list of cookies on this request.
    ///
    /// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cookie>.
    pub fn cookies(&self) -> &[Cookie] {
        for section in &self.sections {
            if let SectionValue::Cookies(cookies) = &section.value {
                return cookies;
            }
        }
        &[]
    }

    /// Returns the basic authentication on this request.
    ///
    /// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication>.
    pub fn basic_auth(&self) -> Option<&KeyValue> {
        for section in &self.sections {
            if let SectionValue::BasicAuth(kv) = &section.value {
                return kv.as_ref();
            }
        }
        None
    }

    /// Returns the options specific for this request.
    pub fn options(&self) -> &[EntryOption] {
        for section in &self.sections {
            if let SectionValue::Options(options) = &section.value {
                return options;
            }
        }
        &[]
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
    /// Returns the captures list of this spec response.
    pub fn captures(&self) -> &[Capture] {
        for section in self.sections.iter() {
            if let SectionValue::Captures(captures) = &section.value {
                return captures;
            }
        }
        &[]
    }

    /// Returns the asserts list of this spec response.
    pub fn asserts(&self) -> &[Assert] {
        for section in self.sections.iter() {
            if let SectionValue::Asserts(asserts) = &section.value {
                return asserts;
            }
        }
        &[]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method(pub String);

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
    Version3,
    VersionAny,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub value: StatusValue,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatusValue {
    Any,
    Specific(u64),
}

pub type Header = KeyValue;

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

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum SectionValue {
    QueryParams(Vec<KeyValue>, bool), // boolean param indicates if we use the short syntax
    BasicAuth(Option<KeyValue>),      // boolean param indicates if we use the short syntax
    FormParams(Vec<KeyValue>, bool),
    MultipartFormData(Vec<MultipartParam>, bool), // boolean param indicates if we use the short syntax
    Cookies(Vec<Cookie>),
    Captures(Vec<Capture>),
    Asserts(Vec<Assert>),
    Options(Vec<EntryOption>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: Template,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: Template,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyValue {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub key: Template,
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
    pub key: Template,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: FileValue,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileValue {
    pub space0: Whitespace,
    pub filename: Template,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub content_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capture {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: Template,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub query: Query,
    pub filters: Vec<(Whitespace, Filter)>,
    pub space3: Whitespace,
    pub redact: bool,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assert {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub query: Query,
    pub filters: Vec<(Whitespace, Filter)>,
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
    Status,
    Version,
    Url,
    Header {
        space0: Whitespace,
        name: Template,
    },
    Cookie {
        space0: Whitespace,
        expr: CookiePath,
    },
    Body,
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
        value: RegexValue,
    },
    Variable {
        space0: Whitespace,
        name: Template,
    },
    Duration,
    Bytes,
    Sha256,
    Md5,
    Certificate {
        space0: Whitespace,
        attribute_name: CertificateAttributeName,
    },
    Ip,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegexValue {
    Template(Template),
    Regex(Regex),
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
            CookieAttributeName::Value(value)
            | CookieAttributeName::Expires(value)
            | CookieAttributeName::MaxAge(value)
            | CookieAttributeName::Domain(value)
            | CookieAttributeName::Path(value)
            | CookieAttributeName::Secure(value)
            | CookieAttributeName::HttpOnly(value)
            | CookieAttributeName::SameSite(value) => value.to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CertificateAttributeName {
    Subject,
    Issuer,
    StartDate,
    ExpireDate,
    SerialNumber,
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
pub enum PredicateValue {
    Base64(Base64),
    Bool(bool),
    File(File),
    Hex(Hex),
    MultilineString(MultilineString),
    Null,
    Number(Number),
    Placeholder(Placeholder),
    Regex(Regex),
    String(Template),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum PredicateFuncValue {
    Equal {
        space0: Whitespace,
        value: PredicateValue,
    },
    NotEqual {
        space0: Whitespace,
        value: PredicateValue,
    },
    GreaterThan {
        space0: Whitespace,
        value: PredicateValue,
    },
    GreaterThanOrEqual {
        space0: Whitespace,
        value: PredicateValue,
    },
    LessThan {
        space0: Whitespace,
        value: PredicateValue,
    },
    LessThanOrEqual {
        space0: Whitespace,
        value: PredicateValue,
    },
    StartWith {
        space0: Whitespace,
        value: PredicateValue,
    },
    EndWith {
        space0: Whitespace,
        value: PredicateValue,
    },
    Contain {
        space0: Whitespace,
        value: PredicateValue,
    },
    Include {
        space0: Whitespace,
        value: PredicateValue,
    },
    Match {
        space0: Whitespace,
        value: PredicateValue,
    },
    IsInteger,
    IsFloat,
    IsBoolean,
    IsString,
    IsCollection,
    IsDate,
    IsIsoDate,
    Exist,
    IsEmpty,
    IsNumber,
}

//
// Primitives
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultilineString {
    pub kind: MultilineStringKind,
    pub attributes: Vec<MultilineStringAttribute>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultilineStringKind {
    Text(Text),
    Json(Text),
    Xml(Text),
    GraphQl(GraphQl),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultilineStringAttribute {
    Escape,
    NoVariable,
}

impl MultilineString {
    pub fn lang(&self) -> &'static str {
        match self.kind {
            MultilineStringKind::Text(_) => "",
            MultilineStringKind::Json(_) => "json",
            MultilineStringKind::Xml(_) => "xml",
            MultilineStringKind::GraphQl(_) => "graphql",
        }
    }

    pub fn value(&self) -> Template {
        match &self.kind {
            MultilineStringKind::Text(text)
            | MultilineStringKind::Json(text)
            | MultilineStringKind::Xml(text) => text.value.clone(),
            MultilineStringKind::GraphQl(text) => text.value.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text {
    pub space: Whitespace,
    pub newline: Whitespace,
    pub value: Template,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphQl {
    pub space: Whitespace,
    pub newline: Whitespace,
    pub value: Template,
    pub variables: Option<GraphQlVariables>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphQlVariables {
    pub space: Whitespace,
    pub value: json::Value,
    pub whitespace: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Base64 {
    pub space0: Whitespace,
    pub value: Vec<u8>,
    pub source: SourceString,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub space0: Whitespace,
    pub filename: Template,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    pub delimiter: Option<char>,
    pub elements: Vec<TemplateElement>,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemplateElement {
    String { value: String, source: SourceString },
    Placeholder(Placeholder),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Whitespace {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Number {
    Float(Float),
    Integer(I64),
    BigInteger(String),
}

// keep Number terminology for both Integer and Decimal Numbers
// different representation for the same float value
// 1.01 and 1.010

#[derive(Clone, Debug)]
pub struct Float {
    value: f64,
    source: SourceString,
}

impl Float {
    pub fn new(value: f64, source: SourceString) -> Float {
        Float { value, source }
    }

    pub fn as_f64(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct U64 {
    value: u64,
    source: SourceString,
}

impl U64 {
    pub fn new(value: u64, source: SourceString) -> U64 {
        U64 { value, source }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }
}

impl fmt::Display for U64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct I64 {
    value: i64,
    source: SourceString,
}

impl I64 {
    pub fn new(value: i64, source: SourceString) -> I64 {
        I64 { value, source }
    }

    pub fn as_i64(&self) -> i64 {
        self.value
    }
}

impl fmt::Display for I64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}
impl Eq for Float {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineTerminator {
    pub space0: Whitespace,
    pub comment: Option<Comment>,
    pub newline: Whitespace,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bytes {
    Json(json::Value),
    Xml(String),
    MultilineString(MultilineString),
    OnelineString(Template),
    Base64(Base64),
    File(File),
    Hex(Hex),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hex {
    pub space0: Whitespace,
    pub value: Vec<u8>,
    pub source: SourceString,
    pub space1: Whitespace,
}

// Literal Regex
#[derive(Clone, Debug)]
pub struct Regex {
    pub inner: regex::Regex,
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.inner.to_string() == other.inner.to_string()
    }
}
impl Eq for Regex {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SourceInfo {
    pub start: Pos,
    pub end: Pos,
}

impl SourceInfo {
    pub fn new(start: Pos, end: Pos) -> SourceInfo {
        SourceInfo { start, end }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Placeholder {
    pub space0: Whitespace,
    pub expr: Expr,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub source_info: SourceInfo,
    pub kind: ExprKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    Variable(Variable),
    Function(Function),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Function {
    NewDate,
    NewUuid,
}

/// Check that variable name is not reserved
/// (would conflicts with an existing function)
pub fn is_variable_reserved(name: &str) -> bool {
    ["getEnv", "newDate", "newUuid"].contains(&name)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryOption {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub kind: OptionKind,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionKind {
    AwsSigV4(Template),
    CaCertificate(Template),
    ClientCert(Template),
    ClientKey(Template),
    Compressed(BooleanOption),
    ConnectTo(Template),
    ConnectTimeout(DurationOption),
    Delay(DurationOption),
    Header(Template),
    Http10(BooleanOption),
    Http11(BooleanOption),
    Http2(BooleanOption),
    Http3(BooleanOption),
    Insecure(BooleanOption),
    IpV4(BooleanOption),
    IpV6(BooleanOption),
    FollowLocation(BooleanOption),
    FollowLocationTrusted(BooleanOption),
    LimitRate(NaturalOption),
    MaxRedirect(CountOption),
    NetRc(BooleanOption),
    NetRcFile(Template),
    NetRcOptional(BooleanOption),
    Output(Template),
    PathAsIs(BooleanOption),
    Proxy(Template),
    Repeat(CountOption),
    Resolve(Template),
    Retry(CountOption),
    RetryInterval(DurationOption),
    Skip(BooleanOption),
    UnixSocket(Template),
    User(Template),
    Variable(VariableDefinition),
    Verbose(BooleanOption),
    VeryVerbose(BooleanOption),
}

impl OptionKind {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanOption {
    Literal(bool),
    Placeholder(Placeholder),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NaturalOption {
    Literal(U64),
    Placeholder(Placeholder),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CountOption {
    Literal(Count),
    Placeholder(Placeholder),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurationOption {
    Literal(Duration),
    Placeholder(Placeholder),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariableDefinition {
    pub source_info: SourceInfo,
    pub name: String,
    pub space0: Whitespace,
    pub space1: Whitespace,
    pub value: VariableValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariableValue {
    Null,
    Bool(bool),
    Number(Number),
    String(Template),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Filter {
    pub source_info: SourceInfo,
    pub value: FilterValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FilterValue {
    Base64Decode,
    Base64Encode,
    Count,
    DaysAfterNow,
    DaysBeforeNow,
    Decode {
        space0: Whitespace,
        encoding: Template,
    },
    Format {
        space0: Whitespace,
        fmt: Template,
    },
    HtmlEscape,
    HtmlUnescape,
    JsonPath {
        space0: Whitespace,
        expr: Template,
    },
    Nth {
        space0: Whitespace,
        n: U64,
    },
    Regex {
        space0: Whitespace,
        value: RegexValue,
    },
    Replace {
        space0: Whitespace,
        old_value: RegexValue,
        space1: Whitespace,
        new_value: Template,
    },
    Split {
        space0: Whitespace,
        sep: Template,
    },
    ToDate {
        space0: Whitespace,
        fmt: Template,
    },
    ToFloat,
    ToInt,
    UrlDecode,
    UrlEncode,
    XPath {
        space0: Whitespace,
        expr: Template,
    },
}

#[cfg(test)]
mod tests {
    use crate::ast::Float;
    use crate::typing::ToSource;

    #[test]
    fn test_float() {
        assert_eq!(
            Float {
                value: 1.0,
                source: "1.0".to_source()
            }
            .to_string(),
            "1.0"
        );
        assert_eq!(
            Float {
                value: 1.01,
                source: "1.01".to_source()
            }
            .to_string(),
            "1.01"
        );
        assert_eq!(
            Float {
                value: 1.01,
                source: "1.010".to_source()
            }
            .to_string(),
            "1.010"
        );
        assert_eq!(
            Float {
                value: -1.333,
                source: "-1.333".to_source()
            }
            .to_string(),
            "-1.333"
        );
    }
}
