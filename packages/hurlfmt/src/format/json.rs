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

use super::serialize_json::*;
use hurl_core::ast::*;

pub fn format(hurl_file: HurlFile) -> String {
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
        if let Some(response) = self.response.clone() {
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
        add_headers(&mut attributes, self.headers.clone());

        if !self.clone().querystring_params().is_empty() {
            let params = self
                .clone()
                .querystring_params()
                .iter()
                .map(|p| p.to_json())
                .collect();
            attributes.push(("query_string_params".to_string(), JValue::List(params)));
        }
        if !self.clone().form_params().is_empty() {
            let params = self
                .clone()
                .form_params()
                .iter()
                .map(|p| p.to_json())
                .collect();
            attributes.push(("form_params".to_string(), JValue::List(params)));
        }
        if !self.clone().multipart_form_data().is_empty() {
            let params = self
                .clone()
                .multipart_form_data()
                .iter()
                .map(|p| p.to_json())
                .collect();
            attributes.push(("multipart_form_data".to_string(), JValue::List(params)));
        }
        if !self.clone().cookies().is_empty() {
            let cookies = self.clone().cookies().iter().map(|c| c.to_json()).collect();
            attributes.push(("cookies".to_string(), JValue::List(cookies)));
        }
        if let Some(body) = self.body.clone() {
            attributes.push(("body".to_string(), body.to_json()));
        }
        JValue::Object(attributes)
    }
}

impl ToJson for Response {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        if let Some(v) = get_json_version(self.version.value.clone()) {
            attributes.push(("version".to_string(), JValue::String(v)))
        }
        if let StatusValue::Specific(n) = self.status.value {
            attributes.push(("status".to_string(), JValue::Number(n.to_string())));
        }
        add_headers(&mut attributes, self.headers.clone());
        if !self.clone().captures().is_empty() {
            let captures = self
                .clone()
                .captures()
                .iter()
                .map(|c| c.to_json())
                .collect();
            attributes.push(("captures".to_string(), JValue::List(captures)));
        }
        if !self.clone().asserts().is_empty() {
            let asserts = self.clone().asserts().iter().map(|a| a.to_json()).collect();
            attributes.push(("asserts".to_string(), JValue::List(asserts)));
        }
        if let Some(body) = self.body.clone() {
            attributes.push(("body".to_string(), body.to_json()));
        }
        JValue::Object(attributes)
    }
}

fn add_headers(attributes: &mut Vec<(String, JValue)>, headers: Vec<Header>) {
    if !headers.is_empty() {
        let headers = JValue::List(headers.iter().map(|h| h.to_json()).collect());
        attributes.push(("headers".to_string(), headers))
    }
}

impl ToJson for Body {
    fn to_json(&self) -> JValue {
        self.value.to_json()
    }
}

