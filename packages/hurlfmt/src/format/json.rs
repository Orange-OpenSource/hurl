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
use base64::engine::general_purpose;
use base64::Engine;
use hurl_core::ast::*;
use hurl_core::typing::Count;

use super::serialize_json::*;

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
                    MultilineString::OneLineText(_) | MultilineString::Text(_) => "text",
                    MultilineString::Json(_) => "json",
                    MultilineString::Xml(_) => "xml",
                    MultilineString::GraphQl(_) => "graphql",
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
        VersionValue::VersionAnyLegacy => None,
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
        let mut attributes = vec![];

        let name = "name".to_string();
        let value = JValue::String(self.kind.name().to_string());
        attributes.push((name, value));

        let name = "value".to_string();
        let value = match &self.kind {
            OptionKind::AwsSigV4(value) => JValue::String(value.to_string()),
            OptionKind::CaCertificate(filename) => JValue::String(filename.to_string()),
            OptionKind::ClientCert(filename) => JValue::String(filename.to_string()),
            OptionKind::ClientKey(filename) => JValue::String(filename.to_string()),
            OptionKind::Compressed(value) => value.to_json(),
            OptionKind::ConnectTo(value) => JValue::String(value.to_string()),
            OptionKind::Delay(value) => value.to_json(),
            OptionKind::FollowLocation(value) => value.to_json(),
            OptionKind::FollowLocationTrusted(value) => value.to_json(),
            OptionKind::Http10(value) => value.to_json(),
            OptionKind::Http11(value) => value.to_json(),
            OptionKind::Http2(value) => value.to_json(),
            OptionKind::Http3(value) => value.to_json(),
            OptionKind::Insecure(value) => value.to_json(),
            OptionKind::IpV4(value) => value.to_json(),
            OptionKind::IpV6(value) => value.to_json(),
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
        attributes.push((name, value));

        JValue::Object(attributes)
    }
}

impl ToJson for BooleanOption {
    fn to_json(&self) -> JValue {
        match self {
            BooleanOption::Literal(value) => JValue::Boolean(*value),
            BooleanOption::Expression(expr) => expr.to_json(),
        }
    }
}

impl ToJson for NaturalOption {
    fn to_json(&self) -> JValue {
        match self {
            NaturalOption::Literal(value) => JValue::Number(value.to_string()),
            NaturalOption::Expression(expr) => expr.to_json(),
        }
    }
}

