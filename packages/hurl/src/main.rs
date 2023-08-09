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
mod cli;

use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;
use std::{env, process};

use crate::cli::options::OptionsError;
use colored::control;
use hurl::report::{html, junit, tap};
use hurl::runner::HurlResult;
use hurl::util::logger::{BaseLogger, Logger, LoggerOptionsBuilder, Verbosity};
use hurl::{output, runner};

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
    /// Filename of the content
    filename: String,
    hurl_result: HurlResult,
}

/// Executes Hurl entry point.
fn main() {
    init_colored();

    let opts = match cli::options::parse() {
        Ok(v) => v,
        Err(e) => match e {
            OptionsError::Info(message) => {
                print!("{message}");
                process::exit(EXIT_OK);
            }
            OptionsError::Error(message) => {
                eprintln!("error: {message}");
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
    let mut runs = vec![];

    for (current, filename) in opts.input_files.iter().enumerate() {
        // We check the input file existence and check that we can read its contents.
        // Once the preconditions succeed, we can parse the Hurl file, and run it.
        if filename != "-" && !Path::new(filename).exists() {
            let message = format!("hurl: cannot access '{filename}': No such file or directory");
            exit_with_error(&message, EXIT_ERROR_PARSING, &base_logger);
        }
        let content = cli::read_to_string(filename.as_str());
        let content = unwrap_or_exit(content, EXIT_ERROR_PARSING, &base_logger);

        let verbosity = Verbosity::from(opts.verbose, opts.very_verbose);
        let logger_options = LoggerOptionsBuilder::new()
            .color(opts.color)
            .error_format(opts.error_format.clone().into())
            .filename(filename)
            .progress_bar(opts.progress_bar)
            .test(opts.test)
            .verbosity(verbosity)
            .build();
        let logger = Logger::from(&logger_options);
        let total = opts.input_files.len();
        logger.test_running(current + 1, total);

        // Run our Hurl file now
        let hurl_result = execute(&content, filename, current_dir, &opts);
        let hurl_result = match hurl_result {
            Ok(h) => h,
            Err(_) => process::exit(EXIT_ERROR_PARSING),
        };

        logger.test_completed(&hurl_result);
        let success = hurl_result.success;

        // We can output the result, either the raw body or a structured JSON representation.
        let output_body = success
            && !opts.interactive
            && matches!(opts.output_type, cli::OutputType::ResponseBody);
        if output_body {
            let include_headers = opts.include;
            let result = output::write_body(
                &hurl_result,
                filename,
                include_headers,
                opts.color,
                &opts.output,
                &logger,
            );
            unwrap_or_exit(result, EXIT_ERROR_RUNTIME, &base_logger);
        }
        if matches!(opts.output_type, cli::OutputType::Json) {
            let result = output::write_json(&hurl_result, &content, filename, &opts.output);
            unwrap_or_exit(result, EXIT_ERROR_RUNTIME, &base_logger);
        }

        let run = HurlRun {
            content,
            filename: filename.to_string(),
            hurl_result,
        };
        runs.push(run);
    }

    if let Some(filename) = opts.junit_file {
        base_logger.debug(format!("Writing JUnit report to {filename}").as_str());
        let result = create_junit_report(&runs, &filename);
        unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
    }

    if let Some(filename) = opts.tap_file {
        base_logger.debug(format!("Writing TAP report to {filename}").as_str());
        let result = create_tap_report(&runs, &filename);
        unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
    }

    if let Some(dir) = opts.html_dir {
        base_logger.debug(format!("Writing HTML report to {}", dir.display()).as_str());
        let result = create_html_report(&runs, &dir);
        unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
    }

    if let Some(filename) = opts.cookie_output_file {
        base_logger.debug(format!("Writing cookies to {filename}").as_str());
        let result = create_cookies_file(&runs, &filename);
        unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
    }

    if opts.test {
        let duration = start.elapsed().as_millis();
        let summary = get_summary(&runs, duration);
        base_logger.info(summary.as_str());
    }

    process::exit(exit_code(&runs));
}

/// Runs a Hurl `content` and returns a result.
fn execute(
    content: &str,
    filename: &str,
    current_dir: &Path,
    cli_options: &cli::options::Options,
) -> Result<HurlResult, String> {
    let variables = &cli_options.variables;
    let runner_options = cli_options.to_runner_options(filename, current_dir);
    let logger_options = cli_options.to_logger_options(filename);
    runner::run(content, &runner_options, variables, &logger_options)
}

#[cfg(target_family = "unix")]
fn init_colored() {
    control::set_override(true);
}

#[cfg(target_family = "windows")]
fn init_colored() {
    control::set_override(true);
    control::set_virtual_terminal(true).expect("set virtual terminal");
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

/// Create a JUnit report for this run.
fn create_junit_report(runs: &[HurlRun], filename: &str) -> Result<(), cli::CliError> {
    let testcases: Vec<junit::Testcase> = runs
        .iter()
        .map(|r| junit::Testcase::from(&r.hurl_result, &r.content, &r.filename))
        .collect();
    junit::write_report(filename, &testcases)?;
    Ok(())
}

/// Create a TAP report for this run.
fn create_tap_report(runs: &[HurlRun], filename: &str) -> Result<(), cli::CliError> {
    let testcases: Vec<tap::Testcase> = runs
        .iter()
        .map(|r| tap::Testcase::from(&r.hurl_result, &r.filename))
        .collect();
    tap::write_report(filename, &testcases)?;
    Ok(())
}

/// Create an HTML report for this run.
fn create_html_report(runs: &[HurlRun], dir_path: &Path) -> Result<(), cli::CliError> {
    // We ensure that the containing folder exists.
    std::fs::create_dir_all(dir_path.join("store")).unwrap();

    let mut testcases = vec![];
    for run in runs.iter() {
        let testcase = html::Testcase::from(&run.hurl_result, &run.filename);
        testcase.write_html(&run.content, &run.hurl_result.entries, dir_path)?;
        testcases.push(testcase);
    }
    html::write_report(dir_path, &testcases)?;
    Ok(())
}

/// Returns an exit code for a list of HurlResult.
fn exit_code(runs: &[HurlRun]) -> i32 {
    let mut count_errors_runner = 0;
    let mut count_errors_assert = 0;
    for run in runs.iter() {
        let errors = run.hurl_result.errors();
        if errors.is_empty() {
        } else if errors.iter().filter(|e| !e.assert).count() == 0 {
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

fn create_cookies_file(runs: &[HurlRun], filename: &str) -> Result<(), cli::CliError> {
    let mut file = match std::fs::File::create(filename) {
        Err(why) => {
            return Err(cli::CliError {
                message: format!("Issue writing to {filename}: {why:?}"),
            });
        }
        Ok(file) => file,
    };
    let mut s = r#"# Netscape HTTP Cookie File
# This file was generated by Hurl

"#
    .to_string();
    match runs.first() {
        None => {
            return Err(cli::CliError {
                message: "Issue fetching results".to_string(),
            });
        }
        Some(run) => {
            for cookie in run.hurl_result.cookies.iter() {
                s.push_str(&cookie.to_string());
                s.push('\n');
            }
        }
    }

    if let Err(why) = file.write_all(s.as_bytes()) {
        return Err(cli::CliError {
            message: format!("Issue writing to {filename}: {why:?}"),
        });
    }
    Ok(())
}

/// Returns the text summary of this Hurl runs.
fn get_summary(runs: &[HurlRun], duration: u128) -> String {
    let total = runs.len();
    let success = runs.iter().filter(|r| r.hurl_result.success).count();
    let success_percent = 100.0 * success as f32 / total as f32;
    let failed = total - success;
    let failed_percent = 100.0 * failed as f32 / total as f32;
    format!(
        "--------------------------------------------------------------------------------\n\
             Executed files:  {total}\n\
             Succeeded files: {success} ({success_percent:.1}%)\n\
             Failed files:    {failed} ({failed_percent:.1}%)\n\
             Duration:        {duration} ms\n"
    )
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use hurl::runner::EntryResult;

    #[test]
    fn create_run_summary() {
        fn new_run(success: bool, entries_count: usize) -> HurlRun {
            let dummy_entry = EntryResult {
                entry_index: 0,
                calls: vec![],
                captures: vec![],
                asserts: vec![],
                errors: vec![],
                time_in_ms: 0,
                compressed: false,
            };
            HurlRun {
                content: String::new(),
                filename: String::new(),
                hurl_result: HurlResult {
                    entries: vec![dummy_entry; entries_count],
                    time_in_ms: 0,
                    success,
                    cookies: vec![],
                },
            }
        }

        let runs = vec![new_run(true, 10), new_run(true, 20), new_run(true, 4)];
        let duration = 128;
        let summary = get_summary(&runs, duration);
        assert_eq!(
            summary,
            "--------------------------------------------------------------------------------\n\
             Executed files:  3\n\
             Succeeded files: 3 (100.0%)\n\
             Failed files:    0 (0.0%)\n\
             Duration:        128 ms\n"
        );

        let runs = vec![new_run(true, 10), new_run(false, 10), new_run(true, 40)];
        let duration = 200;
        let summary = get_summary(&runs, duration);
        assert_eq!(
            summary,
            "--------------------------------------------------------------------------------\n\
            Executed files:  3\n\
            Succeeded files: 2 (66.7%)\n\
            Failed files:    1 (33.3%)\n\
            Duration:        200 ms\n"
        );
    }
}
