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
mod cli;
mod run;

use std::collections::HashSet;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;
use std::{env, process, thread};

use hurl::report::{curl, html, json, junit, tap};
use hurl::runner;
use hurl::runner::HurlResult;
use hurl::util::redacted::Redact;
use hurl_core::input::Input;
use hurl_core::text;

use crate::cli::options::{CliOptions, CliOptionsError};
use crate::cli::{BaseLogger, CliError};

const EXIT_OK: i32 = 0;
const EXIT_ERROR_COMMANDLINE: i32 = 1;
const EXIT_ERROR_PARSING: i32 = 2;
const EXIT_ERROR_RUNTIME: i32 = 3;
const EXIT_ERROR_ASSERT: i32 = 4;
const EXIT_ERROR_UNDEFINED: i32 = 127;

/// Structure that stores the result of an Hurl file execution, and the content of the file.
#[derive(Clone, Debug, PartialEq, Eq)]
struct HurlRun {
    /// Source string for this [`HurlFile`]
    content: String,
    /// Content's source file
    filename: Input,
    hurl_result: HurlResult,
}

/// Executes Hurl entry point.
fn main() {
    text::init_crate_colored();

    let opts = match cli::options::parse() {
        Ok(v) => v,
        Err(e) => match e {
            CliOptionsError::Info(_) => {
                print!("{e}");
                process::exit(EXIT_OK);
            }
            _ => {
                eprintln!("{e}");
                process::exit(EXIT_ERROR_COMMANDLINE);
            }
        },
    };

    // We create a basic logger that can just display info, warning or error generic messages.
    // We'll use a more advanced logger for rich error report when running Hurl files.
    let verbose = opts.verbose || opts.very_verbose || opts.interactive;
    let base_logger = BaseLogger::new(opts.color, verbose);
    let current_dir = env::current_dir();
    let current_dir = unwrap_or_exit(current_dir, EXIT_ERROR_UNDEFINED, &base_logger);
    let current_dir = current_dir.as_path();
    let start = Instant::now();

    let runs = if opts.parallel {
        let available = unwrap_or_exit(
            thread::available_parallelism(),
            EXIT_ERROR_UNDEFINED,
            &base_logger,
        );
        let workers_count = opts.jobs.unwrap_or(available.get());
        base_logger.debug(&format!("Parallel run using {workers_count} workers"));

        run::run_par(&opts.input_files, current_dir, &opts, workers_count)
    } else {
        run::run_seq(&opts.input_files, current_dir, &opts)
    };
    let runs = match runs {
        Ok(r) => r,
        Err(CliError::IO(msg)) => exit_with_error(&msg, EXIT_ERROR_PARSING, &base_logger),
        // In case of parsing error, there is no error because the display of parsing error has been
        // done in the execution of the Hurl files, inside the crates (and not in the main).
        Err(CliError::Parsing) => exit_with_error("", EXIT_ERROR_PARSING, &base_logger),
        Err(CliError::Runtime(msg)) => exit_with_error(&msg, EXIT_ERROR_RUNTIME, &base_logger),
    };

    // Compute duration of the test here to not take reports writings into account.
    let duration = start.elapsed();

    // Write HTML, JUnit, TAP reports on disk.
    if has_report(&opts) {
        let ret = export_results(&runs, &opts, &base_logger);
        unwrap_or_exit(ret, EXIT_ERROR_UNDEFINED, &base_logger);
    }

    if opts.test {
        let summary = cli::summary(&runs, duration);
        base_logger.info(summary.as_str());
    }

    process::exit(exit_code(&runs));
}

/// Unwraps a `result` or exit with message.
fn unwrap_or_exit<T, E>(result: Result<T, E>, code: i32, logger: &BaseLogger) -> T
where
    E: std::fmt::Display,
{
    match result {
        Ok(v) => v,
        Err(e) => exit_with_error(&e.to_string(), code, logger),
    }
}

/// Prints an error message and exits the current process with an exit code.
fn exit_with_error(message: &str, code: i32, logger: &BaseLogger) -> ! {
    if !message.is_empty() {
        logger.error(message);
    }
    process::exit(code);
}

/// Returns `true` if any kind of report should be created, `false` otherwise.
fn has_report(opts: &CliOptions) -> bool {
    opts.curl_file.is_some()
        || opts.junit_file.is_some()
        || opts.tap_file.is_some()
        || opts.html_dir.is_some()
        || opts.json_report_dir.is_some()
        || opts.cookie_output_file.is_some()
}

/// Writes `runs` results on file, in HTML, TAP, JUnit or Cookie file format.
fn export_results(
    runs: &[HurlRun],
    opts: &CliOptions,
    logger: &BaseLogger,
) -> Result<(), CliError> {
    // Compute secrets from the result. As secrets can be redacted during execution, we can't
    // consider only secrets introduced from cli, we have to get secrets produced during execution.
    // We remove identical secrets as there may be a lot of identical secrets (those that come
    // from the command line for instance)
    let secrets = runs
        .iter()
        .flat_map(|r| r.hurl_result.variables.secrets())
        .collect::<HashSet<_>>();
    let secrets = secrets.iter().map(|s| s.as_ref()).collect::<Vec<_>>();

    if let Some(file) = &opts.curl_file {
        create_curl_export(runs, file, &secrets)?;
    }
    if let Some(file) = &opts.junit_file {
        logger.debug(&format!("Writing JUnit report to {}", file.display()));
        create_junit_report(runs, file, &secrets)?;
    }
    if let Some(file) = &opts.tap_file {
        // TAP files doesn't need to be redacted, they don't expose any logs apart from files names.
        logger.debug(&format!("Writing TAP report to {}", file.display()));
        create_tap_report(runs, file)?;
    }
    if let Some(dir) = &opts.html_dir {
        logger.debug(&format!("Writing HTML report to {}", dir.display()));
        create_html_report(runs, dir, &secrets)?;
    }
    if let Some(dir) = &opts.json_report_dir {
        logger.debug(&format!("Writing JSON report to {}", dir.display()));
        create_json_report(runs, dir, &secrets)?;
    }
    if let Some(file) = &opts.cookie_output_file {
        logger.debug(&format!("Writing cookies to {}", file.display()));
        create_cookies_file(runs, file, &secrets)?;
    }
    Ok(())
}

