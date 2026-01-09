/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
use hurl_core::ast::{
    Assert, Base64, Body, BooleanOption, Bytes, Capture, CertificateAttributeName, Comment, Cookie,
    CookiePath, CountOption, DurationOption, Entry, EntryOption, File, FilenameParam,
    FilenameValue, FilterValue, Hex, HurlFile, IntegerValue, JsonValue, KeyValue, LineTerminator,
    Method, MultilineString, MultipartParam, NaturalOption, Number, OptionKind, Placeholder,
    Predicate, PredicateFuncValue, PredicateValue, Query, QueryValue, Regex, RegexValue, Request,
    Response, Section, SectionValue, StatusValue, Template, VariableDefinition, VariableValue,
    VerbosityOption, VersionValue, I64, U64,
};
use hurl_core::types::{Count, Duration, DurationUnit, ToSource};

/// Lint a parsed `HurlFile` to a string.
pub fn lint_hurl_file(file: &HurlFile) -> String {
    file.lint()
}

/// Lint something (usually a Hurl AST node) to a string.
trait Lint {
    fn lint(&self) -> String;
}

impl Lint for Assert {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.query.lint());
        if !self.filters.is_empty() {
            s.push(' ');
            let filters = self
                .filters
                .iter()
                .map(|(_, f)| f.value.lint())
                .collect::<Vec<_>>()
                .join(" ");
            s.push_str(&filters);
        }
        s.push(' ');
        s.push_str(&self.predicate.lint());
        s.push_str(&lint_lt(&self.line_terminator0, true));
        s
    }
}

impl Lint for Base64 {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str("base64,");
        s.push_str(self.source.as_str());
        s.push(';');
        s
    }
}

impl Lint for Body {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.value.lint());
        s.push_str(&lint_lt(&self.line_terminator0, true));
        s
    }
}

impl Lint for BooleanOption {
    fn lint(&self) -> String {
        match self {
            BooleanOption::Literal(value) => value.to_string(),
            BooleanOption::Placeholder(value) => value.lint(),
        }
    }
}

impl Lint for Bytes {
    fn lint(&self) -> String {
        match self {
            Bytes::Json(value) => value.lint(),
            Bytes::Xml(value) => value.clone(),
            Bytes::MultilineString(value) => value.lint(),
            Bytes::OnelineString(value) => value.lint(),
            Bytes::Base64(value) => value.lint(),
            Bytes::File(value) => value.lint(),
            Bytes::Hex(value) => value.lint(),
        }
    }
}

impl Lint for Capture {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.name.lint());
        s.push(':');
        s.push(' ');
        s.push_str(&self.query.lint());
        if !self.filters.is_empty() {
            s.push(' ');
            let filters = self
                .filters
                .iter()
                .map(|(_, f)| f.value.lint())
                .collect::<Vec<_>>()
                .join(" ");
            s.push_str(&filters);
        }
        if self.redacted {
            s.push(' ');
            s.push_str("redact");
        }
        s.push_str(&lint_lt(&self.line_terminator0, true));
        s
    }
}

impl Lint for CertificateAttributeName {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for Cookie {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.name.lint());
        s.push(':');
        s.push(' ');
        s.push_str(&self.value.lint());
        s.push_str(&lint_lt(&self.line_terminator0, true));
        s
    }
}

impl Lint for CookiePath {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for Comment {
    fn lint(&self) -> String {
        format!("#{}", self.value.trim_end())
    }
}

impl Lint for Count {
    fn lint(&self) -> String {
        self.to_string()
    }
}

impl Lint for CountOption {
    fn lint(&self) -> String {
        match self {
            CountOption::Literal(value) => value.lint(),
            CountOption::Placeholder(value) => value.lint(),
        }
    }
}

impl Lint for Entry {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.request.lint());
        if let Some(response) = &self.response {
            s.push_str(&response.lint());
        }
        s
    }
}

impl Lint for EntryOption {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.kind.lint());
        s.push_str(&lint_lt(&self.line_terminator0, true));
        s
    }
}

impl Lint for File {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str("file,");
        s.push_str(&self.filename.lint());
        s.push(';');
        s
    }
}

impl Lint for FilenameParam {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.key.lint());
        s.push(':');
        s.push(' ');
        s.push_str(&self.value.lint());
        s.push_str(&lint_lt(&self.line_terminator0, true));
        s
    }
}

