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
//! Various HTTP structures like requests, responses, cookies etc.
//!
//! The Hurl HTTP engine is not public. It's a wrapper around libcurl and only the models
//! returned by an HTTP exchange are exposed.
pub use self::call::Call;
pub use self::certificate::Certificate;
pub(crate) use self::client::Client;
pub use self::cookie::{CookieAttribute, ResponseCookie};
pub use self::core::Cookie;
pub(crate) use self::core::{Param, RequestCookie};
pub use self::curl_cmd::CurlCmd;
pub(crate) use self::error::HttpError;
pub use self::header::{
    Header, HeaderVec, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_TYPE, COOKIE, EXPECT, USER_AGENT,
};
pub(crate) use self::options::{ClientOptions, Verbosity};
pub use self::request::{IpResolve, Request, RequestedHttpVersion};
pub(crate) use self::request_spec::{Body, FileParam, Method, MultipartParam, RequestSpec};
pub use self::response::{HttpVersion, Response};
#[cfg(test)]
pub use self::tests::*;
pub use self::timings::Timings;
pub use self::url::Url;
pub use self::version::libcurl_version_info;

mod call;
mod certificate;
mod client;
mod cookie;
mod core;
mod curl_cmd;
mod debug;
mod easy_ext;
mod error;
mod header;
mod headers_helper;
mod ip;
mod mimetype;
mod options;
mod request;
mod request_spec;
mod response;
mod response_cookie;
mod response_debug;
mod response_decoding;
#[cfg(test)]
mod tests;
mod timings;
mod timings_debug;
mod url;
mod version;
