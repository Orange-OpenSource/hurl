/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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

use chrono::Utc;
use hurl_core::ast::VersionValue::VersionAnyLegacy;
use hurl_core::ast::{Body, Bytes, Entry, HurlFile, MultilineString, Request, Response, Retry};
use hurl_core::error::Error;
use hurl_core::parser;

use crate::http::{Call, Client};
use crate::runner::runner_options::RunnerOptions;
use crate::runner::{entry, options, EntryResult, HurlResult, Value};
use crate::util::logger::{ErrorFormat, Logger, LoggerOptions};

/// Runs a Hurl `content` and returns a [`HurlResult`] upon completion.
///
/// If `content` is a syntactically correct Hurl file, an `[HurlResult]` is always returned on
/// run completion. The success or failure of the run (due to assertions checks, runtime failures
/// etc..) can be read in the `[HurlResult]` `success` field. If `content` is not syntactically
/// correct, a parsing error is returned. This is the only possible way for this function to fail.
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
    let mut logger = Logger::from(logger_options);

    // Try to parse the content
    let hurl_file = parser::parse_hurl_file(content);
    let hurl_file = match hurl_file {
        Ok(h) => h,
        Err(e) => {
            logger.error_parsing_rich(content, &e);
            return Err(e.description());
        }
    };

    // Now, we have a syntactically correct HurlFile instance, we can run it.
    log_run_info(&hurl_file, runner_options, variables, &logger);

    let result = run_entries(
        &hurl_file.entries,
        content,
        runner_options,
        variables,
        &mut logger,
    );

    Ok(result)
}

