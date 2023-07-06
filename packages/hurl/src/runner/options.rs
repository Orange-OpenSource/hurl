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

use crate::runner::template;
use crate::runner::{Error, RunnerOptions, Value};
use crate::util::logger::{Logger, Verbosity};
use hurl_core::ast::{
    Entry, EntryOption, Float, SectionValue, VariableDefinition, VariableOption, VariableValue,
};
use std::collections::HashMap;
use std::time::Duration;

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
                    EntryOption::ClientCert(option) => {
                        runner_options.client_cert_file = Some(option.filename.value.clone());
                        logger.debug(format!("cert: {}", option.filename.value).as_str());
                    }
                    EntryOption::ClientKey(option) => {
                        runner_options.client_key_file = Some(option.filename.value.clone());
                        logger.debug(format!("key: {}", option.filename.value).as_str());
                    }
                    EntryOption::Compressed(option) => {
                        runner_options.compressed = option.value;
                        logger.debug(format!("compressed: {}", option.value).as_str());
                    }
                    EntryOption::FollowLocation(option) => {
                        runner_options.follow_location = option.value;
                        logger.debug(format!("location: {}", option.value).as_str());
                    }
                    EntryOption::Insecure(option) => {
                        runner_options.insecure = option.value;
                        logger.debug(format!("insecure: {}", option.value).as_str());
                    }
                    EntryOption::MaxRedirect(option) => {
                        runner_options.max_redirect = Some(option.value);
                        logger.debug(format!("max-redirs: {}", option.value).as_str());
                    }
                    EntryOption::PathAsIs(option) => {
                        runner_options.path_as_is = option.value;
                        logger.debug(format!("path-as-is: {}", option.value).as_str());
                    }
                    EntryOption::Proxy(option) => {
                        runner_options.proxy = Some(option.value.clone());
                        logger.debug(format!("proxy: {}", option.value).as_str());
                    }
                    EntryOption::Resolve(option) => {
                        let mut resolves = runner_options.resolves;
                        resolves.push(option.value.clone());
                        runner_options.resolves = resolves;
                        logger.debug(format!("resolve: {}", option.value).as_str());
                    }
                    EntryOption::Retry(option) => {
                        runner_options.retry = option.value;
                        logger.debug(format!("retry: {}", option.value).as_str());
                    }
                    EntryOption::RetryInterval(option) => {
                        runner_options.retry_interval = Duration::from_millis(option.value);
                        logger.debug(format!("retry-interval: {}", option.value).as_str());
                    }
                    EntryOption::Variable(VariableOption {
                        value: VariableDefinition { name, value, .. },
                        ..
                    }) => {
                        let value = eval_variable_value(value, variables)?;
                        logger.debug(format!("variable: {}={}", name, value).as_str());
                        variables.insert(name.clone(), value);
                    }
                    EntryOption::Verbose(option) => {
                        logger.debug(format!("verbose: {}", option.value).as_str());
                    }

                    EntryOption::VeryVerbose(option) => {
                        logger.debug(format!("very-verbose: {}", option.value).as_str());
                    }
                }
            }
        }
    }
    Ok(runner_options)
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
    let mut verbosity = *verbosity;

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
            let s = template::eval_template(template, variables)?;
            Ok(Value::String(s))
        }
    }
}