/// Creates an export of all curl commands for this run.
fn create_curl_export(runs: &[HurlRun], filename: &Path, secrets: &[&str]) -> Result<(), CliError> {
    let results = runs.iter().map(|r| &r.hurl_result).collect::<Vec<_>>();
    curl::write_curl(&results, filename, secrets)?;
    Ok(())
}

/// Creates a JUnit report for this run.
fn create_junit_report(
    runs: &[HurlRun],
    filename: &Path,
    secrets: &[&str],
) -> Result<(), CliError> {
    let testcases = runs
        .iter()
        .map(|r| junit::Testcase::from(&r.hurl_result, &r.content, &r.filename))
        .collect::<Vec<_>>();
    junit::write_report(filename, &testcases, secrets)?;
    Ok(())
}

/// Creates a TAP report for this run.
fn create_tap_report(runs: &[HurlRun], filename: &Path) -> Result<(), CliError> {
    let testcases = runs
        .iter()
        .map(|r| tap::Testcase::from(&r.hurl_result, &r.filename))
        .collect::<Vec<_>>();
    tap::write_report(filename, &testcases)?;
    Ok(())
}

/// Creates an HTML report for this run.
fn create_html_report(runs: &[HurlRun], dir_path: &Path, secrets: &[&str]) -> Result<(), CliError> {
    // We ensure that the containing folder exists.
    let store_path = dir_path.join("store");
    std::fs::create_dir_all(&store_path)?;

    let mut testcases = vec![];
    for run in runs.iter() {
        let result = &run.hurl_result;
        let testcase = html::Testcase::from(result, &run.filename);
        testcase.write_html(&run.content, &result.entries, &store_path, secrets)?;
        testcases.push(testcase);
    }
    html::write_report(dir_path, &testcases)?;
    Ok(())
}

/// Creates an JSON report for this run.
fn create_json_report(runs: &[HurlRun], dir_path: &Path, secrets: &[&str]) -> Result<(), CliError> {
    // We ensure that the containing folder exists.
    let store_path = dir_path.join("store");
    std::fs::create_dir_all(&store_path)?;

    let testcases = runs
        .iter()
        .map(|r| json::Testcase::new(&r.hurl_result, &r.content, &r.filename))
        .collect::<Vec<_>>();

    let index_path = dir_path.join("report.json");
    json::write_report(&index_path, &testcases, &store_path, secrets)?;
    Ok(())
}

/// Returns an exit code for a list of HurlResult.
fn exit_code(runs: &[HurlRun]) -> i32 {
    let mut count_errors_runner = 0;
    let mut count_errors_assert = 0;
    for run in runs.iter() {
        let errors = run.hurl_result.errors();
        if errors.is_empty() {
        } else if errors.iter().filter(|(error, _)| !error.assert).count() == 0 {
            count_errors_assert += 1;
        } else {
            count_errors_runner += 1;
        }
    }
    if count_errors_runner > 0 {
        EXIT_ERROR_RUNTIME
    } else if count_errors_assert > 0 {
        EXIT_ERROR_ASSERT
    } else {
        EXIT_OK
    }
}

/// Export cookies for this run to `filename` file.
///
/// The file format for the cookies is [Netscape cookie format](http://www.cookiecentral.com/faq/#3.5).
fn create_cookies_file(
    runs: &[HurlRun],
    filename: &Path,
    secrets: &[&str],
) -> Result<(), CliError> {
    if let Err(err) = hurl::util::path::create_dir_all(filename) {
        return Err(CliError::IO(format!(
            "Issue creating parent directories for {}: {err:?}",
            filename.display()
        )));
    }

    let mut file = match std::fs::File::create(filename) {
        Err(why) => {
            return Err(CliError::IO(format!(
                "Issue writing to {}: {why:?}",
                filename.display()
            )));
        }
        Ok(file) => file,
    };
    let mut s = r#"# Netscape HTTP Cookie File
# This file was generated by Hurl

"#
    .to_string();

    // TODO: We only serialize the first file. I don't see any valid reason why we're not
    // serializing every file see issue <https://github.com/Orange-OpenSource/hurl/issues/2537>
    match runs.first() {
        None => {
            return Err(CliError::IO("Issue fetching results".to_string()));
        }
        Some(run) => {
            for cookie in run.hurl_result.cookies.iter() {
                s.push_str(&cookie.redact(secrets));
                s.push('\n');
            }
        }
    }

    if let Err(why) = file.write_all(s.as_bytes()) {
        return Err(CliError::IO(format!(
            "Issue writing to {}: {why:?}",
            filename.display()
        )));
    }
    Ok(())
}
