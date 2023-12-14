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

use hurl_core::ast::{
    BooleanOption, Entry, EntryOption, Float, NaturalOption, Number as AstNumber, OptionKind,
    Retry, RetryOption, SectionValue, VariableDefinition, VariableValue,
};

use crate::http::{IpResolve, RequestedHttpVersion};
use crate::runner::template::{eval_expression, eval_template};
use crate::runner::{template, Error, Number, RunnerError, RunnerOptions, Value};
use crate::util::logger::{Logger, Verbosity};

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
                    OptionKind::AwsSigV4(value) => {
                        let value = eval_template(value, variables)?;
                        runner_options.aws_sigv4 = Some(value)
                    }
                    OptionKind::CaCertificate(filename) => {
                        runner_options.cacert_file = Some(filename.value.clone())
                    }
                    OptionKind::ClientCert(filename) => {
                        runner_options.client_cert_file = Some(filename.value.clone())
                    }
                    OptionKind::ClientKey(filename) => {
                        runner_options.client_key_file = Some(filename.value.clone())
                    }
                    OptionKind::Compressed(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        runner_options.compressed = value
                    }
                    OptionKind::ConnectTo(value) => {
                        let value = eval_template(value, variables)?;
                        runner_options.connects_to.push(value)
                    }
                    OptionKind::Delay(value) => {
                        let value = eval_natural_option(value, variables)?;
                        runner_options.delay = Duration::from_millis(value)
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
                            runner_options.http_version = RequestedHttpVersion::Http10
                        }
                    }
                    OptionKind::Http11(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            runner_options.http_version = RequestedHttpVersion::Http11
                        } else {
                            runner_options.http_version = RequestedHttpVersion::Http10
                        }
                    }
                    OptionKind::Http2(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            runner_options.http_version = RequestedHttpVersion::Http2
                        } else {
                            runner_options.http_version = RequestedHttpVersion::Http11
                        }
                    }
                    OptionKind::Http3(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            runner_options.http_version = RequestedHttpVersion::Http3
                        } else {
                            runner_options.http_version = RequestedHttpVersion::Http2
                        }
                    }
                    OptionKind::FollowLocation(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        runner_options.follow_location = value;
                    }
                    OptionKind::Insecure(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        runner_options.insecure = value
                    }
                    OptionKind::IpV4(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        runner_options.ip_resolve = if value {
                            IpResolve::IpV4
                        } else {
                            IpResolve::IpV6
                        }
                    }
                    OptionKind::IpV6(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        runner_options.ip_resolve = if value {
                            IpResolve::IpV6
                        } else {
                            IpResolve::IpV4
                        }
                    }
                    OptionKind::MaxRedirect(value) => {
                        let value = eval_natural_option(value, variables)?;
                        runner_options.max_redirect = Some(value as usize)
                    }
                    OptionKind::Output(filename) => {
                        runner_options.output = Some(filename.value.clone())
                    }
                    OptionKind::PathAsIs(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        runner_options.path_as_is = value
                    }
                    OptionKind::Proxy(value) => {
                        let value = eval_template(value, variables)?;
                        runner_options.proxy = Some(value.to_string())
                    }
                    OptionKind::Resolve(value) => {
                        let value = eval_template(value, variables)?;
                        runner_options.resolves.push(value)
                    }
                    OptionKind::Retry(value) => {
                        let value = eval_retry_option(value, variables)?;
                        runner_options.retry = value
                    }
                    OptionKind::RetryInterval(value) => {
                        let value = eval_natural_option(value, variables)?;
                        runner_options.retry_interval = Duration::from_millis(value)
                    }
                    OptionKind::Skip(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        runner_options.skip = value
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
pub fn get_entry_verbosity(
    entry: &Entry,
    verbosity: &Option<Verbosity>,
    variables: &HashMap<String, Value>,
) -> Result<Option<Verbosity>, Error> {
    let mut verbosity = *verbosity;

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
) -> Result<bool, Error> {
    match boolean_value {
        BooleanOption::Literal(value) => Ok(*value),
        BooleanOption::Expression(expr) => match eval_expression(expr, variables)? {
            Value::Bool(value) => Ok(value),
            v => {
                let inner = RunnerError::TemplateVariableInvalidType {
                    name: expr.variable.name.clone(),
                    value: v.to_string(),
                    expecting: "boolean".to_string(),
                };
                Err(Error::new(expr.variable.source_info, inner, false))
            }
        },
    }
}

fn eval_natural_option(
    natural_value: &NaturalOption,
    variables: &HashMap<String, Value>,
) -> Result<u64, Error> {
    match natural_value {
        NaturalOption::Literal(value) => Ok(*value),
        NaturalOption::Expression(expr) => match eval_expression(expr, variables)? {
            Value::Number(Number::Integer(value)) => {
                if value < 0 {
                    let inner = RunnerError::TemplateVariableInvalidType {
                        name: expr.variable.name.clone(),
                        value: value.to_string(),
                        expecting: "positive integer".to_string(),
                    };
                    Err(Error::new(expr.variable.source_info, inner, false))
                } else {
                    Ok(value as u64)
                }
            }
            v => {
                let inner = RunnerError::TemplateVariableInvalidType {
                    name: expr.variable.name.clone(),
                    value: v.to_string(),
                    expecting: "positive integer".to_string(),
                };
                Err(Error::new(expr.variable.source_info, inner, false))
            }
        },
    }
}

fn eval_retry_option(
    retry_option_value: &RetryOption,
    variables: &HashMap<String, Value>,
) -> Result<Retry, Error> {
    match retry_option_value {
        RetryOption::Literal(retry) => Ok(*retry),
        RetryOption::Expression(expr) => match eval_expression(expr, variables)? {
            Value::Number(Number::Integer(value)) => {
                if value == -1 {
                    Ok(Retry::Infinite)
                } else if value == 0 {
                    Ok(Retry::None)
                } else if value > 0 {
                    Ok(Retry::Finite(value as usize))
                } else {
                    let inner = RunnerError::TemplateVariableInvalidType {
                        name: expr.variable.name.clone(),
                        value: value.to_string(),
                        expecting: "integer".to_string(),
                    };
                    Err(Error::new(expr.variable.source_info, inner, false))
                }
            }
            v => {
                let inner = RunnerError::TemplateVariableInvalidType {
                    name: expr.variable.name.clone(),
                    value: v.to_string(),
                    expecting: "integer".to_string(),
                };
                Err(Error::new(expr.variable.source_info, inner, false))
            }
        },
    }
}

fn eval_variable_value(
    variable_value: &VariableValue,
    variables: &mut HashMap<String, Value>,
) -> Result<Value, Error> {
    match variable_value {
        VariableValue::Null => Ok(Value::Null),
        VariableValue::Bool(v) => Ok(Value::Bool(*v)),
        VariableValue::Number(v) => Ok(eval_number(v)),
        VariableValue::String(template) => {
            let s = template::eval_template(template, variables)?;
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
    use hurl_core::ast::{Expr, Pos, SourceInfo, Variable, Whitespace};

    use super::*;
    use crate::runner::RunnerError;

    fn verbose_option_template() -> BooleanOption {
        // {{verbose}}
        BooleanOption::Expression(Expr {
            space0: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            variable: Variable {
                name: "verbose".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
        })
    }

    fn retry_option_template() -> NaturalOption {
        // {{retry}}
        NaturalOption::Expression(Expr {
            space0: Whitespace {
                value: "".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            variable: Variable {
                name: "retry".to_string(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: Whitespace {
                value: "".to_string(),
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
            error.inner,
            RunnerError::TemplateVariableNotDefined {
                name: "verbose".to_string()
            }
        );

        variables.insert("verbose".to_string(), Value::Number(Number::Integer(10)));
        let error = eval_boolean_option(&verbose_option_template(), &variables)
            .err()
            .unwrap();
        assert_eq!(
            error.inner,
            RunnerError::TemplateVariableInvalidType {
                name: "verbose".to_string(),
                value: "10".to_string(),
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