/// Runs a list of `entries` and returns a [`HurlResult`] upon completion.
/// `content` is the original source content, used to construct `entries`. It is used to construct
/// rich error messages with annotated source code.
fn run_entries(
    entries: &[Entry],
    content: &str,
    runner_options: &RunnerOptions,
    variables: &HashMap<String, Value>,
    logger: &mut Logger,
) -> HurlResult {
    let mut http_client = Client::new();
    let mut entries_result = vec![];
    let mut variables = variables.clone();
    let mut entry_index = 1;
    let mut retry_count = 1;
    let n = if let Some(to_entry) = runner_options.to_entry {
        to_entry
    } else {
        entries.len()
    };
    let default_verbosity = logger.verbosity;
    let start = Instant::now();
    let timestamp = Utc::now().timestamp();

    // Main loop processing each entry.
    // The `entry_index` is not always incremented of each loop tick: an entry can be retried upon
    // errors for instance. Each entry is executed with options that are computed from the global
    // runner options and the "overridden" request options.
    loop {
        if entry_index > n {
            break;
        }
        let entry = &entries[entry_index - 1];

        if let Some(pre_entry) = runner_options.pre_entry {
            let exit = pre_entry(entry.clone());
            if exit {
                break;
            }
        }

        // We compute the new logger for this entry, before entering into the `run` function because
        // entry options can modify the logger and we want the preamble "Executing entry..." to be
        // displayed based on the entry level verbosity.
        logger.verbosity = default_verbosity;
        let entry_verbosity = options::get_entry_verbosity(entry, default_verbosity, &variables);
        if let Ok(entry_verbosity) = entry_verbosity {
            logger.verbosity = entry_verbosity;
        }

        logger.debug_important(
            "------------------------------------------------------------------------------",
        );
        logger.debug_important(&format!("Executing entry {entry_index}"));

        warn_deprecated(entry, logger);

        logger.test_progress(entry_index, n);

        // The real execution of the entry happens here, with the overridden entry options.
        let options = options::get_entry_options(entry, runner_options, &mut variables, logger);
        let entry_result = match &options {
            Err(error) => EntryResult {
                entry_index,
                source_info: entry.source_info(),
                errors: vec![error.clone()],
                ..Default::default()
            },
            Ok(options) => {
                if options.skip {
                    logger.debug("");
                    logger.debug_important(&format!("Entry {entry_index} has been skipped"));
                    entry_index += 1;
                    continue;
                }

                let delay = options.delay;
                let delay_ms = delay.as_millis();
                if delay_ms > 0 {
                    logger.debug("");
                    logger.debug_important(&format!(
                        "Delay entry {entry_index} (x{retry_count} by {delay_ms} ms)"
                    ));
                    thread::sleep(delay);
                };

                entry::run(
                    entry,
                    entry_index,
                    &mut http_client,
                    &mut variables,
                    options,
                    logger,
                )
            }
        };

        // Check if we need to retry.
        let has_error = !entry_result.errors.is_empty();
        let (retry_opts, retry_interval) = match &options {
            Ok(options) => (options.retry, options.retry_interval),
            Err(_) => (runner_options.retry, runner_options.retry_interval),
        };
        // The retry threshold can only reached with a finite positive number of retries
        let retry_max_reached = if let Retry::Finite(r) = retry_opts {
            retry_count > r
        } else {
            false
        };
        // If `retry_max_reached` is true, we print now a warning, before displaying any assert
        // error so any potential error is the last thing displayed to the user.
        // If `retry_max_reached` is not true (for instance `retry`is true, or there is no error
        // we first log the error and a potential warning about retrying.
        logger.test_erase_line();
        if retry_max_reached {
            logger.debug_important("Retry max count reached, no more retry");
            logger.debug("");
        }

        // We logs eventual errors, only if we're not retrying the current entry...
        let retry = !matches!(retry_opts, Retry::None) && !retry_max_reached && has_error;
        if has_error {
            log_errors(&entry_result, content, retry, logger);
        }

        // When --output is overridden on a request level, we output the HTTP response only if the
        // call has succeeded.
        if let Ok(RunnerOptions {
            output: Some(output),
            ..
        }) = options
        {
            if !has_error {
                // TODO: make output write and access error as part of entry result errors.
                // For the moment, we deal the --output request failure as a simple warning and not
                // an error. If we want to treat it as an error, we've to add it to the current
                // `entry_result` errors, and optionally deals with retry if we can't write to the
                // specified path.
                let source_info = entry.source_info();
                if let Err(error) =
                    entry_result.write_response(&output, &runner_options.context_dir, source_info)
                {
                    logger.warning(&error.fixme());
                }
            }
        }
        entries_result.push(entry_result);

        if retry {
            let delay = retry_interval.as_millis();
            logger.debug("");
            logger.debug_important(&format!(
                "Retry entry {entry_index} (x{retry_count} pause {delay} ms)"
            ));
            retry_count += 1;
            // If we retry the entry, we do not want to display a 'blank' progress bar during the
            // sleep delay. During the pause, we artificially show the previously erased progress
            // line.
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
        if !runner_options.continue_on_error && has_error {
            break;
        }

        // We pass to the next entry
        entry_index += 1;
        retry_count = 1;
    }

    let time_in_ms = start.elapsed().as_millis();
    let cookies = http_client.get_cookie_storage();
    let success = is_success(&entries_result);
    HurlResult {
        entries: entries_result,
        time_in_ms,
        success,
        cookies,
        timestamp,
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

// Returns the list of options that have non-default values.
fn get_non_default_options(options: &RunnerOptions) -> Vec<(&'static str, String)> {
    let default_options = RunnerOptions::default();

    let mut non_default_options = vec![];

    if options.continue_on_error != default_options.continue_on_error {
        non_default_options.push(("continue_on_error", options.continue_on_error.to_string()));
    }

    if options.delay != default_options.delay {
        // FIXME: the cast to u64 seems not necessary.
        //  If we dont cast from u128 and try to format! or println!
        //  we have a segfault on Alpine Docker images and Rust 1.68.0, whereas it was
        //  ok with Rust >= 1.67.0.
        non_default_options.push(("delay", format!("{}ms", options.delay.as_millis() as u64)));
    }

    if options.follow_location != default_options.follow_location {
        non_default_options.push(("follow redirect", options.follow_location.to_string()));
    }

    if options.insecure != default_options.insecure {
        non_default_options.push(("insecure", options.insecure.to_string()));
    }

    if options.max_redirect != default_options.max_redirect {
        if let Some(n) = options.max_redirect {
            non_default_options.push(("max redirect", n.to_string()));
        }
    }

    if options.proxy != default_options.proxy {
        if let Some(proxy) = &options.proxy {
            non_default_options.push(("proxy", proxy.to_string()));
        }
    }

    if options.retry != default_options.retry {
        non_default_options.push(("retry", options.retry.to_string()));
    }

    if options.unix_socket != default_options.unix_socket {
        if let Some(unix_socket) = &options.unix_socket {
            non_default_options.push(("unix socket", unix_socket.to_string()));
        }
    }

    non_default_options
}

/// Logs various debug information at the start of `hurl_file` run.
fn log_run_info(
    hurl_file: &HurlFile,
    runner_options: &RunnerOptions,
    variables: &HashMap<String, Value>,
    logger: &Logger,
) {
    if logger.verbosity.is_some() {
        let non_default_options = get_non_default_options(runner_options);
        if !non_default_options.is_empty() {
            logger.debug_important("Options:");
            for (name, value) in non_default_options.iter() {
                logger.debug(&format!("    {name}: {value}"));
            }
        }
    }

    if !variables.is_empty() {
        logger.debug_important("Variables:");
        for (name, value) in variables.iter() {
            logger.debug(&format!("    {name}: {value}"));
        }
    }
    if let Some(to_entry) = runner_options.to_entry {
        logger.debug(&format!(
            "Executing {}/{} entries",
            to_entry,
            hurl_file.entries.len()
        ));
    }
}

/// Logs runner `errors`.
/// If we're going to `retry` the entry, we log errors only in verbose. Otherwise, we log error on stderr.
fn log_errors(entry_result: &EntryResult, content: &str, retry: bool, logger: &Logger) {
    if retry {
        entry_result
            .errors
            .iter()
            .for_each(|e| logger.debug_error(content, e, entry_result.source_info));
        return;
    }

    if logger.error_format == ErrorFormat::Long {
        if let Some(Call { response, .. }) = entry_result.calls.last() {
            response.log_info_all(logger);
        }
    }
    entry_result
        .errors
        .iter()
        .for_each(|error| logger.error_runtime_rich(content, error, entry_result.source_info));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::runner::RunnerOptionsBuilder;

    #[test]
    fn get_non_default_options_returns_empty_when_default() {
        let options = RunnerOptions::default();
        assert!(get_non_default_options(&options).is_empty());
    }

    #[test]
    fn get_non_default_options_returns_only_non_default_options() {
        let options = RunnerOptionsBuilder::new()
            .delay(std::time::Duration::from_millis(500))
            .build();

        let non_default_options = get_non_default_options(&options);
        assert_eq!(non_default_options.len(), 1);

        let first_non_default = non_default_options.first().unwrap();

        assert_eq!(first_non_default.0, "delay");
        assert_eq!(first_non_default.1, "500ms");
    }
}
