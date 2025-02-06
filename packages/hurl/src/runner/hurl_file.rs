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
use std::thread;
use std::time::Instant;

use chrono::Utc;
use hurl_core::ast::{Entry, OptionKind, SourceInfo};
use hurl_core::error::DisplaySourceError;
use hurl_core::input::Input;
use hurl_core::parser;
use hurl_core::typing::Count;

use crate::http::{Call, Client};
use crate::runner::event::EventListener;
use crate::runner::runner_options::RunnerOptions;
use crate::runner::{entry, options, EntryResult, HurlResult, VariableSet};
use crate::util::logger::{ErrorFormat, Logger, LoggerOptions};
use crate::util::term::{Stderr, Stdout, WriteMode};

/// Runs a Hurl `content` and returns a [`HurlResult`] upon completion.
///
/// If `content` is a syntactically correct Hurl file, an [`HurlResult`] is always returned on
/// run completion. The success or failure of the run (due to assertions checks, runtime failures
/// etc...) can be read in the [`HurlResult`] `success` field. If `content` is not syntactically
/// correct, a parsing error is returned. This is the only possible way for this function to fail.
///
/// `filename` indicates an optional file source, used when displaying errors.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use hurl::runner;
/// use hurl::runner::{Value, RunnerOptionsBuilder, VariableSet};
/// use hurl::util::logger::{LoggerOptionsBuilder, Verbosity};
/// use hurl_core::input::Input;
///
/// // A simple Hurl sample
/// let content = r#"
/// GET http://localhost:8000/hello
/// HTTP 200
/// "#;
///
/// let filename = Input::new("sample.hurl");
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
/// let mut variables = VariableSet::new();
/// variables.insert("name".to_string(), Value::String("toto".to_string())).unwrap();
///
/// // Run the Hurl sample
/// let result = runner::run(
///     content,
///     Some(filename).as_ref(),
///     &runner_opts,
///     &variables,
///     &logger_opts
/// );
/// assert!(result.unwrap().success);
/// ```
pub fn run(
    content: &str,
    filename: Option<&Input>,
    runner_options: &RunnerOptions,
    variables: &VariableSet,
    logger_options: &LoggerOptions,
) -> Result<HurlResult, String> {
    // In this method, we run Hurl content sequentially. Standard output and standard error messages
    // are written immediately (in parallel mode, we'll use buffered standard output and error).
    let mut stdout = Stdout::new(WriteMode::Immediate);
    let stderr = Stderr::new(WriteMode::Immediate);

    // We also create a common logger for this run (logger verbosity can eventually be mutated on
    // each entry).
    let secrets = variables.secrets();
    let mut logger = Logger::new(logger_options, stderr, &secrets);

    // Try to parse the content
    let hurl_file = parser::parse_hurl_file(content);
    let hurl_file = match hurl_file {
        Ok(h) => h,
        Err(e) => {
            logger.error_parsing_rich(content, filename, &e);
            return Err(e.description());
        }
    };

    // Now, we have a syntactically correct HurlFile instance, we can run it.
    let result = run_entries(
        &hurl_file.entries,
        content,
        filename,
        runner_options,
        variables,
        &mut stdout,
        None,
        &mut logger,
    );

    if result.success && result.entries.last().is_none() {
        let filename = filename.map_or(String::new(), |f| f.to_string());
        logger.warning(&format!("No entry have been executed for file {filename}"));
    }

    Ok(result)
}

