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
use std::collections::HashMap;
use std::time::Duration;

use hurl_core::ast::*;

use crate::http;
use crate::http::ClientOptions;
use crate::runner::core::{Error, RunnerError, *};
use crate::runner::request::{cookie_storage_clear, cookie_storage_set, eval_request};
use crate::runner::response::{eval_asserts, eval_captures, eval_version_status_asserts};
use crate::runner::runner_options::RunnerOptions;
use crate::runner::value::Value;
use crate::util::logger::{Logger, Verbosity};

/// Runs an `entry` with `http_client` and returns one [`EntryResult`].
///
/// The `calls` field of the [`EntryResult`] contains a list of HTTP requests and responses that have
/// been executed. If `http_client` has been configured to follow redirection, the `calls` list contains
/// every step of the redirection for the first to the last.
/// `variables` are used to render values at runtime, and can be updated by captures.
pub fn run(
    entry: &Entry,
    entry_index: usize,
    http_client: &mut http::Client,
    variables: &mut HashMap<String, Value>,
    runner_options: &RunnerOptions,
    logger: &Logger,
) -> EntryResult {
    let http_request = match eval_request(&entry.request, variables, &runner_options.context_dir) {
        Ok(r) => r,
        Err(error) => {
            return EntryResult {
                entry_index,
                calls: vec![],
                captures: vec![],
                asserts: vec![],
                errors: vec![error],
                time_in_ms: 0,
                compressed: runner_options.compressed,
            };
        }
    };
    let client_options = ClientOptions::from(runner_options, logger.verbosity);

    // Experimental features with cookie storage
    use std::str::FromStr;
    if let Some(s) = cookie_storage_set(&entry.request) {
        if let Ok(cookie) = http::Cookie::from_str(s.as_str()) {
            http_client.add_cookie(&cookie, &client_options);
        } else {
            logger.warning(format!("Cookie string can not be parsed: '{s}'").as_str());
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
    let curl_command =
        http_client.curl_command_line(&http_request, &runner_options.context_dir, &client_options);
    logger.debug(curl_command.as_str());
    logger.debug("");

    // Run the HTTP requests (optionally follow redirection)
    let calls = match http_client.execute_with_redirect(&http_request, &client_options, logger) {
        Ok(calls) => calls,
        Err(http_error) => {
            let runner_error = RunnerError::from(http_error);
            let error = Error {
                source_info: SourceInfo {
                    start: entry.request.url.source_info.start.clone(),
                    end: entry.request.url.source_info.end.clone(),
                },
                inner: runner_error,
                assert: false,
            };
            return EntryResult {
                entry_index,
                calls: vec![],
                captures: vec![],
                asserts: vec![],
                errors: vec![error],
                time_in_ms: 0,
                compressed: client_options.compressed,
            };
        }
    };

    // We runs capture and asserts on the last HTTP request/response chains.
    let call = calls.last().unwrap();
    let http_response = &call.response;
    // `time_in_ms` represent the network time of calls, not including assert processing.
    let time_in_ms = calls
        .iter()
        .map(|call| call.timings.total)
        .sum::<Duration>()
        .as_millis();

    // We proceed asserts and captures in this order:
    // 1. first, check implicit assert on status and version. If KO, test is failed
    // 2. then, we compute captures, we might need them in asserts
    // 3. finally, run the remaining asserts
    let mut all_asserts = vec![];

    if !runner_options.ignore_asserts {
        if let Some(response_spec) = &entry.response {
            let mut asserts = eval_version_status_asserts(response_spec, http_response);
            let errors = asserts_to_errors(&asserts);
            if !errors.is_empty() {
                logger.debug("");
                return EntryResult {
                    entry_index,
                    calls,
                    captures: vec![],
                    asserts,
                    errors,
                    time_in_ms,
                    compressed: client_options.compressed,
                };
            }
            all_asserts.append(&mut asserts);
        }
    };

    let captures = match &entry.response {
        None => vec![],
        Some(response_spec) => match eval_captures(response_spec, http_response, variables) {
            Ok(captures) => captures,
            Err(e) => {
                return EntryResult {
                    entry_index,
                    calls,
                    captures: vec![],
                    asserts: all_asserts,
                    errors: vec![e],
                    time_in_ms,
                    compressed: client_options.compressed,
                };
            }
        },
    };

    if !captures.is_empty() {
        logger.debug_important("Captures:");
        for c in captures.iter() {
            logger.capture(&c.name, &c.value);
        }
    }
    logger.debug("");

    // Compute asserts
    if !runner_options.ignore_asserts {
        if let Some(response_spec) = &entry.response {
            let mut asserts = eval_asserts(
                response_spec,
                variables,
                http_response,
                &runner_options.context_dir,
            );
            all_asserts.append(&mut asserts);
        }
    };

    let errors = asserts_to_errors(&all_asserts);

    EntryResult {
        entry_index,
        calls,
        captures,
        asserts: all_asserts,
        errors,
        time_in_ms,
        compressed: client_options.compressed,
    }
}

/// Converts a list of [`AssertResult`] to a list of [`Error`].
fn asserts_to_errors(asserts: &[AssertResult]) -> Vec<Error> {
    asserts
        .iter()
        .filter_map(|assert| assert.error())
        .map(
            |Error {
                 source_info, inner, ..
             }| Error {
                source_info,
                inner,
                assert: true,
            },
        )
        .collect()
}

impl ClientOptions {
    fn from(runner_options: &RunnerOptions, verbosity: Option<Verbosity>) -> Self {
        ClientOptions {
            aws_sigv4: runner_options.aws_sigv4.clone(),
            cacert_file: runner_options.cacert_file.clone(),
            client_cert_file: runner_options.client_cert_file.clone(),
            client_key_file: runner_options.client_key_file.clone(),
            compressed: runner_options.compressed,
            connect_timeout: runner_options.connect_timeout,
            connects_to: runner_options.connects_to.clone(),
            cookie_input_file: runner_options.cookie_input_file.clone(),
            follow_location: runner_options.follow_location,
            http_version: runner_options.http_version,
            max_redirect: runner_options.max_redirect,
            path_as_is: runner_options.path_as_is,
            proxy: runner_options.proxy.clone(),
            no_proxy: runner_options.no_proxy.clone(),
            insecure: runner_options.insecure,
            resolves: runner_options.resolves.clone(),
            retry: runner_options.retry,
            ssl_no_revoke: runner_options.ssl_no_revoke,
            timeout: runner_options.timeout,
            user: runner_options.user.clone(),
            user_agent: runner_options.user_agent.clone(),
            verbosity: match verbosity {
                Some(Verbosity::Verbose) => Some(http::Verbosity::Verbose),
                Some(Verbosity::VeryVerbose) => Some(http::Verbosity::VeryVerbose),
                _ => None,
            },
        }
    }
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
        logger.debug(format!("Implicit content-type={s}").as_str());
    }
    logger.debug("");
}
