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
use hurl_core::ast::SourceInfo;
use hurl_core::error::Error;

use crate::http::HttpError;
use crate::runner;
use crate::runner::RunnerError;

/// Textual Output for runner errors
impl Error for runner::Error {
    fn source_info(&self) -> SourceInfo {
        self.clone().source_info
    }

    fn description(&self) -> String {
        match &self.inner {
            RunnerError::InvalidUrl(..) => "Invalid URL".to_string(),
            RunnerError::InvalidUrlPrefix(..) => "Invalid URL".to_string(),
            RunnerError::TemplateVariableNotDefined { .. } => "Undefined variable".to_string(),
            RunnerError::HttpConnection { .. } => "HTTP connection".to_string(),
            RunnerError::CouldNotResolveProxyName => "HTTP connection".to_string(),
            RunnerError::CouldNotResolveHost(_) => "HTTP connection".to_string(),
            RunnerError::FailToConnect => "HTTP connection".to_string(),
            RunnerError::Timeout => "HTTP connection".to_string(),
            RunnerError::TooManyRedirect => "HTTP connection".to_string(),
            RunnerError::CouldNotParseResponse => "HTTP connection".to_string(),
            RunnerError::SslCertificate { .. } => "SSL certificate".to_string(),
            RunnerError::PredicateValue { .. } => "Assert - predicate value failed".to_string(),
            RunnerError::InvalidRegex => "Invalid regex".to_string(),
            RunnerError::FileReadAccess { .. } => "File read access".to_string(),
            RunnerError::QueryInvalidXml => "Invalid XML".to_string(),
            RunnerError::QueryInvalidXpathEval => "Invalid XPath expression".to_string(),
            RunnerError::QueryHeaderNotFound => "Header not found".to_string(),
            RunnerError::QueryCookieNotFound => "Cookie not found".to_string(),
            RunnerError::AssertHeaderValueError { .. } => "Assert header value".to_string(),
            RunnerError::AssertBodyValueError { .. } => "Assert body value".to_string(),
            RunnerError::AssertVersion { .. } => "Assert HTTP version".to_string(),
            RunnerError::AssertStatus { .. } => "Assert status code".to_string(),
            RunnerError::QueryInvalidJson => "Invalid JSON".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { .. } => "Invalid JSONPath".to_string(),
            RunnerError::PredicateType => "Assert - inconsistent predicate type".to_string(),
            RunnerError::InvalidDecoding { .. } => "Invalid decoding".to_string(),
            RunnerError::InvalidCharset { .. } => "Invalid charset".to_string(),
            RunnerError::AssertFailure { .. } => "Assert failure".to_string(),
            RunnerError::UnrenderableVariable { .. } => "Unrenderable variable".to_string(),
            RunnerError::NoQueryResult => "No query result".to_string(),
            RunnerError::UnsupportedContentEncoding(..) => "Decompression error".to_string(),
            RunnerError::UnsupportedHttpVersion(..) => "Unsupported HTTP version".to_string(),
            RunnerError::CouldNotUncompressResponse(..) => "Decompression error".to_string(),
            RunnerError::InvalidJson { .. } => "Invalid JSON".to_string(),
            RunnerError::UnauthorizedFileAccess { .. } => "Unauthorized file access".to_string(),
            RunnerError::FilterMissingInput => "Filter Error".to_string(),
            RunnerError::FilterInvalidInput { .. } => "Filter Error".to_string(),
            RunnerError::FilterRegexNoCapture => "Filter Error".to_string(),
            RunnerError::FilterInvalidEncoding { .. } => "Filter Error".to_string(),
            RunnerError::FilterDecode { .. } => "Filter Error".to_string(),
        }
    }

