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
use std::collections::HashMap;
use std::time::Instant;

use crate::http;
use hurl_core::ast::*;

use super::core::*;
use super::entry;

/// Run a Hurl file with the hurl http client
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use hurl_core::parser;
/// use hurl::http;
/// use hurl::runner;
///
/// // Parse Hurl file
/// let filename = "sample.hurl".to_string();
/// let s = r#"
/// GET http://localhost:8000/hello
/// HTTP/1.0 200
/// "#;
/// let hurl_file = parser::parse_hurl_file(s).unwrap();
///
/// // create loggers (function pointer or closure)
/// fn log_verbose(message: &str) { eprintln!("* {}", message); }
/// fn log_error_message(_warning:bool, message: &str) { eprintln!("{}", message); }
/// fn log_error(error: &runner::Error, _warning: bool) { eprintln!("* {:#?}", error); }
///
/// // Create an http client
/// let options = http::ClientOptions::default();
/// let mut client = http::Client::init(options);
///
/// // Define runner options
/// let variables = std::collections::HashMap::new();
/// let options = runner::RunnerOptions {
///        fail_fast: false,
///        variables,
///        to_entry: None,
///        context_dir: PathBuf::new(),
///        ignore_asserts: false,
///        pre_entry: |_| true,
///        post_entry: || true,
///  };
///
/// // Run the hurl file
/// let hurl_results = runner::run_hurl_file(
///     hurl_file,
///     &mut client,
///     filename,
///     &options,
///     &log_verbose,
///     &log_error_message,
///     &log_error,
/// );
/// assert!(hurl_results.success);
///
/// ```
///
pub fn run(
    hurl_file: HurlFile,
    http_client: &mut http::Client,
    filename: String,
    options: &RunnerOptions,
    log_verbose: &impl Fn(&str),
    log_error_message: &impl Fn(bool, &str),
    log_error: &impl Fn(&Error, bool),
) -> HurlResult {
    let mut entries = vec![];
    let mut variables = HashMap::default();

    for (key, value) in options.variables.clone() {
        variables.insert(key.to_string(), value);
    }

    let n = if let Some(to_entry) = options.to_entry {
        to_entry
    } else {
        hurl_file.entries.len()
    };

    let start = Instant::now();
    for (entry_index, entry) in hurl_file
        .entries
        .iter()
        .take(n)
        .cloned()
        .enumerate()
        .collect::<Vec<(usize, Entry)>>()
    {
        let exit = (options.pre_entry)(entry.clone());
        if exit {
            break;
        }

        let entry_results = entry::run(
            entry,
            http_client,
            entry_index,
            &mut variables,
            &log_verbose,
            &log_error_message,
            options,
        );

        for entry_result in entry_results.clone() {
            for e in entry_result.errors.clone() {
                log_error(&e, false);
            }
            entries.push(entry_result.clone());
        }
        let exit = (options.post_entry)();
        if exit || (options.fail_fast && !entry_results.last().unwrap().errors.is_empty()) {
            break;
        }
    }

    let time_in_ms = start.elapsed().as_millis();
    let success = entries
        .iter()
        .flat_map(|e| e.errors.clone())
        .next()
        .is_none();

    let cookies = http_client.get_cookie_storage();
    HurlResult {
        filename,
        entries,
        time_in_ms,
        success,
        cookies,
    }
}
