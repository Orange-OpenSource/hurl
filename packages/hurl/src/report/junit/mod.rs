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

//
// XML JUnit report
//
// It does not really seem to be used nowadays
// The link bellow seems the most "official" spec
// https://www.ibm.com/docs/fr/developer-for-zos/9.1.1?topic=formats-junit-xml-format
//
// One Hurl file will result into one junit <testcase>.
// The <testcase> can include <error> (for Runtime error) or <failure> (for Assert Error)
// Each hurl execution will generate its own <testsuite> within the root <testsuites>.
//
// Example:
//
// $ cat test.xml | xmllint --format -
// <?xml version="1.0"?>
// <testsuites>
//   <testsuite>
//     <testcase id="tests/hello.hurl" time="0.029"/>
//     <testcase id="tests/error_assert_status.hurl" time="0.008">
//       <failure>Assert Status
//   --&gt; tests/error_assert_status.hurl:2:10
//    |
//  2 | HTTP/1.0 200
//    |          ^^^ actual value is &lt;404&gt;
//    |</failure>
//     </testcase>
//     <testcase id="tests/error_body_json.hurl" time="0.000">
//       <error>Undefined Variable
//   --&gt; tests/error_body_json.hurl:3:18
//    |
//  3 |     "success": {{success}}
//    |                  ^^^^^^^ You must set the variable success
//    |</error>
//     </testcase>
//   </testsuite>
// </testsuites>
//

use crate::{cli::CliError, runner::HurlResult};
use libxml::parser::Parser;
use libxml::tree::{Document, Node};

mod result;

type JunitError = String;

///
/// Get XML document
/// The document is created if it does not exist
///
pub fn create_or_get_junit_report(
    file_path: Option<std::path::PathBuf>,
) -> Result<Document, CliError> {
    if let Some(file_path) = file_path {
        if file_path.exists() {
            let parser = Parser::default();
            let doc = parser
                .parse_string(
                    std::fs::read_to_string(file_path.clone()).map_err(|e| CliError {
                        message: format!("Failed to read file {:?}: {:?}", file_path, e),
                    })?,
                )
                .map_err(|e| CliError {
                    message: format!("Failed to parse file {:?}: {:?}", file_path, e),
                })?;
            Ok(doc)
        } else {
            let mut doc = Document::new().map_err(|e| CliError {
                message: format!("Failed to produce junit report: {:?}", e),
            })?;
            let testsuites =
                Node::new("testsuites", None, &doc).expect("Could not create testsuites node");
            doc.set_root_element(&testsuites);
            Ok(doc)
        }
    } else {
        let mut doc = Document::new().map_err(|e| CliError {
            message: format!("Failed to produce junit report: {:?}", e),
        })?;
        let testsuites =
            Node::new("testsuites", None, &doc).expect("Could not create testsuites node");
        doc.set_root_element(&testsuites);
        Ok(doc)
    }
}

///
/// Add testsuite to XML document
///
pub fn add_testsuite(doc: &libxml::tree::Document) -> Result<libxml::tree::Node, JunitError> {
    let mut testsuite = match Node::new("testsuite", None, doc) {
        Ok(v) => v,
        Err(_) => return Err("can not create node testsuite".to_string()),
    };

    let mut testsuites = match doc.get_root_element() {
        Some(v) => v,
        None => return Err("can not get root element".to_string()),
    };
    match testsuites.add_child(&mut testsuite) {
        Ok(_) => {}
        Err(_) => return Err("can not add child".to_string()),
    }
    Ok(testsuite)
}

///
/// Add Testcase in the testsuite the XML document
///
pub fn add_testcase(
    doc: &Document,
    testsuite: &mut Node,
    hurl_result: HurlResult,
    lines: &[String],
) -> Result<(), CliError> {
    let mut testcase = hurl_result.to_testcase(doc, lines)?;
    if testsuite.add_child(&mut testcase).is_err() {
        return Err(CliError {
            message: "can not add child".to_string(),
        });
    }
    Ok(())
}