    fn fixme(&self) -> String {
        match &self.inner {
            RunnerError::InvalidUrl(url) => format!("invalid URL <{url}>"),
            RunnerError::InvalidUrlPrefix(url) => {
                format!("URL <{url}> must start with http:// or https://")
            }
            RunnerError::TemplateVariableNotDefined { name } => {
                format!("you must set the variable {name}")
            }
            RunnerError::HttpConnection { message, .. } => message.to_string(),
            RunnerError::CouldNotResolveProxyName => "could not resolve proxy name".to_string(),
            RunnerError::CouldNotResolveHost(host) => format!("could not resolve host <{host}>"),
            RunnerError::FailToConnect => "fail to connect".to_string(),
            RunnerError::Timeout => "timeout has been reached".to_string(),
            RunnerError::TooManyRedirect => "too many redirect".to_string(),
            RunnerError::CouldNotParseResponse => "could not parse response".to_string(),
            RunnerError::SslCertificate(description) => description.clone(),
            RunnerError::AssertVersion { actual, .. } => format!("actual value is <{actual}>"),
            RunnerError::AssertStatus { actual, .. } => format!("actual value is <{actual}>"),
            RunnerError::PredicateValue(value) => {
                format!("actual value is <{value}>")
            }
            RunnerError::InvalidRegex => "regex expression is not valid".to_string(),
            RunnerError::FileReadAccess { value } => format!("file {value} can not be read"),
            RunnerError::QueryInvalidXml => "the HTTP response is not a valid XML".to_string(),
            RunnerError::QueryHeaderNotFound => {
                "this header has not been found in the response".to_string()
            }
            RunnerError::QueryCookieNotFound => {
                "this cookie has not been found in the response".to_string()
            }
            RunnerError::QueryInvalidXpathEval => "the XPath expression is not valid".to_string(),
            RunnerError::AssertHeaderValueError { actual } => {
                format!("actual value is <{actual}>")
            }
            RunnerError::AssertBodyValueError { actual, .. } => {
                format!("actual value is <{actual}>")
            }
            RunnerError::QueryInvalidJson => "the HTTP response is not a valid JSON".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { value } => {
                format!("the JSONPath expression '{value}' is not valid")
            }
            RunnerError::PredicateType => {
                "predicate type inconsistent with value return by query".to_string()
            }
            RunnerError::InvalidDecoding { charset } => {
                format!("the body can not be decoded with charset '{charset}'")
            }
            RunnerError::InvalidCharset { charset } => {
                format!("the charset '{charset}' is not valid")
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
            RunnerError::UnrenderableVariable { name, value } => {
                format!("variable <{name}> with value {value} can not be rendered")
            }
            RunnerError::NoQueryResult => "The query didn't return any result".to_string(),
            RunnerError::UnsupportedContentEncoding(algorithm) => {
                format!("compression {algorithm} is not supported")
            }
            RunnerError::UnsupportedHttpVersion(version) => {
                format!("{version} is not supported, check --version")
            }

            RunnerError::CouldNotUncompressResponse(algorithm) => {
                format!("could not uncompress response with {algorithm}")
            }
            RunnerError::InvalidJson { value } => {
                format!("actual value is <{value}>")
            }
            RunnerError::UnauthorizedFileAccess { path } => {
                format!(
                    "unauthorized access to file {}, check --file-root option",
                    path.to_str().unwrap()
                )
            }
            RunnerError::FilterMissingInput => "missing value to apply filter".to_string(),
            RunnerError::FilterInvalidInput(message) => {
                format!("invalid filter input: {message}")
            }
            RunnerError::FilterRegexNoCapture => "capture not found".to_string(),
            RunnerError::FilterInvalidEncoding(encoding) => {
                format!("<{encoding}> encoding is not supported")
            }
            RunnerError::FilterDecode(encoding) => {
                format!("value can not be decoded with <{encoding}> encoding")
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
            HttpError::Libcurl {
                code,
                description,
                url,
            } => RunnerError::HttpConnection {
                message: format!("({code}) {description}"),
                url,
            },
            HttpError::LibcurlUnknownOption {
                option,
                minimum_version,
                url,
            } => RunnerError::HttpConnection {
                message: format!(
                    "Option {option} requires libcurl version {minimum_version} or higher"
                ),
                url,
            },
            HttpError::StatuslineIsMissing { url } => RunnerError::HttpConnection {
                message: "status line is missing".to_string(),
                url,
            },
            HttpError::TooManyRedirect => RunnerError::TooManyRedirect,
            HttpError::UnsupportedContentEncoding { description } => {
                RunnerError::UnsupportedContentEncoding(description)
            }
            HttpError::UnsupportedHttpVersion(version) => {
                RunnerError::UnsupportedHttpVersion(version)
            }
            HttpError::InvalidUrl(url) => RunnerError::InvalidUrl(url),
            HttpError::InvalidUrlPrefix(url) => RunnerError::InvalidUrlPrefix(url),
        }
    }
}