#[allow(clippy::too_many_arguments)]
/// Runs a list of `entries` and returns a [`HurlResult`] upon completion.
///
/// `content` is the original source content, used to construct `entries`. It is used to construct
/// rich error messages with annotated source code.
/// New entry run events are reported to `progress` and are usually used to display a progress bar
/// in test mode.
pub fn run_entries(
    entries: &[Entry],
    content: &str,
    filename: Option<&Input>,
    runner_options: &RunnerOptions,
    variables: &VariableSet,
    stdout: &mut Stdout,
    listener: Option<&dyn EventListener>,
    logger: &mut Logger,
) -> HurlResult {
    let mut http_client = Client::new();
    let mut entries_result = vec![];
    let mut variables = variables.clone();
    let mut entry_index = runner_options.from_entry.unwrap_or(1);
    let mut repeat_count = 0;
    let n = runner_options.to_entry.unwrap_or(entries.len());
    let default_verbosity = logger.verbosity;
    let start = Instant::now();
    let timestamp = Utc::now().timestamp();

    log_run_info(entries, runner_options, &variables, logger);

    // Main loop processing each entry.
    // The `entry_index` is not always incremented of each loop tick: an entry can be retried upon
    // errors for instance. Each entry is executed with options that are computed from the global
    // runner options and the "overridden" request options.
    // See <docs/spec/runner/run_cycle.md>
    loop {
        if entry_index > n {
            break;
        }
        let entry = &entries[entry_index - 1];

        if let Some(pre_entry) = runner_options.pre_entry {
            let exit = pre_entry(entry);
            if exit {
                break;
            }
        }

        // We compute the new logger verbosity for this entry, before entering into the `run`
        // function because entry options can modify the logger verbosity and we want the preamble
        // "Executing entry..." to be displayed based on the entry level verbosity.
        logger.verbosity = default_verbosity;
        let entry_verbosity = options::get_entry_verbosity(entry, default_verbosity, &variables);
        if let Ok(entry_verbosity) = entry_verbosity {
            logger.verbosity = entry_verbosity;
        }

        log_run_entry(entry_index, logger);

        // We can report the progression of the run for --test mode.
        if let Some(listener) = listener {
            listener.on_running(entry_index - 1, n);
        }

        // The real execution of the entry happens here, first: we compute the overridden request
        // options.
        let options = options::get_entry_options(entry, runner_options, &mut variables, logger);
        if let Err(error) = &options {
            // If we have error evaluating request options, we consider it as a non retryable error
            // and either break the runner or go to the next entries.
            let entry_result = EntryResult {
                entry_index,
                source_info: entry.source_info(),
                errors: vec![error.clone()],
                ..Default::default()
            };
            log_errors(&entry_result, content, filename, false, logger);
            entries_result.push(entry_result);
            if runner_options.continue_on_error {
                entry_index += 1;
                continue;
            } else {
                break;
            }
        }

        let options = options.unwrap();

        // Should we skip?
        if options.skip {
            logger.debug("");
            logger.debug_important(&format!("Entry {entry_index} has been skipped"));
            entry_index += 1;
            continue;
        }

        // Repeat 0 is equivalent to skip.
        if options.repeat == Some(Count::Finite(0)) {
            logger.debug("");
            logger.debug_important(&format!("Entry {entry_index} is skipped (repeat 0 times)"));
            entry_index += 1;
            continue;
        }

        // Should we delay?
        let delay = options.delay;
        let delay_ms = delay.as_millis();
        if delay_ms > 0 {
            logger.debug("");
            logger.debug_important(&format!("Delay entry {entry_index} (pause {delay_ms} ms)"));
            thread::sleep(delay);
        };

        // Loop for executing HTTP run requests, with optional retry. Only "HTTP" errors in options
        // are taken into account for retry (errors while computing entry options and output error
        // are not retried).
        let results = run_request(
            entry,
            entry_index,
            content,
            filename,
            &mut http_client,
            &options,
            &mut variables,
            stdout,
            logger,
        );

        let has_error = results.last().is_some_and(|r| !r.errors.is_empty());

        entries_result.extend(results);

        if let Some(post_entry) = runner_options.post_entry {
            let exit = post_entry();
            if exit {
                break;
            }
        }
        if !runner_options.continue_on_error && has_error {
            break;
        }

        // We pass to the next entry if the repeat count is reached.
        repeat_count += 1;
        match options.repeat {
            None => {
                repeat_count = 0;
                entry_index += 1;
            }
            Some(Count::Finite(n)) => {
                if repeat_count >= n {
                    repeat_count = 0;
                    entry_index += 1;
                } else {
                    logger.debug_important(&format!(
                        "Repeat entry {entry_index} (x{repeat_count}/{n})"
                    ));
                }
            }
            Some(Count::Infinite) => {
                logger.debug_important(&format!("Repeat entry {entry_index} (x{repeat_count})"));
            }
        }
    }

    let duration = start.elapsed();
    let cookies = http_client.cookie_storage(logger);
    let success = is_success(&entries_result);
    HurlResult {
        entries: entries_result,
        duration,
        success,
        cookies,
        timestamp,
        variables,
    }
}

