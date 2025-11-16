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

//! Compliance Test Suite
//! https://github.com/jsonpath-standard/jsonpath-compliance-test-suite

use std::fmt::{Display, Formatter};

use hurl_core::reader::Pos;
use serde_json::json;

use crate::jsonpath2::{
    self,
    eval::NodeList,
    parser::{ParseError, ParseErrorKind},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestCase {
    name: String,
    document: Option<serde_json::Value>,
    selector: String,
    invalid_selector: bool,
    results: Vec<NodeList>, // contains several results when the order is non-deterministic
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestCaseError {
    testcase: TestCase,
    kind: TestCaseErrorKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TestCaseErrorKind {
    UnexpectedParseError(ParseError),
    InvalidSelector,
    Eval(NodeList),
}

impl TestCase {
    #[allow(clippy::result_large_err)]
    pub fn run(&self) -> Result<(), TestCaseError> {
        let query = match jsonpath2::parse(&self.selector) {
            Ok(value) => {
                if self.invalid_selector {
                    return Err(TestCaseError {
                        testcase: self.clone(),
                        kind: TestCaseErrorKind::InvalidSelector,
                    });
                } else {
                    value
                }
            }
            Err(parse_error) => {
                if self.invalid_selector {
                    return Ok(());
                } else {
                    return Err(TestCaseError {
                        testcase: self.clone(),
                        kind: TestCaseErrorKind::UnexpectedParseError(parse_error),
                    });
                }
            }
        };
        let actual_result = query.eval(&self.document.clone().unwrap());
        for result in &self.results {
            if *result == actual_result {
                return Ok(());
            }
        }
        Err(TestCaseError {
            testcase: self.clone(),
            kind: TestCaseErrorKind::Eval(actual_result),
        })
    }
}

impl Display for TestCaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = format!(">>> {}", self.testcase.name.clone());
        match &self.kind {
            TestCaseErrorKind::UnexpectedParseError(_parse_error) => {
                s.push_str(&format!(
                    "\ncan not parse valid selector <{}>",
                    self.testcase.selector
                ));
            }
            TestCaseErrorKind::InvalidSelector => {
                s.push_str(&format!(
                    "\nshould not parse the invalid selector <{}>",
                    self.testcase.selector
                ));
            }
            TestCaseErrorKind::Eval(values) => {
                s.push_str(&format!(
                    "\ndocument: {}",
                    serde_json::to_string(&self.testcase.document.clone().unwrap()).unwrap()
                ));
                s.push_str(&format!("\nselector: {}", self.testcase.selector));

                if self.testcase.results.len() == 1 {
                    let expected =
                        serde_json::to_string_pretty(&self.testcase.results.first().unwrap())
                            .unwrap();
                    s.push_str(&format!("\nexpected: {expected}"));
                } else {
                    s.push_str("\nexpected &one of these):");
                    for result in &self.testcase.results {
                        let expected = serde_json::to_string_pretty(result).unwrap();
                        s.push_str(&format!("\n  {expected}"));
                    }
                }
                let actual = serde_json::to_string_pretty(values).unwrap();
                s.push_str(&format!("\nactual: {actual}"));
            }
        }
        write!(f, "{s}",)
    }
}

#[test]
fn run_testcase() {
    // UnexpectedParseError
    let testcase = TestCase {
        name: "valid selector that fails".to_string(),
        document: None,
        selector: "xxx".to_string(),
        invalid_selector: false,
        results: vec![],
    };
    let testcase_error = testcase.run().unwrap_err();
    assert_eq!(
        testcase_error,
        TestCaseError {
            testcase,
            kind: TestCaseErrorKind::UnexpectedParseError(ParseError::new(
                Pos::new(1, 1),
                ParseErrorKind::Expecting("$".to_string())
            ))
        }
    );
    assert_eq!(
        testcase_error.to_string(),
        ">>> valid selector that fails\ncan not parse valid selector <xxx>".to_string()
    );

    // InvalidSelector
    let testcase = TestCase {
        name: "invalid selector that succeeds".to_string(),
        document: None,
        selector: "$".to_string(),
        invalid_selector: true,
        results: vec![],
    };
    let testcase_error = testcase.run().unwrap_err();
    assert_eq!(
        testcase_error,
        TestCaseError {
            testcase,
            kind: TestCaseErrorKind::InvalidSelector
        }
    );
    assert_eq!(
        testcase_error.to_string(),
        ">>> invalid selector that succeeds\nshould not parse the invalid selector <$>".to_string()
    );

    // EvalError
    let testcase = TestCase {
        name: "valid eval that fails".to_string(),
        document: Some(json!({"name": "Bob"})),
        selector: "$.name".to_string(),
        invalid_selector: false,
        results: vec![vec![json!("Bill")]],
    };
    let testcase_error = testcase.run().unwrap_err();
    assert_eq!(
        testcase_error,
        TestCaseError {
            testcase,
            kind: TestCaseErrorKind::Eval(vec![json!("Bob")])
        }
    );
    assert_eq!(
        testcase_error.to_string(),
        ">>> valid eval that fails\ndocument: {\"name\":\"Bob\"}\nselector: $.name\nexpected: [\n  \"Bill\"\n]\nactual: [\n  \"Bob\"\n]".to_string()
    );

    // Eval OK
    let test_case = TestCase {
        name: "valid eval that succeeed".to_string(),
        document: Some(json!({"name": "Bob"})),
        selector: "$.name".to_string(),
        invalid_selector: false,
        results: vec![vec![json!("Bob")]],
    };
    assert!(test_case.run().is_ok());
}

fn get_results(test: &serde_json::Value) -> Vec<NodeList> {
    if let Some(results) = test.get("results") {
        let values = results.as_array().unwrap();
        values
            .iter()
            .map(|v| v.as_array().unwrap())
            .cloned()
            .collect::<Vec<NodeList>>()
    } else if let Some(result) = test.get("result") {
        vec![result.as_array().unwrap().to_owned()]
    } else {
        vec![]
    }
}

fn parse_testcase(test: &serde_json::Value) -> TestCase {
    let name = test.get("name").unwrap().as_str().unwrap().to_owned();
    let selector = test.get("selector").unwrap().as_str().unwrap().to_owned();
    let invalid_selector = test.get("invalid_selector").is_some();
    let document = test.get("document").cloned();
    let results = get_results(test);
    TestCase {
        name,
        document,
        selector,
        invalid_selector,
        results,
    }
}

// double vec
fn load_testcases() -> Vec<TestCase> {
    let content = include_str!("cts.json");
    let data: serde_json::Value = serde_json::from_str(content).unwrap();
    let tests = data.get("tests").unwrap().as_array().unwrap();
    let mut testcases = vec![];
    for test in tests {
        let testcase = parse_testcase(test);
        testcases.push(testcase);
    }
    testcases
}

#[test]
fn run() {
    let testcases = load_testcases();
    // TODO: Remove Limit when spec is fully implemented
    let testcases = testcases.iter().take(106);
    let count_total = testcases.len();

    let errors = testcases
        .map(|test_case| test_case.run())
        .collect::<Vec<Result<(), TestCaseError>>>()
        .iter()
        .filter_map(|test_case| test_case.clone().err())
        .collect::<Vec<TestCaseError>>();

    if !errors.is_empty() {
        let count_failed = errors.len();
        let count_passed = count_total - count_failed;
        let mut s = String::new();
        for error in &errors {
            s.push_str(&error.to_string());
            s.push_str("\n\n");
        }
        s.push_str("RFC9535 Compliance tests:\n");
        s.push_str(format!("Total:  {count_total}\n").as_str());
        s.push_str(format!("Passed: {count_passed}\n").as_str());
        s.push_str(format!("Failed: {count_failed}\n").as_str());
        panic!("{}", s);
    }
}
