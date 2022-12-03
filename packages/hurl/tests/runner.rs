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

use std::collections::HashMap;

use hurl::cli;
use hurl::cli::Logger;
use hurl::http;
use hurl::runner;
use hurl::runner::RunnerOptions;
use hurl_core::ast::*;
use hurl_core::parser;

// Can be used for debugging
#[test]
fn test_hurl_file() {
    let filename = "../../integration/tests_ok/bom.hurl";
    let content = cli::read_to_string(filename).expect("Something went wrong reading the file");
    let hurl_file = parser::parse_hurl_file(content.as_str()).unwrap();
    let variables = HashMap::new();
    let mut client = http::Client::new(None);
    let logger = Logger::new(false, false, filename, &content);
    let runner_options = RunnerOptions::default();
    let _hurl_log = runner::run(
        &hurl_file,
        filename,
        &mut client,
        &runner_options,
        &variables,
        &logger,
    );
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
            delimiter: None,
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
                delimiter: None,
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
    let mut client = http::Client::new(None);

    // We construct a Hurl file ast "by hand", with fake source info.
    // In this particular case, the raw content is empty as the Hurl file hasn't
    // been built from a text content.
    let content = "";
    let filename = "filename";
    let logger = Logger::new(false, false, filename, content);

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
    let runner_options = RunnerOptions::default();
    runner::run(
        &hurl_file,
        "filename",
        &mut client,
        &runner_options,
        &variables,
        &logger,
    );
}
