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
use base64::engine::general_purpose;
use base64::Engine;
use hurl_core::ast::{
    Assert, Base64, Body, BooleanOption, Bytes, Capture, CertificateAttributeName, Comment, Cookie,
    CountOption, DurationOption, Entry, EntryOption, File, FileParam, Filter, FilterValue, Header,
    Hex, HurlFile, JsonListElement, JsonValue, KeyValue, MultilineString, MultilineStringKind,
    MultipartParam, NaturalOption, OptionKind, Placeholder, Predicate, PredicateFuncValue,
    PredicateValue, Query, QueryValue, Regex, RegexValue, Request, Response, StatusValue,
    VersionValue,
};
use hurl_core::typing::{Count, Duration};

use crate::format::serialize_json::JValue;

pub fn format(hurl_file: &HurlFile) -> String {
    hurl_file.to_json().format()
}

pub trait ToJson {
    fn to_json(&self) -> JValue;
}

impl ToJson for HurlFile {
    fn to_json(&self) -> JValue {
        JValue::Object(vec![(
            "entries".to_string(),
            JValue::List(self.entries.iter().map(|e| e.to_json()).collect()),
        )])
    }
}

impl ToJson for Entry {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![("request".to_string(), self.request.to_json())];
        if let Some(response) = &self.response {
            attributes.push(("response".to_string(), response.to_json()));
        }
        JValue::Object(attributes)
    }
}

impl ToJson for Request {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![
            (
                "method".to_string(),
                JValue::String(self.method.to_string()),
            ),
            ("url".to_string(), JValue::String(self.url.to_string())),
        ];
        add_headers(&mut attributes, &self.headers);

        if !self.querystring_params().is_empty() {
            let params = self
                .querystring_params()
                .iter()
                .map(|p| p.to_json())
                .collect();
            attributes.push(("query_string_params".to_string(), JValue::List(params)));
        }
        if !self.form_params().is_empty() {
            let params = self.form_params().iter().map(|p| p.to_json()).collect();
            attributes.push(("form_params".to_string(), JValue::List(params)));
        }
        if !self.multipart_form_data().is_empty() {
            let params = self
                .multipart_form_data()
                .iter()
                .map(|p| p.to_json())
                .collect();
            attributes.push(("multipart_form_data".to_string(), JValue::List(params)));
        }
        if !self.cookies().is_empty() {
            let cookies = self.cookies().iter().map(|c| c.to_json()).collect();
            attributes.push(("cookies".to_string(), JValue::List(cookies)));
        }
        if !self.options().is_empty() {
            let options = self.options().iter().map(|c| c.to_json()).collect();
            attributes.push(("options".to_string(), JValue::List(options)));
        }
        if let Some(body) = &self.body {
            attributes.push(("body".to_string(), body.to_json()));
        }

        // Request comments (can be used to check custom commands)
        let comments: Vec<_> = self
            .line_terminators
            .iter()
            .filter_map(|l| l.comment.as_ref())
            .collect();
        if !comments.is_empty() {
            let comments = comments.iter().map(|c| c.to_json()).collect();
            attributes.push(("comments".to_string(), JValue::List(comments)));
        }

        JValue::Object(attributes)
    }
}

impl ToJson for Response {
    /// Transforms this response to a JSON object.
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        if let Some(v) = get_json_version(&self.version.value) {
            attributes.push(("version".to_string(), JValue::String(v)));
        }
        if let StatusValue::Specific(n) = self.status.value {
            attributes.push(("status".to_string(), JValue::Number(n.to_string())));
        }
        add_headers(&mut attributes, &self.headers);
        if !self.captures().is_empty() {
            let captures = self.captures().iter().map(|c| c.to_json()).collect();
            attributes.push(("captures".to_string(), JValue::List(captures)));
        }
        if !self.asserts().is_empty() {
            let asserts = self.asserts().iter().map(|a| a.to_json()).collect();
            attributes.push(("asserts".to_string(), JValue::List(asserts)));
        }
        if let Some(body) = &self.body {
            attributes.push(("body".to_string(), body.to_json()));
        }
        JValue::Object(attributes)
    }
}

