/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
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
use hurl_core::ast::{
    BooleanOption, CountOption, DurationOption, Entry, EntryOption, NaturalOption,
    Number as AstNumber, OptionKind, Placeholder, SectionValue, VariableDefinition, VariableValue,
};
use hurl_core::typing::{BytesPerSec, Count, DurationUnit};

use crate::http::{IpResolve, RequestedHttpVersion};
use crate::runner::template::eval_template;
use crate::runner::{
    expr, Number, Output, RunnerError, RunnerErrorKind, RunnerOptions, Value, VariableSet,
};
use crate::util::logger::{Logger, Verbosity};

/// Returns a new [`RunnerOptions`] based on the `entry` optional Options section
/// and a default `runner_options`.
/// The [`variables`] can also be updated if `variable` keys are present in the section.
pub fn get_entry_options(
    entry: &Entry,
    runner_options: &RunnerOptions,
    variables: &mut VariableSet,
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
                    OptionKind::ConnectTimeout(value) => {
                        let value =
                            eval_duration_option(value, variables, DurationUnit::MilliSecond)?;
                        entry_options.connect_timeout = value;
                    }
                    OptionKind::Delay(value) => {
                        let value =
                            eval_duration_option(value, variables, DurationUnit::MilliSecond)?;
                        entry_options.delay = value;
                    }
                    OptionKind::Header(value) => {
                        let value = eval_template(value, variables)?;
                        entry_options.headers.push(value);
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
                    //
                    // As libcurl tries to reuse connections as much as possible (see <https://curl.se/libcurl/c/CURLOPT_HTTP_VERSION.html>)
                    // > Note that the HTTP version is just a request. libcurl still prioritizes to reuse
                    // > existing connections so it might then reuse a connection using a HTTP version you
                    // > have not asked for.
                    // we don't allow our HTTP client to reuse connection if the user asks for a specific
                    // HTTP version per request.
                    //
                    OptionKind::Http10(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.http_version = RequestedHttpVersion::Http10;
                        }
                        entry_options.allow_reuse = false;
                    }
                    OptionKind::Http11(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.http_version = RequestedHttpVersion::Http11;
                        } else {
                            entry_options.http_version = RequestedHttpVersion::Http10;
                        }
                        entry_options.allow_reuse = false;
                    }
                    OptionKind::Http2(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.http_version = RequestedHttpVersion::Http2;
                        } else {
                            entry_options.http_version = RequestedHttpVersion::Http11;
                        }
                        entry_options.allow_reuse = false;
                    }
                    OptionKind::Http3(value) => {
                        let value = eval_boolean_option(value, variables)?;
                        if value {
                            entry_options.http_version = RequestedHttpVersion::Http3;
                        } else {
                            entry_options.http_version = RequestedHttpVersion::Http2;
                        }
                        entry_options.allow_reuse = false;
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
                    OptionKind::LimitRate(value) => {
                        let value = eval_natural_option(value, variables)?;
                        entry_options.max_send_speed = Some(BytesPerSec(value));
                        entry_options.max_recv_speed = Some(BytesPerSec(value));
                    }
                    OptionKind::MaxRedirect(value) => {
                        let value = eval_count_option(value, variables)?;
                        entry_options.max_redirect = value;
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
                        let value = eval_count_option(value, variables)?;
                        entry_options.repeat = Some(value);
                    }
                    OptionKind::Resolve(value) => {
                        let value = eval_template(value, variables)?;
                        entry_options.resolves.push(value);
                    }
                    OptionKind::Retry(value) => {
                        let value = eval_count_option(value, variables)?;
                        entry_options.retry = Some(value);
                    }
                    OptionKind::RetryInterval(value) => {
                        let value =
                            eval_duration_option(value, variables, DurationUnit::MilliSecond)?;
                        entry_options.retry_interval = value;
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
                    OptionKind::Variable(VariableDefinition {
                        source_info,
                        name,
                        value,
                        ..
                    }) => {
                        let value = eval_variable_value(value, variables)?;
                        if let Err(err) = variables.insert(name.clone(), value) {
                            return Err(err.to_runner_error(*source_info));
                        }
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
    let name = option.kind.identifier();
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
    variables: &VariableSet,
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
    variables: &VariableSet,
) -> Result<bool, RunnerError> {
    match boolean_value {
        BooleanOption::Literal(value) => Ok(*value),
        BooleanOption::Placeholder(Placeholder { expr, .. }) => {
            match expr::eval(expr, variables)? {
                Value::Bool(value) => Ok(value),
                v => {
                    let kind = RunnerErrorKind::ExpressionInvalidType {
                        value: v.repr(),
                        expecting: "boolean".to_string(),
                    };
                    Err(RunnerError::new(expr.source_info, kind, false))
                }
            }
        }
    }
}

fn eval_natural_option(
    natural_value: &NaturalOption,
    variables: &VariableSet,
) -> Result<u64, RunnerError> {
    match natural_value {
        NaturalOption::Literal(value) => Ok(value.as_u64()),
        NaturalOption::Placeholder(Placeholder { expr, .. }) => {
            match expr::eval(expr, variables)? {
                Value::Number(Number::Integer(value)) => {
                    if value > 0 {
                        Ok(value as u64)
                    } else {
                        let kind = RunnerErrorKind::ExpressionInvalidType {
                            value: format!("integer <{value}>"),
                            expecting: "integer > 0".to_string(),
                        };
                        Err(RunnerError::new(expr.source_info, kind, false))
                    }
                }
                v => {
                    let kind = RunnerErrorKind::ExpressionInvalidType {
                        value: v.repr(),
                        expecting: "integer".to_string(),
                    };
                    Err(RunnerError::new(expr.source_info, kind, false))
                }
            }
        }
    }
}

fn eval_count_option(
    count_value: &CountOption,
    variables: &VariableSet,
) -> Result<Count, RunnerError> {
    match count_value {
        CountOption::Literal(repeat) => Ok(*repeat),
        CountOption::Placeholder(Placeholder { expr, .. }) => match expr::eval(expr, variables)? {
            Value::Number(Number::Integer(value)) => {
                if value == -1 {
                    Ok(Count::Infinite)
                } else if value >= 0 {
                    Ok(Count::Finite(value as usize))
                } else {
                    let kind = RunnerErrorKind::ExpressionInvalidType {
                        value: format!("integer <{value}>"),
                        expecting: "integer >= -1".to_string(),
                    };
                    Err(RunnerError::new(expr.source_info, kind, false))
                }
            }
            v => {
                let kind = RunnerErrorKind::ExpressionInvalidType {
                    value: v.repr(),
                    expecting: "integer".to_string(),
                };
                Err(RunnerError::new(expr.source_info, kind, false))
            }
        },
    }
}

/// return duration value in milliseconds
fn eval_duration_option(
    duration_value: &DurationOption,
    variables: &VariableSet,
    default_unit: DurationUnit,
) -> Result<std::time::Duration, RunnerError> {
    let millis = match duration_value {
        DurationOption::Literal(literal) => {
            let unit = literal.unit.unwrap_or(default_unit);

            match unit {
                DurationUnit::MilliSecond => literal.value.as_u64(),
                DurationUnit::Second => literal.value.as_u64() * 1000,
                DurationUnit::Minute => literal.value.as_u64() * 1000 * 60,
            }
        }
        DurationOption::Placeholder(Placeholder { expr, .. }) => match expr::eval(expr, variables)?
        {
            Value::Number(Number::Integer(value)) => {
                if value < 0 {
                    let kind = RunnerErrorKind::ExpressionInvalidType {
                        value: format!("integer <{value}>"),
                        expecting: "positive integer".to_string(),
                    };
                    return Err(RunnerError::new(expr.source_info, kind, false));
                } else {
                    match default_unit {
                        DurationUnit::MilliSecond => value as u64,
                        DurationUnit::Second => (value * 1000) as u64,
                        DurationUnit::Minute => (value * 1000 * 60) as u64,
                    }
                }
            }
            v => {
                let kind = RunnerErrorKind::ExpressionInvalidType {
                    value: v.repr(),
                    expecting: "positive integer".to_string(),
                };
                return Err(RunnerError::new(expr.source_info, kind, false));
            }
        },
    };
    Ok(std::time::Duration::from_millis(millis))
}

fn eval_variable_value(
    variable_value: &VariableValue,
    variables: &mut VariableSet,
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
        AstNumber::Float(value) => Value::Number(Number::Float(value.as_f64())),
        AstNumber::Integer(value) => Value::Number(Number::Integer(value.as_i64())),
        AstNumber::BigInteger(value) => Value::Number(Number::BigInteger(value.clone())),
    }
}

#[cfg(test)]
mod tests {
    use hurl_core::ast::{Expr, ExprKind, Placeholder, SourceInfo, Variable, Whitespace, U64};
    use hurl_core::reader::Pos;
    use hurl_core::typing::{Duration, DurationUnit, ToSource};

    use super::*;
    use crate::runner::RunnerErrorKind;

    fn verbose_option_template() -> BooleanOption {
        // {{verbose}}
        BooleanOption::Placeholder(Placeholder {
            space0: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            expr: Expr {
                kind: ExprKind::Variable(Variable {
                    name: "verbose".to_string(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                }),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            space1: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
        })
    }

    fn retry_option_template() -> DurationOption {
        // {{retry}}
        DurationOption::Placeholder(Placeholder {
            space0: Whitespace {
                value: String::new(),
                source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
            },
            expr: Expr {
                kind: ExprKind::Variable(Variable {
                    name: "retry".to_string(),
                    source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
                }),
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
        let mut variables = VariableSet::default();
        assert!(eval_boolean_option(&BooleanOption::Literal(true), &variables).unwrap());

        variables
            .insert("verbose".to_string(), Value::Bool(true))
            .unwrap();
        assert!(eval_boolean_option(&verbose_option_template(), &variables).unwrap());
    }

    #[test]
    fn test_eval_boolean_option_error() {
        let mut variables = VariableSet::default();
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

        variables
            .insert("verbose".to_string(), Value::Number(Number::Integer(10)))
            .unwrap();
        let error = eval_boolean_option(&verbose_option_template(), &variables)
            .err()
            .unwrap();
        assert_eq!(
            error.kind,
            RunnerErrorKind::ExpressionInvalidType {
                value: "integer <10>".to_string(),
                expecting: "boolean".to_string()
            }
        );
    }

    #[test]
    fn test_eval_natural_option() {
        let mut variables = VariableSet::default();
        assert_eq!(
            eval_duration_option(
                &DurationOption::Literal(Duration::new(
                    U64::new(1, "1".to_source()),
                    Some(DurationUnit::Second)
                )),
                &variables,
                DurationUnit::MilliSecond
            )
            .unwrap(),
            std::time::Duration::from_millis(1000)
        );

        variables
            .insert("retry".to_string(), Value::Number(Number::Integer(10)))
            .unwrap();
        assert_eq!(
            eval_duration_option(
                &retry_option_template(),
                &variables,
                DurationUnit::MilliSecond
            )
            .unwrap(),
            std::time::Duration::from_millis(10)
        );
    }
}
