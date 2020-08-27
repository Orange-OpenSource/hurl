/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
extern crate hurl;

use hurl::core::ast;
use hurl::core::common::{Pos, SourceInfo};
use hurl::runner;
use hurl::format;
use hurl::http;
use std::collections::HashMap;
use hurl::core::ast::{Template, TemplateElement, EncodedString};
use hurl::runner::core::RunnerOptions;


// can be used for debugging
#[test]
fn test_hurl_file() {
    let mut cookie_store = http::cookie::CookieJar::init(vec![]);
    //let filename = "integration/tests/post_json.hurl";
    //let filename = "integration/tests/error_assert_match_utf8.hurl";
    let filename = "integration/tests/error_template_variable_not_renderable.hurl";
    //let filename = "/mnt/secure/repos/work/myshop/integration/src/main/hurl-generated/pcm/pcm-jdd-open-up-150go.hurl";
    let content = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let hurl_file = hurl::parser::parse_hurl_file(content.as_str()).unwrap();
    let variables = HashMap::new();
    let client = http::client::Client::init(http::client::ClientOptions {
        noproxy_hosts: vec![],
        insecure: false,
        redirect: http::client::Redirect::None,
        http_proxy: None,
        https_proxy: None,
        all_proxy: None
    });
    let mut lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(&content)
        .collect();
    // edd an empty line at the end?
    lines.push("");
    let lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();

    let options = RunnerOptions {
        fail_fast: false,
        variables,
        to_entry: None,
    };
    let logger = format::logger::Logger {
        filename: Some(filename.to_string()),
        lines: lines,
        verbose: false,
        color: false
    };

    let _hurl_log = runner::file::run(
        hurl_file,
        client,
        //&mut variables,
        filename.to_string(),
        &mut cookie_store,
        "current_dir".to_string(),
        options,
        logger
    );
//   assert_eq!(1,2)


}

#[cfg(test)]
fn hello_request() -> ast::Request {
    // GET http://localhost;8000/hello
    let source_info = SourceInfo {
        start: Pos { line: 1, column: 1 },
        end: Pos { line: 1, column: 1 },
    };
    let whitespace = ast::Whitespace {
        value: "".to_string(),
        source_info: source_info.clone(),
    };
    let line_terminator = ast::LineTerminator {
        space0: whitespace.clone(),
        comment: None,
        newline: whitespace.clone(),
    };
    ast::Request {
        line_terminators: vec![],
        space0: whitespace.clone(),
        method: ast::Method::Get,
        space1: whitespace.clone(),
        url: Template {
            quotes: false,
            elements: vec![
                TemplateElement::String { value: "http://localhost:8000/hello".to_string(), encoded: "http://localhost:8000/hello".to_string() }
            ],
            source_info: source_info.clone(),
        },
        line_terminator0: ast::LineTerminator {
            space0: whitespace.clone(),
            comment: None,
            newline: whitespace.clone(),
        },
        headers: vec![
            ast::KeyValue {
                line_terminators: vec![],
                space0: whitespace.clone(),
                key: EncodedString {
                    quotes: false,
                    value: "User-Agent".to_string(),
                    encoded: "User-Agent".to_string(),
                    source_info: source_info.clone(),
                },
                space1: whitespace.clone(),
                space2: whitespace.clone(),
                value: Template {
                    quotes: false,
                    elements: vec![
                        TemplateElement::String { value: "test".to_string(), encoded: "test".to_string() }
                    ],
                    source_info: source_info.clone(),
                },
                line_terminator0: line_terminator.clone(),
            }
        ],
        sections: vec![],
        body: None,
        source_info: source_info.clone(),
    }
}

#[test]
fn test_hello() {
    let mut cookie_store = http::cookie::CookieJar::init(vec![]);
    let client = http::client::Client::init(http::client::ClientOptions {
        noproxy_hosts: vec![],
        insecure: false,
        redirect: http::client::Redirect::None,
        http_proxy: None,
        https_proxy: None,
        all_proxy: None
    });
    let source_info = SourceInfo {
        start: Pos { line: 1, column: 1 },
        end: Pos { line: 1, column: 1 },
    };
    let whitespace = ast::Whitespace {
        value: "".to_string(),
        source_info: source_info.clone(),
    };
    let request = hello_request();
    let hurl_file = ast::HurlFile {
        entries: vec![ast::Entry {
            request,
            response: Some(ast::Response {
                line_terminators: vec![],
                version: ast::Version {
                    value: ast::VersionValue::Version11,
                    source_info: source_info.clone(),
                },
                space0: whitespace.clone(),
                status: ast::Status {
                    value: 200,
                    source_info: source_info.clone(),
                },
                space1: whitespace.clone(),
                line_terminator0: ast::LineTerminator {
                    space0: whitespace.clone(),
                    comment: None,
                    newline: whitespace.clone(),
                },
                headers: vec![],
                sections: vec![],
                body: None,
                source_info: source_info.clone(),
            }),
        }],
        line_terminators: vec![],
    };
    let lines = vec![
        String::from("line1")
    ];
    let variables = HashMap::new();
    let options = RunnerOptions {
        fail_fast: true,
        variables,
        to_entry: None,
    };
    let logger = format::logger::Logger {
        filename: None,
        lines,
        verbose: false,
        color: false
    };
    let _hurl_log = runner::file::run(
        hurl_file,
        client,
        String::from("filename"),
        &mut cookie_store,
        "current_dir".to_string(),
        options,
        logger
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
