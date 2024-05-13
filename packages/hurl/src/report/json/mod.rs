/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use crate::runner::{HurlResult, Input};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

/// Exports a list of [`Testcase`] to a JSON file `filename_out`.
///
/// Response file are saved under the `response_dir` directory and referenced by path in JSON report
/// file.
pub fn write_report(
    filename_out: &Path,
    testcases: &[Testcase],
    response_dir: &Path,
) -> Result<(), io::Error> {
    let json: Result<Vec<_>, _> = testcases.iter().map(|t| t.to_json(response_dir)).collect();
    let json = json?;
    let serialized = serde_json::to_string(&json).unwrap();
    let bytes = format!("{serialized}\n");
    let bytes = bytes.into_bytes();
    let mut file_out = File::create(filename_out)?;
    file_out.write_all(&bytes)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Testcase<'a> {
    result: &'a HurlResult,
    content: &'a str,
    filename: &'a Input,
}

impl<'a> Testcase<'a> {
    /// Creates a new `Testcase`.
    pub fn new(hurl_result: &'a HurlResult, content: &'a str, filename: &'a Input) -> Self {
        Testcase {
            result: hurl_result,
            content,
            filename,
        }
    }

    /// Serializes this testcase to JSON.
    fn to_json(&self, response_dir: &Path) -> Result<serde_json::Value, io::Error> {
        self.result
            .to_json(self.content, self.filename, Some(response_dir))
    }
}
