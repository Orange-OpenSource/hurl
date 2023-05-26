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

//! A runner for Hurl files. If you want to execute an Hurl file, this is the right place.

pub use self::core::{AssertResult, CaptureResult, EntryResult, Error, HurlResult, RunnerError};
pub use self::hurl_file::run;
pub use self::runner_options::{RunnerOptions, RunnerOptionsBuilder};
pub use self::value::Value;

// We only expose specific structs from the module `http`.
// We don't want to expose the whole internal HTTP client, but needs those structs as
// they're exposed through `EntryResult`.
pub use crate::http::{Call, Certificate, Cookie, Header, Request, Response, Timings, Version};

mod assert;
mod body;
mod capture;
mod core;
mod entry;
mod error;
mod expr;
mod filter;
mod hurl_file;
mod json;
mod multiline;
mod multipart;
mod predicate;
mod predicate_value;
mod query;
mod regex;
mod request;
mod response;
mod runner_options;
mod template;
mod value;
mod xpath;