fn add_headers(attributes: &mut Vec<(String, JValue)>, headers: &[Header]) {
    if !headers.is_empty() {
        let headers = JValue::List(headers.iter().map(|h| h.to_json()).collect());
        attributes.push(("headers".to_string(), headers));
    }
}

impl ToJson for Body {
    fn to_json(&self) -> JValue {
        self.value.to_json()
    }
}

impl ToJson for Bytes {
    fn to_json(&self) -> JValue {
        match self {
            Bytes::Base64(value) => value.to_json(),
            Bytes::Hex(value) => value.to_json(),
            Bytes::File(value) => value.to_json(),
            Bytes::Json(value) => JValue::Object(vec![
                ("type".to_string(), JValue::String("json".to_string())),
                ("value".to_string(), value.to_json()),
            ]),
            Bytes::Xml(value) => JValue::Object(vec![
                ("type".to_string(), JValue::String("xml".to_string())),
                ("value".to_string(), JValue::String(value.clone())),
            ]),
            Bytes::OnelineString(value) => JValue::Object(vec![
                ("type".to_string(), JValue::String("text".to_string())),
                ("value".to_string(), JValue::String(value.to_string())),
            ]),
            Bytes::MultilineString(multi) => {
                // TODO: check these values. Maybe we want to have the same
                // export when using:
                //
                // ~~~
                // GET https://foo.com
                // ```base64
                // SGVsbG8gd29ybGQ=
                // ```
                //
                // or
                //
                // ~~~
                // GET https://foo.com
                // base64,SGVsbG8gd29ybGQ=;
                // ~~~
                let lang = match multi {
                    MultilineString {
                        kind: MultilineStringKind::Text(_),
                        ..
                    } => "text",
                    MultilineString {
                        kind: MultilineStringKind::Json(_),
                        ..
                    } => "json",
                    MultilineString {
                        kind: MultilineStringKind::Xml(_),
                        ..
                    } => "xml",
                    MultilineString {
                        kind: MultilineStringKind::GraphQl(_),
                        ..
                    } => "graphql",
                };
                JValue::Object(vec![
                    ("type".to_string(), JValue::String(lang.to_string())),
                    ("value".to_string(), JValue::String(multi.to_string())),
                ])
            }
        }
    }
}

impl ToJson for Base64 {
    fn to_json(&self) -> JValue {
        let value = general_purpose::STANDARD.encode(&self.value);
        JValue::Object(vec![
            ("encoding".to_string(), JValue::String("base64".to_string())),
            ("value".to_string(), JValue::String(value)),
        ])
    }
}

impl ToJson for Hex {
    fn to_json(&self) -> JValue {
        let value = general_purpose::STANDARD.encode(&self.value);
        JValue::Object(vec![
            ("encoding".to_string(), JValue::String("base64".to_string())),
            ("value".to_string(), JValue::String(value)),
        ])
    }
}

impl ToJson for File {
    fn to_json(&self) -> JValue {
        JValue::Object(vec![
            ("type".to_string(), JValue::String("file".to_string())),
            (
                "filename".to_string(),
                JValue::String(self.filename.to_string()),
            ),
        ])
    }
}

fn get_json_version(version_value: &VersionValue) -> Option<String> {
    match version_value {
        VersionValue::Version1 => Some("HTTP/1.0".to_string()),
        VersionValue::Version11 => Some("HTTP/1.1".to_string()),
        VersionValue::Version2 => Some("HTTP/2".to_string()),
        VersionValue::Version3 => Some("HTTP/3".to_string()),
        VersionValue::VersionAny => None,
    }
}

impl ToJson for KeyValue {
    fn to_json(&self) -> JValue {
        let attributes = vec![
            ("name".to_string(), JValue::String(self.key.to_string())),
            ("value".to_string(), JValue::String(self.value.to_string())),
        ];
        JValue::Object(attributes)
    }
}

