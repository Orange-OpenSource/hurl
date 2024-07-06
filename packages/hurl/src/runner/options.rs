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
use std::time::Duration;

use hurl_core::ast::{
    BooleanOption, Entry, EntryOption, Float, NaturalOption, Number as AstNumber, OptionKind,
    RepeatOption, RetryOption, SectionValue, VariableDefinition, VariableValue,
};
use hurl_core::typing::{Repeat, Retry};

use crate::http::{IpResolve, RequestedHttpVersion};
use crate::runner::template::{eval_expression, eval_template};
use crate::runner::{Number, Output, RunnerError, RunnerErrorKind, RunnerOptions, Value};
use crate::util::logger::{Logger, Verbosity};

/// Returns a new [`RunnerOptions`] based on the `entry` optional Options section
/// and a default `runner_options`.
/// The [`variables`] can also be updated if `variable` keys are present in the section.
pub fn get_entry_options(
    entry: &Entry,
    runner_options: &RunnerOptions,
    variables: &mut HashMap<String, Value>,
    logger: &mut Logger,
) -> Result<RunnerOptions, RunnerError> {
    let runner_options = runner_options.clone();
    // When used globally (on the command line), `--output` writes the last successful request
    // to `output` file. We don't want to output every entry's response, so we initialise
    // output to `None`.
    let mut entry_options = RunnerOptions {
        output: None,
        ..runner_options
    };
    if !has_options(entry) {
        return Ok(entry_options);
    }

    logger.debug("");
    logger.debug_important("Entry options:");

    for section in &entry.request.sections {
        if let SectionValue::Options(options) = &section.value {
            for option in options.iter() {
                match &option.kind {
                    OptionKind::AwsSigV4(value) => {
                        let value = eval_template(value, variables)?;
                        entry_options.aws_sigv4 = Some(value);
                    }
                    OptionKind::CaCertificate(filename) => {
                        let value = eval_template(filename, variables)?;
                        entry_options.cacert_file = Some(value);
                    }
                    OptionKind::ClientCert(filename) => {
                        let value = eval_template(filename, variables)?;
                        entry_options.client_cert_file = Some(value);
                    }
                    OptionKind::ClientKey(filename) => {
                        let value = eval_template(filename, variables)?;
                        entry_options.client_key_file = Some(value);
                    }
                    OptionKind::Compressed(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.compressed = value;
                    }
                    OptionKind::ConnectTo(value) => {
                        let value = eval_template(value, variables)?;
                        entry_options.connects_to.push(value);
                    }
                    OptionKind::Delay(value) => {
                        let value = eval_natural_option(value, variables)?;
                        entry_options.delay = Duration::from_millis(value);
                    }
                    // HTTP version options (such as http1.0, http1.1, http2 etc...) are activated
                    // through a flag. In an `[Options]` section, the signification of such a flag is:
                    //
                    // - when set to `true`, it's equivalent as using this option on command line
                    //
                    // ```hurl
                    // # Shell equivalent command:
                    // # $ hurl --http1.1 foo.hurl
                    // GET https://foo.com
                    // [Options]
                    // http1.1: true
                    // ```
                    //
                    // - when set to `false`, it's as if the user do not want to use such a version.
                    // So, if such a flag is explicitly set to `false`, we downgrade to the lower
                    // HTTP version:
                    //
                    // ```hurl
                    // # Shell equivalent command:
                    // # $ hurl --http1.1 foo.hurl
                    // GET https://foo.com
                    // [Options]
                    // http2: false
                    // ```
                    OptionKind::Http10(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.http_version = RequestedHttpVersion::Http10;
                        }
                    }
                    OptionKind::Http11(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.http_version = RequestedHttpVersion::Http11;
                        } else {
                            entry_options.http_version = RequestedHttpVersion::Http10;
                        }
                    }
                    OptionKind::Http2(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.http_version = RequestedHttpVersion::Http2;
                        } else {
                            entry_options.http_version = RequestedHttpVersion::Http11;
                        }
                    }
                    OptionKind::Http3(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.http_version = RequestedHttpVersion::Http3;
                        } else {
                            entry_options.http_version = RequestedHttpVersion::Http2;
                        }
                    }
                    OptionKind::FollowLocation(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.follow_location = value;
                    }
                    OptionKind::FollowLocationTrusted(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.follow_location = true;
                        }
                        entry_options.follow_location_trusted = value;
                    }
                    OptionKind::Insecure(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.insecure = value;
                    }
                    OptionKind::IpV4(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.ip_resolve = if value {
                            IpResolve::IpV4
                        } else {
                            IpResolve::IpV6
                        }
                    }
                    OptionKind::IpV6(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.ip_resolve = if value {
                            IpResolve::IpV6
                        } else {
                            IpResolve::IpV4
                        }
                    }
                    OptionKind::MaxRedirect(value) => {
                        let value = eval_natural_option(value, variables)?;
                        entry_options.max_redirect = Some(value as usize);
                    }
                    OptionKind::NetRc(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.netrc = value;
                    }
                    OptionKind::NetRcFile(value) => {
                        let filename = eval_template(value, variables)?;
                        entry_options.netrc_file = Some(filename);
                    }
                    OptionKind::NetRcOptional(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.netrc_optional = value;
                    }
                    OptionKind::Output(output) => {
                        let filename = eval_template(output, variables)?;
                        let output = Output::new(&filename);
                        entry_options.output = Some(output);
                    }
                    OptionKind::PathAsIs(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.path_as_is = value;
                    }
                    OptionKind::Proxy(value) => {
                        let value = eval_template(value, variables)?;
                        entry_options.proxy = Some(value);
                    }
                    OptionKind::Repeat(value) => {
                        let value = eval_repeat_option(value, variables)?;
                        entry_options.repeat = Some(value);
                    }
                    OptionKind::Resolve(value) => {
                        let value = eval_template(value, variables)?;
                        entry_options.resolves.push(value);
                    }
                    OptionKind::Retry(value) => {
                        let value = eval_retry_option(value, variables)?;
                        entry_options.retry = Some(value);
                    }
                    OptionKind::RetryInterval(value) => {
                        let value = eval_natural_option(value, variables)?;
                        entry_options.retry_interval = Duration::from_millis(value);
                    }
                    OptionKind::Skip(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        entry_options.skip = value;
                    }
                    OptionKind::UnixSocket(value) => {
                        let value = eval_template(value, variables)?;
                        entry_options.unix_socket = Some(value);
                    }
                    OptionKind::User(value) => {
                        let value = eval_template(value, variables)?;
                        entry_options.user = Some(value);
                    }
                    OptionKind::Variable(VariableDefinition { name, value, .. }) => {
                        let value = eval_variable_value(value, variables)?;
                        variables.insert(name.clone(), value);
                    }
                    // verbose and very-verbose option have been previously processed as they
                    // can impact the logging. We compute here their values to check the potential
                    // templatized error.
                    OptionKind::Verbose(value) => {
                        eval_boolean_option(value, variables)?;
                    }
                    OptionKind::VeryVerbose(value) => {
                        eval_boolean_option(value, variables)?;
                    }
                }
                log_option(option, logger);
            }
        }
    }
    Ok(entry_options)
}

