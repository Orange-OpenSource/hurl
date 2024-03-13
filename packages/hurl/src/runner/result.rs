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
use hurl_core::ast::{Pos, SourceInfo};

use crate::http::{Call, Cookie};
use crate::runner::error::Error;
use crate::runner::output::Output;
use crate::runner::value::Value;
use crate::runner::RunnerError;
use crate::util::path::ContextDir;
use crate::util::term::Stdout;

/// Represents the result of a valid Hurl file execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlResult {
    /// The entries result for this run.
    pub entries: Vec<EntryResult>,
    /// Duration in milliseconds of the run.
    pub time_in_ms: u128,
    /// `true` if the run is successful, `false` if there has been runtime or asserts errors.
    pub success: bool,
    /// The list of cookies at the end of the run.
    pub cookies: Vec<Cookie>,
    /// Start of the run (in "UNIX timestamp").
    pub timestamp: i64,
}

impl HurlResult {
    /// Returns all the effective errors of this `HurlResult`, with the source information
    /// of the entry where the error happens.
    ///
    /// The errors are only the "effective" ones: those that are due to retry are
    /// ignored.
    pub fn errors(&self) -> Vec<(&Error, SourceInfo)> {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryResult {
    pub entry_index: usize,
    pub source_info: SourceInfo,
    pub calls: Vec<Call>,
    pub captures: Vec<CaptureResult>,
    pub asserts: Vec<AssertResult>,
    pub errors: Vec<Error>,
    pub time_in_ms: u128,
    // The entry has been executed with `--compressed` option:
    // server is requested to send compressed response, and the response should be uncompressed
    // when outputted on stdout.
    pub compressed: bool,
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
            time_in_ms: 0,
            compressed: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssertResult {
    Version {
        actual: String,
        expected: String,
        source_info: SourceInfo,
    },
    Status {
        actual: u64,
        expected: u64,
        source_info: SourceInfo,
    },
    Header {
        actual: Result<String, Error>,
        expected: String,
        source_info: SourceInfo,
    },
    Body {
        actual: Result<Value, Error>,
        expected: Result<Value, Error>,
        source_info: SourceInfo,
    },
    Explicit {
        actual: Result<Option<Value>, Error>,
        source_info: SourceInfo,
        predicate_result: Option<PredicateResult>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureResult {
    pub name: String,
    pub value: Value,
}

pub type PredicateResult = Result<(), Error>;

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
    ) -> Result<(), Error> {
        let Some(call) = self.calls.last() else {
            return Ok(());
        };
        // We check file access authorization for file output when a context dir has been given
        if let Output::File(filename) = output {
            if !context_dir.is_access_allowed(&filename.to_string_lossy()) {
                let inner = RunnerError::UnauthorizedFileAccess {
                    path: filename.clone(),
                };
                return Err(Error::new(source_info, inner, false));
            }
        }
        let response = &call.response;
        if self.compressed {
            let bytes = match response.uncompress_body() {
                Ok(bytes) => bytes,
                Err(e) => {
                    // TODO: pass a [`SourceInfo`] in case of error
                    // We may pass a [`SourceInfo`] as a parameter of this method to make
                    // a more accurate error (for instance a [`SourceInfo`] pointing at
                    // `output: foo.bin`
                    let source_info = SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0));
                    return Err(Error::new(source_info, e.into(), false));
                }
            };
            output.write(&bytes, stdout, Some(context_dir))
        } else {
            output.write(&response.body, stdout, Some(context_dir))
        }
    }
}