impl Lint for FilenameValue {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str("file,");
        s.push_str(&self.filename.lint());
        s.push(';');
        if let Some(content_type) = &self.content_type {
            s.push(' ');
            s.push_str(&content_type.lint());
        }
        s
    }
}

impl Lint for FilterValue {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str(self.identifier());
        match self {
            FilterValue::Decode { encoding, .. } => {
                s.push(' ');
                s.push_str(&encoding.lint());
            }
            FilterValue::Format { fmt, .. } => {
                s.push(' ');
                s.push_str(&fmt.lint());
            }
            FilterValue::DateFormat { fmt, .. } => {
                s.push(' ');
                s.push_str(&fmt.lint());
            }
            FilterValue::JsonPath { expr, .. } => {
                s.push(' ');
                s.push_str(&expr.lint());
            }
            FilterValue::Nth { n, .. } => {
                s.push(' ');
                s.push_str(&n.lint());
            }
            FilterValue::Regex { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            FilterValue::Replace {
                old_value,
                new_value,
                ..
            } => {
                s.push(' ');
                s.push_str(&old_value.lint());
                s.push(' ');
                s.push_str(&new_value.lint());
            }
            FilterValue::Split { sep, .. } => {
                s.push(' ');
                s.push_str(&sep.lint());
            }
            FilterValue::ReplaceRegex {
                pattern, new_value, ..
            } => {
                s.push(' ');
                s.push_str(&pattern.lint());
                s.push(' ');
                s.push_str(&new_value.lint());
            }
            FilterValue::ToDate { fmt, .. } => {
                s.push(' ');
                s.push_str(&fmt.lint());
            }
            FilterValue::UrlQueryParam { param, .. } => {
                s.push(' ');
                s.push_str(&param.lint());
            }
            FilterValue::XPath { expr, .. } => {
                s.push(' ');
                s.push_str(&expr.lint());
            }
            FilterValue::Base64Decode
            | FilterValue::Base64Encode
            | FilterValue::Base64UrlSafeDecode
            | FilterValue::Base64UrlSafeEncode
            | FilterValue::Count
            | FilterValue::DaysAfterNow
            | FilterValue::DaysBeforeNow
            | FilterValue::First
            | FilterValue::HtmlEscape
            | FilterValue::HtmlUnescape
            | FilterValue::Last
            | FilterValue::Location
            | FilterValue::ToFloat
            | FilterValue::ToHex
            | FilterValue::ToInt
            | FilterValue::ToString
            | FilterValue::UrlDecode
            | FilterValue::UrlEncode
            | FilterValue::Utf8Decode
            | FilterValue::Utf8Encode => {}
        }
        s
    }
}

impl Lint for Hex {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str("hex,");
        s.push_str(self.source.as_str());
        s.push(';');
        s
    }
}

impl Lint for HurlFile {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.entries.iter().for_each(|e| s.push_str(&e.lint()));
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s
    }
}

impl Lint for IntegerValue {
    fn lint(&self) -> String {
        match self {
            IntegerValue::Literal(value) => value.lint(),
            IntegerValue::Placeholder(value) => value.lint(),
        }
    }
}

impl Lint for I64 {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for JsonValue {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for KeyValue {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.key.lint());
        s.push(':');
        if !self.value.is_empty() {
            s.push(' ');
            s.push_str(&self.value.lint());
        }
        s.push_str(&lint_lt(&self.line_terminator0, true));
        s
    }
}

fn lint_lt(lt: &LineTerminator, is_trailing: bool) -> String {
    let mut s = String::new();
    if let Some(comment) = &lt.comment {
        if is_trailing {
            // if line terminator is a trailing terminator, we keep the leading whitespaces
            // to keep user alignment.
            s.push_str(lt.space0.as_str());
        }
        s.push_str(&comment.lint());
    };
    // We always terminate a file by a newline
    if lt.newline.value.is_empty() {
        s.push('\n');
    } else {
        s.push_str(&lt.newline.value);
    }
    s
}

