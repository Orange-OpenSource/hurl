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

use std::fs::File;

use xmltree::{Element, XMLNode};

pub use testcase::Testcase;

use crate::cli::CliError;

mod testcase;

pub fn create_report(filename: String, testcases: Vec<Testcase>) -> Result<(), CliError> {
    let mut testsuites = vec![];

    let path = std::path::Path::new(&filename);
    if path.exists() {
        let s = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(why) => {
                return Err(CliError {
                    message: format!("Issue reading {} to string to {:?}", path.display(), why),
                });
            }
        };
        let root = Element::parse(s.as_bytes()).unwrap();
        for child in root.children {
            if let XMLNode::Element(_) = child.clone() {
                testsuites.push(child.clone());
            }
        }
    }

    let testsuite = create_testsuite(testcases);
    testsuites.push(testsuite);
    let report = Element {
        name: "testsuites".to_string(),
        prefix: None,
        namespace: None,
        namespaces: None,
        attributes: indexmap::map::IndexMap::new(),
        children: testsuites,
    };
    let file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(CliError {
                message: format!("Failed to produce junit report: {:?}", e),
            });
        }
    };
    match report.write(file) {
        Ok(_) => Ok(()),
        Err(e) => Err(CliError {
            message: format!("Failed to produce junit report: {:?}", e),
        }),
    }
}

fn create_testsuite(testcases: Vec<Testcase>) -> XMLNode {
    let children = testcases
        .iter()
        .map(|t| XMLNode::Element(t.to_xml()))
        .collect();
    let element = Element {
        name: "testsuite".to_string(),
        prefix: None,
        namespace: None,
        namespaces: None,
        attributes: indexmap::map::IndexMap::new(),
        children,
    };
    XMLNode::Element(element)
}
