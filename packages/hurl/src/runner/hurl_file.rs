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

use crate::cli::Logger;
use crate::{cli, http};
use hurl_core::ast::*;

use super::core::*;
use super::entry;

/// Run a Hurl file with the hurl http client
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use hurl::cli::Logger;
/// use hurl_core::parser;
/// use hurl::http;
/// use hurl::http::ContextDir;
/// use hurl::runner;
///
/// // Parse Hurl file
/// let filename = "sample.hurl";
/// let s = r#"
/// GET http://localhost:8000/hello
/// HTTP/1.0 200
/// "#;
/// let hurl_file = parser::parse_hurl_file(s).unwrap();
///
/// // Create an HTTP client
/// let options = http::ClientOptions::default();
/// let mut client = http::Client::init(options);
///
/// let logger = Logger::default();
///
/// // Define runner options
/// let variables = std::collections::HashMap::new();
/// let options = runner::RunnerOptions {
///        fail_fast: false,
///        variables,
///        to_entry: None,
///        context_dir: ContextDir::default(),
///        ignore_asserts: false,
///        very_verbose: false,
///        pre_entry: |_| true,
///        post_entry: || true,
///  };
///
/// // FIXME: put lines in hurl_file?
///
/// // Run the hurl file
/// let hurl_results = runner::run(
///     hurl_file,
///     &vec!(),
///     filename,
///     &mut client,
///     &options,
///     &logger,
/// );
/// assert!(hurl_results.success);
///
/// ```
///
pub fn run(
    hurl_file: HurlFile,
    lines: &Vec<&str>,
    filename: &str,
    http_client: &mut http::Client,
    options: &RunnerOptions,
    logger: &Logger,
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
            options,
            logger,
        );

        for entry_result in entry_results.clone() {
            for e in entry_result.errors.clone() {
                let error_message = cli::error_string(lines, filename, &e);
                logger.error(format!("{}\n", &error_message).as_str());
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
    let filename = filename.to_string();
    HurlResult {
        filename,
        entries,
        time_in_ms,
        success,
        cookies,
    }
}
