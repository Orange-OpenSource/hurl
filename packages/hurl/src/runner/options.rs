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
    Entry, EntryOption, Float, OptionKind, SectionValue, VariableDefinition, VariableValue,
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
            for option in options.iter() {
                match &option.kind {
                    OptionKind::AwsSigV4(value) => runner_options.aws_sigv4 = Some(value.clone()),
                    OptionKind::CaCertificate(filename) => {
                        runner_options.cacert_file = Some(filename.value.clone())
                    }
                    OptionKind::ClientCert(filename) => {
                        runner_options.client_cert_file = Some(filename.value.clone())
                    }
                    OptionKind::ClientKey(filename) => {
                        runner_options.client_key_file = Some(filename.value.clone())
                    }
                    OptionKind::Compressed(value) => runner_options.compressed = *value,
                    OptionKind::ConnectTo(value) => runner_options.connects_to.push(value.clone()),
                    OptionKind::Delay(value) => {
                        runner_options.delay = Duration::from_millis(*value)
                    }
                    OptionKind::Insecure(value) => runner_options.insecure = *value,
                    OptionKind::FollowLocation(value) => runner_options.follow_location = *value,
                    OptionKind::MaxRedirect(value) => runner_options.max_redirect = Some(*value),
                    OptionKind::PathAsIs(value) => runner_options.path_as_is = *value,
                    OptionKind::Proxy(value) => runner_options.proxy = Some(value.clone()),
                    OptionKind::Resolve(value) => runner_options.resolves.push(value.clone()),
                    OptionKind::Retry(value) => runner_options.retry = *value,
                    OptionKind::RetryInterval(value) => {
                        runner_options.retry_interval = Duration::from_millis(*value)
                    }
                    OptionKind::Variable(VariableDefinition { name, value, .. }) => {
                        let value = eval_variable_value(value, variables)?;
                        variables.insert(name.clone(), value);
                    }
                    // verbose and very-verbose option have been previously processed as thy
                    // can impact the logging.
                    OptionKind::Verbose(_) => {}
                    OptionKind::VeryVerbose(_) => {}
                }
                log_option(option, logger);
            }
        }
    }
    Ok(runner_options)
}

/// Logs an entry option.
fn log_option(option: &EntryOption, logger: &Logger) {
    let name = option.kind.name();
    let value = option.kind.value_as_str();
    logger.debug(&format!("{name}: {value}"));
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
                match &option.kind {
                    OptionKind::Verbose(value) => {
                        verbosity = if *value {
                            Some(Verbosity::Verbose)
                        } else {
                            None
                        }
                    }
                    OptionKind::VeryVerbose(value) => {
                        verbosity = if *value {
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
        VariableValue::Null => Ok(Value::Null),
        VariableValue::Bool(v) => Ok(Value::Bool(*v)),
        VariableValue::Integer(v) => Ok(Value::Integer(*v)),
        VariableValue::Float(Float { value, .. }) => Ok(Value::Float(*value)),
        VariableValue::String(template) => {
            let s = template::eval_template(template, variables)?;
            Ok(Value::String(s))
        }
    }
}
