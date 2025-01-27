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
use hurl_core::error::{DisplaySourceError, OutputFormat};
use hurl_core::input::Input;

use crate::report::junit::xml::Element;
use crate::runner::HurlResult;
use crate::util::redacted::Redact;

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
    pub fn from(hurl_result: &HurlResult, content: &str, filename: &Input) -> Testcase {
        let id = filename.to_string();
        let name = filename.to_string();
        let time_in_ms = hurl_result.duration.as_millis();
        let mut failures = vec![];
        let mut errors = vec![];

        for (error, entry_src_info) in hurl_result.errors() {
            let message = error.to_string(
                &name,
                content,
                Some(entry_src_info),
                OutputFormat::Terminal(false),
            );
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
    pub fn to_xml(&self, secrets: &[&str]) -> Element {
        let time_in_seconds = format!("{:.3}", self.time_in_ms as f64 / 1000.0);

        let mut element = Element::new("testcase")
            .attr("id", &self.id)
            .attr("name", &self.name)
            .attr("time", &time_in_seconds);

        for failure in self.failures.iter() {
            let failure = failure.redact(secrets);
            element = element.add_child(Element::new("failure").text(&failure));
        }

        for error in self.errors.iter() {
            let error = error.redact(secrets);
            element = element.add_child(Element::new("error").text(&error));
        }
        element
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
    use std::time::Duration;

    use hurl_core::ast::SourceInfo;
    use hurl_core::input::Input;
    use hurl_core::reader::Pos;

    use crate::http::HttpError;
    use crate::report::junit::testcase::Testcase;
    use crate::report::junit::xml::XmlDocument;
    use crate::runner::{EntryResult, HurlResult, RunnerError, RunnerErrorKind};

    #[test]
    fn test_create_testcase_success() {
        let hurl_result = HurlResult {
            duration: Duration::from_millis(230),
            success: true,
            ..Default::default()
        };

        let content = "";
        let secrets = [];
        let filename = Input::new("test.hurl");
        let element = Testcase::from(&hurl_result, content, &filename).to_xml(&secrets);
        let doc = XmlDocument::new(element);
        assert_eq!(
            doc.to_string().unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" name="test.hurl" time="0.230" />"#
        );
    }

    #[test]
    fn test_create_testcase_failure() {
        let content = r#"GET http://localhost:8000/not_found
HTTP/1.0 200
"#;
        let filename = Input::new("test.hurl");
        let secrets = [];
        let hurl_result = HurlResult {
            entries: vec![EntryResult {
                entry_index: 1,
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 35)),
                errors: vec![RunnerError::new(
                    SourceInfo::new(Pos::new(2, 10), Pos::new(2, 13)),
                    RunnerErrorKind::AssertStatus {
                        actual: "404".to_string(),
                    },
                    true,
                )],
                ..Default::default()
            }],
            duration: Duration::from_millis(230),
            success: false,
            ..Default::default()
        };

        let element = Testcase::from(&hurl_result, content, &filename).to_xml(&secrets);
        let doc = XmlDocument::new(element);
        assert_eq!(
            doc.to_string().unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" name="test.hurl" time="0.230"><failure>Assert status code
  --&gt; test.hurl:2:10
   |
   | GET http://localhost:8000/not_found
 2 | HTTP/1.0 200
   |          ^^^ actual value is &lt;404&gt;
   |</failure></testcase>"#
        );
    }

    #[test]
    fn test_create_testcase_error() {
        let content = "GET http://unknown";
        let filename = Input::new("test.hurl");
        let secrets = ["unknown"];
        let hurl_result = HurlResult {
            entries: vec![EntryResult {
                entry_index: 1,
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 18)),
                errors: vec![RunnerError::new(
                    SourceInfo::new(Pos::new(1, 5), Pos::new(1, 19)),
                    RunnerErrorKind::Http(HttpError::Libcurl {
                        code: 6,
                        description: "Could not resolve host: unknown".to_string(),
                    }),
                    false,
                )],
                ..Default::default()
            }],
            duration: Duration::from_millis(230),
            success: false,
            ..Default::default()
        };
        let element = Testcase::from(&hurl_result, content, &filename).to_xml(&secrets);
        let doc = XmlDocument::new(element);
        assert_eq!(
            doc.to_string().unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><testcase id="test.hurl" name="test.hurl" time="0.230"><error>HTTP connection
  --&gt; test.hurl:1:5
   |
 1 | GET http://***
   |     ^^^^^^^^^^^^^^ (6) Could not resolve host: ***
   |</error></testcase>"#
        );
    }
}
