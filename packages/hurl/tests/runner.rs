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
use hurl::cli;
use hurl::http;
use hurl::runner;
use hurl::runner::RunnerOptions;
use hurl_core::ast::*;
use hurl_core::parser;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn log_verbose(message: &str) {
    eprintln!("* {}", message);
}
pub fn log_error_message(_warning: bool, message: &str) {
    eprintln!("{}", message);
}
pub fn log_runner_error(error: &runner::Error, _warning: bool) {
    eprintln!("* {:#?}", error);
}

// can be used for debugging
#[test]
fn test_hurl_file() {
    let filename = "../../integration/tests_ok/bom.hurl";
    let content = cli::read_to_string(filename).expect("Something went wrong reading the file");
    let hurl_file = parser::parse_hurl_file(content.as_str()).unwrap();
    let variables = HashMap::new();
    let options = http::ClientOptions::default();
    let mut client = http::Client::init(options);
    let mut lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(&content)
        .collect();
    // edd an empty line at the end?
    lines.push("");

    let options = RunnerOptions {
        fail_fast: false,
        variables,
        to_entry: None,
        context_dir: PathBuf::new(),
        ignore_asserts: false,
        very_verbose: false,
        pre_entry: |_| true,
        post_entry: || true,
    };

    let log_verbose: fn(&str) = log_verbose;
    let log_error_message: fn(bool, &str) = log_error_message;
    let log_runner_error: fn(&runner::Error, bool) = log_runner_error;

    let _hurl_log = runner::run_hurl_file(
        hurl_file,
        &mut client,
        //&mut variables,
        filename,
        &options,
        &log_verbose,
        &log_error_message,
        &log_runner_error,
    );
    //   assert_eq!(1,2)
}

#[cfg(test)]
fn hello_request() -> Request {
    // GET http://localhost;8000/hello
    let source_info = SourceInfo {
        start: Pos { line: 1, column: 1 },
        end: Pos { line: 1, column: 1 },
    };
    let whitespace = Whitespace {
        value: "".to_string(),
        source_info: source_info.clone(),
    };
    let line_terminator = LineTerminator {
        space0: whitespace.clone(),
        comment: None,
        newline: whitespace.clone(),
    };
    Request {
        line_terminators: vec![],
        space0: whitespace.clone(),
        method: Method::Get,
        space1: whitespace.clone(),
        url: Template {
            quotes: false,
            elements: vec![TemplateElement::String {
                value: "http://localhost:8000/hello".to_string(),
                encoded: "http://localhost:8000/hello".to_string(),
            }],
            source_info: source_info.clone(),
        },
        line_terminator0: LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        },
        headers: vec![KeyValue {
            line_terminators: vec![],
            space0: whitespace.clone(),
            key: EncodedString {
                quotes: false,
                value: "User-Agent".to_string(),
                encoded: "User-Agent".to_string(),
                source_info: source_info.clone(),
            },
            space1: whitespace.clone(),
            space2: whitespace,
            value: Template {
                quotes: false,
                elements: vec![TemplateElement::String {
                    value: "test".to_string(),
                    encoded: "test".to_string(),
                }],
                source_info: source_info.clone(),
            },
            line_terminator0: line_terminator,
        }],
        sections: vec![],
        body: None,
        source_info,
    }
}

#[test]
fn test_hello() {
    let options = http::ClientOptions::default();
    let mut client = http::Client::init(options);
    let source_info = SourceInfo {
        start: Pos { line: 1, column: 1 },
        end: Pos { line: 1, column: 1 },
    };
    let whitespace = Whitespace {
        value: "".to_string(),
        source_info: source_info.clone(),
    };
    let request = hello_request();
    let hurl_file = HurlFile {
        entries: vec![Entry {
            request,
            response: Some(Response {
                line_terminators: vec![],
                version: Version {
                    value: VersionValue::Version11,
                    source_info: source_info.clone(),
                },
                space0: whitespace.clone(),
                status: Status {
                    value: StatusValue::Specific(200),
                    source_info: source_info.clone(),
                },
                space1: whitespace.clone(),
                line_terminator0: LineTerminator {
                    space0: whitespace.clone(),
                    comment: None,
                    newline: whitespace,
                },
                headers: vec![],
                sections: vec![],
                body: None,
                source_info,
            }),
        }],
        line_terminators: vec![],
    };
    let variables = HashMap::new();
    let options = RunnerOptions {
        fail_fast: true,
        variables,
        to_entry: None,
        context_dir: PathBuf::new(),
        ignore_asserts: false,
        very_verbose: false,
        pre_entry: |_| true,
        post_entry: || true,
    };
    let log_verbose: fn(&str) = log_verbose;
    let log_error_message: fn(bool, &str) = log_error_message;
    let log_runner_error: fn(&runner::Error, bool) = log_runner_error;
    let _hurl_log = runner::run_hurl_file(
        hurl_file,
        &mut client,
        "filename",
        &options,
        &log_verbose,
        &log_error_message,
        &log_runner_error,
    );
    //assert_eq!(hurl_log.entries.len(), 1);
    //assert_eq!(hurl_log.entries.get(0).unwrap().response.status, 200);
    //    assert!(hurl_log
    //        .entries
    //        .get(0)
    //        .unwrap()
    //        .asserts
    //        .get(0)
    //        .unwrap()
    //        .clone()
    //        .success());
}
