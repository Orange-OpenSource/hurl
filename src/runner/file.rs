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
use std::collections::HashMap;
use std::time::Instant;

use crate::core::ast::*;
use crate::core::common::Value;
use crate::http;


use super::core::*;
use super::super::format;
use super::entry;
use crate::core::common::FormatError;



/// Run a Hurl file with the hurl http client
///
/// # Example
///
/// ```
/// use hurl::http;
/// use hurl::runner;
/// use hurl::format;
///
/// // Parse Hurl file
/// let filename = "sample.hurl".to_string();
/// let s = r#"
/// GET http://localhost:8000/hello
/// HTTP/1.0 200
/// "#;
/// let hurl_file = hurl::parser::parse_hurl_file(s).unwrap();
///
/// // Create an http client
/// let options = http::ClientOptions {
///        follow_location: false,
///        max_redirect: None,
///        cookie_input_file: None,
///        proxy: None,
///        no_proxy: None,
///        verbose: false,
///        insecure: false,
/// };
/// let mut client = http::Client::init(options);
///
/// // Define runner options
/// let variables = std::collections::HashMap::new();
/// let options = runner::core::RunnerOptions {
///        fail_fast: false,
///        variables,
///        to_entry: None,
///  };
///
/// // create a logger
/// // It needs the text input as lines for reporting errors
/// let lines = regex::Regex::new(r"\n|\r\n").unwrap().split(&s).map(|l| l.to_string()).collect();
/// let logger = format::logger::Logger {
///        filename: Some(filename.clone()),
///        lines,
///        verbose: false,
///        color: false
///    };
///
/// // Run the hurl file
/// let context_dir = "current_dir".to_string();
/// let hurl_results = runner::file::run(
///     hurl_file,
///     &mut client,
///     filename,
///     context_dir,
///     options,
///     logger
/// );
/// assert!(hurl_results.success);
///
/// ```
pub fn run(
    hurl_file: HurlFile,
    http_client: &mut http::Client,
    filename: String,
    context_dir: String,
    options: RunnerOptions,
    logger: format::logger::Logger,
) -> HurlResult {
    let mut entries = vec![];
    let mut variables = HashMap::default();

    for (key, value) in options.variables {
        variables.insert(key.to_string(), Value::String(value.to_string()));
    }

    let n = if let Some(to_entry) = options.to_entry {
        to_entry
    } else {
        hurl_file.entries.len()
    };

    let start = Instant::now();
    for (entry_index, entry) in hurl_file.entries.iter().take(n).cloned().enumerate().collect::<Vec<(usize, Entry)>>() {
        let entry_result = entry::run(entry, http_client, entry_index, &mut variables, context_dir.clone(), &logger);
        entries.push(entry_result.clone());
        for e in entry_result.errors.clone() {
            let error = format::error::Error {
                source_info: e.clone().source_info,
                description: e.clone().description(),
                fixme: e.fixme(),
                lines: vec![],
                filename: "".to_string(),
                warning: false,
                color: false,
            };
            logger.clone().error(&error);
        }

        if options.fail_fast && !entry_result.errors.is_empty() {
            break;
        }
    }
    let time_in_ms = start.elapsed().as_millis();
    let success = entries.iter().flat_map(|e| e.errors.clone()).next().is_none();

    let cookies = http_client.get_cookie_storage();
    HurlResult {
        filename,
        entries,
        time_in_ms,
        success,
        cookies,
    }
}

