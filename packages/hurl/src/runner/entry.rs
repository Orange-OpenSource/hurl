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
use hurl_core::ast::*;

use super::core::*;
use super::core::{Error, RunnerError};
use super::request::eval_request;
use super::response::{eval_asserts, eval_captures};
use super::value::Value;
use crate::runner::request::{cookie_storage_clear, cookie_storage_set};

/// Run an entry with the hurl http client
/// Return one or more EntryResults (if following redirect)
///
/// # Examples
///
/// ```
/// use hurl::http;
/// use hurl::runner;
///
/// // Create an http client
//// let client = http::client::Client::init(http::client::ClientOptions {
////        noproxy_hosts: vec![],
////        insecure: false,
////        redirect: http::client::Redirect::None,
////        http_proxy: None,
////        https_proxy: None,
////        all_proxy: None
////    });
/// ```
pub fn run(
    entry: Entry,
    http_client: &mut http::Client,
    entry_index: usize,
    variables: &mut HashMap<String, Value>,
    options: &RunnerOptions,
    logger: &Logger,
) -> Vec<EntryResult> {
    let http_request = match eval_request(entry.request.clone(), variables, &options.context_dir) {
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

    logger.debug_important(
        "------------------------------------------------------------------------------",
    );
    logger.debug_important(format!("Executing entry {}", entry_index + 1).as_str());

    //
    // Experimental features
    // with cookie storage
    //
    use std::str::FromStr;
    if let Some(s) = cookie_storage_set(entry.request.clone()) {
        if let Ok(cookie) = http::Cookie::from_str(s.as_str()) {
            http_client.add_cookie(cookie);
        } else {
            logger.warning(format!("Cookie string can not be parsed: '{}'", s).as_str());
        }
    }
    if cookie_storage_clear(entry.request.clone()) {
        http_client.clear_cookie_storage();
    }

    logger.debug("");
    logger.debug_important("Cookie store:");
    for cookie in http_client.get_cookie_storage() {
        logger.debug(cookie.to_string().as_str());
    }
    logger.debug("");
    log_request_spec(&http_request, logger);
    logger.debug("Request can be run with the following curl command:");
    logger.debug(http_client.curl_command_line(&http_request).as_str());
    logger.debug("");

    let calls = match http_client.execute_with_redirect(&http_request, logger) {
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
                        start: entry.request.url.source_info.start,
                        end: entry.request.url.source_info.end,
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
            asserts = if options.ignore_asserts {
                vec![]
            } else {
                match entry.response.clone() {
                    None => vec![],
                    Some(response) => eval_asserts(
                        response,
                        variables,
                        http_response.clone(),
                        &options.context_dir,
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

/// Logs a HTTP request spec
///
/// # Arguments
///
/// * `request` - An HTTP request spec
/// * `logger` - A logger
///
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
