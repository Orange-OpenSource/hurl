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
use std::collections::HashMap;
use std::thread;
use std::time::Instant;

use crate::http;
use crate::runner::core::*;
use crate::runner::entry;
use crate::runner::runner_options::RunnerOptions;
use crate::runner::Value;
use crate::util::logger::Logger;
use crate::util::logger::LoggerBuilder;
use hurl_core::ast::VersionValue::VersionAnyLegacy;
use hurl_core::ast::*;

/// Runs a `hurl_file`, issue from the given `content`and `filename` and
/// returns a [`HurlResult`] upon completion.
///
/// `filename` and `content` are used to display rich logs (for parsing error or asserts
/// failures).
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use std::path::PathBuf;
/// use hurl_core::parser;
/// use hurl::runner;
/// use hurl::runner::{Value, RunnerOptionsBuilder, Verbosity};
/// use hurl::util::logger::LoggerBuilder;
///
/// // Parse the Hurl file
/// let filename = "sample.hurl";
/// let content = r#"
/// GET http://localhost:8000/hello
/// HTTP/1.1 200
/// "#;
/// let hurl_file = parser::parse_hurl_file(content).unwrap();
///
/// let logger = LoggerBuilder::new().build();
///
/// // Define runner options
/// let runner_options = RunnerOptionsBuilder::new()
///     .follow_location(true)
///     .verbosity(Some(Verbosity::Verbose))
///     .build();
///
/// // set variables
/// let mut variables = HashMap::default();
/// variables.insert("name".to_string(), Value::String("toto".to_string()));
///
/// // Run the Hurl file
/// let hurl_results = runner::run(
///     &hurl_file,
///     content,
///     filename,
///     &runner_options,
///     &variables,
///     &logger
/// );
/// assert!(hurl_results.success);
/// ```
pub fn run(
    hurl_file: &HurlFile,
    content: &str,
    filename: &str,
    runner_options: &RunnerOptions,
    variables: &HashMap<String, Value>,
    logger: &Logger,
) -> HurlResult {
    let cookie_input_file = runner_options.cookie_input_file.clone();
    let mut http_client = http::Client::new(cookie_input_file);
    let mut entries = vec![];
    let mut variables = variables.clone();
    let mut entry_index = 1;
    let mut retry_count = 1;
    let n = if let Some(to_entry) = runner_options.to_entry {
        to_entry
    } else {
        hurl_file.entries.len()
    };
    let start = Instant::now();

    loop {
        if entry_index > n {
            break;
        }
        let entry = &hurl_file.entries[entry_index - 1];

        // We compute these new overridden options for this entry, before entering into the `run`
        // function because entry options can modify the logger and we want the preamble
        // "Executing entry..." to be displayed based on the entry level verbosity.
        let entry_verbosity = entry::get_entry_verbosity(entry, &runner_options.verbosity);
        let logger = LoggerBuilder::new()
            .color(logger.color)
            .verbose(entry_verbosity.is_some())
            .test(logger.test)
            .progress_bar(entry_verbosity.is_none() && logger.progress_bar)
            .build();

        if let Some(pre_entry) = runner_options.pre_entry {
            let exit = pre_entry(entry.clone());
            if exit {
                break;
            }
        }

        logger.debug_important(
            "------------------------------------------------------------------------------",
        );
        logger.debug_important(format!("Executing entry {entry_index}").as_str());

        warn_deprecated(entry, filename, &logger);

        logger.test_progress(entry_index, n);

        let options_result =
            entry::get_entry_options(entry, runner_options, &mut variables, &logger);
        let entry_result = match &options_result {
            Ok(options) => entry::run(
                entry,
                entry_index,
                &mut http_client,
                &mut variables,
                options,
                &logger,
            ),
            Err(error) => EntryResult {
                entry_index,
                calls: vec![],
                captures: vec![],
                asserts: vec![],
                errors: vec![error.clone()],
                time_in_ms: 0,
                compressed: false,
            },
        };

        // Check if we need to retry.
        let has_error = !entry_result.errors.is_empty();
        let (retry, retry_max_count, retry_interval) = match &options_result {
            Ok(options) => (
                options.retry,
                options.retry_max_count,
                options.retry_interval,
            ),
            Err(_) => (
                runner_options.retry,
                runner_options.retry_max_count,
                runner_options.retry_interval,
            ),
        };
        let retry_max_reached = match retry_max_count {
            None => false,
            Some(r) => retry_count > r,
        };
        if retry_max_reached {
            logger.debug("");
            logger.debug_important("Retry max count reached, no more retry");
        }
        let retry = retry && !retry_max_reached && has_error;

        // If we're going to retry the entry, we log error only in verbose. Otherwise,
        // we log error on stderr.
        for e in &entry_result.errors {
            logger.test_erase_line();
            if retry {
                logger.debug_error(filename, content, e);
            } else {
                logger.error_rich(filename, content, e);
            }
        }
        entries.push(entry_result);

        if let Some(post_entry) = runner_options.post_entry {
            let exit = post_entry();
            if exit {
                break;
            }
        }

        if retry {
            let delay = retry_interval.as_millis();
            logger.debug("");
            logger.debug_important(
                format!("Retry entry {entry_index} (x{retry_count} pause {delay} ms)").as_str(),
            );
            retry_count += 1;
            thread::sleep(retry_interval);
            continue;
        }
        if runner_options.fail_fast && has_error {
            break;
        }

        // We pass to the next entry
        entry_index += 1;
        retry_count = 1;
    }

    logger.test_erase_line();

    let time_in_ms = start.elapsed().as_millis();
    let cookies = http_client.get_cookie_storage();
    let success = is_success(&entries);
    HurlResult {
        entries,
        time_in_ms,
        success,
        cookies,
    }
}

