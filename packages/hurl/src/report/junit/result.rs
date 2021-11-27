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

use super::JunitError;
use crate::cli;
use crate::runner::{EntryResult, Error, HurlResult};
use libxml::tree::{Document, Node};

impl HurlResult {
    ///
    /// Export Hurl result to an XML Junit <testcase>
    ///
    pub fn to_testcase(
        &self,
        doc: &Document,
        lines: &[String],
    ) -> Result<libxml::tree::Node, JunitError> {
        let mut node = Node::new("testcase", None, doc).expect("XML Node");
        node.set_attribute("id", &self.filename)
            .expect("Set attribute id");
        let time_in_seconds = format!("{:.3}", self.time_in_ms as f64 / 1000.0);
        node.set_attribute("time", &time_in_seconds)
            .expect("Set attribute duration");

        for entry in self.entries.clone() {
            for mut error in entry.errors(doc, lines, self.filename.clone())? {
                if node.add_child(&mut error).is_err() {
                    return Err("can not add child".to_string());
                }
            }
        }
        Ok(node)
    }
}

impl EntryResult {
    ///
    /// Export entry result errors to XML Junit <error>/<failure>
    ///
    fn errors(
        &self,
        doc: &Document,
        lines: &[String],
        filename: String,
    ) -> Result<Vec<Node>, JunitError> {
        let mut errors = vec![];
        for error in self.errors.clone() {
            let error = error.to_junit_error(doc, lines, filename.clone())?;
            errors.push(error);
        }
        Ok(errors)
    }
}

impl Error {
    ///
    /// Export Hurl runner error to an XML Junit <error>/<failure>
    ///
    fn to_junit_error(
        &self,
        doc: &Document,
        lines: &[String],
        filename: String,
    ) -> Result<Node, JunitError> {
        let node_name = if self.assert { "failure" } else { "error" };
        let mut node = if let Ok(value) = Node::new(node_name, None, doc) {
            value
        } else {
            return Err("Can not create node".to_string());
        };
        let message = cli::error_string(lines, filename, self);
        if node.append_text(message.as_str()).is_err() {
            return Err("Can not append text".to_string());
        }
        Ok(node)
    }
}

#[cfg(test)]
mod test {
    use crate::runner::{EntryResult, Error, HurlResult, RunnerError};
    use hurl_core::ast::SourceInfo;
    use libxml::tree::Document;

    #[test]
    fn test_create_testcase_success() {
        let doc = Document::new().unwrap();
        let lines = vec![];
        let hurl_result = HurlResult {
            filename: "test.hurl".to_string(),
            entries: vec![],
            time_in_ms: 230,
            success: true,
            cookies: vec![],
        };
        assert_eq!(
            doc.node_to_string(&hurl_result.to_testcase(&doc, &lines).unwrap()),
            r#"<testcase id="test.hurl" time="0.230"/>"#
        );
    }

    #[test]
    fn test_create_testcase_failure() {
        let lines = vec![
            "GET http://localhost:8000/not_found".to_string(),
            "HTTP/1.0 200".to_string(),
        ];
        let doc = Document::new().unwrap();
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
        assert_eq!(
            doc.node_to_string(&hurl_result.to_testcase(&doc, &lines).unwrap()),
            r#"<testcase id="test.hurl" time="0.230"><failure>Assert Status
  --&gt; test.hurl:2:10
   |
 2 | HTTP/1.0 200
   |          ^^^ actual value is &lt;404&gt;
   |</failure></testcase>"#
        );
    }

    #[test]
    fn test_create_testcase_error() {
        let lines = vec!["GET http://unknown".to_string()];
        let doc = Document::new().unwrap();
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
        assert_eq!(
            doc.node_to_string(&hurl_result.to_testcase(&doc, &lines).unwrap()),
            r#"<testcase id="test.hurl" time="0.230"><error>Http Connection
  --&gt; test.hurl:1:5
   |
 1 | GET http://unknown
   |     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown
   |</error></testcase>"#
        );
    }
}
