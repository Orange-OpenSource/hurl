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

use std::fs::File;
use std::io::Write;
use std::path::Path;

use regex::Regex;

use super::Testcase;
use crate::report::ReportError;
use crate::util::path::create_dir_all;

/// See <https://testanything.org/tap-version-13-specification.html>
const TAP_REPORT_VERSION_MARKER: &str = "TAP version 13";

/// Creates/Append a Tap report from a list of `testcases`
pub fn write_report(filename: &Path, testcases: &[Testcase]) -> Result<(), ReportError> {
    let mut all_testcases = vec![];

    let existing_testcases = parse_tap_file(filename)?;
    for testcase in existing_testcases.iter() {
        all_testcases.push(testcase);
    }
    for testcase in testcases {
        all_testcases.push(testcase);
    }
    write_tap_file(filename, &all_testcases)
}

/// Creates a Tap from a list of `testcases`.
fn write_tap_file(filename: &Path, testcases: &[&Testcase]) -> Result<(), ReportError> {
    if let Err(err) = create_dir_all(filename) {
        return Err(ReportError::from_error(
            err,
            filename,
            "Issue writing TAP report",
        ));
    }
    let mut file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(ReportError::from_error(
                e,
                filename,
                "Issue writing TAP report",
            ))
        }
    };
    let start = 1;
    let end = testcases.len();

    let mut s = format!("{}\n", TAP_REPORT_VERSION_MARKER);
    s.push_str(format!("{start}..{end}\n").as_str());

    for (i, testcase) in testcases.iter().enumerate() {
        let state = if testcase.success { "ok" } else { "not ok" };
        let number = i + 1;
        let description = &testcase.description;
        s.push_str(format!("{state} {number} - {description}\n").as_str());
    }
    match file.write_all(s.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(ReportError::from_error(
            e,
            filename,
            "Issue writing TAP report",
        )),
    }
}

/// Parse Tap report file
fn parse_tap_file(filename: &Path) -> Result<Vec<Testcase>, ReportError> {
    if !filename.exists() {
        return Ok(vec![]);
    }
    let s = match std::fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            return Err(ReportError::from_error(
                e,
                filename,
                "Issue reading TAP report",
            ))
        }
    };
    parse_tap_report(&s)
}

/// Parse Tap report
fn parse_tap_report(s: &str) -> Result<Vec<Testcase>, ReportError> {
    let mut testcases = vec![];
    let mut lines: Vec<&str> = s.lines().collect::<Vec<&str>>();
    if !lines.is_empty() {
        let mut header = lines.remove(0);
        // A tap report may have a protocol version header as per TAP
        if header.eq_ignore_ascii_case(TAP_REPORT_VERSION_MARKER) {
            header = lines.remove(0);
        }
        let re = Regex::new(r"^1\.\.\d+.*$").unwrap();
        if !re.is_match(header) {
            return Err(ReportError::from_string(&format!(
                "Invalid TAP Header <{header}>"
            )));
        }
        for line in lines {
            let line = line.trim();
            if !line.is_empty() {
                let testcase = Testcase::parse(line)?;
                testcases.push(testcase);
            }
        }
    }
    Ok(testcases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tap_report() {
        let s = r#"1..3
ok 1 - tests_ok/test.1.hurl
 ok 2  -tests_ok/test.2.hurl
not ok 3 - tests_ok/test.3.hurl

"#;
        assert_eq!(
            parse_tap_report(s).unwrap(),
            vec![
                Testcase {
                    description: "tests_ok/test.1.hurl".to_string(),
                    success: true
                },
                Testcase {
                    description: "tests_ok/test.2.hurl".to_string(),
                    success: true
                },
                Testcase {
                    description: "tests_ok/test.3.hurl".to_string(),
                    success: false
                }
            ]
        );
    }

    #[test]
    fn test_parse_tap_report_with_version() {
        let s = r#"TAP version 13
1..3
ok 1 - tests_ok/test.1.hurl
 ok 2  -tests_ok/test.2.hurl
not ok 3 - tests_ok/test.3.hurl

"#;
        assert_eq!(
            parse_tap_report(s).unwrap(),
            vec![
                Testcase {
                    description: "tests_ok/test.1.hurl".to_string(),
                    success: true
                },
                Testcase {
                    description: "tests_ok/test.2.hurl".to_string(),
                    success: true
                },
                Testcase {
                    description: "tests_ok/test.3.hurl".to_string(),
                    success: false
                }
            ]
        );

        let s = r#"TAP version 13
1..5 # TAP header can have comments
ok 1 - test.1.hurl
ok 2 - test.2.hurl
not ok 3 - test.3.hurl
not ok 4 - test.4.hurl
ok 5 - test.5.hurl
"#;
        assert_eq!(
            parse_tap_report(s).unwrap(),
            vec![
                Testcase {
                    description: "test.1.hurl".to_string(),
                    success: true
                },
                Testcase {
                    description: "test.2.hurl".to_string(),
                    success: true
                },
                Testcase {
                    description: "test.3.hurl".to_string(),
                    success: false
                },
                Testcase {
                    description: "test.4.hurl".to_string(),
                    success: false
                },
                Testcase {
                    description: "test.5.hurl".to_string(),
                    success: true
                }
            ]
        );
    }

    #[test]
    fn test_parse_error() {
        let s = r#"Dummy header
ok 1 - test.1.hurl
ok 2 - test.2.hurl
not ok 3 - test.3.hurl
"#;
        assert!(parse_tap_report(s).is_err());
    }
}
