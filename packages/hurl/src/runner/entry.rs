/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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

use hurl_core::ast::*;

use crate::http;
use crate::http::ClientOptions;
use crate::runner::cache::BodyCache;
use crate::runner::error::RunnerError;
use crate::runner::result::{AssertResult, EntryResult};
use crate::runner::runner_options::RunnerOptions;
use crate::runner::value::Value;
use crate::runner::{request, response, CaptureResult, RunnerErrorKind};
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
    logger: &mut Logger,
) -> EntryResult {
    let compressed = runner_options.compressed;
    let source_info = entry.source_info();
    let context_dir = &runner_options.context_dir;

    // Evaluates our source requests given our set of variables
    let http_request = match request::eval_request(&entry.request, variables, context_dir) {
        Ok(r) => r,
        Err(error) => {
            return EntryResult {
                entry_index,
                source_info,
                errors: vec![error],
                compressed,
                ..Default::default()
            };
        }
    };
    let client_options = ClientOptions::from(runner_options, logger.verbosity);

    // Experimental features with cookie storage
    use std::str::FromStr;
    if let Some(s) = request::cookie_storage_set(&entry.request) {
        if let Ok(cookie) = http::Cookie::from_str(s.as_str()) {
            http_client.add_cookie(&cookie, &client_options);
        } else {
            logger.warning(&format!("Cookie string can not be parsed: '{s}'"));
        }
    }
    if request::cookie_storage_clear(&entry.request) {
        http_client.clear_cookie_storage(&client_options);
    }

    log_request(
        http_client,
        &http_request,
        runner_options,
        &client_options,
        logger,
    );

    // Run the HTTP requests (optionally follow redirection)
    let calls = match http_client.execute_with_redirect(&http_request, &client_options, logger) {
        Ok(calls) => calls,
        Err(http_error) => {
            let start = entry.request.url.source_info.start;
            let end = entry.request.url.source_info.end;
            let error_source_info = SourceInfo::new(start, end);
            let error =
                RunnerError::new(error_source_info, RunnerErrorKind::Http(http_error), false);
            return EntryResult {
                entry_index,
                source_info,
                errors: vec![error],
                compressed,
                ..Default::default()
            };
        }
    };

    // Now, we can compute capture and asserts on the last HTTP request/response chains.
    let call = calls.last().unwrap();
    let http_response = &call.response;

    // `transfer_duration` represent the network time of calls, not including assert processing.
    let transfer_duration = calls.iter().map(|call| call.timings.total).sum();

    // We proceed asserts and captures in this order:
    // 1. first, check implicit assert on status and version. If KO, test is failed
    // 2. then, we compute captures, we might need them in asserts
    // 3. finally, run the remaining asserts
    let mut cache = BodyCache::new();
    let mut asserts = vec![];

    if !runner_options.ignore_asserts {
        if let Some(response_spec) = &entry.response {
            let mut status_asserts =
                response::eval_version_status_asserts(response_spec, http_response);
            let errors = asserts_to_errors(&status_asserts);
            asserts.append(&mut status_asserts);
            if !errors.is_empty() {
                logger.debug("");
                return EntryResult {
                    entry_index,
                    source_info,
                    calls,
                    captures: vec![],
                    asserts,
                    errors,
                    transfer_duration,
                    compressed,
                };
            }
        }
    };

    let captures = match &entry.response {
        None => vec![],
        Some(response_spec) => {
            match response::eval_captures(response_spec, http_response, &mut cache, variables) {
                Ok(captures) => captures,
                Err(e) => {
                    return EntryResult {
                        entry_index,
                        source_info,
                        calls,
                        captures: vec![],
                        asserts,
                        errors: vec![e],
                        transfer_duration,
                        compressed,
                    };
                }
            }
        }
    };
    log_captures(&captures, logger);
    logger.debug("");

    // Compute asserts
    if !runner_options.ignore_asserts {
        if let Some(response_spec) = &entry.response {
            let mut other_asserts = response::eval_asserts(
                response_spec,
                variables,
                http_response,
                &mut cache,
                context_dir,
            );
            asserts.append(&mut other_asserts);
        }
    };

    let errors = asserts_to_errors(&asserts);

    EntryResult {
        entry_index,
        source_info,
        calls,
        captures,
        asserts,
        errors,
        transfer_duration,
        compressed,
    }
}

/// Converts a list of [`AssertResult`] to a list of [`RunnerError`].
fn asserts_to_errors(asserts: &[AssertResult]) -> Vec<RunnerError> {
    asserts
        .iter()
        .filter_map(|assert| assert.error())
        .map(
            |RunnerError {
                 source_info,
                 kind: inner,
                 ..
             }| RunnerError::new(source_info, inner, true),
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
            follow_location_trusted: runner_options.follow_location_trusted,
            http_version: runner_options.http_version,
            ip_resolve: runner_options.ip_resolve,
            max_filesize: runner_options.max_filesize,
            max_redirect: runner_options.max_redirect,
            netrc: runner_options.netrc,
            netrc_file: runner_options.netrc_file.clone(),
            netrc_optional: runner_options.netrc_optional,
            path_as_is: runner_options.path_as_is,
            proxy: runner_options.proxy.clone(),
            no_proxy: runner_options.no_proxy.clone(),
            insecure: runner_options.insecure,
            resolves: runner_options.resolves.clone(),
            ssl_no_revoke: runner_options.ssl_no_revoke,
            timeout: runner_options.timeout,
            unix_socket: runner_options.unix_socket.clone(),
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

/// Logs this HTTP `request`.
fn log_request(
    http_client: &mut http::Client,
    request: &http::RequestSpec,
    runner_options: &RunnerOptions,
    client_options: &ClientOptions,
    logger: &mut Logger,
) {
    logger.debug("");
    logger.debug_important("Cookie store:");
    for cookie in &http_client.cookie_storage() {
        logger.debug(&cookie.to_string());
    }

    logger.debug("");
    logger.debug_important("Request:");
    logger.debug(&format!("{} {}", request.method, request.url));
    for header in &request.headers {
        logger.debug(&header.to_string());
    }
    if !request.querystring.is_empty() {
        logger.debug("[QueryStringParams]");
        for param in &request.querystring {
            logger.debug(&param.to_string());
        }
    }
    if !request.form.is_empty() {
        logger.debug("[FormParams]");
        for param in &request.form {
            logger.debug(&param.to_string());
        }
    }
    if !request.multipart.is_empty() {
        logger.debug("[MultipartFormData]");
        for param in &request.multipart {
            logger.debug(&param.to_string());
        }
    }
    if !request.cookies.is_empty() {
        logger.debug("[Cookies]");
        for cookie in &request.cookies {
            logger.debug(&cookie.to_string());
        }
    }
    logger.debug("");
    logger.debug("Request can be run with the following curl command:");
    let context_dir = &runner_options.context_dir;
    let output = &runner_options.output;
    let curl_command =
        http_client.curl_command_line(request, context_dir, output.as_ref(), client_options);
    logger.debug(&curl_command);
    logger.debug("");
}

/// Logs the `captures` from the entry HTTP response.
fn log_captures(captures: &[CaptureResult], logger: &mut Logger) {
    if captures.is_empty() {
        return;
    }
    logger.debug_important("Captures:");
    for c in captures.iter() {
        logger.capture(&c.name, &c.value);
    }
}
