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
//! Serialize a Hurl run result to a file.
//!
//! There are two supported serialisation:
//!
//! - JSON: the whole run is serialized to JSON (like the [HAR](https://en.wikipedia.org/wiki/HAR_(file_format)) format)
//!   [`self::json::write_json`]
//! - raw: the last response of a run is serialized to a file. The body can be automatically uncompress
//!   or written as it [`self::raw::write_last_body`]
mod error;
mod json;
mod raw;

pub use self::error::OutputError;
pub use self::json::write_json;
pub use self::raw::write_last_body;
