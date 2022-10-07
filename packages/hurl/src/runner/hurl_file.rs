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
use crate::http;
use crate::runner::entry::get_entry_verbosity;
use crate::runner::Value;
use hurl_core::ast::*;

use super::core::*;
use super::entry;

/// Runs a `hurl_file`, issue from the given `filename` file and `content`, with
/// an `http_client`. Returns a [`HurlResult`] upon completion.
///
/// `filename` and `content` are used to display line base logs (for parsing error or asserts
/// failures).
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use std::path::PathBuf;
/// use hurl::cli::Logger;
/// use hurl_core::parser;
/// use hurl::http;
/// use hurl::http::ContextDir;
/// use hurl::runner;
/// use hurl::runner::Value;
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
/// let mut client = http::Client::new(None);
/// let logger = Logger::new(false, false, filename, s);
///
/// // Define runner options
/// let runner_options = runner::RunnerOptions {
///   very_verbose: true,
///   ..runner::RunnerOptions::default()
/// };
///
/// // set variables
/// let mut variables = HashMap::default();
/// variables.insert("name".to_string(), Value::String("toto".to_string()));
///
/// // Run the hurl file
/// let hurl_results = runner::run(
///     &hurl_file,
///     filename,
///     &mut client,
///     &runner_options,
///     &variables,
///     &logger
/// );
/// assert!(hurl_results.success);
/// ```
pub fn run(
    hurl_file: &HurlFile,
    filename: &str,
    http_client: &mut http::Client,
    runner_options: &RunnerOptions,
    variables: &HashMap<String, Value>,
    logger: &Logger,
) -> HurlResult {
    let mut entries = vec![];
    let mut variables = variables.clone();

    let n = if let Some(to_entry) = runner_options.to_entry {
        to_entry
    } else {
        hurl_file.entries.len()
    };

    let start = Instant::now();
    for (entry_index, entry) in hurl_file
        .entries
        .iter()
        .take(n)
        .enumerate()
        .collect::<Vec<(usize, &Entry)>>()
    {
        if let Some(pre_entry) = runner_options.pre_entry {
            let exit = pre_entry(entry.clone());
            if exit {
                break;
            }
        }

        // We compute these new overridden options for this entry, before entering into the `run`
        // function because entry options can modify the logger and we want the preamble
        // "Executing entry..." to be displayed based on the entry level verbosity.
        let entry_verbosity = get_entry_verbosity(entry, &runner_options.verbosity);
        let logger = &Logger::new(
            logger.color,
            entry_verbosity.is_some(),
            logger.filename,
            logger.content,
        );

        logger.debug_important(
            "------------------------------------------------------------------------------",
        );
        logger.debug_important(format!("Executing entry {}", entry_index + 1).as_str());

        let entry_result =
            match entry::get_entry_options(entry, runner_options, &mut variables, logger) {
                Ok(runner_options) => {
                    entry::run(entry, http_client, &mut variables, &runner_options, logger)
                }
                Err(error) => EntryResult {
                    calls: vec![],
                    captures: vec![],
                    asserts: vec![],
                    errors: vec![error],
                    time_in_ms: 0,
                    compressed: false,
                },
            };
        for e in &entry_result.errors {
            logger.error_rich(e);
        }
        entries.push(entry_result.clone());

        if let Some(post_entry) = runner_options.post_entry {
            let exit = post_entry();
            if exit {
                break;
            }
        }

        if runner_options.fail_fast && !entry_result.errors.is_empty() {
            break;
        }
    }

    let time_in_ms = start.elapsed().as_millis();
    let success = entries
        .iter()
        .flat_map(|e| e.errors.iter())
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
