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
        let mut attributes = vec![];
        attributes.push(("request".to_string(), self.request.to_json()));
        if let Some(response) = self.response.clone() {
            attributes.push(("response".to_string(), response.to_json()));
        }
        JValue::Object(attributes)
    }
}

impl ToJson for Request {
    fn to_json(&self) -> JValue {
        let mut attributes = vec![];
        attributes.push((
            "method".to_string(),
            JValue::String(self.method.to_string()),
        ));
        attributes.push(("url".to_string(), JValue::String(self.url.to_string())));
        add_headers(&mut attributes, self.headers.clone());
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
        JValue::Object(attributes)
    }
}

fn add_headers(attributes: &mut Vec<(String, JValue)>, headers: Vec<Header>) {
    if !headers.is_empty() {
        let headers = JValue::List(headers.iter().map(|h| h.to_json()).collect());
        attributes.push(("headers".to_string(), headers))
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
        let mut attributes = vec![];
        attributes.push(("name".to_string(), JValue::String(self.key.value.clone())));
        attributes.push(("value".to_string(), JValue::String(self.value.to_string())));
        JValue::Object(attributes)
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
                            encoded: "unused".to_string()
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
}