impl Lint for Method {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for MultipartParam {
    fn lint(&self) -> String {
        let mut s = String::new();
        match self {
            MultipartParam::Param(param) => s.push_str(&param.lint()),
            MultipartParam::FilenameParam(param) => s.push_str(&param.lint()),
        }
        s
    }
}

impl Lint for MultilineString {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for NaturalOption {
    fn lint(&self) -> String {
        match self {
            NaturalOption::Literal(value) => value.lint(),
            NaturalOption::Placeholder(value) => value.lint(),
        }
    }
}

impl Lint for Number {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for OptionKind {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str(self.identifier());
        s.push(':');
        s.push(' ');
        let value = match self {
            OptionKind::AwsSigV4(value) => value.lint(),
            OptionKind::CaCertificate(value) => value.lint(),
            OptionKind::ClientCert(value) => value.lint(),
            OptionKind::ClientKey(value) => value.lint(),
            OptionKind::Compressed(value) => value.lint(),
            OptionKind::ConnectTo(value) => value.lint(),
            OptionKind::ConnectTimeout(value) => {
                lint_duration_option(value, DurationUnit::MilliSecond)
            }
            OptionKind::Delay(value) => lint_duration_option(value, DurationUnit::MilliSecond),
            OptionKind::Digest(value) => value.lint(),
            OptionKind::Header(value) => value.lint(),
            OptionKind::Http10(value) => value.lint(),
            OptionKind::Http11(value) => value.lint(),
            OptionKind::Http2(value) => value.lint(),
            OptionKind::Http3(value) => value.lint(),
            OptionKind::Insecure(value) => value.lint(),
            OptionKind::IpV4(value) => value.lint(),
            OptionKind::IpV6(value) => value.lint(),
            OptionKind::FollowLocation(value) => value.lint(),
            OptionKind::FollowLocationTrusted(value) => value.lint(),
            OptionKind::LimitRate(value) => value.lint(),
            OptionKind::MaxRedirect(value) => value.lint(),
            OptionKind::MaxTime(value) => lint_duration_option(value, DurationUnit::MilliSecond),
            OptionKind::Negotiate(value) => value.lint(),
            OptionKind::NetRc(value) => value.lint(),
            OptionKind::NetRcFile(value) => value.lint(),
            OptionKind::NetRcOptional(value) => value.lint(),
            OptionKind::Ntlm(value) => value.lint(),
            OptionKind::Output(value) => value.lint(),
            OptionKind::PathAsIs(value) => value.lint(),
            OptionKind::PinnedPublicKey(value) => value.lint(),
            OptionKind::Proxy(value) => value.lint(),
            OptionKind::Repeat(value) => value.lint(),
            OptionKind::Resolve(value) => value.lint(),
            OptionKind::Retry(value) => value.lint(),
            OptionKind::RetryInterval(value) => {
                lint_duration_option(value, DurationUnit::MilliSecond)
            }
            OptionKind::Skip(value) => value.lint(),
            OptionKind::UnixSocket(value) => value.lint(),
            OptionKind::User(value) => value.lint(),
            OptionKind::Variable(value) => value.lint(),
            OptionKind::Verbose(value) => value.lint(),
            OptionKind::Verbosity(value) => value.lint(),
            OptionKind::VeryVerbose(value) => value.lint(),
        };
        s.push_str(&value);
        s
    }
}

impl Lint for Query {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str(self.value.identifier());
        match &self.value {
            QueryValue::Status => {}
            QueryValue::Version => {}
            QueryValue::Url => {}
            QueryValue::Header { name, .. } => {
                s.push(' ');
                s.push_str(&name.lint());
            }
            QueryValue::Cookie { expr, .. } => {
                s.push(' ');
                s.push_str(&expr.lint());
            }
            QueryValue::Body => {}
            QueryValue::Xpath { expr, .. } => {
                s.push(' ');
                s.push_str(&expr.lint());
            }
            QueryValue::Jsonpath { expr, .. } => {
                s.push(' ');
                s.push_str(&expr.lint());
            }
            QueryValue::Regex { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            QueryValue::Variable { name, .. } => {
                s.push(' ');
                s.push_str(&name.lint());
            }
            QueryValue::Duration => {}
            QueryValue::Bytes => {}
            QueryValue::RawBytes => {}
            QueryValue::Sha256 => {}
            QueryValue::Md5 => {}
            QueryValue::Certificate { attribute_name, .. } => {
                s.push(' ');
                s.push_str(&attribute_name.lint());
            }
            QueryValue::Ip => {}
            QueryValue::Redirects => {}
        }
        s
    }
}

impl Lint for Placeholder {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for Predicate {
    fn lint(&self) -> String {
        let mut s = String::new();
        if self.not {
            s.push_str("not");
            s.push(' ');
        }
        s.push_str(&self.predicate_func.value.lint());
        s
    }
}

impl Lint for PredicateFuncValue {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str(self.identifier());
        match self {
            PredicateFuncValue::Equal { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::NotEqual { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::GreaterThan { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::GreaterThanOrEqual { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::LessThan { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::LessThanOrEqual { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::StartWith { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::EndWith { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::Contain { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::Include { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::Match { value, .. } => {
                s.push(' ');
                s.push_str(&value.lint());
            }
            PredicateFuncValue::Exist
            | PredicateFuncValue::IsBoolean
            | PredicateFuncValue::IsCollection
            | PredicateFuncValue::IsDate
            | PredicateFuncValue::IsEmpty
            | PredicateFuncValue::IsFloat
            | PredicateFuncValue::IsInteger
            | PredicateFuncValue::IsIpv4
            | PredicateFuncValue::IsIpv6
            | PredicateFuncValue::IsIsoDate
            | PredicateFuncValue::IsList
            | PredicateFuncValue::IsNumber
            | PredicateFuncValue::IsObject
            | PredicateFuncValue::IsString
            | PredicateFuncValue::IsUuid => {}
        }
        s
    }
}

impl Lint for PredicateValue {
    fn lint(&self) -> String {
        match self {
            PredicateValue::Base64(value) => value.lint(),
            PredicateValue::Bool(value) => value.to_string(),
            PredicateValue::File(value) => value.lint(),
            PredicateValue::Hex(value) => value.lint(),
            PredicateValue::MultilineString(value) => value.lint(),
            PredicateValue::Null => "null".to_string(),
            PredicateValue::Number(value) => value.lint(),
            PredicateValue::Placeholder(value) => value.lint(),
            PredicateValue::Regex(value) => value.lint(),
            PredicateValue::String(value) => value.lint(),
        }
    }
}

impl Lint for Regex {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for RegexValue {
    fn lint(&self) -> String {
        match self {
            RegexValue::Template(value) => value.lint(),
            RegexValue::Regex(value) => value.lint(),
        }
    }
}

impl Lint for Request {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.method.lint());
        s.push(' ');
        s.push_str(&self.url.lint());
        s.push_str(&lint_lt(&self.line_terminator0, true));

        self.headers.iter().for_each(|h| s.push_str(&h.lint()));

        // We rewrite our file and reorder the various section.
        if let Some(section) = get_option_section(self) {
            s.push_str(&section.lint());
        }
        if let Some(section) = get_query_params_section(self) {
            s.push_str(&section.lint());
        }
        if let Some(section) = get_basic_auth_section(self) {
            s.push_str(&section.lint());
        }
        if let Some(section) = get_form_params_section(self) {
            s.push_str(&section.lint());
        }
        if let Some(section) = get_multipart_section(self) {
            s.push_str(&section.lint());
        }
        if let Some(section) = get_cookies_section(self) {
            s.push_str(&section.lint());
        }
        if let Some(body) = &self.body {
            s.push_str(&body.lint());
        }
        s
    }
}

impl Lint for Response {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push_str(&self.version.value.lint());
        s.push(' ');
        s.push_str(&self.status.value.lint());
        s.push_str(&lint_lt(&self.line_terminator0, true));

        self.headers.iter().for_each(|h| s.push_str(&h.lint()));

        if let Some(section) = get_captures_section(self) {
            s.push_str(&section.lint());
        }
        if let Some(section) = get_asserts_section(self) {
            s.push_str(&section.lint());
        }
        if let Some(body) = &self.body {
            s.push_str(&body.lint());
        }
        s
    }
}

impl Lint for Section {
    fn lint(&self) -> String {
        let mut s = String::new();
        self.line_terminators
            .iter()
            .for_each(|lt| s.push_str(&lint_lt(lt, false)));
        s.push('[');
        s.push_str(self.identifier());
        s.push(']');
        s.push_str(&lint_lt(&self.line_terminator0, true));
        s.push_str(&self.value.lint());
        s
    }
}

impl Lint for SectionValue {
    fn lint(&self) -> String {
        let mut s = String::new();
        match self {
            SectionValue::QueryParams(params, _) => {
                params.iter().for_each(|p| s.push_str(&p.lint()));
            }
            SectionValue::BasicAuth(Some(auth)) => {
                s.push_str(&auth.lint());
            }
            SectionValue::BasicAuth(_) => {}
            SectionValue::FormParams(params, _) => {
                params.iter().for_each(|p| s.push_str(&p.lint()));
            }
            SectionValue::MultipartFormData(params, _) => {
                params.iter().for_each(|p| s.push_str(&p.lint()));
            }
            SectionValue::Cookies(cookies) => {
                cookies.iter().for_each(|c| s.push_str(&c.lint()));
            }
            SectionValue::Captures(captures) => {
                captures.iter().for_each(|c| s.push_str(&c.lint()));
            }
            SectionValue::Asserts(asserts) => {
                asserts.iter().for_each(|a| s.push_str(&a.lint()));
            }
            SectionValue::Options(options) => {
                options.iter().for_each(|o| s.push_str(&o.lint()));
            }
        }
        s
    }
}

impl Lint for StatusValue {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for Template {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for U64 {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for VariableDefinition {
    fn lint(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.name);
        s.push('=');
        s.push_str(&self.value.lint());
        s
    }
}

impl Lint for VariableValue {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for VersionValue {
    fn lint(&self) -> String {
        self.to_source().to_string()
    }
}

impl Lint for VerbosityOption {
    fn lint(&self) -> String {
        self.to_string()
    }
}

fn get_asserts_section(response: &Response) -> Option<&Section> {
    for s in &response.sections {
        if let SectionValue::Asserts(_) = s.value {
            return Some(s);
        }
    }
    None
}

fn get_captures_section(response: &Response) -> Option<&Section> {
    for s in &response.sections {
        if let SectionValue::Captures(_) = s.value {
            return Some(s);
        }
    }
    None
}

fn get_cookies_section(request: &Request) -> Option<&Section> {
    for s in &request.sections {
        if let SectionValue::Cookies(_) = s.value {
            return Some(s);
        }
    }
    None
}

fn get_form_params_section(request: &Request) -> Option<&Section> {
    for s in &request.sections {
        if let SectionValue::FormParams(_, _) = s.value {
            return Some(s);
        }
    }
    None
}

fn get_option_section(request: &Request) -> Option<&Section> {
    for s in &request.sections {
        if let SectionValue::Options(_) = s.value {
            return Some(s);
        }
    }
    None
}

fn get_multipart_section(request: &Request) -> Option<&Section> {
    for s in &request.sections {
        if let SectionValue::MultipartFormData(_, _) = s.value {
            return Some(s);
        }
    }
    None
}

fn get_query_params_section(request: &Request) -> Option<&Section> {
    for s in &request.sections {
        if let SectionValue::QueryParams(_, _) = s.value {
            return Some(s);
        }
    }
    None
}

fn get_basic_auth_section(request: &Request) -> Option<&Section> {
    for s in &request.sections {
        if let SectionValue::BasicAuth(Some(_)) = s.value {
            return Some(s);
        }
    }
    None
}

fn lint_duration_option(option: &DurationOption, default_unit: DurationUnit) -> String {
    match option {
        DurationOption::Literal(duration) => lint_duration(duration, default_unit),
        DurationOption::Placeholder(expr) => expr.lint(),
    }
}

fn lint_duration(duration: &Duration, default_unit: DurationUnit) -> String {
    let mut s = String::new();
    s.push_str(&duration.value.lint());
    let unit = duration.unit.unwrap_or(default_unit);
    s.push_str(&unit.to_string());
    s
}

#[cfg(test)]
mod tests {
    use crate::linter::lint_hurl_file;
    use hurl_core::parser;

    #[test]
    fn test_lint_hurl_file() {
        let src = r#"
    # comment 1
  #comment 2 with trailing spaces    
  GET   https://foo.com
[Form]
  bar : baz
  [Options]
 location : true
HTTP   200"#;
        let file = parser::parse_hurl_file(src).unwrap();
        let linted = lint_hurl_file(&file);
        assert_eq!(
            linted,
            r#"
# comment 1
#comment 2 with trailing spaces
GET https://foo.com
[Options]
location: true
[Form]
bar: baz
HTTP 200
"#
        );
    }
}
