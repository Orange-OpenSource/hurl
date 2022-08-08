/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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

use crate::cli;
use crate::runner::HurlResult;

use xmltree::{Element, XMLNode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Testcase {
    id: String,
    time_in_ms: u128,
    failures: Vec<String>,
    errors: Vec<String>,
}

impl Testcase {
    ///
    /// create an XML Junit <testcase> from an Hurl result
    ///
    pub fn from_hurl_result(hurl_result: &HurlResult, content: &str) -> Testcase {
        let id = hurl_result.filename.clone();
        let time_in_ms = hurl_result.time_in_ms;
        let mut failures = vec![];
        let mut errors = vec![];

        for error in hurl_result.errors() {
            let message = cli::error_string_no_color(&hurl_result.filename, content, &error);
            if error.assert {
                failures.push(message);
            } else {
                errors.push(message);
            };
        }
        Testcase {
            id,
            time_in_ms,
            failures,
            errors,
        }
    }

    // Serialize to XML
    pub fn to_xml(&self) -> xmltree::Element {
        let name = "testcase".to_string();
        let mut attributes = indexmap::map::IndexMap::new();
        attributes.insert("id".to_string(), self.id.clone());
        let time_in_seconds = format!("{:.3}", self.time_in_ms as f64 / 1000.0);
        attributes.insert("time".to_string(), time_in_seconds);

        let mut children = vec![];
        for message in self.failures.clone() {
            let element = Element {
                prefix: None,
                namespace: None,
                namespaces: None,
                name: "failure".to_string(),
                attributes: indexmap::map::IndexMap::new(),
                children: vec![XMLNode::Text(message)],
            };
            children.push(XMLNode::Element(element));
        }
        for message in self.errors.clone() {
            let element = Element {
                prefix: None,
                namespace: None,
                namespaces: None,
                name: "error".to_string(),
                attributes: indexmap::map::IndexMap::new(),
                children: vec![XMLNode::Text(message)],
            };
            children.push(XMLNode::Element(element));
        }
        Element {
            name,
            prefix: None,
            namespace: None,
            namespaces: None,
            attributes,
            children,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::report::junit::testcase::Testcase;
    use crate::runner::{EntryResult, Error, HurlResult, RunnerError};
    use hurl_core::ast::SourceInfo;

    #[test]
    fn test_create_testcase_success() {
        let hurl_result = HurlResult {
            filename: "test.hurl".to_string(),
            entries: vec![],
            time_in_ms: 230,
            success: true,
            cookies: vec![],
        };

        let mut buffer = Vec::new();
        let content = "";
        Testcase::from_hurl_result(&hurl_result, &content)
            .to_xml()
            .write(&mut buffer)
            .unwrap();
        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" time="0.230" />"#
        );
    }

    #[test]
    fn test_create_testcase_failure() {
        let content = r#"GET http://localhost:8000/not_found
HTTP/1.0 200
"#;
        let hurl_result = HurlResult {
            filename: "test.hurl".to_string(),
            entries: vec![EntryResult {
                request: None,
                response: None,
                captures: vec![],
                asserts: vec![],
                errors: vec![Error {
                    source_info: SourceInfo::init(2, 10, 2, 13),
                    inner: RunnerError::AssertStatus {
                        actual: "404".to_string(),
                    },
                    assert: true,
                }],
                time_in_ms: 0,
            }],
            time_in_ms: 230,
            success: true,
            cookies: vec![],
        };
        let mut buffer = Vec::new();
        Testcase::from_hurl_result(&hurl_result, &content)
            .to_xml()
            .write(&mut buffer)
            .unwrap();
        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" time="0.230"><failure>Assert status code
  --> test.hurl:2:10
   |
 2 | HTTP/1.0 200
   |          ^^^ actual value is &lt;404>
   |</failure></testcase>"#
        );
    }

    #[test]
    fn test_create_testcase_error() {
        let content = "GET http://unknown";
        let hurl_result = HurlResult {
            filename: "test.hurl".to_string(),
            entries: vec![EntryResult {
                request: None,
                response: None,
                captures: vec![],
                asserts: vec![],
                errors: vec![Error {
                    source_info: SourceInfo::init(1, 5, 1, 19),
                    inner: RunnerError::HttpConnection {
                        url: "http://unknown".to_string(),
                        message: "(6) Could not resolve host: unknown".to_string(),
                    },
                    assert: false,
                }],
                time_in_ms: 0,
            }],
            time_in_ms: 230,
            success: true,
            cookies: vec![],
        };
        let mut buffer = Vec::new();
        Testcase::from_hurl_result(&hurl_result, content)
            .to_xml()
            .write(&mut buffer)
            .unwrap();
        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" time="0.230"><error>HTTP connection
  --> test.hurl:1:5
   |
 1 | GET http://unknown
   |     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown
   |</error></testcase>"#
        );
    }
}
