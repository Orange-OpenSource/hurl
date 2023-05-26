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

use hurl_core::ast::VersionValue::VersionAnyLegacy;
use hurl_core::ast::*;
use hurl_core::error::Error;
use hurl_core::parser;

use crate::http;
use crate::http::Call;
use crate::runner::core::*;
use crate::runner::runner_options::RunnerOptions;
use crate::runner::{entry, Value};
use crate::util::logger::{ErrorFormat, Logger, LoggerOptions, LoggerOptionsBuilder};

/// Runs a Hurl `content` and returns a [`HurlResult`] upon completion.
///
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use hurl::runner;
/// use hurl::runner::{Value, RunnerOptionsBuilder};
/// use hurl::util::logger::{LoggerOptionsBuilder, Verbosity};
///
/// // A simple Hurl sample
/// let content = r#"
/// GET http://localhost:8000/hello
/// HTTP 200
/// "#;
///
/// // Define runner and logger options
/// let runner_opts = RunnerOptionsBuilder::new()
///     .follow_location(true)
///     .build();
/// let logger_opts = LoggerOptionsBuilder::new()
///     .verbosity(Some(Verbosity::Verbose))
///     .build();
///
/// // Set variables
/// let mut variables = HashMap::default();
/// variables.insert("name".to_string(), Value::String("toto".to_string()));
///
/// // Run the Hurl sample
/// let result = runner::run(
///     content,
///     &runner_opts,
///     &variables,
///     &logger_opts
/// );
/// assert!(result.unwrap().success);
/// ```
pub fn run(
    content: &str,
    runner_options: &RunnerOptions,
    variables: &HashMap<String, Value>,
    logger_options: &LoggerOptions,
) -> Result<HurlResult, String> {
    let logger = Logger::from(logger_options);

    // Try to parse the content
    let hurl_file = parser::parse_hurl_file(content);
    let hurl_file = match hurl_file {
        Ok(h) => h,
        Err(e) => {
            logger.error_rich(content, &e);
            return Err(e.description());
        }
    };

    log_run_info(&hurl_file, runner_options, variables, &logger);

    // Now, we have a syntactically correct HurlFile instance, we can run it.
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

        // We compute the new logger for this entry, before entering into the `run`
        // function because entry options can modify the logger and we want the preamble
        // "Executing entry..." to be displayed based on the entry level verbosity.
        let logger = get_entry_logger(entry, logger_options);
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

        warn_deprecated(entry, &logger);

        logger.test_progress(entry_index, n);

        // The real execution of the entry happens here, with the overridden entry options.
        let options = entry::get_entry_options(entry, runner_options, &mut variables, &logger);
        let entry_result = match &options {
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
        let (retry, retry_max_count, retry_interval) = match &options {
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
            Some(r) => retry_count > r,
            None => false,
        };
        let retry = retry && !retry_max_reached && has_error;

        // If `retry_max_reached` is true, we print now a warning, before displaying
        // any assert error so any potential error is the last thing displayed to the user.
        // If `retry_max_reached` is not true (for instance `retry`is true, or there is no error
        // we first log the error and a potential warning about retrying.
        logger.test_erase_line();
        if retry_max_reached {
            logger.debug_important("Retry max count reached, no more retry");
            logger.debug("");
        }
        if has_error {
            log_errors(&entry_result, content, retry, &logger);
        }

        entries.push(entry_result);

        if retry {
            let delay = retry_interval.as_millis();
            logger.debug("");
            logger.debug_important(
                format!("Retry entry {entry_index} (x{retry_count} pause {delay} ms)").as_str(),
            );
            retry_count += 1;
            // If we retry the entry, we do not want to display a 'blank' progress bar
            // during the sleep delay. During the pause, we artificially show the previsoulsy
            // erased progress line.
            logger.test_progress(entry_index, n);
            thread::sleep(retry_interval);
            logger.test_erase_line();
            continue;
        }

        if let Some(post_entry) = runner_options.post_entry {
            let exit = post_entry();
            if exit {
                break;
            }
        }
        if runner_options.fail_fast && has_error {
            break;
        }

        // We pass to the next entry
        entry_index += 1;
        retry_count = 1;
    }

    let time_in_ms = start.elapsed().as_millis();
    let cookies = http_client.get_cookie_storage();
    let success = is_success(&entries);
    Ok(HurlResult {
        entries,
        time_in_ms,
        success,
        cookies,
    })
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
fn warn_deprecated(entry: &Entry, logger: &Logger) {
    // HTTP/* is used instead of HTTP.
    if let Some(response) = &entry.response {
        let filename = &logger.filename;
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
        let filename = &logger.filename;
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
        let filename = &logger.filename;
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

/// Logs various debug information at the start of `hurl_file` run.
fn log_run_info(
    hurl_file: &HurlFile,
    runner_options: &RunnerOptions,
    variables: &HashMap<String, Value>,
    logger: &Logger,
) {
    logger.debug_important("Options:");
    logger.debug(format!("    fail fast: {}", runner_options.fail_fast).as_str());
    logger.debug(format!("    follow redirect: {}", runner_options.follow_location).as_str());
    logger.debug(format!("    insecure: {}", runner_options.insecure).as_str());
    if let Some(n) = runner_options.max_redirect {
        logger.debug(format!("    max redirect: {n}").as_str());
    }
    if let Some(proxy) = &runner_options.proxy {
        logger.debug(format!("    proxy: {proxy}").as_str());
    }
    logger.debug(format!("    retry: {}", runner_options.retry).as_str());
    if let Some(n) = runner_options.retry_max_count {
        logger.debug(format!("    retry max count: {n}").as_str());
    }
    if !variables.is_empty() {
        logger.debug_important("Variables:");
        for (name, value) in variables.iter() {
            logger.debug(format!("    {name}: {value}").as_str());
        }
    }
    if let Some(to_entry) = runner_options.to_entry {
        logger
            .debug(format!("Executing {}/{} entries", to_entry, hurl_file.entries.len()).as_str());
    }
}

/// Logs runner `errors`.
/// If we're going to `retry` the entry, we log errors only in verbose. Otherwise, we log error on stderr.
fn log_errors(entry_result: &EntryResult, content: &str, retry: bool, logger: &Logger) {
    if retry {
        entry_result
            .errors
            .iter()
            .for_each(|e| logger.debug_error(content, e));
        return;
    }

    if logger.error_format == ErrorFormat::Long {
        if let Some(Call { response, .. }) = entry_result.calls.last() {
            response.log_all(logger);
        } else {
            logger.info("<No HTTP response>");
            logger.info("");
        }
    }
    entry_result
        .errors
        .iter()
        .for_each(|e| logger.error_rich(content, e));
}

/// Creates a new logger for this entry.
/// Verbosity can be overridden at entry level with an Options section so each
/// entry has its own logger.
fn get_entry_logger(entry: &Entry, logger_options: &LoggerOptions) -> Logger {
    let entry_verbosity = entry::get_entry_verbosity(entry, &logger_options.verbosity);
    let entry_logger_options = LoggerOptionsBuilder::new()
        .color(logger_options.color)
        .filename(&logger_options.filename)
        .error_format(logger_options.error_format)
        .progress_bar(entry_verbosity.is_none() && logger_options.progress_bar)
        .verbosity(entry_verbosity)
        .test(logger_options.test)
        .build();
    Logger::from(&entry_logger_options)
}
