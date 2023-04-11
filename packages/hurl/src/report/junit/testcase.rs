/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use xmltree::{Element, XMLNode};

use crate::runner::HurlResult;
use crate::util::logger;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Testcase {
    id: String,
    name: String,
    time_in_ms: u128,
    failures: Vec<String>,
    errors: Vec<String>,
}

impl Testcase {
    /// Creates an XML Junit &lt;testcase&gt; from an [`HurlResult`].
    pub fn from(hurl_result: &HurlResult, content: &str, filename: &str) -> Testcase {
        let id = filename.to_string();
        let name = filename.to_string();
        let time_in_ms = hurl_result.time_in_ms;
        let mut failures = vec![];
        let mut errors = vec![];

        for error in hurl_result.errors() {
            let message = logger::error_string(filename, content, error, false);
            if error.assert {
                failures.push(message);
            } else {
                errors.push(message);
            };
        }
        Testcase {
            id,
            name,
            time_in_ms,
            failures,
            errors,
        }
    }

    /// Serializes this testcase to XML.
    pub fn to_xml(&self) -> Element {
        let name = "testcase".to_string();
        let mut attributes = indexmap::map::IndexMap::new();
        attributes.insert("id".to_string(), self.id.clone());
        attributes.insert("name".to_string(), self.name.clone());
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

    pub fn get_error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn get_fail_count(&self) -> usize {
        self.failures.len()
    }
}

#[cfg(test)]
mod test {
    use hurl_core::ast::SourceInfo;

    use crate::report::junit::testcase::Testcase;
    use crate::runner::{EntryResult, Error, HurlResult, RunnerError};

    #[test]
    fn test_create_testcase_success() {
        let hurl_result = HurlResult {
            entries: vec![],
            time_in_ms: 230,
            success: true,
            cookies: vec![],
        };

        let mut buffer = Vec::new();
        let content = "";
        let filename = "test.hurl";
        Testcase::from(&hurl_result, content, filename)
            .to_xml()
            .write(&mut buffer)
            .unwrap();
        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" name="test.hurl" time="0.230" />"#
        );
    }

    #[test]
    fn test_create_testcase_failure() {
        let content = r#"GET http://localhost:8000/not_found
HTTP/1.0 200
"#;
        let filename = "test.hurl";
        let hurl_result = HurlResult {
            entries: vec![EntryResult {
                entry_index: 1,
                calls: vec![],
                captures: vec![],
                asserts: vec![],
                errors: vec![Error {
                    source_info: SourceInfo::new(2, 10, 2, 13),
                    inner: RunnerError::AssertStatus {
                        actual: "404".to_string(),
                    },
                    assert: true,
                }],
                time_in_ms: 0,
                compressed: false,
            }],
            time_in_ms: 230,
            success: true,
            cookies: vec![],
        };
        let mut buffer = Vec::new();
        Testcase::from(&hurl_result, content, filename)
            .to_xml()
            .write(&mut buffer)
            .unwrap();
        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" name="test.hurl" time="0.230"><failure>Assert status code
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
        let filename = "test.hurl";
        let hurl_result = HurlResult {
            entries: vec![EntryResult {
                entry_index: 1,
                calls: vec![],
                captures: vec![],
                asserts: vec![],
                errors: vec![Error {
                    source_info: SourceInfo::new(1, 5, 1, 19),
                    inner: RunnerError::HttpConnection {
                        url: "http://unknown".to_string(),
                        message: "(6) Could not resolve host: unknown".to_string(),
                    },
                    assert: false,
                }],
                time_in_ms: 0,
                compressed: false,
            }],
            time_in_ms: 230,
            success: true,
            cookies: vec![],
        };
        let mut buffer = Vec::new();
        Testcase::from(&hurl_result, content, filename)
            .to_xml()
            .write(&mut buffer)
            .unwrap();
        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" name="test.hurl" time="0.230"><error>HTTP connection
  --> test.hurl:1:5
   |
 1 | GET http://unknown
   |     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown
   |</error></testcase>"#
        );
    }
}
