use std::fmt;

/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use crate::ast::json;
use crate::reader::Pos;
use crate::typing::{Count, Duration};

///
/// Hurl AST
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlFile {
    pub entries: Vec<EntryOrDirective>,
    pub line_terminators: Vec<LineTerminator>,
}

impl HurlFile {
    /// Iterates over the available entries
    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries
            .iter()
            .filter_map(|entry_or_directive| match entry_or_directive {
                EntryOrDirective::Entry(e) => Some(e),
                _ => None,
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntryOrDirective {
    Entry(Entry),
    Directive(Directive),
}

impl EntryOrDirective {
    pub fn source_info(&self) -> SourceInfo {
        match self {
            EntryOrDirective::Entry(e) => e.source_info(),
            EntryOrDirective::Directive(d) => d.source_info(),
        }
    }
}

impl From<Entry> for EntryOrDirective {
    fn from(v: Entry) -> Self {
        EntryOrDirective::Entry(v)
    }
}

impl From<Directive> for EntryOrDirective {
    fn from(v: Directive) -> Self {
        EntryOrDirective::Directive(v)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Directive {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub line_terminator0: LineTerminator,
    pub value: DirectiveValue,
    pub source_info: SourceInfo,
}

impl Directive {
    /// Returns the source information for this directive.
    pub fn source_info(&self) -> SourceInfo {
        self.space0.source_info
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectiveValue {
    Include(Include),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Include {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub path: Template,
    pub line_terminator0: LineTerminator,
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
    pub fn querystring_params(&self) -> Vec<KeyValue> {
        for section in &self.sections {
            if let SectionValue::QueryParams(params, _) = &section.value {
                return params.clone();
            }
        }
        vec![]
    }
    pub fn form_params(&self) -> Vec<KeyValue> {
        for section in &self.sections {
            if let SectionValue::FormParams(params, _) = &section.value {
                return params.clone();
            }
        }
        vec![]
    }
    pub fn multipart_form_data(&self) -> Vec<MultipartParam> {
        for section in &self.sections {
            if let SectionValue::MultipartFormData(params, _) = &section.value {
                return params.clone();
            }
        }
        vec![]
    }

    pub fn cookies(&self) -> Vec<Cookie> {
        for section in &self.sections {
            if let SectionValue::Cookies(cookies) = &section.value {
                return cookies.clone();
            }
        }
        vec![]
    }

    pub fn basic_auth(&self) -> Option<KeyValue> {
        for section in &self.sections {
            if let SectionValue::BasicAuth(kv) = &section.value {
                return kv.clone();
            }
        }
        None
    }

    pub fn options(&self) -> Vec<EntryOption> {
        for section in &self.sections {
            if let SectionValue::Options(options) = &section.value {
                return options.clone();
            }
        }
        vec![]
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
    VersionAnyLegacy,
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

impl Section {
    pub fn name(&self) -> &str {
        match self.value {
            SectionValue::Asserts(_) => "Asserts",
            SectionValue::QueryParams(_, true) => "Query",
            SectionValue::QueryParams(_, false) => "QueryStringParams",
            SectionValue::BasicAuth(_) => "BasicAuth",
            SectionValue::FormParams(_, true) => "Form",
            SectionValue::FormParams(_, false) => "FormParams",
            SectionValue::Cookies(_) => "Cookies",
            SectionValue::Captures(_) => "Captures",
            SectionValue::MultipartFormData(_, true) => "Multipart",
            SectionValue::MultipartFormData(_, false) => "MultipartFormData",
            SectionValue::Options(_) => "Options",
        }
    }
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
        operator: bool,
    },
    NotEqual {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    GreaterThan {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    GreaterThanOrEqual {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    LessThan {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    LessThanOrEqual {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
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
    pub encoded: String,
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
    // TODO: explain the difference between value and encoded
    String { value: String, encoded: String },
    Placeholder(Placeholder),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment {
    pub value: String,
    pub source_info: SourceInfo,
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
    pub value: f64,
    pub encoded: String, // as defined in Hurl
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct U64 {
    value: u64,
    encoded: String, // as defined in Hurl
}

impl U64 {
    pub fn new(value: u64, encoded: String) -> U64 {
        U64 { value, encoded }
    }

    pub fn as_u64(&self) -> u64 {
        self.value
    }
}

impl fmt::Display for U64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.encoded)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct I64 {
    value: i64,
    encoded: String, // as defined in Hurl
}

impl I64 {
    pub fn new(value: i64, encoded: String) -> I64 {
        I64 { value, encoded }
    }

    pub fn as_i64(&self) -> i64 {
        self.value
    }
}

impl fmt::Display for I64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.encoded)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.encoded == other.encoded
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
    pub encoded: String,
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

impl OptionKind {
    pub fn name(&self) -> &'static str {
        match self {
            OptionKind::AwsSigV4(_) => "aws-sigv4",
            OptionKind::CaCertificate(_) => "cacert",
            OptionKind::ClientCert(_) => "cert",
            OptionKind::ClientKey(_) => "key",
            OptionKind::Compressed(_) => "compressed",
            OptionKind::ConnectTo(_) => "connect-to",
            OptionKind::ConnectTimeout(_) => "connect-timeout",
            OptionKind::Delay(_) => "delay",
            OptionKind::FollowLocation(_) => "location",
            OptionKind::FollowLocationTrusted(_) => "location-trusted",
            OptionKind::Http10(_) => "http1.0",
            OptionKind::Http11(_) => "http1.1",
            OptionKind::Http2(_) => "http2",
            OptionKind::Http3(_) => "http3",
            OptionKind::Insecure(_) => "insecure",
            OptionKind::IpV4(_) => "ipv4",
            OptionKind::IpV6(_) => "ipv6",
            OptionKind::LimitRate(_) => "limit-rate",
            OptionKind::MaxRedirect(_) => "max-redirs",
            OptionKind::NetRc(_) => "netrc",
            OptionKind::NetRcFile(_) => "netrc-file",
            OptionKind::NetRcOptional(_) => "netrc-optional",
            OptionKind::Output(_) => "output",
            OptionKind::PathAsIs(_) => "path-as-is",
            OptionKind::Proxy(_) => "proxy",
            OptionKind::Repeat(_) => "repeat",
            OptionKind::Resolve(_) => "resolve",
            OptionKind::Retry(_) => "retry",
            OptionKind::RetryInterval(_) => "retry-interval",
            OptionKind::Skip(_) => "skip",
            OptionKind::UnixSocket(_) => "unix-socket",
            OptionKind::User(_) => "user",
            OptionKind::Variable(_) => "variable",
            OptionKind::Verbose(_) => "verbose",
            OptionKind::VeryVerbose(_) => "very-verbose",
        }
    }

    pub fn value_as_str(&self) -> String {
        match self {
            OptionKind::AwsSigV4(value) => value.to_string(),
            OptionKind::CaCertificate(filename) => filename.to_string(),
            OptionKind::ClientCert(filename) => filename.to_string(),
            OptionKind::ClientKey(filename) => filename.to_string(),
            OptionKind::Compressed(value) => value.to_string(),
            OptionKind::ConnectTo(value) => value.to_string(),
            OptionKind::ConnectTimeout(value) => value.to_string(),
            OptionKind::Delay(value) => value.to_string(),
            OptionKind::FollowLocation(value) => value.to_string(),
            OptionKind::FollowLocationTrusted(value) => value.to_string(),
            OptionKind::Http10(value) => value.to_string(),
            OptionKind::Http11(value) => value.to_string(),
            OptionKind::Http2(value) => value.to_string(),
            OptionKind::Http3(value) => value.to_string(),
            OptionKind::Insecure(value) => value.to_string(),
            OptionKind::IpV4(value) => value.to_string(),
            OptionKind::IpV6(value) => value.to_string(),
            OptionKind::LimitRate(value) => value.to_string(),
            OptionKind::MaxRedirect(value) => value.to_string(),
            OptionKind::NetRc(value) => value.to_string(),
            OptionKind::NetRcFile(filename) => filename.to_string(),
            OptionKind::NetRcOptional(value) => value.to_string(),
            OptionKind::Output(filename) => filename.to_string(),
            OptionKind::PathAsIs(value) => value.to_string(),
            OptionKind::Proxy(value) => value.to_string(),
            OptionKind::Repeat(value) => value.to_string(),
            OptionKind::Resolve(value) => value.to_string(),
            OptionKind::Retry(value) => value.to_string(),
            OptionKind::RetryInterval(value) => value.to_string(),
            OptionKind::Skip(value) => value.to_string(),
            OptionKind::UnixSocket(value) => value.to_string(),
            OptionKind::User(value) => value.to_string(),
            OptionKind::Variable(VariableDefinition { name, value, .. }) => {
                format!("{name}={value}")
            }
            OptionKind::Verbose(value) => value.to_string(),
            OptionKind::VeryVerbose(value) => value.to_string(),
        }
    }
}

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
