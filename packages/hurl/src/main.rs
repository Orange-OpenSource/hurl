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

use std::io::prelude::*;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::time::Instant;

use atty::Stream;
use colored::*;

use hurl::cli;
use hurl::cli::{CliError, CliOptions, Logger, OutputType};
use hurl::http;
use hurl::http::{ContextDir, Verbosity};
use hurl::report;
use hurl::report::canonicalize_filename;
use hurl::runner;
use hurl::runner::{HurlResult, RunnerError, RunnerOptions};
use hurl::util::logger::BaseLogger;
use hurl_core::ast::{Pos, SourceInfo};
use hurl_core::error::Error;
use hurl_core::parser;

#[cfg(target_family = "unix")]
pub fn init_colored() {
    control::set_override(true);
}

const EXIT_OK: i32 = 0;
const EXIT_ERROR_COMMANDLINE: i32 = 1;
const EXIT_ERROR_PARSING: i32 = 2;
const EXIT_ERROR_RUNTIME: i32 = 3;
const EXIT_ERROR_ASSERT: i32 = 4;
const EXIT_ERROR_UNDEFINED: i32 = 127;

#[cfg(target_family = "windows")]
pub fn init_colored() {
    colored::control::set_override(true);
    colored::control::set_virtual_terminal(true).expect("set virtual terminal");
}

#[cfg(target_family = "unix")]
pub fn write_bytes(buf: &[u8]) -> Result<(), CliError> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(buf).map_err(|_| CliError {
        message: "Error writing output".to_string(),
    })
}

