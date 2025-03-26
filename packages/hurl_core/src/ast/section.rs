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

use crate::ast::option::EntryOption;
use crate::ast::primitive::{
    Base64, File, Hex, KeyValue, LineTerminator, MultilineString, Number, Placeholder, Regex,
    SourceInfo, Template, Whitespace,
};
use crate::ast::Filter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub line_terminator0: LineTerminator,
    pub value: SectionValue,
    pub source_info: SourceInfo,
}

impl Section {
    /// Returns the Hurl string identifier of this section.
    pub fn identifier(&self) -> &'static str {
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

#[allow(clippy::large_enum_variant)]
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
    pub content_type: Option<Template>,
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
    Redirects,
}

impl QueryValue {
    /// Returns the Hurl string identifier of this query type.
    pub fn identifier(&self) -> &'static str {
        match self {
            QueryValue::Status => "status",
            QueryValue::Version => "version",
            QueryValue::Url => "url",
            QueryValue::Header { .. } => "header",
            QueryValue::Cookie { .. } => "cookie",
            QueryValue::Body => "body",
            QueryValue::Xpath { .. } => "xpath",
            QueryValue::Jsonpath { .. } => "jsonpath",
            QueryValue::Regex { .. } => "regex",
            QueryValue::Variable { .. } => "variable",
            QueryValue::Duration => "duration",
            QueryValue::Bytes => "bytes",
            QueryValue::Sha256 => "sha256",
            QueryValue::Md5 => "md5",
            QueryValue::Certificate { .. } => "certificate",
            QueryValue::Ip => "ip",
            QueryValue::Redirects => "redirects",
        }
    }
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

impl fmt::Display for CookiePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = self.name.to_string();
        if let Some(attribute) = &self.attribute {
            let s = format!("[{}]", attribute.identifier());
            buf.push_str(s.as_str());
        }
        write!(f, "{buf}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CookieAttribute {
    pub space0: Whitespace,
    pub name: CookieAttributeName,
    pub space1: Whitespace,
}

impl CookieAttribute {
    fn identifier(&self) -> &'static str {
        match self.name {
            CookieAttributeName::MaxAge(_) => "Max-Age",
            CookieAttributeName::Value(_) => "Value",
            CookieAttributeName::Expires(_) => "Expires",
            CookieAttributeName::Domain(_) => "Domain",
            CookieAttributeName::Path(_) => "Path",
            CookieAttributeName::Secure(_) => "Secure",
            CookieAttributeName::HttpOnly(_) => "HttpOnly",
            CookieAttributeName::SameSite(_) => "SameSite",
        }
    }
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

impl CertificateAttributeName {
    /// Returns the Hurl string identifier of this certificate attribute name.
    pub fn identifier(&self) -> &'static str {
        match self {
            CertificateAttributeName::Subject => "Subject",
            CertificateAttributeName::Issuer => "Issuer",
            CertificateAttributeName::StartDate => "Start-Date",
            CertificateAttributeName::ExpireDate => "Expire-Date",
            CertificateAttributeName::SerialNumber => "Serial-Number",
        }
    }
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
    IsIpv4,
    IsIpv6,
}

impl PredicateFuncValue {
    /// Returns the Hurl string identifier of this predicate.
    pub fn identifier(&self) -> &'static str {
        match self {
            PredicateFuncValue::Equal { .. } => "==",
            PredicateFuncValue::NotEqual { .. } => "!=",
            PredicateFuncValue::GreaterThan { .. } => ">",
            PredicateFuncValue::GreaterThanOrEqual { .. } => ">=",
            PredicateFuncValue::LessThan { .. } => "<",
            PredicateFuncValue::LessThanOrEqual { .. } => "<=",
            PredicateFuncValue::StartWith { .. } => "startsWith",
            PredicateFuncValue::EndWith { .. } => "endsWith",
            PredicateFuncValue::Contain { .. } => "contains",
            PredicateFuncValue::Include { .. } => "includes",
            PredicateFuncValue::Match { .. } => "matches",
            PredicateFuncValue::IsInteger => "isInteger",
            PredicateFuncValue::IsFloat => "isFloat",
            PredicateFuncValue::IsBoolean => "isBoolean",
            PredicateFuncValue::IsString => "isString",
            PredicateFuncValue::IsCollection => "isCollection",
            PredicateFuncValue::IsDate => "isDate",
            PredicateFuncValue::IsIsoDate => "isIsoDate",
            PredicateFuncValue::Exist => "exists",
            PredicateFuncValue::IsEmpty => "isEmpty",
            PredicateFuncValue::IsNumber => "isNumber",
            PredicateFuncValue::IsIpv4 => "isIpv4",
            PredicateFuncValue::IsIpv6 => "isIpv6",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::primitive::{SourceInfo, Template, TemplateElement};
    use crate::reader::Pos;
    use crate::typing::ToSource;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    #[test]
    fn test_cookie_path() {
        assert_eq!(
            CookiePath {
                name: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "LSID".to_string(),
                        source: "unused".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                attribute: Some(CookieAttribute {
                    space0: whitespace(),
                    name: CookieAttributeName::MaxAge("Max-Age".to_string()),
                    space1: whitespace(),
                }),
            }
            .to_string(),
            "LSID[Max-Age]".to_string()
        );
    }
}