impl ToJson for MultipartParam {
    fn to_json(&self) -> JValue {
        match self {
            MultipartParam::Param(param) => param.to_json(),
            MultipartParam::FileParam(param) => param.to_json(),
        }
    }
}

impl ToJson for FileParam {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![
            ("name".to_string(), JValue::String(self.key.to_string())),
            (
                "filename".to_string(),
                JValue::String(self.value.filename.to_string()),
            ),
        ];
        if let Some(content_type) = self.value.content_type.clone() {
            attributes.push(("content_type".to_string(), JValue::String(content_type)));
        }
        JValue::Object(attributes)
    }
}

impl ToJson for Cookie {
    fn to_json(&self) -> JValue {
        let attributes = vec![
            ("name".to_string(), JValue::String(self.name.to_string())),
            ("value".to_string(), JValue::String(self.value.to_string())),
        ];
        JValue::Object(attributes)
    }
}

impl ToJson for EntryOption {
    fn to_json(&self) -> JValue {
        let value = match &self.kind {
            OptionKind::AwsSigV4(value) => JValue::String(value.to_string()),
            OptionKind::CaCertificate(filename) => JValue::String(filename.to_string()),
            OptionKind::ClientCert(filename) => JValue::String(filename.to_string()),
            OptionKind::ClientKey(filename) => JValue::String(filename.to_string()),
            OptionKind::Compressed(value) => value.to_json(),
            OptionKind::ConnectTo(value) => JValue::String(value.to_string()),
            OptionKind::ConnectTimeout(value) => value.to_json(),
            OptionKind::Delay(value) => value.to_json(),
            OptionKind::FollowLocation(value) => value.to_json(),
            OptionKind::FollowLocationTrusted(value) => value.to_json(),
            OptionKind::Header(value) => JValue::String(value.to_string()),
            OptionKind::Http10(value) => value.to_json(),
            OptionKind::Http11(value) => value.to_json(),
            OptionKind::Http2(value) => value.to_json(),
            OptionKind::Http3(value) => value.to_json(),
            OptionKind::Insecure(value) => value.to_json(),
            OptionKind::IpV4(value) => value.to_json(),
            OptionKind::IpV6(value) => value.to_json(),
            OptionKind::LimitRate(value) => value.to_json(),
            OptionKind::MaxRedirect(value) => value.to_json(),
            OptionKind::NetRc(value) => value.to_json(),
            OptionKind::NetRcFile(filename) => JValue::String(filename.to_string()),
            OptionKind::NetRcOptional(value) => value.to_json(),
            OptionKind::Output(filename) => JValue::String(filename.to_string()),
            OptionKind::PathAsIs(value) => value.to_json(),
            OptionKind::Proxy(value) => JValue::String(value.to_string()),
            OptionKind::Repeat(value) => value.to_json(),
            OptionKind::Resolve(value) => JValue::String(value.to_string()),
            OptionKind::Retry(value) => value.to_json(),
            OptionKind::RetryInterval(value) => value.to_json(),
            OptionKind::Skip(value) => value.to_json(),
            OptionKind::UnixSocket(value) => JValue::String(value.to_string()),
            OptionKind::User(value) => JValue::String(value.to_string()),
            OptionKind::Variable(value) => {
                JValue::String(format!("{}={}", value.name, value.value))
            }
            OptionKind::Verbose(value) => value.to_json(),
            OptionKind::VeryVerbose(value) => value.to_json(),
        };

        // If the value contains the unit such as `{ "value": 10, "unit": "second" }`
        // The JSON for this option should still have one level
        // for example: { "name": "delay", "value": 10, "unit", "second" }
        let attributes = if let JValue::Object(mut attributes) = value {
            attributes.push((
                "name".to_string(),
                JValue::String(self.kind.identifier().to_string()),
            ));
            attributes
        } else {
            vec![
                (
                    "name".to_string(),
                    JValue::String(self.kind.identifier().to_string()),
                ),
                ("value".to_string(), value),
            ]
        };
        JValue::Object(attributes)
    }
}