/// Runs an HTTP request and optional retry it until there are no HTTP errors. Returns a list of
/// [`EntryResult`].
#[allow(clippy::too_many_arguments)]
fn run_request(
    entry: &Entry,
    entry_index: usize,
    content: &str,
    filename: Option<&Input>,
    http_client: &mut Client,
    options: &RunnerOptions,
    variables: &mut VariableSet,
    stdout: &mut Stdout,
    logger: &mut Logger,
) -> Vec<EntryResult> {
    let mut results = vec![];
    let mut retry_count = 1;

    loop {
        let mut result = entry::run(entry, entry_index, http_client, variables, options, logger);

        // Check if we need to retry.
        let mut has_error = !result.errors.is_empty();

        // The retry threshold can only be reached with a finite positive number of retries
        let retry_max_reached = if let Some(Count::Finite(r)) = options.retry {
            retry_count > r
        } else {
            false
        };
        // If `retry_max_reached` is true, we print now a warning, before displaying any assert
        // error so any potential error is the last thing displayed to the user.
        // If `retry_max_reached` is not true (for instance `retry`is true, or there is no error
        // we first log the error and a potential warning about retrying.
        if retry_max_reached {
            logger.debug_important("Retry max count reached, no more retry");
            logger.debug("");
        }

        // We log eventual errors, only if we're not retrying the current entry...
        // The retry does not take into account a possible output Error
        let retry = options.retry.is_some() && !retry_max_reached && has_error;

        // When --output is overridden on a request level, we output the HTTP response only if the
        // call has succeeded. Output errors are not taken into account for retrying requests.
        if let Some(output) = &options.output {
            if !has_error {
                let source_info = get_output_source_info(entry);
                if let Err(error) =
                    result.write_response(output, &options.context_dir, stdout, source_info)
                {
                    result.errors.push(error);
                    has_error = true;
                }
            }
        }

        if has_error {
            log_errors(&result, content, filename, retry, logger);
        }
        results.push(result);

        // No retry, we leave the HTTP run requests loop.
        if !retry {
            break;
        }

        let delay = options.retry_interval.as_millis();
        logger.debug("");
        logger.debug_important(&format!(
            "Retry entry {entry_index} (x{retry_count} pause {delay} ms)"
        ));
        retry_count += 1;
        // If we retry the entry, we do not want to display a 'blank' progress bar during the
        // sleep delay. During the pause, we artificially show the previously erased progress
        // line.
        thread::sleep(options.retry_interval);

        // TODO: We keep this log because we don't want to change stderr with the changes
        // introduced by <https://github.com/Orange-OpenSource/hurl/issues/1973>
        log_run_entry(entry_index, logger);
    }

    results
}

/// Use source_info from output option if this option has been defined
fn get_output_source_info(entry: &Entry) -> SourceInfo {
    let mut source_info = entry.source_info();
    for option_entry in entry.request.options() {
        if let OptionKind::Output(value) = &option_entry.kind {
            source_info = value.source_info;
        }
    }
    source_info
}

/// Returns `true` if all the entries results are successful, `false` otherwise.
///
/// For a given list of entry results, only the last one on the same index is checked.
///
/// For instance:
///
/// - `entry result a`: `entry index 1` (retried)
/// - `entry result b`: `entry index 1`
/// - `entry result c`: `entry index 2`
/// - `entry result d`: `entry index 3` (retried)
/// - `entry result e`: `entry index 3`
///
/// Only the entry result b, c and e are checked for the success state.
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
        non_default_options.push(("max redirect", options.max_redirect.to_string()));
    }

    if options.proxy != default_options.proxy {
        if let Some(proxy) = &options.proxy {
            non_default_options.push(("proxy", proxy.to_string()));
        }
    }

    if options.retry != default_options.retry {
        let value = match options.retry {
            Some(retry) => retry.to_string(),
            None => "none".to_string(),
        };
        non_default_options.push(("retry", value));
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
    entries: &[Entry],
    runner_options: &RunnerOptions,
    variables: &VariableSet,
    logger: &mut Logger,
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

    let variables = variables
        .iter()
        .filter(|(_, variable)| !variable.is_secret())
        .collect::<Vec<_>>();
    if !variables.is_empty() {
        logger.debug_important("Variables:");
        for (name, variable) in variables.iter() {
            logger.debug(&format!("    {name}: {}", variable.value()));
        }
    }
    if let Some(to_entry) = runner_options.to_entry {
        logger.debug(&format!("Executing {to_entry}/{} entries", entries.len()));
    }
}

/// Logs runner `errors`.
/// If we're going to `retry` the entry, we log errors only in verbose. Otherwise, we log error on stderr.
fn log_errors(
    entry_result: &EntryResult,
    content: &str,
    filename: Option<&Input>,
    retry: bool,
    logger: &mut Logger,
) {
    if retry {
        entry_result.errors.iter().for_each(|error| {
            logger.debug_error(content, filename, error, entry_result.source_info);
        });
        return;
    }

    if logger.error_format == ErrorFormat::Long {
        if let Some(Call { response, .. }) = entry_result.calls.last() {
            response.log_info_all(logger);
        }
    }
    entry_result.errors.iter().for_each(|error| {
        logger.error_runtime_rich(content, filename, error, entry_result.source_info);
    });
}

/// Logs the header indicating the begin of the entry run.
fn log_run_entry(entry_index: usize, logger: &mut Logger) {
    logger.debug_important(
        "------------------------------------------------------------------------------",
    );
    logger.debug_important(&format!("Executing entry {entry_index}"));
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
