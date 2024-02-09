/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
use hurl_core::ast::JsonValue;
use hurl_core::ast::{Pos, SourceInfo};

use crate::http::{Call, Cookie};
use crate::runner::error::Error;
use crate::runner::output::Output;
use crate::runner::value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlResult {
    pub entries: Vec<EntryResult>,
    pub time_in_ms: u128,
    pub success: bool,
    pub cookies: Vec<Cookie>,
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
                        errors.extend(new_errors)
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
    JsonBody {
        actual: Result<JsonValue, Error>,
        expected: Result<JsonValue, Error>,
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
    /// Writes the last HTTP response of this entry result to the file `filename`.
    /// The HTTP response can be decompressed if the entry's `compressed` option has been set.
    pub fn write_response(&self, output: &Output) -> Result<(), Error> {
        match self.calls.last() {
            Some(call) => {
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
                    output.write(&bytes)
                } else {
                    output.write(&response.body)
                }
            }
            None => Ok(()),
        }
    }
}
