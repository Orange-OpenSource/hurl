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

//! This crate provides a function to run a Hurl formatted content.
//! Hurl uses a plain text format to run and tests HTTP requests. The fully documented
//! format is available at <https://hurl.dev>
//!
//! A Hurl sample:
//! ```hurl
//! # Get home:
//! GET https://example.org
//! HTTP 200
//! [Captures]
//! csrf_token: xpath "string(//meta[@name='_csrf_token']/@content)"
//!
//!
//! # Do login!
//! POST https://example.org/login?user=toto&password=1234
//! X-CSRF-TOKEN: {{csrf_token}}
//! HTTP 302
//! ```
//!
//! The main function of this crate is [`runner::run`].
//!
//! This crate works on Windows, macOS and Linux.
mod html;
pub mod http;
mod json;
mod jsonpath;
pub mod output;
#[doc(hidden)]
pub mod parallel;
pub mod report;
pub mod runner;
pub mod util;
