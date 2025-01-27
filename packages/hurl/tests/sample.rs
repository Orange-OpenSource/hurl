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
use std::str::FromStr;
use std::time::Duration;

use hurl::http::{Call, HttpVersion, Request, Response, Url};
use hurl::runner;
use hurl::runner::{EntryResult, HurlResult, RunnerOptionsBuilder, VariableSet};
use hurl::util::logger::LoggerOptionsBuilder;
use hurl::util::path::ContextDir;
use hurl_core::input::Input;
use hurl_core::typing::Count;

#[test]
fn simple_sample() {
    // The purpose of the check_* functions:
    // - assert against hard coded values
    // - check that function parameters type are public through the hurl crate
    fn check_result(result: &HurlResult) {
        assert!(result.success);
        assert_eq!(result.cookies.len(), 0);
        assert_eq!(result.entries.len(), 1);
        assert!(result.duration.as_millis() < 1000);
    }

    fn check_entry(entry: &EntryResult) {
        assert_eq!(entry.entry_index, 1);
        assert_eq!(entry.calls.len(), 1);
        assert_eq!(entry.captures.len(), 1);
        assert_eq!(entry.asserts.len(), 3); // HTTP version + status code + implicit body
        assert_eq!(entry.errors.len(), 0);
        assert!(entry.transfer_duration.as_millis() < 1000);
        assert!(!entry.compressed);
    }

    fn check_call(_: &Call) {}

    fn check_request(request: &Request) {
        assert_eq!(
            request.url,
            Url::from_str("http://localhost:8000/hello").unwrap()
        );
        assert_eq!(request.method, "GET");
        let header_names = request
            .headers
            .iter()
            .map(|h| h.name.clone())
            .collect::<Vec<_>>();
        assert!(header_names.contains(&"Accept".to_string()));
        assert!(header_names.contains(&"Host".to_string()));
        assert!(header_names.contains(&"User-Agent".to_string()));
        assert_eq!(request.body.len(), 0);
    }

    fn check_response(response: &Response) {
        assert_eq!(response.version, HttpVersion::Http11);
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.len(), 6);
        let header_names = response
            .headers
            .iter()
            .map(|h| h.name.clone())
            .collect::<Vec<_>>();
        assert!(header_names.contains(&"Connection".to_string()));
        assert!(header_names.contains(&"Content-Length".to_string()));
        assert!(header_names.contains(&"Content-Type".to_string()));
        assert!(header_names.contains(&"Date".to_string()));
        assert!(header_names.contains(&"Server".to_string())); // There are two 'Server' HTTP headers
        assert_eq!(response.body.len(), 12);
        assert!(response.duration < Duration::from_secs(1));
        assert_eq!(
            response.url,
            Url::from_str("http://localhost:8000/hello").unwrap()
        );
        assert!(response.certificate.is_none());
    }

    let content = r#"
    GET http://localhost:8000/hello
    HTTP 200
    [Captures]
    data: body
    `Hello World!`
    "#;

    let filename = Some(Input::new("foo.hurl"));

    // Define runner and logger options
    let runner_opts = RunnerOptionsBuilder::new()
        .aws_sigv4(None)
        .cacert_file(None)
        .compressed(false)
        .connect_timeout(Duration::from_secs(300))
        .context_dir(&ContextDir::default())
        .cookie_input_file(None)
        .fail_fast(false)
        .follow_location(false)
        .ignore_asserts(false)
        .insecure(false)
        .max_redirect(Count::Finite(10))
        .no_proxy(None)
        .post_entry(None)
        .pre_entry(None)
        .proxy(None)
        .retry(None)
        .retry_interval(Duration::from_secs(1))
        .timeout(Duration::from_secs(300))
        .to_entry(None)
        .unix_socket(None)
        .user(None)
        .user_agent(None)
        .build();

    let logger_opts = LoggerOptionsBuilder::new()
        .color(false)
        .verbosity(None)
        .build();

    // Set variables
    let variables = VariableSet::new();

    // Run the hurl file and check data:
    let result = runner::run(
        content,
        filename.as_ref(),
        &runner_opts,
        &variables,
        &logger_opts,
    )
    .unwrap();
    check_result(&result);

    let entry = result.entries.first().unwrap();
    check_entry(entry);

    let call = entry.calls.first().unwrap();
    check_call(call);

    let request = &call.request;
    check_request(request);

    let response = &call.response;
    check_response(response);
}