#[cfg(target_family = "windows")]
pub fn write_bytes(buf: &[u8]) -> Result<(), CliError> {
    if atty::is(Stream::Stdout) {
        println!("{}", String::from_utf8_lossy(buf));
        Ok(())
    } else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(buf).map_err(|_| CliError {
            message: "Error writing output".to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Progress {
    pub current: usize,
    pub total: usize,
}

/// Runs a Hurl file and returns a result.
///
/// # Arguments
///
/// * `filename` - Filename of the Hurl file, "-" is used for stdin
/// * `content` - Content of the Hurl file
/// * `current_dir` - The current directory of execution (absolute)
/// * `cli_options` - Options for this run
/// * `progress` - The progression of this execution
/// * `logger` - The logger
fn execute(
    filename: &str,
    content: &str,
    current_dir: &Path,
    cli_options: &CliOptions,
    progress: &Option<Progress>,
    logger: &Logger,
) -> HurlResult {
    if let Some(Progress { current, total }) = progress {
        logger.test_running(current + 1, *total);
    }

    match parser::parse_hurl_file(content) {
        Err(e) => {
            logger.error_rich(&e);
            std::process::exit(EXIT_ERROR_PARSING);
        }
        Ok(hurl_file) => {
            logger.debug_important("Options:");
            logger.debug(format!("    fail fast: {}", cli_options.fail_fast).as_str());
            logger.debug(format!("    insecure: {}", cli_options.insecure).as_str());
            logger.debug(format!("    follow redirect: {}", cli_options.follow_location).as_str());
            if let Some(n) = cli_options.max_redirect {
                logger.debug(format!("    max redirect: {}", n).as_str());
            }
            if let Some(proxy) = &cli_options.proxy {
                logger.debug(format!("    proxy: {}", proxy).as_str());
            }
            if !cli_options.variables.is_empty() {
                logger.debug_important("Variables:");
                for (name, value) in cli_options.variables.clone() {
                    logger.debug(format!("    {}: {}", name, value).as_str());
                }
            }
            if let Some(to_entry) = cli_options.to_entry {
                if to_entry < hurl_file.entries.len() {
                    logger.debug(
                        format!("Executing {}/{} entries", to_entry, hurl_file.entries.len())
                            .as_str(),
                    );
                } else {
                    logger.debug("Executing all entries");
                }
            }
            let cacert_file = cli_options.cacert_file.clone();
            let follow_location = cli_options.follow_location;
            let verbosity = match (cli_options.verbose, cli_options.very_verbose) {
                (true, true) => Some(Verbosity::VeryVerbose),
                (true, _) => Some(Verbosity::Verbose),
                _ => None,
            };
            let insecure = cli_options.insecure;
            let max_redirect = cli_options.max_redirect;
            let proxy = cli_options.proxy.clone();
            let no_proxy = cli_options.no_proxy.clone();
            let cookie_input_file = cli_options.cookie_input_file.clone();
            let timeout = cli_options.timeout;
            let connect_timeout = cli_options.connect_timeout;
            let user = cli_options.user.clone();
            let user_agent = cli_options.user_agent.clone();
            let compressed = cli_options.compressed;
            let file_root = match cli_options.file_root {
                Some(ref filename) => Path::new(filename),
                None => {
                    if filename == "-" {
                        current_dir
                    } else {
                        let path = Path::new(filename);
                        path.parent().unwrap()
                    }
                }
            };
            let context_dir = ContextDir::new(current_dir, file_root);

            let options = http::ClientOptions {
                cacert_file,
                follow_location,
                max_redirect,
                cookie_input_file,
                proxy,
                no_proxy,
                verbosity,
                insecure,
                timeout,
                connect_timeout,
                user,
                user_agent,
                compressed,
                context_dir: context_dir.clone(),
            };

            let mut client = http::Client::init(options);

            let pre_entry = if cli_options.interactive {
                cli::interactive::pre_entry
            } else {
                |_| false
            };
            let post_entry = if cli_options.interactive {
                cli::interactive::post_entry
            } else {
                || false
            };
            let fail_fast = cli_options.fail_fast;
            let variables = cli_options.variables.clone();
            let to_entry = cli_options.to_entry;
            let ignore_asserts = cli_options.ignore_asserts;
            let very_verbose = cli_options.very_verbose;
            let options = RunnerOptions {
                fail_fast,
                variables,
                to_entry,
                context_dir,
                ignore_asserts,
                very_verbose,
                pre_entry,
                post_entry,
            };
            let result = runner::run(&hurl_file, filename, &mut client, &options, logger);
            if cli_options.progress {
                logger.test_completed(result.success);
            }
            result
        }
    }
}

/// Unwraps a result or exit with message.
///
/// # Arguments
///
/// * result - Something to unwrap
/// * logger - A logger to log the error
fn unwrap_or_exit<T>(result: Result<T, CliError>, code: i32, logger: &BaseLogger) -> T {
    match result {
        Ok(v) => v,
        Err(e) => exit(&e.message, code, logger),
    }
}

/// Prints an error message and exits the current process with an exit code.
fn exit(message: &str, code: i32, logger: &BaseLogger) -> ! {
    logger.error(message);
    std::process::exit(code);
}

/// Executes Hurl entry point.
fn main() {
    let version_info = format!(
        "{} {}",
        clap::crate_version!(),
        http::libcurl_version_info().join(" ")
    );
    let app = cli::app(version_info.as_str());
    let matches = app.clone().get_matches();
    init_colored();

    let verbose = cli::has_flag(&matches, "verbose")
        || cli::has_flag(&matches, "very_verbose")
        || cli::has_flag(&matches, "interactive");
    let color = cli::output_color(&matches);
    let base_logger = BaseLogger::new(color, verbose);
    let cli_options = cli::parse_options(&matches, &base_logger);
    let cli_options = unwrap_or_exit(cli_options, EXIT_ERROR_UNDEFINED, &base_logger);

    let mut filenames = vec![];
    if let Some(values) = cli::get_strings(&matches, "INPUT") {
        for value in values {
            filenames.push(value.to_string());
        }
    };
    for filename in &cli_options.glob_files {
        filenames.push(filename.to_string());
    }

    if filenames.is_empty() && atty::is(Stream::Stdin) {
        let error = if app.clone().print_help().is_err() {
            "Panic during printing help"
        } else {
            ""
        };
        exit(error, EXIT_ERROR_COMMANDLINE, &base_logger);
    } else if filenames.is_empty() {
        filenames.push("-".to_string());
    }

    let current_dir = match std::env::current_dir() {
        Ok(c) => c,
        Err(error) => {
            exit(
                error.to_string().as_str(),
                EXIT_ERROR_UNDEFINED,
                &base_logger,
            );
        }
    };
    let current_dir = current_dir.as_path();
    let cookies_output_file = match cli_options.cookie_output_file.clone() {
        None => None,
        Some(filename) => {
            let result = cookies_output_file(&filename, filenames.len());
            let filename = unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
            Some(filename)
        }
    };

    let start = Instant::now();
    let mut hurl_results = vec![];
    let mut testcases = vec![];

    for (current, filename) in filenames.iter().enumerate() {
        if filename != "-" && !Path::new(filename).exists() {
            let message = format!(
                "hurl: cannot access '{}': No such file or directory",
                filename
            );
            exit(&message, EXIT_ERROR_PARSING, &base_logger);
        }
        let content = cli::read_to_string(filename);
        let content = unwrap_or_exit(content, EXIT_ERROR_PARSING, &base_logger);

        let progress = if cli_options.progress {
            Some(Progress {
                current,
                total: filenames.len(),
            })
        } else {
            None
        };
        let logger = Logger::new(color, verbose, filename, &content);

        let hurl_result = execute(
            filename,
            &content,
            current_dir,
            &cli_options,
            &progress,
            &logger,
        );
        hurl_results.push(hurl_result.clone());

        if matches!(cli_options.output_type, OutputType::ResponseBody)
            && hurl_result.errors().is_empty()
            && !cli_options.interactive
        {
            // By default, we output the body response bytes of the last entry
            if let Some(entry_result) = hurl_result.entries.last() {
                if let Some(response) = entry_result.response.clone() {
                    let mut output = vec![];

                    // If include options is set, we output the HTTP response headers
                    // with status and version (to mimic curl outputs)
                    if cli_options.include {
                        let status_line =
                            format!("HTTP/{} {}\n", response.version, response.status);
                        output.append(&mut status_line.into_bytes());
                        for header in response.headers.clone() {
                            let header_line = format!("{}: {}\n", header.name, header.value);
                            output.append(&mut header_line.into_bytes());
                        }
                        output.append(&mut "\n".to_string().into_bytes());
                    }
                    let body = if cli_options.compressed {
                        match response.uncompress_body() {
                            Ok(bytes) => bytes,
                            Err(e) => {
                                // FIXME: we convert to a runner::Error to be able to use fixme
                                // method. Can we do otherwise (without creating an artificial
                                // error a first character).
                                let error = runner::Error {
                                    source_info: SourceInfo {
                                        start: Pos { line: 0, column: 0 },
                                        end: Pos { line: 0, column: 0 },
                                    },
                                    inner: RunnerError::from(e),
                                    assert: false,
                                };
                                let message = error.fixme();
                                exit(&message, EXIT_ERROR_RUNTIME, &base_logger);
                            }
                        }
                    } else {
                        response.body
                    };
                    output.append(&mut body.clone());
                    let result = write_output(&output, &cli_options.output);
                    unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
                } else {
                    logger.info("No response has been received");
                }
            } else {
                let source = if filename.as_str() == "-" {
                    "".to_string()
                } else {
                    format!("for file {}", filename).to_string()
                };
                logger.warning(format!("No entry have been executed {}", source).as_str());
            };
        }

        if matches!(cli_options.output_type, OutputType::Json) {
            let json_result = hurl_result.to_json(&content);
            let serialized = serde_json::to_string(&json_result).unwrap();
            let s = format!("{}\n", serialized);
            let result = write_output(&s.into_bytes(), &cli_options.output);
            unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
        }
        if cli_options.junit_file.is_some() {
            let testcase = report::Testcase::from_hurl_result(&hurl_result, &content);
            testcases.push(testcase);
        }
    }

    if let Some(filename) = cli_options.junit_file.clone() {
        base_logger.debug(format!("Writing Junit report to {}", filename).as_str());
        let result = report::create_junit_report(filename, testcases);
        unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
    }

    if let Some(dir_path) = cli_options.html_dir {
        base_logger.debug(format!("Writing html report to {}", dir_path.display()).as_str());
        let result = report::write_html_report(dir_path.clone(), hurl_results.clone());
        unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);

        for filename in filenames {
            let result = format_html(filename.as_str(), &dir_path);
            unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
        }
    }

    if let Some(file_path) = cookies_output_file {
        base_logger.debug(format!("Writing cookies to {}", file_path.display()).as_str());
        let result = write_cookies_file(&file_path, &hurl_results);
        unwrap_or_exit(result, EXIT_ERROR_UNDEFINED, &base_logger);
    }

    if cli_options.summary {
        let duration = start.elapsed().as_millis();
        let summary = get_summary(duration, &hurl_results);
        base_logger.info(summary.as_str());
    }

    std::process::exit(exit_code(&hurl_results));
}

/// Returns an exit code for a list of HurlResult.
fn exit_code(hurl_results: &[HurlResult]) -> i32 {
    let mut count_errors_runner = 0;
    let mut count_errors_assert = 0;
    for hurl_result in hurl_results {
        let errors = hurl_result.clone().errors();
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

fn format_html(input_file: &str, dir_path: &Path) -> Result<(), CliError> {
    let relative_input_file = canonicalize_filename(input_file);
    let absolute_input_file = dir_path.join(format!("{}.html", relative_input_file));

    let parent = absolute_input_file.parent().expect("a parent");
    std::fs::create_dir_all(parent).unwrap();
    let mut file = match std::fs::File::create(&absolute_input_file) {
        Err(why) => {
            return Err(CliError {
                message: format!(
                    "Issue writing to {}: {:?}",
                    absolute_input_file.display(),
                    why
                ),
            });
        }
        Ok(file) => file,
    };
    let content = cli::read_to_string(input_file).expect("readable hurl file");
    let hurl_file = parser::parse_hurl_file(content.as_str()).expect("valid hurl file");

    let s = hurl_core::format::format_html(hurl_file, true);

    if let Err(why) = file.write_all(s.as_bytes()) {
        return Err(CliError {
            message: format!(
                "Issue writing to {}: {:?}",
                absolute_input_file.display(),
                why
            ),
        });
    }
    Ok(())
}

fn write_output(bytes: &Vec<u8>, filename: &Option<String>) -> Result<(), CliError> {
    match filename {
        None => write_bytes(bytes.as_slice()),
        Some(filename) => {
            let path = Path::new(filename.as_str());
            let mut file = match std::fs::File::create(&path) {
                Err(why) => {
                    return Err(CliError {
                        message: format!("Issue writing to {}: {:?}", path.display(), why),
                    });
                }
                Ok(file) => file,
            };
            file.write_all(bytes.as_slice())
                .expect("writing bytes to file");
            Ok(())
        }
    }
}

fn cookies_output_file(filename: &str, n: usize) -> Result<PathBuf, CliError> {
    if n > 1 {
        Err(CliError {
            message: "Only save cookies for a unique session".to_string(),
        })
    } else {
        let path = Path::new(&filename);
        Ok(path.to_path_buf())
    }
}

fn write_cookies_file(file_path: &Path, hurl_results: &[HurlResult]) -> Result<(), CliError> {
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            return Err(CliError {
                message: format!("Issue writing to {}: {:?}", file_path.display(), why),
            });
        }
        Ok(file) => file,
    };
    let mut s = r#"# Netscape HTTP Cookie File
# This file was generated by hurl

"#
    .to_string();
    match hurl_results.first() {
        None => {
            return Err(CliError {
                message: "Issue fetching results".to_string(),
            });
        }
        Some(result) => {
            for cookie in result.cookies.clone() {
                s.push_str(cookie.to_string().as_str());
                s.push('\n');
            }
        }
    }

    if let Err(why) = file.write_all(s.as_bytes()) {
        return Err(CliError {
            message: format!("Issue writing to {}: {:?}", file_path.display(), why),
        });
    }
    Ok(())
}

fn get_summary(duration: u128, hurl_results: &[HurlResult]) -> String {
    let total = hurl_results.len();
    let success = hurl_results.iter().filter(|r| r.success).count();
    let failed = total - success;
    let mut s =
        "--------------------------------------------------------------------------------\n"
            .to_string();
    s.push_str(format!("Executed:  {}\n", total).as_str());
    s.push_str(
        format!(
            "Succeeded: {} ({:.1}%)\n",
            success,
            100.0 * success as f32 / total as f32
        )
        .as_str(),
    );
    s.push_str(
        format!(
            "Failed:    {} ({:.1}%)\n",
            failed,
            100.0 * failed as f32 / total as f32
        )
        .as_str(),
    );
    s.push_str(format!("Duration:  {}ms\n", duration).as_str());
    s
}