impl ToJson for CountOption {
    fn to_json(&self) -> JValue {
        match self {
            CountOption::Literal(value) => value.to_json(),
            CountOption::Expression(expr) => expr.to_json(),
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
    match query_value {
        QueryValue::Status => {
            attributes.push(("type".to_string(), JValue::String("status".to_string())));
        }
        QueryValue::Url => {
            attributes.push(("type".to_string(), JValue::String("url".to_string())));
        }
        QueryValue::Body => {
            attributes.push(("type".to_string(), JValue::String("body".to_string())));
        }
        QueryValue::Jsonpath { expr, .. } => {
            attributes.push(("type".to_string(), JValue::String("jsonpath".to_string())));
            attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
        }
        QueryValue::Header { name, .. } => {
            attributes.push(("type".to_string(), JValue::String("header".to_string())));
            attributes.push(("name".to_string(), JValue::String(name.to_string())));
        }
        QueryValue::Cookie { expr, .. } => {
            attributes.push(("type".to_string(), JValue::String("cookie".to_string())));
            attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
        }
        QueryValue::Xpath { expr, .. } => {
            attributes.push(("type".to_string(), JValue::String("xpath".to_string())));
            attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
        }
        QueryValue::Regex { value, .. } => {
            attributes.push(("type".to_string(), JValue::String("regex".to_string())));
            attributes.push(("expr".to_string(), value.to_json()));
        }
        QueryValue::Variable { name, .. } => {
            attributes.push(("type".to_string(), JValue::String("variable".to_string())));
            attributes.push(("name".to_string(), JValue::String(name.to_string())));
        }
        QueryValue::Duration => {
            attributes.push(("type".to_string(), JValue::String("duration".to_string())));
        }
        QueryValue::Bytes => {
            attributes.push(("type".to_string(), JValue::String("bytes".to_string())));
        }
        QueryValue::Sha256 => {
            attributes.push(("type".to_string(), JValue::String("sha256".to_string())));
        }
        QueryValue::Md5 => {
            attributes.push(("type".to_string(), JValue::String("md5".to_string())));
        }
        QueryValue::Certificate {
            attribute_name: field,
            ..
        } => {
            attributes.push((
                "type".to_string(),
                JValue::String("certificate".to_string()),
            ));
            attributes.push(("expr".to_string(), field.to_json()));
        }
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
        let value = match self {
            CertificateAttributeName::Subject => "Subject",
            CertificateAttributeName::Issuer => "Issuer",
            CertificateAttributeName::StartDate => "Start-Date",
            CertificateAttributeName::ExpireDate => "Expire-Date",
            CertificateAttributeName::SerialNumber => "Serial-Number",
        };
        JValue::String(value.to_string())
    }
}

impl ToJson for Predicate {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        if self.not {
            attributes.push(("not".to_string(), JValue::Boolean(true)));
        }
        match self.predicate_func.value.clone() {
            PredicateFuncValue::Equal { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("equal".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::NotEqual { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("not-equal".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::GreaterThan { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("greater".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::GreaterThanOrEqual { value, .. } => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("greater-or-equal".to_string()),
                ));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::LessThan { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("less".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::LessThanOrEqual { value, .. } => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("less-or-equal".to_string()),
                ));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::StartWith { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("start-with".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::EndWith { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("end-with".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::Contain { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("contain".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::Include { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("include".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::Match { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("match".to_string())));
                add_predicate_value(&mut attributes, value);
            }
            PredicateFuncValue::IsInteger => {
                attributes.push(("type".to_string(), JValue::String("isInteger".to_string())));
            }
            PredicateFuncValue::IsFloat => {
                attributes.push(("type".to_string(), JValue::String("isFloat".to_string())));
            }
            PredicateFuncValue::IsBoolean => {
                attributes.push(("type".to_string(), JValue::String("isBoolean".to_string())));
            }
            PredicateFuncValue::IsString => {
                attributes.push(("type".to_string(), JValue::String("isString".to_string())));
            }
            PredicateFuncValue::IsCollection => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("isCollection".to_string()),
                ));
            }
            PredicateFuncValue::IsDate => {
                attributes.push(("type".to_string(), JValue::String("isDate".to_string())));
            }
            PredicateFuncValue::IsIsoDate => {
                attributes.push(("type".to_string(), JValue::String("isIsoDate".to_string())));
            }
            PredicateFuncValue::Exist => {
                attributes.push(("type".to_string(), JValue::String("exist".to_string())));
            }
            PredicateFuncValue::IsEmpty => {
                attributes.push(("type".to_string(), JValue::String("isEmpty".to_string())));
            }
            PredicateFuncValue::IsNumber => {
                attributes.push(("type".to_string(), JValue::String("isNumber".to_string())));
            }
        }
        JValue::Object(attributes)
    }
}

fn add_predicate_value(attributes: &mut Vec<(String, JValue)>, predicate_value: PredicateValue) {
    let (value, encoding) = json_predicate_value(predicate_value);
    attributes.push(("value".to_string(), value));
    if let Some(encoding) = encoding {
        attributes.push(("encoding".to_string(), JValue::String(encoding)));
    }
}

fn json_predicate_value(predicate_value: PredicateValue) -> (JValue, Option<String>) {
    match predicate_value {
        PredicateValue::String(value) => (JValue::String(value.to_string()), None),
        PredicateValue::MultilineString(value) => (JValue::String(value.value().to_string()), None),
        PredicateValue::Bool(value) => (JValue::Boolean(value), None),
        PredicateValue::Null => (JValue::Null, None),
        PredicateValue::Number(value) => (JValue::Number(value.to_string()), None),
        PredicateValue::File(value) => (value.to_json(), None),
        PredicateValue::Hex(value) => {
            let base64_string = general_purpose::STANDARD.encode(value.value);
            (JValue::String(base64_string), Some("base64".to_string()))
        }
        PredicateValue::Base64(value) => {
            let base64_string = general_purpose::STANDARD.encode(value.value);
            (JValue::String(base64_string), Some("base64".to_string()))
        }
        PredicateValue::Expression(value) => (JValue::String(value.to_string()), None),
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
            JsonValue::Expression(exp) => JValue::String(format!("{{{{{exp}}}}}")),
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
        match self {
            FilterValue::Count => {
                attributes.push(("type".to_string(), JValue::String("count".to_string())));
            }
            FilterValue::DaysAfterNow => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("daysAfterNow".to_string()),
                ));
            }
            FilterValue::DaysBeforeNow => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("daysBeforeNow".to_string()),
                ));
            }
            FilterValue::Decode { encoding, .. } => {
                attributes.push(("type".to_string(), JValue::String("decode".to_string())));
                attributes.push(("encoding".to_string(), JValue::String(encoding.to_string())));
            }
            FilterValue::Format { fmt, .. } => {
                attributes.push(("type".to_string(), JValue::String("format".to_string())));
                attributes.push(("fmt".to_string(), JValue::String(fmt.to_string())));
            }
            FilterValue::JsonPath { expr, .. } => {
                attributes.push(("type".to_string(), JValue::String("jsonpath".to_string())));
                attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
            }
            FilterValue::Nth { n, .. } => {
                attributes.push(("type".to_string(), JValue::String("nth".to_string())));
                attributes.push(("n".to_string(), JValue::Number(n.to_string())));
            }
            FilterValue::HtmlEscape => {
                attributes.push(("type".to_string(), JValue::String("htmlEscape".to_string())));
            }
            FilterValue::HtmlUnescape => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("htmlUnescape".to_string()),
                ));
            }
            FilterValue::Regex { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("regex".to_string())));
                attributes.push(("expr".to_string(), value.to_json()));
            }
            FilterValue::Replace {
                old_value,
                new_value,
                ..
            } => {
                attributes.push(("type".to_string(), JValue::String("replace".to_string())));
                attributes.push(("old_value".to_string(), old_value.to_json()));
                attributes.push((
                    "new_value".to_string(),
                    JValue::String(new_value.to_string()),
                ));
            }
            FilterValue::UrlEncode => {
                attributes.push(("type".to_string(), JValue::String("urlEncode".to_string())));
            }
            FilterValue::UrlDecode => {
                attributes.push(("type".to_string(), JValue::String("urlDecode".to_string())));
            }
            FilterValue::Split { sep, .. } => {
                attributes.push(("type".to_string(), JValue::String("split".to_string())));
                attributes.push(("sep".to_string(), JValue::String(sep.to_string())));
            }
            FilterValue::ToDate { fmt, .. } => {
                attributes.push(("type".to_string(), JValue::String("toDate".to_string())));
                attributes.push(("fmt".to_string(), JValue::String(fmt.to_string())));
            }
            FilterValue::ToFloat => {
                attributes.push(("type".to_string(), JValue::String("toFloat".to_string())));
            }
            FilterValue::ToInt => {
                attributes.push(("type".to_string(), JValue::String("toInt".to_string())));
            }
            FilterValue::XPath { expr, .. } => {
                attributes.push(("type".to_string(), JValue::String("xpath".to_string())));
                attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
            }
        }
        JValue::Object(attributes)
    }
}

impl ToJson for Expr {
    fn to_json(&self) -> JValue {
        JValue::String(format!("{{{{{}}}}}", self))
    }
}

impl ToJson for Comment {
    fn to_json(&self) -> JValue {
        JValue::String(self.value.to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use hurl_core::reader::Pos;

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
                        encoded: "not_used".to_string(),
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
                            encoded: "unused".to_string(),
                        }],
                        source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                    },
                    space1: whitespace(),
                    space2: whitespace(),
                    value: Template {
                        delimiter: None,
                        elements: vec![TemplateElement::String {
                            value: "Bar".to_string(),
                            encoded: "unused".to_string(),
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
                        encoded: "10".to_string(),
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
                    encoded: "unused".to_string(),
                }],
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: whitespace(),
            space2: whitespace(),
            query: header_query(),
            filters: vec![],
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
                    value: PredicateValue::Number(Number::Integer(value)),
                    operator: false,
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
                ("type".to_string(), JValue::String("equal".to_string())),
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
                        ("type".to_string(), JValue::String("equal".to_string())),
                        ("value".to_string(), JValue::Number("10".to_string()))
                    ])
                )
            ]),
        );
    }
}
