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

//! A runner for Hurl files. If you want to execute an Hurl file, this is the right place.

pub use self::error::{RunnerError, RunnerErrorKind};
#[doc(hidden)]
pub use self::event::EventListener;
pub use self::http_response::HttpResponse;
pub use self::hurl_file::run;
#[doc(hidden)]
pub use self::hurl_file::run_entries;
pub use self::number::Number;
pub use self::output::Output;
pub use self::result::{AssertResult, CaptureResult, EntryResult, HurlResult};
pub use self::runner_options::{RunnerOptions, RunnerOptionsBuilder};
pub use self::value::{EvalError, Value};
pub use self::variable::{Variable, VariableSet, Visibility};

mod assert;
mod body;
mod cache;
mod capture;
mod diff;
mod entry;
mod error;
mod event;
mod expr;
mod filter;
mod function;
mod http_response;
mod hurl_file;
mod json;
mod multiline;
mod multipart;
mod number;
mod options;
mod output;
mod predicate;
mod predicate_value;
mod query;
mod regex;
mod request;
mod response;
mod result;
mod runner_options;
mod template;
mod value;
mod value_impl;
mod variable;
mod xpath;
