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
        url: String,
    },
    LibcurlUnknownOption {
        option: String,
        minimum_version: String,
        url: String,
    },
    StatuslineIsMissing {
        url: String,
    },
    TooManyRedirect,
    UnsupportedContentEncoding {
        description: String,
    },
    UnsupportedHttpVersion(RequestedHttpVersion),
    InvalidUrl(String),
    InvalidUrlPrefix(String),
}

impl From<curl::Error> for HttpError {
    fn from(err: curl::Error) -> Self {
        let code = err.code() as i32;
        let description = err.description().to_string();
        let url = String::new();
        HttpError::Libcurl {
            code,
            description,
            url,
        }
    }
}
