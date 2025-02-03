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

//! XML JUnit report.
//!
//! The link below seems the most "official" spec
//! <https://www.ibm.com/docs/fr/developer-for-zos/9.1.1?topic=formats-junit-xml-format>
//!
//! One Hurl file will result into one JUnit `<testcase>`.
//!
//! The `<testcase>` can include `<error>` (for runtime error) or `<failure>` (for assert error)
//! Each Hurl execution will generate its own `<testsuite>` within the root `<testsuites>`.
//!
//! # Example:
//!
//! ```shell
//! $ cat test.xml | xmllint --format -
//! <?xml version="1.0"?>
//! <testsuites>
//!   <testsuite>
//!     <testcase id="tests/hello.hurl" name="tests/hello.hurl" time="0.029"/>
//!     <testcase id="tests/error_assert_status.hurl" name="tests/error_assert_status.hurl" time="0.008">
//!       <failure>Assert Status
//!   --> tests/error_assert_status.hurl:2:10
//!    |
//!  2 | HTTP/1.0 200
//!    |          ^^^ actual value is <404>
//!    |</failure>
//!     </testcase>
//!     <testcase id="tests/error_body_json.hurl" time="0.000">
//!       <error>Undefined Variable
//!   --> tests/error_body_json.hurl:3:18
//!    |
//!  3 |     "success": {{success}}
//!    |                  ^^^^^^^ You must set the variable success
//!    |</error>
//!     </testcase>
//!   </testsuite>
//! </testsuites>
//! ```
//!
mod testcase;
mod xml;
use std::fs::File;
use std::path::Path;

pub use testcase::Testcase;

use crate::report::junit::xml::{Element, XmlDocument};
use crate::report::ReportError;
use crate::util::path::create_dir_all;

/// Creates a JUnit from a list of `testcases`.
///
///  `secrets` strings are redacted from messages (mainly assert error messages).
pub fn write_report(
    filename: &Path,
    testcases: &[Testcase],
    secrets: &[&str],
) -> Result<(), ReportError> {
    if let Err(err) = create_dir_all(filename) {
        return Err(ReportError::from_error(
            err,
            filename,
            "Issue writing JUnit report",
        ));
    }

    // If there is an existing JUnit report, we parse it to insert a new testsuite.
    let mut root = if filename.exists() {
        let file = match File::open(filename) {
            Ok(s) => s,
            Err(e) => {
                return Err(ReportError::from_error(
                    e,
                    filename,
                    "Issue reading JUnit report",
                ))
            }
        };
        let doc = XmlDocument::parse(file).unwrap();
        doc.root.unwrap()
    } else {
        Element::new("testsuites")
    };

    let testsuite = create_testsuite(testcases, secrets);
    root = root.add_child(testsuite);

    let doc = XmlDocument::new(root);
    let file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(ReportError::from_error(
                e,
                filename,
                "Issue writing JUnit report",
            ))
        }
    };
    match doc.write(file) {
        Ok(_) => Ok(()),
        Err(e) => Err(ReportError::from_string(&format!(
            "Failed to produce Junit report: {e:?}"
        ))),
    }
}

/// Returns a testsuite as a XML object, from a list of `testcases`.
///
/// `secrets` strings are redacted from the produced HTML.
fn create_testsuite(testcases: &[Testcase], secrets: &[&str]) -> Element {
    let mut tests = 0;
    let mut errors = 0;
    let mut failures = 0;

    for cases in testcases.iter() {
        tests += 1;
        errors += cases.get_error_count();
        failures += cases.get_fail_count();
    }

    let mut element = Element::new("testsuite")
        .attr("tests", &tests.to_string())
        .attr("errors", &errors.to_string())
        .attr("failures", &failures.to_string());

    for testcase in testcases.iter() {
        let child = testcase.to_xml(secrets);
        element = element.add_child(child);
    }
    element
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use hurl_core::ast::SourceInfo;
    use hurl_core::input::Input;
    use hurl_core::reader::Pos;

    use crate::http::HttpError;
    use crate::report::junit::xml::XmlDocument;
    use crate::report::junit::{create_testsuite, Testcase};
    use crate::runner::{EntryResult, HurlResult, RunnerError, RunnerErrorKind};

    #[test]
    fn create_junit_report() {
        let content = "GET http://localhost:8000/not_found\n\
                       HTTP/1.0 200";
        let filename = Input::new("test.hurl");
        let secrets = [];
        let mut testcases = vec![];
        let res = HurlResult {
            duration: Duration::from_millis(124),
            success: true,
            ..Default::default()
        };
        let tc = Testcase::from(&res, content, &filename);
        testcases.push(tc);

        let res = HurlResult {
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
            duration: Duration::from_millis(200),
            success: true,
            ..Default::default()
        };
        let tc = Testcase::from(&res, content, &filename);
        testcases.push(tc);

        let res = HurlResult {
            entries: vec![EntryResult {
                entry_index: 1,
                source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 35)),
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
            success: true,
            ..Default::default()
        };
        let tc = Testcase::from(&res, content, &filename);
        testcases.push(tc);

        let suite = create_testsuite(&testcases, &secrets);
        let doc = XmlDocument::new(suite);
        assert_eq!(
            doc.to_string().unwrap(),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <testsuite tests=\"3\" errors=\"1\" failures=\"1\">\
                <testcase id=\"test.hurl\" name=\"test.hurl\" time=\"0.124\" />\
                <testcase id=\"test.hurl\" name=\"test.hurl\" time=\"0.200\">\
                    <failure>Assert status code\n  \
                    --&gt; test.hurl:2:10\n   \
                      |\n   \
                      | GET http://localhost:8000/not_found\n \
                    2 | HTTP/1.0 200\n   \
                      |          ^^^ actual value is &lt;404&gt;\n   \
                      |\
                    </failure>\
                </testcase>\
                <testcase id=\"test.hurl\" name=\"test.hurl\" time=\"0.230\">\
                    <error>HTTP connection\n  --&gt; test.hurl:1:5\n   |\n 1 | GET http://localhost:8000/not_found\n   |     ^^^^^^^^^^^^^^ (6) Could not resolve host: unknown\n   |\
                    </error>\
                </testcase>\
            </testsuite>"
        );
    }
}
