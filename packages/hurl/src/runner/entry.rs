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
use crate::runner::template::eval_template;

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
    logger: &Logger,
) -> EntryResult {
    let http_request = match eval_request(&entry.request, variables, &runner_options.context_dir) {
        Ok(r) => r,
        Err(error) => {
            return EntryResult {
                calls: vec![],
                captures: vec![],
                asserts: vec![],
                errors: vec![error],
                time_in_ms: 0,
                compressed: runner_options.compressed,
            };
        }
    };
    let client_options = http::ClientOptions::from(runner_options);

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
            .curl_command_line(&http_request, &runner_options.context_dir, &client_options)
            .as_str(),
    );
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
    let (http_request, http_response) = calls.last().unwrap();
    let calls: Vec<Call> = calls
        .iter()
        .map(|(req, resp)| Call {
            request: req.clone(),
            response: resp.clone(),
        })
        .collect();
    let time_in_ms = calls.iter().map(|c| c.response.duration.as_millis()).sum();

    // Compute captures
    let captures = match &entry.response {
        None => vec![],
        Some(response_spec) => match eval_captures(response_spec, http_response, variables) {
            Ok(captures) => captures,
            Err(e) => {
                return EntryResult {
                    calls,
                    captures: vec![],
                    asserts: vec![],
                    errors: vec![e],
                    time_in_ms,
                    compressed: client_options.compressed,
                };
            }
        },
    };

    // Update variables now!

    // Set variables implicitly
    // For the time-being, the variable self_request_url should only be used by the query <url>
    // In the long-term, we should set a request object with several fields that will be accessed
    // with the expression {{request.url}}
    variables.insert(
        "self_request_url".to_string(),
        Value::String(http_request.url.clone()),
    );
    for c in captures.iter() {
        variables.insert(c.name.clone(), c.value.clone());
    }
    if !captures.is_empty() {
        logger.debug_important("Captures:");
        for c in captures.iter() {
            logger.capture(&c.name, &c.value);
        }
    }
    logger.debug("");

    // Compute asserts
    let asserts = if runner_options.ignore_asserts {
        vec![]
    } else {
        match &entry.response {
            None => vec![],
            Some(response_spec) => eval_asserts(
                response_spec,
                variables,
                http_response,
                &runner_options.context_dir,
            ),
        }
    };

    let errors = asserts
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
        .collect();

    EntryResult {
        calls,
        captures,
        asserts,
        errors,
        time_in_ms,
        compressed: client_options.compressed,
    }
}

impl From<&RunnerOptions> for ClientOptions {
    fn from(runner_options: &RunnerOptions) -> Self {
        ClientOptions {
            cacert_file: runner_options.cacert_file.clone(),
            follow_location: runner_options.follow_location,
            max_redirect: runner_options.max_redirect,
            cookie_input_file: runner_options.cookie_input_file.clone(),
            proxy: runner_options.proxy.clone(),
            no_proxy: runner_options.no_proxy.clone(),
            verbosity: runner_options.verbosity.as_ref().map(|v| match v {
                Verbosity::Verbose => http::Verbosity::Verbose,
                Verbosity::VeryVerbose => http::Verbosity::VeryVerbose,
            }),
            insecure: runner_options.insecure,
            timeout: runner_options.timeout,
            connect_timeout: runner_options.connect_timeout,
            user: runner_options.user.clone(),
            user_agent: runner_options.user_agent.clone(),
            compressed: runner_options.compressed,
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
        logger.debug(format!("Implicit content-type={}", s).as_str());
    }
    logger.debug("");
}

/// Returns a new [`RunnerOptions`] based on the `entry` optional Options section
/// and a default `runner_options`.
/// The [`variables`] can also be updated if `variable` keys are present in the section.
pub fn get_entry_options(
    entry: &Entry,
    runner_options: &RunnerOptions,
    variables: &mut HashMap<String, Value>,
    logger: &Logger,
) -> Result<RunnerOptions, Error> {
    let mut runner_options = runner_options.clone();
    if !has_options(entry) {
        return Ok(runner_options);
    }

    logger.debug("");
    logger.debug_important("Entry options:");

    for section in &entry.request.sections {
        if let SectionValue::Options(options) = &section.value {
            for option in options {
                match option {
                    EntryOption::CaCertificate(option) => {
                        runner_options.cacert_file = Some(option.filename.value.clone());
                        logger.debug(format!("cacert: {}", option.filename.value).as_str());
                    }
                    EntryOption::Compressed(option) => {
                        runner_options.compressed = option.value;
                        logger.debug(format!("compressed: {}", option.value).as_str());
                    }
                    EntryOption::Insecure(option) => {
                        runner_options.insecure = option.value;
                        logger.debug(format!("insecure: {}", option.value).as_str());
                    }
                    EntryOption::FollowLocation(option) => {
                        runner_options.follow_location = option.value;
                        logger.debug(format!("location: {}", option.value).as_str());
                    }
                    EntryOption::MaxRedirect(option) => {
                        runner_options.max_redirect = Some(option.value);
                        logger.debug(format!("max-redirs: {}", option.value).as_str());
                    }
                    EntryOption::Variable(VariableOption {
                        value: VariableDefinition { name, value, .. },
                        ..
                    }) => {
                        let value = eval_variable_value(value, variables)?;
                        variables.insert(name.clone(), value);
                    }
                    EntryOption::Verbose(option) => {
                        runner_options.verbosity = if option.value {
                            Some(Verbosity::Verbose)
                        } else {
                            None
                        };
                        logger.debug(format!("verbose: {}", option.value).as_str());
                    }

                    EntryOption::VeryVerbose(option) => {
                        runner_options.verbosity = if option.value {
                            Some(Verbosity::VeryVerbose)
                        } else {
                            None
                        };
                        logger.debug(format!("very-verbose: {}", option.value).as_str());
                    }
                }
            }
        }
    }
    Ok(runner_options)
}

fn eval_variable_value(
    variable_value: &VariableValue,
    variables: &mut HashMap<String, Value>,
) -> Result<Value, Error> {
    match variable_value {
        VariableValue::Null {} => Ok(Value::Null),
        VariableValue::Bool(v) => Ok(Value::Bool(*v)),
        VariableValue::Integer(v) => Ok(Value::Integer(*v)),
        VariableValue::Float(Float { value, .. }) => Ok(Value::Float(*value)),
        VariableValue::String(template) => {
            let s = eval_template(template, variables)?;
            Ok(Value::String(s))
        }
    }
}

/// Returns [`true`] if this `entry` has an Option section, [`false`] otherwise.
fn has_options(entry: &Entry) -> bool {
    entry
        .request
        .sections
        .iter()
        .any(|s| matches!(s.value, SectionValue::Options(_)))
}

/// Returns the overridden `entry` verbosity, or the default `verbosity` file.
pub fn get_entry_verbosity(entry: &Entry, verbosity: &Option<Verbosity>) -> Option<Verbosity> {
    let mut verbosity = verbosity.clone();

    for section in &entry.request.sections {
        if let SectionValue::Options(options) = &section.value {
            for option in options {
                match option {
                    EntryOption::Verbose(option) => {
                        verbosity = if option.value {
                            Some(Verbosity::Verbose)
                        } else {
                            None
                        }
                    }
                    EntryOption::VeryVerbose(option) => {
                        verbosity = if option.value {
                            Some(Verbosity::VeryVerbose)
                        } else {
                            None
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    verbosity
}
