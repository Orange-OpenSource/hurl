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
use std::path::PathBuf;
use std::time::Duration;

use crate::http;
use crate::http::ContextDir;
use hurl_core::ast::{Entry, SourceInfo};

use super::value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerOptions {
    pub cacert_file: Option<String>,
    pub compressed: bool,
    pub connect_timeout: Duration,
    pub context_dir: ContextDir,
    pub cookie_input_file: Option<String>,
    pub fail_fast: bool,
    pub follow_location: bool,
    pub ignore_asserts: bool,
    pub insecure: bool,
    pub max_redirect: Option<usize>,
    pub no_proxy: Option<String>,
    pub post_entry: Option<fn() -> bool>,
    pub pre_entry: Option<fn(Entry) -> bool>,
    pub proxy: Option<String>,
    pub retry: bool,
    pub retry_interval: Duration,
    pub timeout: Duration,
    pub to_entry: Option<usize>,
    pub user: Option<String>,
    pub user_agent: Option<String>,
    pub verbosity: Option<Verbosity>,
    pub very_verbose: bool, // If true, log body response in verbose mode.
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verbosity {
    Verbose,
    VeryVerbose,
}

impl Default for RunnerOptions {
    fn default() -> Self {
        RunnerOptions {
            cacert_file: None,
            compressed: false,
            connect_timeout: Duration::from_secs(300),
            context_dir: Default::default(),
            cookie_input_file: None,
            fail_fast: false,
            follow_location: false,
            ignore_asserts: false,
            insecure: false,
            max_redirect: Some(50),
            no_proxy: None,
            post_entry: None,
            pre_entry: None,
            proxy: None,
            retry: false,
            retry_interval: Duration::from_millis(1000),
            timeout: Duration::from_secs(300),
            to_entry: None,
            user: None,
            user_agent: None,
            verbosity: None,
            very_verbose: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlResult {
    pub filename: String,
    pub entries: Vec<EntryResult>,
    pub time_in_ms: u128,
    pub success: bool,
    pub cookies: Vec<http::Cookie>,
}

impl HurlResult {
    pub fn errors(&self) -> Vec<&Error> {
        let mut errors = vec![];
        let mut next_entries = self.entries.iter().skip(1);
        for entry in self.entries.iter() {
            match next_entries.next() {
                None => errors.extend(&entry.errors),
                Some(next) => {
                    if next.entry_index != entry.entry_index {
                        errors.extend(&entry.errors)
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
    pub calls: Vec<Call>,
    pub captures: Vec<CaptureResult>,
    pub asserts: Vec<AssertResult>,
    pub errors: Vec<Error>,
    pub time_in_ms: u128,
    pub compressed: bool, // The entry has been executed with `--compressed` option
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub request: http::Request,
    pub response: http::Response,
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

// endregion

// region error

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub source_info: SourceInfo,
    pub inner: RunnerError,
    pub assert: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunnerError {
    TemplateVariableNotDefined {
        name: String,
    },
    VariableNotDefined {
        name: String,
    },
    InvalidJson {
        value: String,
    },
    InvalidUrl(String),

    HttpConnection {
        url: String,
        message: String,
    },
    CouldNotResolveProxyName,
    CouldNotResolveHost(String),
    FailToConnect,
    Timeout,
    TooManyRedirect,
    CouldNotParseResponse,
    SslCertificate(String),

    UnsupportedContentEncoding(String),
    CouldNotUncompressResponse(String),

    FileReadAccess {
        value: String,
    },
    InvalidDecoding {
        charset: String,
    },
    InvalidCharset {
        charset: String,
    },

    // Query
    QueryHeaderNotFound,
    QueryCookieNotFound,
    QueryInvalidJsonpathExpression {
        value: String,
    },
    QueryInvalidXpathEval,
    QueryInvalidXml,
    QueryInvalidJson,
    NoQueryResult,

    SubqueryInvalidInput(String),

    // Predicate
    PredicateType,
    PredicateValue(Value),
    AssertFailure {
        actual: String,
        expected: String,
        type_mismatch: bool,
    },
    InvalidRegex(),

    AssertHeaderValueError {
        actual: String,
    },
    AssertBodyValueError {
        actual: String,
        expected: String,
    },
    AssertVersion {
        actual: String,
    },
    AssertStatus {
        actual: String,
    },

    UnrenderableVariable {
        value: String,
    },

    UnauthorizedFileAccess {
        path: PathBuf,
    },
}

// endregion