/// Returns `true` if all the entries ar successful, `false` otherwise.
///
/// For a given list of entry, only the last one on the same index is checked.
/// For instance:
/// entry a:1, entry b:1, entry c:2, entry d:3, entry e:3
/// Only the entry b, c and e are checked for the success state.
fn is_success(entries: &[EntryResult]) -> bool {
    let mut next_entries = entries.iter().skip(1);
    for entry in entries.iter() {
        match next_entries.next() {
            None => return entry.errors.is_empty(),
            Some(next) => {
                if next.entry_index != entry.entry_index && !entry.errors.is_empty() {
                    return false;
                }
            }
        }
    }
    true
}

/// Logs deprecated syntax and provides alternatives.
fn warn_deprecated(entry: &Entry, filename: &str, logger: &Logger) {
    // HTTP/* is used instead of HTTP.
    if let Some(response) = &entry.response {
        let version = &response.version;
        let source_info = &version.source_info;
        let line = &source_info.start.line;
        let column = &source_info.start.column;
        if version.value == VersionAnyLegacy {
            logger.warning(
                format!(
                    "{filename}:{line}:{column} 'HTTP/*' keyword is deprecated, please use 'HTTP' instead"
                )
                .as_str(),
            );
        }
    }

    // one line string with ```something``` syntax instead of `something`
    if let Request {
        body:
            Some(Body {
                value: Bytes::MultilineString(MultilineString::OneLineText(template)),
                ..
            }),
        ..
    } = &entry.request
    {
        let source_info = &template.source_info;
        let line = &source_info.start.line;
        let column = &source_info.start.column;
        let template = template.to_string();
        logger.warning(
            format!(
                "{filename}:{line}:{column} '```{template}```' request body is deprecated, please use '`{template}`' instead"
            )
            .as_str(),
        );
    }

    if let Some(Response {
        body:
            Some(Body {
                value: Bytes::MultilineString(MultilineString::OneLineText(template)),
                ..
            }),
        ..
    }) = &entry.response
    {
        let source_info = &template.source_info;
        let line = &source_info.start.line;
        let column = &source_info.start.column;
        let template = template.to_string();
        logger.warning(
            format!(
                "{filename}:{line}:{column} '```{template}```' response body is deprecated, please use '`{template}`' instead"
            )
            .as_str(),
        );
    }
}
