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
//! This module regroups methods on AST nodes to be serialized as Hurl strings and expose
//! Hurl file format identifier that can be used, for instance, as identifier when exporting
//! an Hurl AST to a JSON representation.
use core::fmt;

use crate::ast::{
    BooleanOption, CertificateAttributeName, CookieAttribute, CookieAttributeName, CookiePath,
    CountOption, DurationOption, Expr, ExprKind, FilterValue, Function, Hex, Method,
    MultilineString, MultilineStringAttribute, MultilineStringKind, NaturalOption, Number,
    OptionKind, Placeholder, PredicateFuncValue, QueryValue, Regex, Section, SectionValue, Status,
    StatusValue, Template, TemplateElement, Variable, VariableDefinition, VariableValue, Version,
    VersionValue,
};

impl fmt::Display for BooleanOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BooleanOption::Literal(v) => write!(f, "{}", v),
            BooleanOption::Placeholder(v) => write!(f, "{}", v),
        }
    }
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

impl fmt::Display for CookiePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = self.name.to_string();
        if let Some(attribute) = &self.attribute {
            let s = format!("[{attribute}]");
            buf.push_str(s.as_str());
        }
        write!(f, "{buf}")
    }
}

impl fmt::Display for CookieAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.name {
            CookieAttributeName::MaxAge(_) => "Max-Age",
            CookieAttributeName::Value(_) => "Value",
            CookieAttributeName::Expires(_) => "Expires",
            CookieAttributeName::Domain(_) => "Domain",
            CookieAttributeName::Path(_) => "Path",
            CookieAttributeName::Secure(_) => "Secure",
            CookieAttributeName::HttpOnly(_) => "HttpOnly",
            CookieAttributeName::SameSite(_) => "SameSite",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for CountOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountOption::Literal(v) => write!(f, "{}", v),
            CountOption::Placeholder(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for DurationOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurationOption::Literal(v) => write!(f, "{}", v),
            DurationOption::Placeholder(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprKind::Variable(variable) => write!(f, "{}", variable),
            ExprKind::Function(function) => write!(f, "{}", function),
        }
    }
}

impl FilterValue {
    /// Returns the Hurl identifier for this filter type.
    pub fn identifier(&self) -> &'static str {
        match self {
            FilterValue::Base64Decode => "base64Decode",
            FilterValue::Base64Encode => "base64Encode",
            FilterValue::Count => "count",
            FilterValue::DaysAfterNow => "daysAfterNow",
            FilterValue::DaysBeforeNow => "daysBeforeNow",
            FilterValue::Decode { .. } => "decode",
            FilterValue::Format { .. } => "format",
            FilterValue::HtmlEscape => "htmlEscape",
            FilterValue::HtmlUnescape => "htmlUnescape",
            FilterValue::JsonPath { .. } => "jsonpath",
            FilterValue::Nth { .. } => "nth",
            FilterValue::Regex { .. } => "regex",
            FilterValue::Replace { .. } => "replace",
            FilterValue::Split { .. } => "split",
            FilterValue::ToDate { .. } => "toDate",
            FilterValue::ToFloat => "toFloat",
            FilterValue::ToInt => "toInt",
            FilterValue::UrlDecode => "urlDecode",
            FilterValue::UrlEncode => "urlEncode",
            FilterValue::XPath { .. } => "xpath",
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::NewDate => write!(f, "newDate"),
            Function::NewUuid => write!(f, "newUuid"),
        }
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hex,{}{}{};",
            self.space0.value, self.source, self.space1.value
        )
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for MultilineString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body = match &self.kind {
            MultilineStringKind::Text(text)
            | MultilineStringKind::Json(text)
            | MultilineStringKind::Xml(text) => text.value.to_string(),
            MultilineStringKind::GraphQl(graphql) => {
                let var = match &graphql.variables {
                    None => String::new(),
                    Some(var) => {
                        format!(
                            "variables{}{}{}",
                            var.space.value, var.value, var.whitespace.value
                        )
                    }
                };
                format!("{}{}", graphql.value, var)
            }
        };
        write!(f, "{body}")
    }
}

impl fmt::Display for MultilineStringAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultilineStringAttribute::Escape => write!(f, "escape"),
            MultilineStringAttribute::NoVariable => write!(f, "novariable"),
        }
    }
}

impl fmt::Display for NaturalOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NaturalOption::Literal(v) => write!(f, "{}", v),
            NaturalOption::Placeholder(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Float(value) => write!(f, "{}", value),
            Number::Integer(value) => write!(f, "{}", value),
            Number::BigInteger(value) => write!(f, "{}", value),
        }
    }
}

impl OptionKind {
    /// Returns the Hurl string identifier of this option.
    pub fn identifier(&self) -> &'static str {
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
            OptionKind::Header(_) => "header",
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

    /// Returns the string representation of this option.
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
            OptionKind::Header(value) => value.to_string(),
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

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
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
        }
    }
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
        }
    }
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
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

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for StatusValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusValue::Any => write!(f, "*"),
            StatusValue::Specific(v) => write!(f, "{v}"),
        }
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for element in self.elements.iter() {
            buffer.push_str(element.to_string().as_str());
        }
        write!(f, "{buffer}")
    }
}

impl fmt::Display for TemplateElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            TemplateElement::String { value, .. } => value.clone(),
            TemplateElement::Placeholder(value) => format!("{{{{{value}}}}}"),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for VariableDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VariableValue::Null => "null".to_string(),
            VariableValue::Bool(value) => value.to_string(),
            VariableValue::Number(n) => n.to_string(),
            VariableValue::String(s) => s.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{CookieAttributeName, SourceInfo, Whitespace};
    use crate::reader::Pos;
    use crate::typing::ToSource;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    fn variable_placeholder() -> Placeholder {
        Placeholder {
            space0: whitespace(),
            expr: Expr {
                kind: ExprKind::Variable(Variable {
                    name: "name".to_string(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                }),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: whitespace(),
        }
    }

    fn hello_template() -> Template {
        Template {
            delimiter: None,
            elements: vec![
                TemplateElement::String {
                    value: "Hello ".to_string(),
                    source: "Hello ".to_source(),
                },
                TemplateElement::Placeholder(variable_placeholder()),
                TemplateElement::String {
                    value: "!".to_string(),
                    source: "!".to_source(),
                },
            ],
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    #[test]
    fn test_template() {
        assert_eq!(hello_template().to_string(), "Hello {{name}}!");
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
