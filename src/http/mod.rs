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

pub use self::client::{Client, ClientOptions, HttpError};
pub use self::core::{Cookie, Header};
#[cfg(test)]
pub use self::request::tests::*;
pub use self::request::{FileParam, Method, MultipartParam, Param, Request, RequestCookie};
#[cfg(test)]
pub use self::response::tests::*;
pub use self::response::{Response, Version};

mod client;
mod core;
mod request;
mod response;
