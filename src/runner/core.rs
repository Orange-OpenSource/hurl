/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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

use serde::{Deserialize, Serialize};

use crate::core::common::{FormatError, SourceInfo, Value};
use crate::http::libcurl;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerOptions {
    pub fail_fast: bool,
    pub variables: HashMap<String, String>,
    pub to_entry: Option<usize>,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HurlResult {
    pub filename: String,
    pub entries: Vec<EntryResult>,
    pub time_in_ms: u128,
    pub success: bool,
    pub cookies: Vec<libcurl::core::Cookie>,
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
    pub request: Option<libcurl::core::Request>,
    pub response: Option<libcurl::core::Response>,
    //pub captures: Vec<(String, Value)>,
    pub captures: Vec<CaptureResult>,
    pub asserts: Vec<AssertResult>,
    pub errors: Vec<Error>,
    pub time_in_ms: u128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssertResult {
    Version { actual: String, expected: String, source_info: SourceInfo },
    Status { actual: u64, expected: u64, source_info: SourceInfo },
    Header { actual: Result<String, Error>, expected: String, source_info: SourceInfo },
    Body { actual: Result<Value, Error>, expected: Result<Value, Error>, source_info: SourceInfo },
    Explicit { actual: Result<Option<Value>, Error>, source_info: SourceInfo, predicate_result: Option<PredicateResult> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureResult {
    pub name: String,
    pub value: Value,
}

pub type PredicateResult = Result<(), Error>;

// endregion


// region error

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Error {
    pub source_info: SourceInfo,
    pub inner: RunnerError,
    pub assert: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunnerError {
    TemplateVariableNotDefined { name: String },
    VariableNotDefined { name: String },
    InvalidURL(String),
    HttpConnection { url: String, message: String },
    FileReadAccess { value: String },
    InvalidDecoding { charset: String },
    InvalidCharset { charset: String },

    // Query
    QueryHeaderNotFound,
    QueryCookieNotFound,
    QueryInvalidJsonpathExpression { value: String },
    QueryInvalidXpathEval,
    QueryInvalidXml,
    QueryInvalidJson,
    NoQueryResult,

    SubqueryInvalidInput,

    // Predicate
    PredicateType,
    PredicateValue(Value),
    AssertFailure { actual: String, expected: String, type_mismatch: bool },
    InvalidRegex(),

    AssertHeaderValueError { actual: String },
    AssertBodyValueError { actual: String, expected: String },
    AssertVersion { actual: String },
    AssertStatus { actual: String },

    UnrenderableVariable { value: String },

}


impl FormatError for Error {
    fn source_info(&self) -> SourceInfo {
        self.clone().source_info
    }

    fn description(&self) -> String {
        match &self.inner {
            RunnerError::InvalidURL(..) => "Invalid url".to_string(),
            RunnerError::TemplateVariableNotDefined { .. } => "Undefined Variable".to_string(),
            RunnerError::VariableNotDefined { .. } => "Undefined Variable".to_string(),
            RunnerError::HttpConnection { .. } => "Http Connection".to_string(),
            RunnerError::PredicateValue { .. } => "Assert - Predicate Value Failed".to_string(),
            RunnerError::InvalidRegex {} => "Invalid regex".to_string(),
            RunnerError::FileReadAccess { .. } => "File ReadAccess".to_string(),
            RunnerError::QueryInvalidXml { .. } => "Invalid XML".to_string(),
            RunnerError::QueryInvalidXpathEval {} => "Invalid xpath expression".to_string(),
            RunnerError::QueryHeaderNotFound {} => "Header not Found".to_string(),
            RunnerError::QueryCookieNotFound {} => "Cookie not Found".to_string(),
            RunnerError::AssertHeaderValueError { .. } => "Assert Header Value".to_string(),
            RunnerError::AssertBodyValueError { .. } => "Assert Body Value".to_string(),
            RunnerError::AssertVersion { .. } => "Assert Http Version".to_string(),
            RunnerError::AssertStatus { .. } => "Assert Status".to_string(),
            RunnerError::QueryInvalidJson { .. } => "Invalid Json".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { .. } => "Invalid jsonpath".to_string(),
            RunnerError::PredicateType { .. } => "Assert - Inconsistent predicate type".to_string(),
            RunnerError::SubqueryInvalidInput { .. } => "Subquery error".to_string(),
            RunnerError::InvalidDecoding { ..} => "Invalid Decoding".to_string(),
            RunnerError::InvalidCharset { ..} =>  "Invalid Charset".to_string(),
            RunnerError::AssertFailure { .. } => "Assert Failure".to_string(),
            RunnerError::UnrenderableVariable { .. } => "Unrenderable Variable".to_string(),
            RunnerError::NoQueryResult { .. } => "No query result".to_string(),

        }
    }

    fn fixme(&self) -> String {
        match &self.inner {
            RunnerError::InvalidURL(url) => format!("Invalid url <{}>", url),
            RunnerError::TemplateVariableNotDefined { name } => format!("You must set the variable {}", name),
            RunnerError::HttpConnection { url, .. } => format!("can not connect to {}", url),
            RunnerError::AssertVersion { actual, .. } => format!("actual value is <{}>", actual),
            RunnerError::AssertStatus { actual, .. } => format!("actual value is <{}>", actual),
            RunnerError::PredicateValue(value) => format!("actual value is <{}>", value.to_string()),
            RunnerError::InvalidRegex {} => "Regex expression is not valid".to_string(),
            RunnerError::FileReadAccess { value } => format!("File {} can not be read", value),
            RunnerError::QueryInvalidXml { .. } => "The Http response is not a valid XML".to_string(),
            RunnerError::QueryHeaderNotFound {} => "This header has not been found in the response".to_string(),
            RunnerError::QueryCookieNotFound {} => "This cookie has not been found in the response".to_string(),
            RunnerError::QueryInvalidXpathEval {} => "The xpath expression is not valid".to_string(),
            RunnerError::AssertHeaderValueError { actual } => format!("actual value is <{}>", actual),
            RunnerError::AssertBodyValueError { actual, .. } => format!("actual value is <{}>", actual),
            RunnerError::QueryInvalidJson { .. } => "The http response is not a valid json".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { value } => format!("the jsonpath expression '{}' is not valid", value),
            RunnerError::PredicateType { .. } => "predicate type inconsistent with value return by query".to_string(),
            RunnerError::SubqueryInvalidInput => "Type from query result and subquery do not match".to_string(),
            RunnerError::InvalidDecoding { charset } => format!("The body can not be decoded with charset '{}'", charset),
            RunnerError::InvalidCharset { charset } => format!("The charset '{}' is not valid", charset),
            RunnerError::AssertFailure { actual, expected, .. } => format!("actual:   {}\nexpected: {}", actual, expected),
            RunnerError::VariableNotDefined { name } => format!("You must set the variable {}", name),
            RunnerError::UnrenderableVariable { value } => format!("value {} can not be rendered", value),
            RunnerError::NoQueryResult { .. } => "The query didn't return any result".to_string(),
        }
    }
}

// endregion

