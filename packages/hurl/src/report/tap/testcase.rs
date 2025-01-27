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
use hurl_core::input::Input;

use crate::report::ReportError;
use crate::runner::HurlResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Testcase {
    pub(crate) description: String,
    pub(crate) success: bool,
}

impl Testcase {
    /// Creates an Tap &lt;testcase&gt; from an [`HurlResult`].
    pub fn from(hurl_result: &HurlResult, filename: &Input) -> Testcase {
        let description = filename.to_string();
        let success = hurl_result.errors().is_empty();
        Testcase {
            description,
            success,
        }
    }

    /// Creates an Tap &lt;testcase&gt; from a TAP line
    /// ok 1 - this is the first test
    /// nok 2 - this is the second test
    pub fn parse(line: &str) -> Result<Testcase, ReportError> {
        let mut line = line;
        let success = if line.starts_with("ok") {
            line = &line[2..];
            true
        } else if line.starts_with("not ok") {
            line = &line[6..];
            false
        } else {
            return Err(ReportError::from_string(&format!(
                "Invalid TAP line <{line}> - must start with ok or nok"
            )));
        };

        let description = match line.find('-') {
            None => {
                return Err(ReportError::from_string(&format!(
                    "Invalid TAP line <{line}> - missing '-' separator"
                )));
            }
            Some(index) => {
                if line.split_at(index).0.trim().parse::<usize>().is_err() {
                    return Err(ReportError::from_string(&format!(
                        "Invalid TAP line <{line}> - missing test number"
                    )));
                }
                line.split_at(index).1[1..].trim().to_string()
            }
        };
        Ok(Testcase {
            description,
            success,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tap_test_line() {
        assert_eq!(
            Testcase::parse("toto").err().unwrap().to_string(),
            "Invalid TAP line <toto> - must start with ok or nok".to_string()
        );

        assert_eq!(
            Testcase::parse("ok 1 - tests_ok/test.1.hurl").unwrap(),
            Testcase {
                description: "tests_ok/test.1.hurl".to_string(),
                success: true
            }
        );
    }
}
