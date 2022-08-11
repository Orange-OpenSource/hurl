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

use crate::cli::Logger;
use crate::http;
use crate::http::ClientOptions;
use hurl_core::ast::*;

use super::core::*;
use super::core::{Error, RunnerError};
use super::request::eval_request;
use super::response::{eval_asserts, eval_captures};
use super::value::Value;
use crate::runner::request::{cookie_storage_clear, cookie_storage_set};

/// Runs an `entry` with `http_client` and returns one or more
/// [`EntryResult`] (if following redirect).
///
/// `variables` are used to render values at runtime, and can be updated
/// by captures.
pub fn run(
    entry: &Entry,
    http_client: &mut http::Client,
    variables: &mut HashMap<String, Value>,
    runner_options: &RunnerOptions,
    client_options: &ClientOptions,
    logger: &Logger,
) -> Vec<EntryResult> {
    let http_request = match eval_request(&entry.request, variables, &runner_options.context_dir) {
        Ok(r) => r,
        Err(error) => {
            return vec![EntryResult {
                request: None,
                response: None,
                captures: vec![],
                asserts: vec![],
                errors: vec![error],
                time_in_ms: 0,
            }];
        }
    };

    // We computes overridden options for this entry.
    let client_options = get_entry_options(entry, client_options, logger);

    // Experimental features
    // with cookie storage
    use std::str::FromStr;
    if let Some(s) = cookie_storage_set(&entry.request) {
        if let Ok(cookie) = http::Cookie::from_str(s.as_str()) {
            http_client.add_cookie(&cookie, &client_options);
        } else {
            logger.warning(format!("Cookie string can not be parsed: '{}'", s).as_str());
        }
    }
    if cookie_storage_clear(&entry.request) {
        http_client.clear_cookie_storage(&client_options);
    }

    logger.debug("");
    logger.debug_important("Cookie store:");
    for cookie in http_client.get_cookie_storage() {
        logger.debug(cookie.to_string().as_str());
    }
    logger.debug("");
    log_request_spec(&http_request, logger);
    logger.debug("Request can be run with the following curl command:");
    logger.debug(
        http_client
            .curl_command_line(&http_request, &client_options)
            .as_str(),
    );
    logger.debug("");

    let calls = match http_client.execute_with_redirect(&http_request, &client_options, logger) {
        Ok(calls) => calls,
        Err(http_error) => {
            let runner_error = RunnerError::from(http_error);
            return vec![EntryResult {
                request: None,
                response: None,
                captures: vec![],
                asserts: vec![],
                errors: vec![Error {
                    source_info: SourceInfo {
                        start: entry.request.url.source_info.start.clone(),
                        end: entry.request.url.source_info.end.clone(),
                    },
                    inner: runner_error,
                    assert: false,
                }],
                time_in_ms: 0,
            }];
        }
    };

    let mut entry_results = vec![];
    for (i, (http_request, http_response)) in calls.iter().enumerate() {
        let mut captures = vec![];
        let mut asserts = vec![];
        let mut errors = vec![];
        let time_in_ms = http_response.duration.as_millis();

        // We runs capture and asserts on the last HTTP request/response chains.
        if i == calls.len() - 1 {
            captures = match entry.response.clone() {
                None => vec![],
                Some(response) => match eval_captures(response, http_response, variables) {
                    Ok(captures) => captures,
                    Err(e) => {
                        return vec![EntryResult {
                            request: Some(http_request.clone()),
                            response: Some(http_response.clone()),
                            captures: vec![],
                            asserts: vec![],
                            errors: vec![e],
                            time_in_ms,
                        }];
                    }
                },
            };
            // Update variables now!
            for capture_result in captures.clone() {
                variables.insert(capture_result.name, capture_result.value);
            }
            asserts = if runner_options.ignore_asserts {
                vec![]
            } else {
                match entry.response.clone() {
                    None => vec![],
                    Some(response) => eval_asserts(
                        response,
                        variables,
                        http_response.clone(),
                        &runner_options.context_dir,
                    ),
                }
            };
            errors = asserts
                .iter()
                .filter_map(|assert| assert.clone().error())
                .map(
                    |Error {
                         source_info, inner, ..
                     }| Error {
                        source_info,
                        inner,
                        assert: true,
                    },
                )
                .collect();

            if !captures.is_empty() {
                logger.debug_important("Captures:");
                for capture in captures.clone() {
                    logger.debug(format!("{}: {}", capture.name, capture.value).as_str());
                }
            }
            logger.debug("");
        }

        let entry_result = EntryResult {
            request: Some(http_request.clone()),
            response: Some(http_response.clone()),
            captures,
            asserts,
            errors,
            time_in_ms,
        };
        entry_results.push(entry_result);
    }

    entry_results
}

/// Logs this HTTP `request` spec.
fn log_request_spec(request: &http::RequestSpec, logger: &Logger) {
    logger.debug_important("Request:");
    logger.debug(format!("{} {}", request.method, request.url).as_str());
    for header in &request.headers {
        logger.debug(header.to_string().as_str());
    }
    if !request.querystring.is_empty() {
        logger.debug("[QueryStringParams]");
        for param in &request.querystring {
            logger.debug(param.to_string().as_str());
        }
    }
    if !request.form.is_empty() {
        logger.debug("[FormParams]");
        for param in &request.form {
            logger.debug(param.to_string().as_str());
        }
    }
    if !request.multipart.is_empty() {
        logger.debug("[MultipartFormData]");
        for param in &request.multipart {
            logger.debug(param.to_string().as_str());
        }
    }
    if !request.cookies.is_empty() {
        logger.debug("[Cookies]");
        for cookie in &request.cookies {
            logger.debug(cookie.to_string().as_str());
        }
    }
    if let Some(s) = &request.content_type {
        logger.debug("");
        logger.debug(format!("Implicit content-type={}", s).as_str());
    }
    logger.debug("");
}

/// Returns a new [`ClientOptions`] based on the `entry` optional Options section
/// and a default `client_options`.
fn get_entry_options(
    entry: &Entry,
    client_options: &ClientOptions,
    logger: &Logger,
) -> ClientOptions {
    let mut client_options = client_options.clone();

    let has_options = entry
        .request
        .sections
        .iter()
        .any(|s| matches!(s.value, SectionValue::Options(_)));
    if has_options {
        logger.debug("");
        logger.debug_important("Entry options:");
    }

    for section in &entry.request.sections {
        if let SectionValue::Options(options) = &section.value {
            for option in options {
                let EntryOption::Insecure(insecure_option) = option;
                client_options.insecure = insecure_option.value;
                logger.debug(format!("insecure: {}", client_options.insecure).as_str());
            }
        }
    }
    client_options
}
