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
use std::path::PathBuf;

use hurl_core::ast::SourceInfo;

use crate::http::{HttpError, RequestedHttpVersion};

/// Represents a single instance of a runtime error, usually triggered by running a
/// [`hurl_core::ast::Entry`]. Running a Hurl content (see [`crate::runner::run`]) returns a list of
/// result for each entry. Each entry result can contain a list of [`Error`]. The runtime error variant
/// is defined in [`RunnerError`]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub source_info: SourceInfo,
    pub inner: RunnerError,
    pub assert: bool,
}

impl Error {
    pub fn new(source_info: SourceInfo, inner: RunnerError, assert: bool) -> Error {
        Error {
            source_info,
            inner,
            assert,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunnerError {
    AssertBodyValueError {
        actual: String,
        expected: String,
    },
    AssertFailure {
        actual: String,
        expected: String,
        type_mismatch: bool,
    },
    AssertHeaderValueError {
        actual: String,
    },
    AssertStatus {
        actual: String,
    },
    AssertVersion {
        actual: String,
    },
    CouldNotParseResponse,
    CouldNotUncompressResponse(String),
    FileReadAccess {
        file: String,
    },
    // I/O write error on a path
    FileWriteAccess {
        file: String,
        error: String,
    },
    FilterDecode(String),
    FilterInvalidEncoding(String),
    FilterInvalidInput(String),
    FilterMissingInput,
    HttpConnection(String),
    InvalidJson {
        value: String,
    },
    InvalidCharset {
        charset: String,
    },
    InvalidDecoding {
        charset: String,
    },
    InvalidRegex,
    InvalidUrl(String),
    InvalidUrlPrefix(String),
    NoQueryResult,
    QueryHeaderNotFound,
    QueryInvalidJsonpathExpression {
        value: String,
    },
    QueryInvalidXpathEval,
    QueryInvalidXml,
    QueryInvalidJson,
    TemplateVariableNotDefined {
        name: String,
    },
    TemplateVariableInvalidType {
        name: String,
        value: String,
        expecting: String,
    },
    TooManyRedirect,
    UnsupportedContentEncoding(String),
    UnsupportedHttpVersion(RequestedHttpVersion),
    UnrenderableVariable {
        name: String,
        value: String,
    },
    UnauthorizedFileAccess {
        path: PathBuf,
    },
}

/// Textual Output for runner errors
impl hurl_core::error::Error for Error {
    fn source_info(&self) -> SourceInfo {
        self.source_info
    }

    fn description(&self) -> String {
        match &self.inner {
            RunnerError::AssertBodyValueError { .. } => "Assert body value".to_string(),
            RunnerError::AssertFailure { .. } => "Assert failure".to_string(),
            RunnerError::AssertHeaderValueError { .. } => "Assert header value".to_string(),
            RunnerError::AssertStatus { .. } => "Assert status code".to_string(),
            RunnerError::AssertVersion { .. } => "Assert HTTP version".to_string(),
            RunnerError::CouldNotParseResponse => "HTTP connection".to_string(),
            RunnerError::CouldNotUncompressResponse(..) => "Decompression error".to_string(),
            RunnerError::FileReadAccess { .. } => "File read access".to_string(),
            RunnerError::FileWriteAccess { .. } => "File write access".to_string(),
            RunnerError::FilterDecode { .. } => "Filter Error".to_string(),
            RunnerError::FilterInvalidEncoding { .. } => "Filter Error".to_string(),
            RunnerError::FilterInvalidInput { .. } => "Filter Error".to_string(),
            RunnerError::FilterMissingInput => "Filter Error".to_string(),
            RunnerError::HttpConnection { .. } => "HTTP connection".to_string(),
            RunnerError::InvalidCharset { .. } => "Invalid charset".to_string(),
            RunnerError::InvalidDecoding { .. } => "Invalid decoding".to_string(),
            RunnerError::InvalidJson { .. } => "Invalid JSON".to_string(),
            RunnerError::InvalidRegex => "Invalid regex".to_string(),
            RunnerError::InvalidUrl(..) => "Invalid URL".to_string(),
            RunnerError::InvalidUrlPrefix(..) => "Invalid URL".to_string(),
            RunnerError::NoQueryResult => "No query result".to_string(),
            RunnerError::QueryHeaderNotFound => "Header not found".to_string(),
            RunnerError::QueryInvalidJson => "Invalid JSON".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { .. } => "Invalid JSONPath".to_string(),
            RunnerError::QueryInvalidXml => "Invalid XML".to_string(),
            RunnerError::QueryInvalidXpathEval => "Invalid XPath expression".to_string(),
            RunnerError::TemplateVariableInvalidType { .. } => "Invalid variable type".to_string(),
            RunnerError::TemplateVariableNotDefined { .. } => "Undefined variable".to_string(),
            RunnerError::TooManyRedirect => "HTTP connection".to_string(),
            RunnerError::UnauthorizedFileAccess { .. } => "Unauthorized file access".to_string(),
            RunnerError::UnrenderableVariable { .. } => "Unrenderable variable".to_string(),
            RunnerError::UnsupportedContentEncoding(..) => "Decompression error".to_string(),
            RunnerError::UnsupportedHttpVersion(..) => "Unsupported HTTP version".to_string(),
        }
    }

    fn fixme(&self) -> String {
        match &self.inner {
            RunnerError::AssertBodyValueError { actual, .. } => {
                format!("actual value is <{actual}>")
            }
            RunnerError::AssertFailure {
                actual,
                expected,
                type_mismatch,
                ..
            } => {
                let additional = if *type_mismatch {
                    "\n>>> types between actual and expected are not consistent"
                } else {
                    ""
                };
                format!("actual:   {actual}\nexpected: {expected}{additional}")
            }
            RunnerError::AssertHeaderValueError { actual } => {
                format!("actual value is <{actual}>")
            }
            RunnerError::AssertStatus { actual, .. } => format!("actual value is <{actual}>"),
            RunnerError::AssertVersion { actual, .. } => format!("actual value is <{actual}>"),
            RunnerError::CouldNotParseResponse => "could not parse response".to_string(),
            RunnerError::CouldNotUncompressResponse(algorithm) => {
                format!("could not uncompress response with {algorithm}")
            }
            RunnerError::FileReadAccess { file } => format!("file {file} can not be read"),
            RunnerError::FileWriteAccess { file, error } => {
                format!("{file} can not be written ({error})")
            }
            RunnerError::FilterDecode(encoding) => {
                format!("value can not be decoded with <{encoding}> encoding")
            }
            RunnerError::FilterInvalidEncoding(encoding) => {
                format!("<{encoding}> encoding is not supported")
            }
            RunnerError::FilterInvalidInput(message) => {
                format!("invalid filter input: {message}")
            }
            RunnerError::FilterMissingInput => "missing value to apply filter".to_string(),
            RunnerError::HttpConnection(message) => message.to_string(),
            RunnerError::InvalidCharset { charset } => {
                format!("the charset '{charset}' is not valid")
            }
            RunnerError::InvalidDecoding { charset } => {
                format!("the body can not be decoded with charset '{charset}'")
            }
            RunnerError::InvalidJson { value } => {
                format!("actual value is <{value}>")
            }
            RunnerError::InvalidRegex => "regex expression is not valid".to_string(),
            RunnerError::InvalidUrl(url) => format!("invalid URL <{url}>"),
            RunnerError::InvalidUrlPrefix(url) => {
                format!("URL <{url}> must start with http:// or https://")
            }
            RunnerError::NoQueryResult => "The query didn't return any result".to_string(),
            RunnerError::QueryHeaderNotFound => {
                "this header has not been found in the response".to_string()
            }
            RunnerError::QueryInvalidJson => "the HTTP response is not a valid JSON".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { value } => {
                format!("the JSONPath expression '{value}' is not valid")
            }
            RunnerError::QueryInvalidXml => "the HTTP response is not a valid XML".to_string(),
            RunnerError::QueryInvalidXpathEval => "the XPath expression is not valid".to_string(),
            RunnerError::TemplateVariableInvalidType {
                value, expecting, ..
            } => {
                format!("expecting {expecting}, actual value is <{value}>")
            }
            RunnerError::TemplateVariableNotDefined { name } => {
                format!("you must set the variable {name}")
            }
            RunnerError::TooManyRedirect => "too many redirect".to_string(),
            RunnerError::UnauthorizedFileAccess { path } => {
                format!(
                    "unauthorized access to file {}, check --file-root option",
                    path.to_str().unwrap()
                )
            }
            RunnerError::UnrenderableVariable { name, value } => {
                format!("variable <{name}> with value {value} can not be rendered")
            }
            RunnerError::UnsupportedContentEncoding(algorithm) => {
                format!("compression {algorithm} is not supported")
            }
            RunnerError::UnsupportedHttpVersion(version) => {
                format!("{version} is not supported, check --version")
            }
        }
    }
}

impl From<HttpError> for RunnerError {
    /// Converts a HttpError to a RunnerError.
    fn from(item: HttpError) -> Self {
        match item {
            HttpError::CouldNotParseResponse => RunnerError::CouldNotParseResponse,
            HttpError::CouldNotUncompressResponse { description } => {
                RunnerError::CouldNotUncompressResponse(description)
            }
            HttpError::InvalidCharset { charset } => RunnerError::InvalidCharset { charset },
            HttpError::InvalidDecoding { charset } => RunnerError::InvalidDecoding { charset },
            HttpError::InvalidUrl(url) => RunnerError::InvalidUrl(url),
            HttpError::InvalidUrlPrefix(url) => RunnerError::InvalidUrlPrefix(url),
            HttpError::Libcurl { code, description } => {
                RunnerError::HttpConnection(format!("({code}) {description}"))
            }
            HttpError::LibcurlUnknownOption {
                option,
                minimum_version,
            } => RunnerError::HttpConnection(format!(
                "Option {option} requires libcurl version {minimum_version} or higher"
            )),
            HttpError::StatuslineIsMissing => {
                RunnerError::HttpConnection("status line is missing".to_string())
            }
            HttpError::TooManyRedirect => RunnerError::TooManyRedirect,
            HttpError::UnsupportedContentEncoding { description } => {
                RunnerError::UnsupportedContentEncoding(description)
            }
            HttpError::UnsupportedHttpVersion(version) => {
                RunnerError::UnsupportedHttpVersion(version)
            }
        }
    }
}
