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

pub use self::client::Client;
pub use self::context_dir::ContextDir;
pub use self::cookie::{CookieAttribute, ResponseCookie};
pub use self::core::{Cookie, Param, RequestCookie};
pub use self::error::HttpError;
pub use self::header::Header;
pub use self::options::{ClientOptions, Verbosity};
pub use self::request::Request;
#[cfg(test)]
pub use self::request_spec::tests::*;
pub use self::request_spec::{Body, FileParam, Method, MultipartParam, RequestSpec};
#[cfg(test)]
pub use self::response::tests::*;
pub use self::response::{Response, Version};
pub use self::version::libcurl_version_info;

mod client;
mod context_dir;
mod cookie;
mod core;
mod debug;
mod error;
mod header;
mod mimetype;
mod options;
mod request;
mod request_debug;
mod request_decoding;
mod request_spec;
mod request_spec_curl_args;
mod response;
mod response_cookie;
mod response_debug;
mod response_decoding;
mod version;
