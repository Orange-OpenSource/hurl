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
    Bytes, KeyValue, LineTerminator, SourceInfo, Template, Whitespace, I64,
};
use crate::ast::section::{
    Assert, Capture, Cookie, MultipartParam, RegexValue, Section, SectionValue,
};
use crate::ast::{BasicAuth, QueryParams};
use crate::typing::{SourceString, ToSource};

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

    /// Returns true if the request or the response uses multilines string attributes
    pub fn use_multiline_string_body_with_attributes(&self) -> bool {
        if let Some(Body {
            value: Bytes::MultilineString(multiline),
            ..
        }) = &self.request.body
        {
            if multiline.has_attributes() {
                return true;
            }
        }
        if let Some(response) = &self.response {
            if let Some(Body {
                value: Bytes::MultilineString(multiline),
                ..
            }) = &response.body
            {
                if multiline.has_attributes() {
                    return true;
                }
            }
        }
        false
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
    pub headers: Vec<KeyValue>,
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
            if let SectionValue::QueryParams(QueryParams { params, .. }) = &section.value {
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
            if let SectionValue::BasicAuth(BasicAuth(kv)) = &section.value {
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
    pub headers: Vec<KeyValue>,
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
pub struct Method(String);

impl Method {
    /// Creates a new AST element method/
    pub fn new(method: &str) -> Method {
        Method(method.to_string())
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToSource for Method {
    fn to_source(&self) -> SourceString {
        self.0.to_source()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub value: VersionValue,
    pub source_info: SourceInfo,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VersionValue {
    Version1,
    Version11,
    Version2,
    Version3,
    VersionAny,
}

impl fmt::Display for VersionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VersionValue::Version1 => "HTTP/1.0",
            VersionValue::Version11 => "HTTP/1.1",
            VersionValue::Version2 => "HTTP/2",
            VersionValue::Version3 => "HTTP/3",
            VersionValue::VersionAny => "HTTP",
        };
        write!(f, "{s}")
    }
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

impl fmt::Display for StatusValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusValue::Any => write!(f, "*"),
            StatusValue::Specific(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub value: Bytes,
    pub line_terminator0: LineTerminator,
}

/// Check that variable name is not reserved
/// (would conflicts with an existing function)
pub fn is_variable_reserved(name: &str) -> bool {
    ["getEnv", "newDate", "newUuid"].contains(&name)
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
    Base64UrlSafeDecode,
    Base64UrlSafeEncode,
    Count,
    DaysAfterNow,
    DaysBeforeNow,
    Decode {
        space0: Whitespace,
        encoding: Template,
    },
    First,
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
    Last,
    Location,
    Nth {
        space0: Whitespace,
        n: I64,
    },
    Regex {
        space0: Whitespace,
        value: RegexValue,
    },
    Replace {
        space0: Whitespace,
        old_value: Template,
        space1: Whitespace,
        new_value: Template,
    },
    ReplaceRegex {
        space0: Whitespace,
        pattern: RegexValue,
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
    ToHex,
    ToInt,
    ToString,
    UrlDecode,
    UrlEncode,
    UrlQueryParam {
        space0: Whitespace,
        param: Template,
    },
    XPath {
        space0: Whitespace,
        expr: Template,
    },
}

impl FilterValue {
    /// Returns the Hurl identifier for this filter type.
    pub fn identifier(&self) -> &'static str {
        match self {
            FilterValue::Base64Decode => "base64Decode",
            FilterValue::Base64Encode => "base64Encode",
            FilterValue::Base64UrlSafeDecode => "base64UrlSafeDecode",
            FilterValue::Base64UrlSafeEncode => "base64UrlSafeEncode",
            FilterValue::Count => "count",
            FilterValue::DaysAfterNow => "daysAfterNow",
            FilterValue::DaysBeforeNow => "daysBeforeNow",
            FilterValue::Decode { .. } => "decode",
            FilterValue::First => "first",
            FilterValue::Format { .. } => "format",
            FilterValue::HtmlEscape => "htmlEscape",
            FilterValue::HtmlUnescape => "htmlUnescape",
            FilterValue::JsonPath { .. } => "jsonpath",
            FilterValue::Last => "last",
            FilterValue::Location => "location",
            FilterValue::Nth { .. } => "nth",
            FilterValue::Regex { .. } => "regex",
            FilterValue::Replace { .. } => "replace",
            FilterValue::ReplaceRegex { .. } => "replaceRegex",
            FilterValue::Split { .. } => "split",
            FilterValue::ToDate { .. } => "toDate",
            FilterValue::ToFloat => "toFloat",
            FilterValue::ToHex => "toHex",
            FilterValue::ToInt => "toInt",
            FilterValue::ToString => "toString",
            FilterValue::UrlDecode => "urlDecode",
            FilterValue::UrlEncode => "urlEncode",
            FilterValue::UrlQueryParam { .. } => "urlQueryParam",
            FilterValue::XPath { .. } => "xpath",
        }
    }
}