impl ToJson for Bytes {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        match self {
            Bytes::Base64 { encoded, .. } => {
                attributes.push(("type".to_string(), JValue::String("base64".to_string())));
                attributes.push(("value".to_string(), JValue::String(encoded.clone())));
            }
            Bytes::Json { value } => {
                attributes.push(("type".to_string(), JValue::String("json".to_string())));
                attributes.push(("value".to_string(), value.to_json()));
            }
            Bytes::Xml { value } => {
                attributes.push(("type".to_string(), JValue::String("xml".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.clone())));
            }
            Bytes::RawString { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("raw-string".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            Bytes::File { filename, .. } => {
                attributes.push(("type".to_string(), JValue::String("file".to_string())));
                attributes.push((
                    "filename".to_string(),
                    JValue::String(filename.value.clone()),
                ));
            }
        }
        JValue::Object(attributes)
    }
}

fn get_json_version(version_value: VersionValue) -> Option<String> {
    match version_value {
        VersionValue::Version1 => Some("HTTP/1.0".to_string()),
        VersionValue::Version11 => Some("HTTP/1.1".to_string()),
        VersionValue::Version2 => Some("HTTP/2".to_string()),
        VersionValue::VersionAny => None,
    }
}

impl ToJson for KeyValue {
    fn to_json(&self) -> JValue {
        let attributes = vec![
            ("name".to_string(), JValue::String(self.key.value.clone())),
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
            ("name".to_string(), JValue::String(self.key.value.clone())),
            (
                "filename".to_string(),
                JValue::String(self.value.filename.value.clone()),
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
            ("name".to_string(), JValue::String(self.name.value.clone())),
            (
                "value".to_string(),
                JValue::String(self.value.value.clone()),
            ),
        ];
        JValue::Object(attributes)
    }
}

impl ToJson for Capture {
    fn to_json(&self) -> JValue {
        let attributes = vec![
            ("name".to_string(), JValue::String(self.name.value.clone())),
            ("query".to_string(), self.query.to_json()),
        ];
        JValue::Object(attributes)
    }
}

impl ToJson for Assert {
    fn to_json(&self) -> JValue {
        let attributes = vec![
            ("query".to_string(), self.query.to_json()),
            ("predicate".to_string(), self.predicate.to_json()),
        ];
        JValue::Object(attributes)
    }
}

impl ToJson for Query {
    fn to_json(&self) -> JValue {
        let mut attributes = query_value_attributes(&self.value);
        if let Some((_, subquery)) = self.subquery.clone() {
            attributes.push(("subquery".to_string(), subquery.to_json()));
        }
        JValue::Object(attributes)
    }
}

fn query_value_attributes(query_value: &QueryValue) -> Vec<(String, JValue)> {
    let mut attributes = vec![];
    match query_value {
        QueryValue::Status {} => {
            attributes.push(("type".to_string(), JValue::String("status".to_string())));
        }
        QueryValue::Body {} => {
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
        QueryValue::Regex { expr, .. } => {
            attributes.push(("type".to_string(), JValue::String("regex".to_string())));
            attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
        }
        QueryValue::Variable { name, .. } => {
            attributes.push(("type".to_string(), JValue::String("variable".to_string())));
            attributes.push(("name".to_string(), JValue::String(name.to_string())));
        }
        QueryValue::Duration {} => {
            attributes.push(("type".to_string(), JValue::String("duration".to_string())));
        }
        QueryValue::Bytes {} => {
            attributes.push(("type".to_string(), JValue::String("bytes".to_string())));
        }
        QueryValue::Sha256 {} => {
            attributes.push(("type".to_string(), JValue::String("sha256".to_string())));
        }
    };
    attributes
}

impl ToJson for Subquery {
    fn to_json(&self) -> JValue {
        self.value.to_json()
    }
}

impl ToJson for SubqueryValue {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        match self {
            SubqueryValue::Regex { expr, .. } => {
                attributes.push(("type".to_string(), JValue::String("regex".to_string())));
                attributes.push(("expr".to_string(), JValue::String(expr.to_string())));
            }
            SubqueryValue::Count { .. } => {
                attributes.push(("type".to_string(), JValue::String("count".to_string())));
            }
        }
        JValue::Object(attributes)
    }
}

impl ToJson for Predicate {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        if self.not {
            attributes.push(("not".to_string(), JValue::Boolean(true)))
        }
        match self.predicate_func.value.clone() {
            PredicateFuncValue::EqualInt { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("equal".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::EqualBool { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("equal".to_string())));
                attributes.push(("value".to_string(), JValue::Boolean(value)));
            }
            PredicateFuncValue::EqualString { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("equal".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::EqualFloat { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("equal".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::EqualNull { .. } => {
                attributes.push(("type".to_string(), JValue::String("equal".to_string())));
                attributes.push(("value".to_string(), JValue::Null));
            }
            PredicateFuncValue::EqualHex { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("equal".to_string())));
                let value = JValue::Object(vec![
                    (
                        "value".to_string(),
                        JValue::String(base64::encode(&value.value)),
                    ),
                    ("encoding".to_string(), JValue::String("base64".to_string())),
                ]);
                attributes.push(("value".to_string(), value));
            }
            PredicateFuncValue::EqualExpression { value, .. } => {
                attributes.push((
                    "type".to_string(),
                    JValue::String(
                        "\
                equal"
                            .to_string(),
                    ),
                ));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::NotEqualInt { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("not-equal".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::NotEqualBool { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("not-equal".to_string())));
                attributes.push(("value".to_string(), JValue::Boolean(value)));
            }
            PredicateFuncValue::NotEqualString { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("not-equal".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::NotEqualFloat { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("not-equal".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::NotEqualNull { .. } => {
                attributes.push(("type".to_string(), JValue::String("not-equal".to_string())));
                attributes.push(("value".to_string(), JValue::Null));
            }
            PredicateFuncValue::NotEqualHex { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("not-equal".to_string())));
                let value = JValue::Object(vec![
                    (
                        "value".to_string(),
                        JValue::String(base64::encode(&value.value)),
                    ),
                    ("encoding".to_string(), JValue::String("base64".to_string())),
                ]);
                attributes.push(("value".to_string(), value));
            }
            PredicateFuncValue::NotEqualExpression { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("not-equal".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::GreaterThanInt { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("greater".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::GreaterThanFloat { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("greater".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::GreaterThanOrEqualInt { value, .. } => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("greater-or-equal".to_string()),
                ));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::GreaterThanOrEqualFloat { value, .. } => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("greater-or-equal".to_string()),
                ));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::LessThanInt { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("less".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::LessThanFloat { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("less".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::LessThanOrEqualInt { value, .. } => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("less-or-equal".to_string()),
                ));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::LessThanOrEqualFloat { value, .. } => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("less-or-equal".to_string()),
                ));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::CountEqual { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("count".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::StartWith { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("start-with".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::Contain { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("contain".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::IncludeString { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("include".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::IncludeInt { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("include".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::IncludeFloat { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("include".to_string())));
                attributes.push(("value".to_string(), JValue::Number(value.to_string())));
            }
            PredicateFuncValue::IncludeBool { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("include".to_string())));
                attributes.push(("value".to_string(), JValue::Boolean(value)));
            }
            PredicateFuncValue::IncludeNull { .. } => {
                attributes.push(("type".to_string(), JValue::String("include".to_string())));
                attributes.push(("value".to_string(), JValue::Null));
            }
            PredicateFuncValue::IncludeExpression { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("include".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::Match { value, .. } => {
                attributes.push(("type".to_string(), JValue::String("match".to_string())));
                attributes.push(("value".to_string(), JValue::String(value.to_string())));
            }
            PredicateFuncValue::IsInteger {} => {
                attributes.push(("type".to_string(), JValue::String("isInteger".to_string())));
            }
            PredicateFuncValue::IsFloat {} => {
                attributes.push(("type".to_string(), JValue::String("isFloat".to_string())));
            }
            PredicateFuncValue::IsBoolean {} => {
                attributes.push(("type".to_string(), JValue::String("isBoolean".to_string())));
            }
            PredicateFuncValue::IsString {} => {
                attributes.push(("type".to_string(), JValue::String("isString".to_string())));
            }
            PredicateFuncValue::IsCollection {} => {
                attributes.push((
                    "type".to_string(),
                    JValue::String("isCollection".to_string()),
                ));
            }
            PredicateFuncValue::Exist {} => {
                attributes.push(("type".to_string(), JValue::String("exist".to_string())));
            }
        }
        JValue::Object(attributes)
    }
}

impl ToJson for hurl_core::ast::JsonValue {
    fn to_json(&self) -> JValue {
        match self {
            JsonValue::Null {} => JValue::Null {},
            JsonValue::Number(s) => JValue::Number(s.to_string()),
            JsonValue::String(s) => JValue::String(s.to_string()),
            JsonValue::Boolean(v) => JValue::Boolean(*v),
            JsonValue::List { elements, .. } => {
                JValue::List(elements.iter().map(|e| e.to_json()).collect())
            }
            JsonValue::Object { elements, .. } => JValue::Object(
                elements
                    .iter()
                    .map(|elem| (elem.name.clone(), elem.value.to_json()))
                    .collect(),
            ),
            JsonValue::Expression(exp) => JValue::String(format!("{{{{{}}}}}", exp.to_string())),
        }
    }
}

impl ToJson for hurl_core::ast::JsonListElement {
    fn to_json(&self) -> JValue {
        self.value.to_json()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn whitespace() -> Whitespace {
        Whitespace {
            value: "".to_string(),
            source_info: SourceInfo::init(0, 0, 0, 0),
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
                method: Method::Get,
                space1: whitespace(),
                url: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "http://example.com".to_string(),
                        encoded: "not_used".to_string(),
                    }],
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                line_terminator0: line_terminator(),
                headers: vec![KeyValue {
                    line_terminators: vec![],
                    space0: whitespace(),
                    key: EncodedString {
                        value: "Foo".to_string(),
                        encoded: "unused".to_string(),
                        quotes: false,
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    space1: whitespace(),
                    space2: whitespace(),
                    value: Template {
                        quotes: false,
                        elements: vec![TemplateElement::String {
                            value: "Bar".to_string(),
                            encoded: "unused".to_string(),
                        }],
                        source_info: SourceInfo::init(0, 0, 0, 0),
                    },
                    line_terminator0: line_terminator(),
                }],
                sections: vec![],
                body: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
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
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                space0: whitespace(),
                status: Status {
                    value: StatusValue::Specific(200),
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                space1: whitespace(),
                line_terminator0: line_terminator(),
                headers: vec![],
                sections: vec![],
                body: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
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
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                space0: whitespace(),
                status: Status {
                    value: StatusValue::Any,
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
                space1: whitespace(),
                line_terminator0: line_terminator(),
                headers: vec![],
                sections: vec![],
                body: None,
                source_info: SourceInfo::init(0, 0, 0, 0),
            }
            .to_json(),
            JValue::Object(vec![])
        );
    }

    fn header_query() -> Query {
        Query {
            source_info: SourceInfo::init(0, 0, 0, 0),
            value: QueryValue::Header {
                space0: whitespace(),
                name: Template {
                    quotes: false,
                    elements: vec![TemplateElement::String {
                        value: "Content-Length".to_string(),
                        encoded: "10".to_string(),
                    }],
                    source_info: SourceInfo::init(0, 0, 0, 0),
                },
            },
            subquery: None,
        }
    }

    fn header_capture() -> Capture {
        Capture {
            line_terminators: vec![],
            space0: whitespace(),
            name: EncodedString {
                value: "size".to_string(),
                encoded: "unused".to_string(),
                quotes: false,
                source_info: SourceInfo::init(0, 0, 0, 0),
            },
            space1: whitespace(),
            space2: whitespace(),
            query: header_query(),
            line_terminator0: line_terminator(),
        }
    }

    fn header_assert() -> Assert {
        Assert {
            line_terminators: vec![],
            space0: whitespace(),
            query: header_query(),
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
                source_info: SourceInfo::init(0, 0, 0, 0),
                value: PredicateFuncValue::EqualInt {
                    space0: whitespace(),
                    value,
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
