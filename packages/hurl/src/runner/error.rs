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
use colored::Colorize;
use std::path::PathBuf;

use hurl_core::ast::SourceInfo;
use hurl_core::error::DisplaySourceError;

use crate::http::{HttpError, RequestedHttpVersion};

/// Represents a single instance of a runtime error, usually triggered by running a
/// [`hurl_core::ast::Entry`]. Running a Hurl content (see [`crate::runner::run`]) returns a list of
/// result for each entry. Each entry result can contain a list of [`RunnerError`]. The runtime error variant
/// is defined in [`RunnerErrorKind`]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerError {
    pub source_info: SourceInfo,
    pub kind: RunnerErrorKind,
    pub assert: bool,
}

impl RunnerError {
    pub fn new(source_info: SourceInfo, kind: RunnerErrorKind, assert: bool) -> RunnerError {
        RunnerError {
            source_info,
            kind,
            assert,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunnerErrorKind {
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
    CouldNotUncompressResponse(String),
    /// I/O read error on `path`.
    FileReadAccess {
        path: PathBuf,
    },
    /// I/O write error on `path`.
    FileWriteAccess {
        path: PathBuf,
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
    InvalidUrl(String, String),
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
    /// Unauthorized file access, check `--file-root` option.
    UnauthorizedFileAccess {
        path: PathBuf,
    },
}

/// Textual Output for runner errors
impl DisplaySourceError for RunnerError {
    fn source_info(&self) -> SourceInfo {
        self.source_info
    }

    fn description(&self) -> String {
        match &self.kind {
            RunnerErrorKind::AssertBodyValueError { .. } => "Assert body value".to_string(),
            RunnerErrorKind::AssertFailure { .. } => "Assert failure".to_string(),
            RunnerErrorKind::AssertHeaderValueError { .. } => "Assert header value".to_string(),
            RunnerErrorKind::AssertStatus { .. } => "Assert status code".to_string(),
            RunnerErrorKind::AssertVersion { .. } => "Assert HTTP version".to_string(),
            RunnerErrorKind::CouldNotUncompressResponse(..) => "Decompression error".to_string(),
            RunnerErrorKind::FileReadAccess { .. } => "File read access".to_string(),
            RunnerErrorKind::FileWriteAccess { .. } => "File write access".to_string(),
            RunnerErrorKind::FilterDecode { .. } => "Filter error".to_string(),
            RunnerErrorKind::FilterInvalidEncoding { .. } => "Filter error".to_string(),
            RunnerErrorKind::FilterInvalidInput { .. } => "Filter error".to_string(),
            RunnerErrorKind::FilterMissingInput => "Filter error".to_string(),
            RunnerErrorKind::HttpConnection { .. } => "HTTP connection".to_string(),
            RunnerErrorKind::InvalidCharset { .. } => "Invalid charset".to_string(),
            RunnerErrorKind::InvalidDecoding { .. } => "Invalid decoding".to_string(),
            RunnerErrorKind::InvalidJson { .. } => "Invalid JSON".to_string(),
            RunnerErrorKind::InvalidRegex => "Invalid regex".to_string(),
            RunnerErrorKind::InvalidUrl(..) => "Invalid URL".to_string(),
            RunnerErrorKind::NoQueryResult => "No query result".to_string(),
            RunnerErrorKind::QueryHeaderNotFound => "Header not found".to_string(),
            RunnerErrorKind::QueryInvalidJson => "Invalid JSON".to_string(),
            RunnerErrorKind::QueryInvalidJsonpathExpression { .. } => {
                "Invalid JSONPath".to_string()
            }
            RunnerErrorKind::QueryInvalidXml => "Invalid XML".to_string(),
            RunnerErrorKind::QueryInvalidXpathEval => "Invalid XPath expression".to_string(),
            RunnerErrorKind::TemplateVariableInvalidType { .. } => {
                "Invalid variable type".to_string()
            }
            RunnerErrorKind::TemplateVariableNotDefined { .. } => "Undefined variable".to_string(),
            RunnerErrorKind::TooManyRedirect => "HTTP connection".to_string(),
            RunnerErrorKind::UnauthorizedFileAccess { .. } => {
                "Unauthorized file access".to_string()
            }
            RunnerErrorKind::UnrenderableVariable { .. } => "Unrenderable variable".to_string(),
            RunnerErrorKind::UnsupportedContentEncoding(..) => "Decompression error".to_string(),
            RunnerErrorKind::UnsupportedHttpVersion(..) => "Unsupported HTTP version".to_string(),
        }
    }

    fn fixme(&self, content: &[&str], color: bool) -> String {
        match &self.kind {
            RunnerErrorKind::AssertBodyValueError { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    color_red_multiline_string(&message)
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::AssertFailure {
                actual,
                expected,
                type_mismatch,
                ..
            } => {
                let additional = if *type_mismatch {
                    "\n   >>> types between actual and expected are not consistent"
                } else {
                    ""
                };
                let message = format!("   actual:   {actual}\n   expected: {expected}{additional}");
                if color {
                    color_red_multiline_string(&message)
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::AssertHeaderValueError { actual } => {
                let message = &format!("actual value is <{actual}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::AssertStatus { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::AssertVersion { actual, .. } => {
                let message = &format!("actual value is <{actual}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::CouldNotUncompressResponse(algorithm) => {
                let message = &format!("could not uncompress response with {algorithm}");

                // only add carets if source_info is set
                // TODO: add additional attribute in the error to be more explicit?
                let message = if self.source_info.start.line == 0 {
                    message.to_string()
                } else {
                    hurl_core::error::add_carets(message, self.source_info, content)
                };
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::FileReadAccess { path } => {
                let message = &format!("file {} can not be read", path.to_string_lossy());
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::FileWriteAccess { path, error } => {
                format!("{} can not be written ({error})", path.to_string_lossy())
            }
            RunnerErrorKind::FilterDecode(encoding) => {
                let message = &format!("value can not be decoded with <{encoding}> encoding");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::FilterInvalidEncoding(encoding) => {
                let message = &format!("<{encoding}> encoding is not supported");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::FilterInvalidInput(message) => {
                let message = &format!("invalid filter input: {message}");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::FilterMissingInput => {
                let message = "missing value to apply filter";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::HttpConnection(message) => {
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::InvalidCharset { charset } => {
                let message = &format!("the charset '{charset}' is not valid");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::InvalidDecoding { charset } => {
                let message = &format!("the body can not be decoded with charset '{charset}'");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::InvalidJson { value } => {
                let message = &format!("actual value is <{value}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::InvalidRegex => {
                let message = "regex expression is not valid";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::InvalidUrl(url, reason) => {
                let message = &format!("invalid URL <{url}> ({reason})");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::NoQueryResult => {
                let message = "The query didn't return any result";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::QueryHeaderNotFound => {
                let message = "this header has not been found in the response";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::QueryInvalidJson => {
                let message = "the HTTP response is not a valid JSON";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::QueryInvalidJsonpathExpression { value } => {
                let message = &format!("the JSONPath expression '{value}' is not valid");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::QueryInvalidXml => {
                let message = "the HTTP response is not a valid XML";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::QueryInvalidXpathEval => {
                let message = "the XPath expression is not valid";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::TemplateVariableInvalidType {
                value, expecting, ..
            } => {
                let message = &format!("expecting {expecting}, actual value is <{value}>");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::TemplateVariableNotDefined { name } => {
                let message = &format!("you must set the variable {name}");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::TooManyRedirect => {
                let message = "too many redirect";
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::UnauthorizedFileAccess { path } => {
                let message = &format!(
                    "unauthorized access to file {}, check --file-root option",
                    path.to_string_lossy()
                );
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::UnrenderableVariable { name, value } => {
                let message = &format!("variable <{name}> with value {value} can not be rendered");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::UnsupportedContentEncoding(algorithm) => {
                let message = &format!("compression {algorithm} is not supported");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
            RunnerErrorKind::UnsupportedHttpVersion(version) => {
                let message = &format!("{version} is not supported, check --version");
                let message = hurl_core::error::add_carets(message, self.source_info, content);
                if color {
                    message.red().bold().to_string()
                } else {
                    message.to_string()
                }
            }
        }
    }

    fn show_source_line(&self) -> bool {
        true
    }
}

/// Color each line separately
fn color_red_multiline_string(s: &str) -> String {
    let lines = split_lines(s);
    lines
        .iter()
        .map(|line| line.red().bold().to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

/// Splits this `text` to a list of LF/CRLF separated lines.
fn split_lines(text: &str) -> Vec<&str> {
    regex::Regex::new(r"\n|\r\n").unwrap().split(text).collect()
}

impl From<HttpError> for RunnerErrorKind {
    /// Converts a HttpError to a RunnerError.
    fn from(item: HttpError) -> Self {
        match item {
            HttpError::CouldNotParseResponse => {
                RunnerErrorKind::HttpConnection("could not parse Response".to_string())
            }
            HttpError::CouldNotUncompressResponse { description } => {
                RunnerErrorKind::CouldNotUncompressResponse(description)
            }
            HttpError::InvalidCharset { charset } => RunnerErrorKind::InvalidCharset { charset },
            HttpError::InvalidDecoding { charset } => RunnerErrorKind::InvalidDecoding { charset },
            HttpError::InvalidUrl(url, reason) => RunnerErrorKind::InvalidUrl(url, reason),
            HttpError::Libcurl { code, description } => {
                RunnerErrorKind::HttpConnection(format!("({code}) {description}"))
            }
            HttpError::LibcurlUnknownOption {
                option,
                minimum_version,
            } => RunnerErrorKind::HttpConnection(format!(
                "Option {option} requires libcurl version {minimum_version} or higher"
            )),
            HttpError::StatuslineIsMissing => {
                RunnerErrorKind::HttpConnection("status line is missing".to_string())
            }
            HttpError::TooManyRedirect => RunnerErrorKind::TooManyRedirect,
            HttpError::UnsupportedContentEncoding { description } => {
                RunnerErrorKind::UnsupportedContentEncoding(description)
            }
            HttpError::UnsupportedHttpVersion(version) => {
                RunnerErrorKind::UnsupportedHttpVersion(version)
            }
        }
    }
}
