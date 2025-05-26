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
use hurl::runner;
use hurl::runner::{
    AssertResult, CaptureResult, EntryResult, HurlResult, RunnerError, RunnerOptionsBuilder,
    VariableSet,
};
use hurl::util::logger::{ErrorFormat, LoggerOptionsBuilder, Verbosity};
use hurl::util::path::ContextDir;
use hurl_core::input::Input;
use hurl_core::typing::Count;
use std::fmt::Debug;
use std::process::exit;
use std::time::Duration;
use std::{env, fs};

/// Run a Hurl file and dumps results.
/// This sample is used to detect public APIs change for Hurl crates.
/// It depends on the last released version (usually the version prior to the SNAPSHOT one).
/// In the CI, this sample is build using SNAPSHOT version and executed.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Missing Hurl file as input");
        exit(1);
    }
    let filename = &args[1];
    let input = Input::new(filename);
    let content = fs::read_to_string(&args[1]).expect("Should have been able to read the file");

    let logger_opts = LoggerOptionsBuilder::new()
        .color(false)
        .error_format(ErrorFormat::Short)
        .verbosity(Some(Verbosity::Verbose))
        .build();

    // Define runner options
    let runner_opts = RunnerOptionsBuilder::new()
        .aws_sigv4(None)
        .cacert_file(None)
        .client_cert_file(None)
        .client_key_file(None)
        .compressed(false)
        .connect_timeout(Duration::from_secs(300))
        .connects_to(&[])
        .context_dir(&ContextDir::default())
        .cookie_input_file(None)
        .continue_on_error(false)
        .delay(Duration::from_millis(0))
        .follow_location(false)
        .ignore_asserts(false)
        .insecure(false)
        .max_redirect(Count::Infinite)
        .no_proxy(None)
        .path_as_is(true)
        .post_entry(None)
        .pre_entry(None)
        .proxy(None)
        .resolves(&[])
        .retry(None)
        .retry_interval(Duration::from_secs(1))
        .ssl_no_revoke(false)
        .unix_socket(None)
        .timeout(Duration::from_secs(300))
        .to_entry(None)
        .user(None)
        .user_agent(None)
        .build();

    // Set variables
    let variables = VariableSet::default();

    // Run the hurl file
    let result = runner::run(
        &content,
        Some(&input),
        &runner_opts,
        &variables,
        &logger_opts,
    )
    .unwrap();

    print_result(&result, filename);
}

/// Prints a Hurl result
fn print_result(results: &HurlResult, filename: &str) {
    let level = 0;
    print(level, "file", filename);
    print(level, "success", &results.success.to_string());
    print(level, "duration", &results.duration.as_millis().to_string());
    if results.entries.is_empty() {
        print(level, "entries", "-");
    } else {
        print(level, "entries", "");
        results.entries.iter().for_each(print_entry);
    }
}

/// Prints a Hurl entry.
fn print_entry(entry: &EntryResult) {
    let level = 1;
    print(level, "index", &entry.entry_index.to_string());
    print(
        level,
        "transfer duration (ms)",
        &entry.transfer_duration.as_millis().to_string(),
    );
    print(level, "compressed", &entry.compressed.to_string());
    if entry.captures.is_empty() {
        print(level, "captures", "-");
    } else {
        print(level, "captures", "");
        entry.captures.iter().for_each(print_capture);
    }
    if entry.asserts.is_empty() {
        print(level, "asserts", "-");
    } else {
        print(level, "asserts", "");
        entry.asserts.iter().for_each(print_assert);
    }
    if entry.errors.is_empty() {
        print(level, "errors", "-");
    } else {
        print(level, "errors", "");
        entry.errors.iter().for_each(print_error);
    }
    // FIXME: Call is not public
    // if entry.calls.is_empty() {
    //     print(level, "calls", "-");
    // } else {
    //     print(level, "calls", "");
    //     entry.calls.iter().for_each(print_call);
    // }
}

/// Prints a capture.
fn print_capture(capture: &CaptureResult) {
    let level = 2;
    print(level, "name", &capture.name);
    print(level, "value", &capture.value.to_string());
}

/// Prints an assert.
fn print_assert(assert: &AssertResult) {
    let level = 2;
    let kind = match assert {
        AssertResult::Version { .. } => "version",
        AssertResult::Status { .. } => "status",
        AssertResult::Header { .. } => "header",
        AssertResult::Body { .. } => "body",
        AssertResult::Explicit { .. } => "explicit",
    };
    print(level, "type", kind);
}

/// Prints an error.
fn print_error(error: &RunnerError) {
    let level = 2;
    print_dbg(level, "type", &error.kind);
}

// /// Prints a call.
// fn print_call(call: &Call) {
//     let level = 2;
//     print(level, "request", "");
//     print_request(&call.request);
//     print(level, "response", "");
//     print_response(&call.response);
// }
//
// /// Prints an HTTP request
// fn print_request(request: &Request) {
//     let level = 3;
//     print(level, "method", &request.method);
//     print(level, "url", &request.url);
//     if request.headers.is_empty() {
//         print(level, "headers", "-");
//     } else {
//         print(level, "headers", "");
//         request.headers.iter().for_each(print_header);
//     }
//     print(level, "body (bytes)", &request.body.len().to_string());
// }
//
// /// Prints an HTTP response
// fn print_response(response: &Response) {
//     let level = 3;
//     print(level, "url", &response.url);
//     print(level, "version", &response.version.to_string());
//     print(level, "status", &response.status.to_string());
//     if response.headers.is_empty() {
//         print(level, "headers", "-");
//     } else {
//         print(level, "headers", "");
//         response.headers.iter().for_each(print_header);
//     }
//     print(level, "body (bytes)", &response.body.len().to_string());
//     print(
//         level,
//         "duration (ms)",
//         &response.duration.as_millis().to_string(),
//     );
// }
//
// /// Print an HTTP header
// fn print_header(header: &Header) {
//     let level = 4;
//     print(level, "name", &header.name);
//     print(level, "value", &header.value);
// }

fn print(level: usize, key: &str, value: &str) {
    let prefix = " ".repeat(level * 2);
    let len = 28 - key.len() - prefix.len();
    let space = " ".repeat(len);
    println!("{prefix}{key}{space}: {value}");
}

fn print_dbg(level: usize, key: &str, value: impl Debug) {
    let prefix = " ".repeat(level * 2);
    let len = 28 - key.len() - prefix.len();
    let space = " ".repeat(len);
    println!("{prefix}{key}{space}: {value:?}");
}