impl ToJson for BooleanOption {
    fn to_json(&self) -> JValue {
        match self {
            BooleanOption::Literal(value) => JValue::Boolean(*value),
            BooleanOption::Placeholder(placeholder) => placeholder.to_json(),
        }
    }
}

impl ToJson for CountOption {
    fn to_json(&self) -> JValue {
        match self {
            CountOption::Literal(value) => value.to_json(),
            CountOption::Placeholder(placeholder) => placeholder.to_json(),
        }
    }
}

impl ToJson for Count {
    fn to_json(&self) -> JValue {
        match self {
            Count::Finite(n) => JValue::Number(n.to_string()),
            Count::Infinite => JValue::Number("-1".to_string()),
        }
    }
}

impl ToJson for DurationOption {
    fn to_json(&self) -> JValue {
        match self {
            DurationOption::Literal(value) => value.to_json(),
            DurationOption::Placeholder(placeholder) => placeholder.to_json(),
        }
    }
}

impl ToJson for Duration {
    fn to_json(&self) -> JValue {
        if let Some(unit) = self.unit {
            let mut attributes =
                vec![("value".to_string(), JValue::Number(self.value.to_string()))];
            attributes.push(("unit".to_string(), JValue::String(unit.to_string())));
            JValue::Object(attributes)
        } else {
            JValue::Number(self.value.to_string())
        }
    }
}

impl ToJson for Capture {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![
            ("name".to_string(), JValue::String(self.name.to_string())),
            ("query".to_string(), self.query.to_json()),
        ];
        if !self.filters.is_empty() {
            let filters = JValue::List(self.filters.iter().map(|(_, f)| f.to_json()).collect());
            attributes.push(("filters".to_string(), filters));
        }
        if self.redact {
            attributes.push(("redact".to_string(), JValue::Boolean(true)));
        }
        JValue::Object(attributes)
    }
}

impl ToJson for Assert {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![("query".to_string(), self.query.to_json())];
        if !self.filters.is_empty() {
            let filters = JValue::List(self.filters.iter().map(|(_, f)| f.to_json()).collect());
            attributes.push(("filters".to_string(), filters));
        }
        attributes.push(("predicate".to_string(), self.predicate.to_json()));
        JValue::Object(attributes)
    }
}

impl ToJson for Query {
    fn to_json(&self) -> JValue {
        let attributes = query_value_attributes(&self.value);
        JValue::Object(attributes)
    }
}

fn query_value_attributes(query_value: &QueryValue) -> Vec<(String, JValue)> {
    let mut attributes = vec![];
    let att_type = JValue::String(query_value.identifier().to_string());
    attributes.push(("type".to_string(), att_type));

    match query_value {
        QueryValue::Jsonpath { expr, .. } => {
            attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
        }
        QueryValue::Header { name, .. } => {
            attributes.push(("name".to_string(), JValue::String(name.to_string())));
        }
        QueryValue::Cookie { expr, .. } => {
            attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
        }
        QueryValue::Xpath { expr, .. } => {
            attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
        }
        QueryValue::Regex { value, .. } => {
            attributes.push(("expr".to_string(), value.to_json()));
        }
        QueryValue::Variable { name, .. } => {
            attributes.push(("name".to_string(), JValue::String(name.to_string())));
        }
        QueryValue::Certificate {
            attribute_name: field,
            ..
        } => {
            attributes.push(("expr".to_string(), field.to_json()));
        }
        _ => {}
    };
    attributes
}

impl ToJson for RegexValue {
    fn to_json(&self) -> JValue {
        match self {
            RegexValue::Template(template) => JValue::String(template.to_string()),
            RegexValue::Regex(regex) => regex.to_json(),
        }
    }
}

impl ToJson for Regex {
    fn to_json(&self) -> JValue {
        let attributes = vec![
            ("type".to_string(), JValue::String("regex".to_string())),
            ("value".to_string(), JValue::String(self.to_string())),
        ];
        JValue::Object(attributes)
    }
}

