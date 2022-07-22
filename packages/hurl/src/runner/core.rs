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
use std::collections::HashMap;
use std::path::PathBuf;

use crate::http;
use crate::http::ContextDir;
use hurl_core::ast::{Entry, SourceInfo};

use super::value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerOptions {
    pub fail_fast: bool,
    pub variables: HashMap<String, Value>,
    pub to_entry: Option<usize>,
    pub context_dir: ContextDir,
    pub ignore_asserts: bool,
    pub very_verbose: bool, // If true, log body response in verbose mode.
    pub pre_entry: fn(Entry) -> bool,
    pub post_entry: fn() -> bool,
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
    pub fn errors(&self) -> Vec<Error> {
        self.entries.iter().flat_map(|e| e.errors.clone()).collect()
    }

    pub fn success(&self) -> bool {
        self.errors().is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryResult {
    pub request: Option<http::Request>,
    pub response: Option<http::Response>,
    //pub captures: Vec<(String, Value)>,
    pub captures: Vec<CaptureResult>,
    pub asserts: Vec<AssertResult>,
    pub errors: Vec<Error>,
    pub time_in_ms: u128,
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