/// Logs an entry option.
fn log_option(option: &EntryOption, logger: &mut Logger) {
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
pub fn get_entry_verbosity(
    entry: &Entry,
    default_verbosity: Option<Verbosity>,
    variables: &HashMap<String, Value>,
) -> Result<Option<Verbosity>, RunnerError> {
    let mut verbosity = default_verbosity;

    for section in &entry.request.sections {
        if let SectionValue::Options(options) = &section.value {
            for option in options {
                match &option.kind {
                    OptionKind::Verbose(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        verbosity = if value {
                            Some(Verbosity::Verbose)
                        } else {
                            None
                        }
                    }
                    OptionKind::VeryVerbose(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        verbosity = if value {
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
    Ok(verbosity)
}

fn eval_boolean_option(
    boolean_value: &BooleanOption,
    variables: &HashMap<String, Value>,
) -> Result<bool, RunnerError> {
    match boolean_value {
        BooleanOption::Literal(value) => Ok(*value),
        BooleanOption::Expression(expr) => match eval_expression(expr, variables)? {
            Value::Bool(value) => Ok(value),
            v => {
                let kind = RunnerErrorKind::TemplateVariableInvalidType {
                    name: expr.variable.name.clone(),
                    value: v.format(),
                    expecting: "boolean".to_string(),
                };
                Err(RunnerError::new(expr.variable.source_info, kind, false))
            }
        },
    }
}

/// Evals a natural option value (>=0), given a set of `variables`.
fn eval_natural_option(
    natural_value: &NaturalOption,
    variables: &HashMap<String, Value>,
) -> Result<u64, RunnerError> {
    match natural_value {
        NaturalOption::Literal(value) => Ok(*value),
        NaturalOption::Expression(expr) => match eval_expression(expr, variables)? {
            Value::Number(Number::Integer(value)) => {
                if value < 0 {
                    let kind = RunnerErrorKind::TemplateVariableInvalidType {
                        name: expr.variable.name.clone(),
                        value: format!("integer <{value}>"),
                        expecting: "positive integer".to_string(),
                    };
                    Err(RunnerError::new(expr.variable.source_info, kind, false))
                } else {
                    Ok(value as u64)
                }
            }
            v => {
                let kind = RunnerErrorKind::TemplateVariableInvalidType {
                    name: expr.variable.name.clone(),
                    value: v.format(),
                    expecting: "positive integer".to_string(),
                };
                Err(RunnerError::new(expr.variable.source_info, kind, false))
            }
        },
    }
}

/// Render an AST repeat option with a `variables` set.
fn eval_repeat_option(
    repeat_option_value: &RepeatOption,
    variables: &HashMap<String, Value>,
) -> Result<Repeat, RunnerError> {
    match repeat_option_value {
        RepeatOption::Literal(repeat) => Ok(*repeat),
        RepeatOption::Expression(expr) => match eval_expression(expr, variables)? {
            Value::Number(Number::Integer(value)) => {
                if value == -1 {
                    Ok(Repeat::Forever)
                } else if value >= 0 {
                    Ok(Repeat::Count(value as usize))
                } else {
                    let kind = RunnerErrorKind::TemplateVariableInvalidType {
                        name: expr.variable.name.clone(),
                        value: format!("integer <{value}>"),
                        expecting: "integer".to_string(),
                    };
                    Err(RunnerError::new(expr.variable.source_info, kind, false))
                }
            }
            v => {
                let kind = RunnerErrorKind::TemplateVariableInvalidType {
                    name: expr.variable.name.clone(),
                    value: v.format(),
                    expecting: "integer".to_string(),
                };
                Err(RunnerError::new(expr.variable.source_info, kind, false))
            }
        },
    }
}

/// Render an AST retry option with a `variables` set.
fn eval_retry_option(
    retry_option_value: &RetryOption,
    variables: &HashMap<String, Value>,
) -> Result<Retry, RunnerError> {
    match retry_option_value {
        RetryOption::Literal(retry) => Ok(*retry),
        RetryOption::Expression(expr) => match eval_expression(expr, variables)? {
            Value::Number(Number::Integer(value)) => {
                if value == -1 {
                    Ok(Retry::Infinite)
                } else if value >= 0 {
                    Ok(Retry::Finite(value as usize))
                } else {
                    let kind = RunnerErrorKind::TemplateVariableInvalidType {
                        name: expr.variable.name.clone(),
                        value: format!("integer <{value}>"),
                        expecting: "integer >= -1".to_string(),
                    };
                    Err(RunnerError::new(expr.variable.source_info, kind, false))
                }
            }
            v => {
                let kind = RunnerErrorKind::TemplateVariableInvalidType {
                    name: expr.variable.name.clone(),
                    value: v.to_string(),
                    expecting: "integer".to_string(),
                };
                Err(RunnerError::new(expr.variable.source_info, kind, false))
            }
        },
    }
}

fn eval_variable_value(
    variable_value: &VariableValue,
    variables: &mut HashMap<String, Value>,
) -> Result<Value, RunnerError> {
    match variable_value {
        VariableValue::Null => Ok(Value::Null),
        VariableValue::Bool(v) => Ok(Value::Bool(*v)),
        VariableValue::Number(v) => Ok(eval_number(v)),
        VariableValue::String(template) => {
            let s = eval_template(template, variables)?;
            Ok(Value::String(s))
        }
    }
}

fn eval_number(number: &AstNumber) -> Value {
    match number {
        AstNumber::Float(Float { value, .. }) => Value::Number(Number::Float(*value)),
        AstNumber::Integer(value) => Value::Number(Number::Integer(*value)),
        AstNumber::BigInteger(value) => Value::Number(Number::BigInteger(value.clone())),
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Expr, SourceInfo, Variable, Whitespace};
    use hurl_core::reader::Pos;

    use super::*;
    use crate::runner::RunnerErrorKind;

    fn verbose_option_template() -> BooleanOption {
        // {{verbose}}
        BooleanOption::Expression(Expr {
            space0: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            variable: Variable {
                name: "verbose".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
        })
    }

    fn retry_option_template() -> NaturalOption {
        // {{retry}}
        NaturalOption::Expression(Expr {
            space0: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            variable: Variable {
                name: "retry".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
        })
    }

    #[test]
    fn test_eval_boolean_option() {
        let mut variables = HashMap::default();
        assert!(eval_boolean_option(&BooleanOption::Literal(true), &variables).unwrap());

        variables.insert("verbose".to_string(), Value::Bool(true));
        assert!(eval_boolean_option(&verbose_option_template(), &variables).unwrap());
    }

    #[test]
    fn test_eval_boolean_option_error() {
        let mut variables = HashMap::default();
        let error = eval_boolean_option(&verbose_option_template(), &variables)
            .err()
            .unwrap();
        assert!(!error.assert);
        assert_eq!(
            error.kind,
            RunnerErrorKind::TemplateVariableNotDefined {
                name: "verbose".to_string()
            }
        );

        variables.insert("verbose".to_string(), Value::Number(Number::Integer(10)));
        let error = eval_boolean_option(&verbose_option_template(), &variables)
            .err()
            .unwrap();
        assert_eq!(
            error.kind,
            RunnerErrorKind::TemplateVariableInvalidType {
                name: "verbose".to_string(),
                value: "integer <10>".to_string(),
                expecting: "boolean".to_string()
            }
        );
    }

    #[test]
    fn test_eval_natural_option() {
        let mut variables = HashMap::default();
        assert_eq!(
            eval_natural_option(&NaturalOption::Literal(1), &variables).unwrap(),
            1
        );

        variables.insert("retry".to_string(), Value::Number(Number::Integer(10)));
        assert_eq!(
            eval_natural_option(&retry_option_template(), &variables).unwrap(),
            10
        );
    }
}