impl ToJson for CertificateAttributeName {
    fn to_json(&self) -> JValue {
        JValue::String(self.identifier().to_string())
    }
}

impl ToJson for Predicate {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        if self.not {
            attributes.push(("not".to_string(), JValue::Boolean(true)));
        }
        let identifier = self.predicate_func.value.identifier();
        attributes.push(("type".to_string(), JValue::String(identifier.to_string())));

        match &self.predicate_func.value {
            PredicateFuncValue::Equal { value, .. } => add_predicate_value(&mut attributes, value),
            PredicateFuncValue::NotEqual { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::GreaterThan { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::GreaterThanOrEqual { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::LessThan { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::LessThanOrEqual { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::StartWith { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::EndWith { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::Contain { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::Include { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::Match { value, .. } => {
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::IsInteger
            | PredicateFuncValue::IsFloat
            | PredicateFuncValue::IsBoolean
            | PredicateFuncValue::IsString
            | PredicateFuncValue::IsCollection
            | PredicateFuncValue::IsDate
            | PredicateFuncValue::IsIsoDate
            | PredicateFuncValue::Exist
            | PredicateFuncValue::IsEmpty
            | PredicateFuncValue::IsNumber => {}
        }
        JValue::Object(attributes)
    }
}

fn add_predicate_value(attributes: &mut Vec<(String, JValue)>, predicate_value: &PredicateValue) {
    let (value, encoding) = json_predicate_value(predicate_value);
    attributes.push(("value".to_string(), value));
    if let Some(encoding) = encoding {
        attributes.push(("encoding".to_string(), JValue::String(encoding)));
    }
}

fn json_predicate_value(predicate_value: &PredicateValue) -> (JValue, Option<String>) {
    match predicate_value {
        PredicateValue::String(value) => (JValue::String(value.to_string()), None),
        PredicateValue::MultilineString(value) => (JValue::String(value.value().to_string()), None),
        PredicateValue::Bool(value) => (JValue::Boolean(*value), None),
        PredicateValue::Null => (JValue::Null, None),
        PredicateValue::Number(value) => (JValue::Number(value.to_string()), None),
        PredicateValue::File(value) => (value.to_json(), None),
        PredicateValue::Hex(value) => {
            let base64_string = general_purpose::STANDARD.encode(value.value.clone());
            (JValue::String(base64_string), Some("base64".to_string()))
        }
        PredicateValue::Base64(value) => {
            let base64_string = general_purpose::STANDARD.encode(value.value.clone());
            (JValue::String(base64_string), Some("base64".to_string()))
        }
        PredicateValue::Placeholder(value) => (JValue::String(value.to_string()), None),
        PredicateValue::Regex(value) => {
            (JValue::String(value.to_string()), Some("regex".to_string()))
        }
    }
}

impl ToJson for JsonValue {
    fn to_json(&self) -> JValue {
        match self {
            JsonValue::Null => JValue::Null,
            JsonValue::Number(s) => JValue::Number(s.to_string()),
            JsonValue::String(s) => JValue::String(s.to_string()),
            JsonValue::Boolean(v) => JValue::Boolean(*v),
            JsonValue::List { elements, .. } => {
                JValue::List(elements.iter().map(|e| e.to_json()).collect())
            }
            JsonValue::Object { elements, .. } => JValue::Object(
                elements
                    .iter()
                    .map(|elem| (elem.name.to_string(), elem.value.to_json()))
                    .collect(),
            ),
            JsonValue::Placeholder(exp) => JValue::String(format!("{{{{{exp}}}}}")),
        }
    }
}

impl ToJson for JsonListElement {
    fn to_json(&self) -> JValue {
        self.value.to_json()
    }
}

impl ToJson for Filter {
    fn to_json(&self) -> JValue {
        self.value.to_json()
    }
}

impl ToJson for FilterValue {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        let att_name = "type".to_string();
        let att_value = JValue::String(self.identifier().to_string());
        attributes.push((att_name, att_value));

        match self {
            FilterValue::Decode { encoding, .. } => {
                attributes.push(("encoding".to_string(), JValue::String(encoding.to_string())));
            }
            FilterValue::Format { fmt, .. } => {
                attributes.push(("fmt".to_string(), JValue::String(fmt.to_string())));
            }
            FilterValue::JsonPath { expr, .. } => {
                attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
            }
            FilterValue::Nth { n, .. } => {
                attributes.push(("n".to_string(), JValue::Number(n.to_string())));
            }
            FilterValue::Regex { value, .. } => {
                attributes.push(("expr".to_string(), value.to_json()));
            }
            FilterValue::Replace {
                old_value,
                new_value,
                ..
            } => {
                attributes.push(("old_value".to_string(), old_value.to_json()));
                attributes.push((
                    "new_value".to_string(),
                    JValue::String(new_value.to_string()),
                ));
            }
            FilterValue::Split { sep, .. } => {
                attributes.push(("sep".to_string(), JValue::String(sep.to_string())));
            }
            FilterValue::ToDate { fmt, .. } => {
                attributes.push(("fmt".to_string(), JValue::String(fmt.to_string())));
            }
            FilterValue::XPath { expr, .. } => {
                attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
            }
            _ => {}
        }
        JValue::Object(attributes)
    }
}

impl ToJson for Placeholder {
    fn to_json(&self) -> JValue {
        JValue::String(format!("{{{{{}}}}}", self))
    }
}

impl ToJson for Comment {
    fn to_json(&self) -> JValue {
        JValue::String(self.value.to_string())
    }
}

impl ToJson for NaturalOption {
    fn to_json(&self) -> JValue {
        match self {
            NaturalOption::Literal(value) => JValue::Number(value.to_string()),
            NaturalOption::Placeholder(placeholder) => placeholder.to_json(),
        }
    }
}
#[cfg(test)]
pub mod tests {
    use hurl_core::ast::{
        LineTerminator, Method, Number, PredicateFunc, SourceInfo, Status, Template,
        TemplateElement, Version, Whitespace, I64,
    };
    use hurl_core::reader::Pos;
    use hurl_core::typing::ToSource;

    use super::*;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: String::new(),
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
        }
    }

    fn line_terminator() -> LineTerminator {
        LineTerminator {
            space0: whitespace(),
            comment: None,
            newline: whitespace(),
        }
    }

    #[test]
    pub fn test_request() {
        assert_eq!(
            Request {
                line_terminators: vec![],
                space0: whitespace(),
                method: Method("GET".to_string()),
                space1: whitespace(),
                url: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "http://example.com".to_string(),
                        source: "not_used".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                line_terminator0: line_terminator(),
                headers: vec![KeyValue {
                    line_terminators: vec![],
                    space0: whitespace(),
                    key: Template {
                        delimiter: None,
                        elements: vec![TemplateElement::String {
                            value: "Foo".to_string(),
                            source: "unused".to_source(),
                        }],
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    },
                    space1: whitespace(),
                    space2: whitespace(),
                    value: Template {
                        delimiter: None,
                        elements: vec![TemplateElement::String {
                            value: "Bar".to_string(),
                            source: "unused".to_source(),
                        }],
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    },
                    line_terminator0: line_terminator(),
                }],
                sections: vec![],
                body: None,
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            }
            .to_json(),
            JValue::Object(vec![
                ("method".to_string(), JValue::String("GET".to_string())),
                (
                    "url".to_string(),
                    JValue::String("http://example.com".to_string())
                ),
                (
                    "headers".to_string(),
                    JValue::List(vec![JValue::Object(vec![
                        ("name".to_string(), JValue::String("Foo".to_string())),
                        ("value".to_string(), JValue::String("Bar".to_string()))
                    ])])
                )
            ])
        );
    }

    #[test]
    pub fn test_response() {
        assert_eq!(
            Response {
                line_terminators: vec![],
                version: Version {
                    value: VersionValue::Version11,
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space0: whitespace(),
                status: Status {
                    value: StatusValue::Specific(200),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space1: whitespace(),
                line_terminator0: line_terminator(),
                headers: vec![],
                sections: vec![],
                body: None,
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            }
            .to_json(),
            JValue::Object(vec![
                (
                    "version".to_string(),
                    JValue::String("HTTP/1.1".to_string())
                ),
                ("status".to_string(), JValue::Number("200".to_string()))
            ])
        );
        assert_eq!(
            Response {
                line_terminators: vec![],
                version: Version {
                    value: VersionValue::VersionAny,
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space0: whitespace(),
                status: Status {
                    value: StatusValue::Any,
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
                space1: whitespace(),
                line_terminator0: line_terminator(),
                headers: vec![],
                sections: vec![],
                body: None,
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            }
            .to_json(),
            JValue::Object(vec![])
        );
    }

    fn header_query() -> Query {
        Query {
            source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            value: QueryValue::Header {
                space0: whitespace(),
                name: Template {
                    delimiter: None,
                    elements: vec![TemplateElement::String {
                        value: "Content-Length".to_string(),
                        source: "Content-Length".to_source(),
                    }],
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                },
            },
        }
    }

    fn header_capture() -> Capture {
        Capture {
            line_terminators: vec![],
            space0: whitespace(),
            name: Template {
                delimiter: None,
                elements: vec![TemplateElement::String {
                    value: "size".to_string(),
                    source: "unused".to_source(),
                }],
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: whitespace(),
            space2: whitespace(),
            query: header_query(),
            filters: vec![],
            space3: whitespace(),
            redact: false,
            line_terminator0: line_terminator(),
        }
    }

    fn header_assert() -> Assert {
        Assert {
            line_terminators: vec![],
            space0: whitespace(),
            query: header_query(),
            filters: vec![],
            space1: whitespace(),
            predicate: equal_int_predicate(10),
            line_terminator0: line_terminator(),
        }
    }

    fn equal_int_predicate(value: i64) -> Predicate {
        Predicate {
            not: false,
            space0: whitespace(),
            predicate_func: PredicateFunc {
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                value: PredicateFuncValue::Equal {
                    space0: whitespace(),
                    value: PredicateValue::Number(Number::Integer(I64::new(
                        value,
                        value.to_string().to_source(),
                    ))),
                },
            },
        }
    }

    #[test]
    pub fn test_query() {
        assert_eq!(
            header_query().to_json(),
            JValue::Object(vec![
                ("type".to_string(), JValue::String("header".to_string())),
                (
                    "name".to_string(),
                    JValue::String("Content-Length".to_string())
                ),
            ])
        );
    }

    #[test]
    pub fn test_capture() {
        assert_eq!(
            header_capture().to_json(),
            JValue::Object(vec![
                ("name".to_string(), JValue::String("size".to_string())),
                (
                    "query".to_string(),
                    JValue::Object(vec![
                        ("type".to_string(), JValue::String("header".to_string())),
                        (
                            "name".to_string(),
                            JValue::String("Content-Length".to_string())
                        ),
                    ])
                ),
            ])
        );
    }

    #[test]
    pub fn test_predicate() {
        assert_eq!(
            equal_int_predicate(10).to_json(),
            JValue::Object(vec![
                ("type".to_string(), JValue::String("==".to_string())),
                ("value".to_string(), JValue::Number("10".to_string()))
            ]),
        );
    }

    #[test]
    pub fn test_assert() {
        assert_eq!(
            header_assert().to_json(),
            JValue::Object(vec![
                (
                    "query".to_string(),
                    JValue::Object(vec![
                        ("type".to_string(), JValue::String("header".to_string())),
                        (
                            "name".to_string(),
                            JValue::String("Content-Length".to_string())
                        ),
                    ])
                ),
                (
                    "predicate".to_string(),
                    JValue::Object(vec![
                        ("type".to_string(), JValue::String("==".to_string())),
                        ("value".to_string(), JValue::Number("10".to_string()))
                    ])
                )
            ]),
        );
    }
}
