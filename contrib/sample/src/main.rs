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

use hurl::cli::Logger;
use hurl::http::{ContextDir, Header, Request, Response};
use hurl::runner::{AssertResult, Call, CaptureResult, EntryResult, Error, HurlResult};
use hurl::{http, runner};
use hurl_core::parser;
use std::collections::HashMap;
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
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Parse Hurl file
    let hurl_file = parser::parse_hurl_file(&contents).expect("Invalid Hurl file");

    // Create an HTTP client
    let mut client = http::Client::new(None);
    let logger = Logger::new(false, false, file_path, &contents);

    // Define runner options
    let runner_options = runner::RunnerOptions {
        cacert_file: None,
        compressed: false,
        connect_timeout: Duration::from_secs(300),
        context_dir: ContextDir::default(),
        cookie_input_file: None,
        fail_fast: false,
        follow_location: false,
        ignore_asserts: false,
        insecure: false,
        max_redirect: None,
        no_proxy: None,
        post_entry: None,
        pre_entry: None,
        proxy: None,
        retry: false,
        retry_interval: Duration::from_secs(1),
        retry_max_count: Some(10),
        timeout: Duration::from_secs(300),
        to_entry: None,
        user: None,
        user_agent: None,
        verbosity: None,
        very_verbose: true,
    };

    // Set variables
    let variables = HashMap::default();

    // Run the hurl file
    let results = runner::run(
        &hurl_file,
        file_path,
        &mut client,
        &runner_options,
        &variables,
        &logger,
    );

    print_results(&results);
}

/// Prints a Hurl result
fn print_results(results: &HurlResult) {
    let level = 0;
    print(level, "file", &results.filename);
    print(level, "success", &results.success.to_string());
    print(level, "duration", &results.time_in_ms.to_string());
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
    print(level, "duration (ms)", &entry.time_in_ms.to_string());
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
    if entry.calls.is_empty() {
        print(level, "calls", "-");
    } else {
        print(level, "calls", "");
        entry.calls.iter().for_each(print_call);
    }
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
fn print_error(error: &Error) {
    let level = 2;
    print_dbg(level, "type", &error.inner);
}

/// Prints a call.
fn print_call(call: &Call) {
    let level = 2;
    print(level, "request", "");
    print_request(&call.request);
    print(level, "response", "");
    print_response(&call.response);
}

/// Prints an HTTP request
fn print_request(request: &Request) {
    let level = 3;
    print(level, "method", &request.method);
    print(level, "url", &request.url);
    if request.headers.is_empty() {
        print(level, "headers", "-");
    } else {
        print(level, "headers", "");
        request.headers.iter().for_each(print_header);
    }
    print(level, "body (bytes)", &request.body.len().to_string());
}

/// Prints an HTTP response
fn print_response(response: &Response) {
    let level = 3;
    print(level, "url", &response.url);
    print(level, "version", &response.version.to_string());
    print(level, "status", &response.status.to_string());
    if response.headers.is_empty() {
        print(level, "headers", "-");
    } else {
        print(level, "headers", "");
        response.headers.iter().for_each(print_header);
    }
    print(level, "body (bytes)", &response.body.len().to_string());
    print(
        level,
        "duration (ms)",
        &response.duration.as_millis().to_string(),
    );
}

/// Print an HTTP header
fn print_header(header: &Header) {
    let level = 4;
    print(level, "name", &header.name);
    print(level, "value", &header.value);
}

fn print(level: usize, key: &str, value: &str) {
    let prefix = " ".repeat(level * 2);
    let len = 20 - key.len() - prefix.len();
    let space = " ".repeat(len);
    println!("{prefix}{key}{space}: {value}");
}

fn print_dbg(level: usize, key: &str, value: impl Debug) {
    let prefix = " ".repeat(level * 2);
    let len = 20 - key.len() - prefix.len();
    let space = " ".repeat(len);
    println!("{prefix}{key}{space}: {value:?}");
}
