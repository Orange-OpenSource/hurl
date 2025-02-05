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

use crate::http::RequestedHttpVersion;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpError {
    CouldNotParseResponse,
    CouldNotUncompressResponse {
        description: String,
    },
    InvalidCharset {
        charset: String,
    },
    InvalidDecoding {
        charset: String,
    },
    Libcurl {
        code: i32,
        description: String,
    },
    LibcurlUnknownOption {
        option: String,
        minimum_version: String,
    },
    NoPrimaryIp,
    TooManyRedirect,
    UnsupportedContentEncoding {
        description: String,
    },
    UnsupportedHttpVersion(RequestedHttpVersion),
    /// Request URL is invalid (URL and reason)
    InvalidUrl(String, String),
    /// The maximum response size has been exceeded.
    /// This error can be raised even if libcurl has been configured to respect a given maximum
    /// file size.
    AllowedResponseSizeExceeded(u64),
}

impl From<curl::Error> for HttpError {
    fn from(err: curl::Error) -> Self {
        let code = err.code() as i32;
        let description = err.description().to_string();
        HttpError::Libcurl { code, description }
    }
}

impl HttpError {
    pub fn description(&self) -> String {
        match self {
            HttpError::AllowedResponseSizeExceeded(_) => "HTTP connection".to_string(),
            HttpError::CouldNotParseResponse => "HTTP connection".to_string(),
            HttpError::CouldNotUncompressResponse { .. } => "Decompression error".to_string(),
            HttpError::InvalidCharset { .. } => "Invalid charset".to_string(),
            HttpError::InvalidDecoding { .. } => "Invalid decoding".to_string(),
            HttpError::InvalidUrl(..) => "Invalid URL".to_string(),
            HttpError::Libcurl { .. } => "HTTP connection".to_string(),
            HttpError::LibcurlUnknownOption { .. } => "HTTP connection".to_string(),
            HttpError::NoPrimaryIp => "HTTP connection".to_string(),
            HttpError::TooManyRedirect => "HTTP connection".to_string(),
            HttpError::UnsupportedContentEncoding { .. } => "Decompression error".to_string(),
            HttpError::UnsupportedHttpVersion(_) => "Unsupported HTTP version".to_string(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            HttpError::AllowedResponseSizeExceeded(max_size) => {
                format!("exceeded the maximum allowed file size ({max_size} bytes)")
            }
            HttpError::CouldNotParseResponse => "could not parse Response".to_string(),
            HttpError::CouldNotUncompressResponse { description } => {
                format!("could not uncompress response with {description}")
            }
            HttpError::InvalidCharset { charset } => {
                format!("the charset '{charset}' is not valid")
            }
            HttpError::InvalidDecoding { charset } => {
                format!("the body can not be decoded with charset '{charset}'")
            }
            HttpError::InvalidUrl(url, reason) => {
                format!("invalid URL <{url}> ({reason})").to_string()
            }
            HttpError::Libcurl { code, description } => format!("({code}) {description}"),
            HttpError::LibcurlUnknownOption {
                option,
                minimum_version,
            } => format!("Option {option} requires libcurl version {minimum_version} or higher"),
            HttpError::NoPrimaryIp => "No primary IP found in response".to_string(),
            HttpError::TooManyRedirect => "too many redirect".to_string(),
            HttpError::UnsupportedHttpVersion(version) => {
                format!("{version} is not supported, check --version").to_string()
            }
            HttpError::UnsupportedContentEncoding { description } => {
                format!("compression {description} is not supported").to_string()
            }
        }
    }
}
