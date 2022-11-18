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
mod html;
mod junit;

pub use html::write_html_report;
pub use junit::create_report as create_junit_report;
pub use junit::Testcase;
use std::path::Path;

/// Returns the canonical fullname relative to / (technically a relative path)
/// The function will panic if the input file does not exist
pub fn canonicalize_filename(input_file: &str) -> String {
    let relative_input_file = Path::new(input_file).canonicalize().expect("existing file");
    let relative_input_file = relative_input_file.to_string_lossy();
    relative_input_file.trim_start_matches('/').to_string()
}
