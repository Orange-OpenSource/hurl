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
use std::time::Duration;

use hurl_core::ast::SourceInfo;
use hurl_core::reader::Pos;

use crate::http::{Call, Cookie, CurlCmd};
use crate::runner::error::RunnerError;
use crate::runner::output::Output;
use crate::runner::value::Value;
use crate::runner::{RunnerErrorKind, VariableSet};
use crate::util::path::ContextDir;
use crate::util::term::Stdout;

/// Represents the result of a valid Hurl file execution.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HurlResult {
    /// The entries result for this run.
    pub entries: Vec<EntryResult>,
    /// Total duration of the run, including asserts and results computation.
    pub duration: Duration,
    /// `true` if the run is successful, `false` if there has been runtime or asserts errors.
    pub success: bool,
    /// The list of cookies at the end of the run.
    pub cookies: Vec<Cookie>,
    /// Start of the run (in "UNIX timestamp").
    pub timestamp: i64,
    /// The set of variables, updated at the end of the run execution.
    pub variables: VariableSet,
}

impl HurlResult {
    /// Returns all the effective errors of this `HurlResult`, with the source information
    /// of the entry where the error happens.
    ///
    /// The errors are only the "effective" ones: those that are due to retry are
    /// ignored.
    pub fn errors(&self) -> Vec<(&RunnerError, SourceInfo)> {
        let mut errors = vec![];
        let mut next_entries = self.entries.iter().skip(1);
        for entry in self.entries.iter() {
            match next_entries.next() {
                None => {
                    let new_errors = entry.errors.iter().map(|error| (error, entry.source_info));
                    errors.extend(new_errors);
                }
                Some(next) => {
                    if next.entry_index != entry.entry_index {
                        let new_errors =
                            entry.errors.iter().map(|error| (error, entry.source_info));
                        errors.extend(new_errors);
                    }
                }
            }
        }
        errors
    }
}

/// Represents the execution result of an entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryResult {
    /// 1-based index of the entry on the file execution.
    pub entry_index: usize,
    /// Source information of this entry.
    pub source_info: SourceInfo,
    /// List of HTTP request / response pair.
    pub calls: Vec<Call>,
    /// List of captures.
    pub captures: Vec<CaptureResult>,
    /// List of asserts.
    pub asserts: Vec<AssertResult>,
    /// List of errors.
    pub errors: Vec<RunnerError>,

    /// Effective duration of all the HTTP transfers, excluding asserts and captures processing.
    pub transfer_duration: Duration,
    /// The entry has been executed with `--compressed` option:
    /// server is requested to send compressed response, and the response should be uncompressed
    /// when outputted on stdout.
    pub compressed: bool,
    /// The debug curl command line from this entry result.
    pub curl_cmd: CurlCmd,
}

impl Default for EntryResult {
    fn default() -> Self {
        EntryResult {
            entry_index: 1,
            source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
            calls: vec![],
            captures: vec![],
            asserts: vec![],
            errors: vec![],
            transfer_duration: Duration::from_millis(0),
            compressed: false,
            curl_cmd: CurlCmd::default(),
        }
    }
}

/// Result of a Hurl assertions, either implicit or explicit.
///
/// ## Example
///
/// ```hurl
///  GET https://foo.com
///  HTTP 200
///  x-baz: bar
///  [Asserts]
///  header "toto" == "tutu"
///  jsonpath "$.state" = "running"
/// ```
///
/// In this Hurl sample, everything after the keyword `HTTP` is an assertion. We distinguish two
/// types of assertions: implicit and explicit.
///
/// - `HTTP 200`: implicit status code assert
/// - `x-baz: bar`: implicit HTTP header assert
/// - `header "toto" == "tutu"`: explicit HTTP header assert
/// - `jsonpath "$.state" = "running"`: explicit JSONPath assert on HTTP body response
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssertResult {
    /// Implicit HTTP version assert (like HTTP/3, HTTP/2 etc...).
    Version {
        actual: String,
        expected: String,
        source_info: SourceInfo,
    },
    /// Implicit HTTP status code assert.
    Status {
        actual: u64,
        expected: u64,
        source_info: SourceInfo,
    },
    /// Implicit HTTP response header assert.
    Header {
        actual: Result<String, RunnerError>,
        expected: String,
        source_info: SourceInfo,
    },
    /// Implicit HTTP response body assert.
    Body {
        actual: Result<Value, RunnerError>,
        expected: Result<Value, RunnerError>,
        source_info: SourceInfo,
    },
    /// Explicit assert on HTTP response.
    Explicit {
        actual: Result<Option<Value>, RunnerError>,
        source_info: SourceInfo,
        predicate_result: Option<PredicateResult>,
    },
}

/// Represents a [capture](https://hurl.dev/docs/capturing-response.html) of an HTTP response.
///
/// Captures are data extracted by querying the HTTP response. Captures can be part of the response
/// body, headers, cookies etc... Captures can be used to re-inject data in next HTTP requests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureResult {
    /// Name of the capture.
    pub name: String,
    /// Value of the capture.
    pub value: Value,
}

pub type PredicateResult = Result<(), RunnerError>;

impl EntryResult {
    /// Writes the last HTTP response of this entry result to this `output`.
    /// The HTTP response can be decompressed if the entry's `compressed` option has been set.
    /// This method checks if the response has write access to this output, given a `context_dir`.
    pub fn write_response(
        &self,
        output: &Output,
        context_dir: &ContextDir,
        stdout: &mut Stdout,
        source_info: SourceInfo,
    ) -> Result<(), RunnerError> {
        let Some(call) = self.calls.last() else {
            return Ok(());
        };
        let response = &call.response;
        if self.compressed {
            let bytes = match response.uncompress_body() {
                Ok(bytes) => bytes,
                Err(e) => {
                    return Err(RunnerError::new(
                        source_info,
                        RunnerErrorKind::Http(e),
                        false,
                    ));
                }
            };
            output.write_with_context_dir(&bytes, stdout, context_dir, source_info)
        } else {
            output.write_with_context_dir(&response.body, stdout, context_dir, source_info)
        }
    }
}
