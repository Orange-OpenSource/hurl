/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
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
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

use hurl::pretty::PrettyMode;
use hurl::runner::Value;
use hurl_core::types::{BytesPerSec, Count, DurationUnit};

use super::context::{
    HURL_CONNECT_TIMEOUT, HURL_DELAY, HURL_ERROR_FORMAT, HURL_FOLLOW_LOCATION,
    HURL_FOLLOW_LOCATION_TRUSTED, HURL_HEADER, HURL_JOBS, HURL_LIMIT_RATE, HURL_MAX_FILESIZE,
    HURL_MAX_REDIRS, HURL_MAX_TIME, HURL_RETRY, HURL_RETRY_INTERVAL, HURL_VERBOSITY,
};
use super::variables::TypeKind;
use super::{
    duration, secret, variables, CliOptions, CliOptionsError, ErrorFormat, HttpVersion, IpResolve,
    OutputType, RunContext, Verbosity,
};

fn compressed(context: &RunContext, default_value: bool) -> bool {
    context.compressed_env_var().unwrap_or(default_value)
}

fn color(context: &RunContext, default_value: bool) -> bool {
    if let Some(no_color) = context.no_color_env_var() {
        return !no_color;
    }
    if let Some(color) = context.color_env_var() {
        return color;
    }
    default_value
}

fn connect_timeout(
    context: &RunContext,
    default_value: Duration,
) -> Result<Duration, CliOptionsError> {
    match context.connect_timeout_env_var() {
        Some(timeout) => duration::duration_from_str(timeout, DurationUnit::Second)
            .map_err(|e| err_from_cli_err(e, HURL_CONNECT_TIMEOUT)),
        None => Ok(default_value),
    }
}

fn continue_on_error(context: &RunContext, default_value: bool) -> bool {
    context.continue_on_error_env_var().unwrap_or(default_value)
}

fn delay(context: &RunContext, default_value: Duration) -> Result<Duration, CliOptionsError> {
    match context.delay_env_var() {
        Some(delay) => duration::duration_from_str(delay, DurationUnit::MilliSecond)
            .map_err(|e| err_from_cli_err(e, HURL_DELAY)),
        None => Ok(default_value),
    }
}

fn error_format(
    context: &RunContext,
    default_value: ErrorFormat,
) -> Result<ErrorFormat, CliOptionsError> {
    match context.error_format_env_var() {
        Some(error_format) => {
            ErrorFormat::from_str(error_format).map_err(|e| err_from_cli_err(e, HURL_ERROR_FORMAT))
        }
        None => Ok(default_value),
    }
}

fn follow_location(context: &RunContext, default_value: bool) -> Result<bool, CliOptionsError> {
    let value = match (
        context.follow_location_env_var(),
        context.follow_location_trusted_env_var(),
    ) {
        (Some(true), _) => true,
        (Some(false), Some(true)) => {
            let error = format!(
                "Invalid environment variables configuration {} {}",
                HURL_FOLLOW_LOCATION, HURL_FOLLOW_LOCATION_TRUSTED
            );
            return Err(CliOptionsError::Error(error));
        }
        (Some(false), _) => false,
        (None, Some(true)) => true,
        (None, _) => default_value,
    };
    Ok(value)
}

fn follow_location_trusted(context: &RunContext, default_value: bool) -> bool {
    context
        .follow_location_trusted_env_var()
        .unwrap_or(default_value)
}

fn headers(
    context: &RunContext,
    default_value: Vec<String>,
) -> Result<Vec<String>, CliOptionsError> {
    let mut all_headers = default_value;
    if let Some(header) = context.header_env_var() {
        let headers = header.split("|").map(|h| h.to_string()).collect::<Vec<_>>();
        for h in &headers {
            if !h.contains(':') {
                let msg = format!("Invalid header <{h}> missing `:`");
                return Err(err_from_cli_err(CliOptionsError::Error(msg), HURL_HEADER));
            }
        }
        all_headers.extend(headers);
    }
    Ok(all_headers)
}

fn http_version(context: &RunContext, default_value: Option<HttpVersion>) -> Option<HttpVersion> {
    if let Some(http3) = context.http3_env_var() {
        if http3 {
            Some(HttpVersion::V3)
        } else {
            Some(HttpVersion::V2)
        }
    } else if let Some(http2) = context.http2_env_var() {
        if http2 {
            Some(HttpVersion::V2)
        } else {
            Some(HttpVersion::V11)
        }
    } else if let Some(http11) = context.http11_env_var() {
        if http11 {
            Some(HttpVersion::V11)
        } else {
            Some(HttpVersion::V10)
        }
    } else if let Some(true) = context.http10_env_var() {
        Some(HttpVersion::V10)
    } else {
        default_value
    }
}

fn insecure(context: &RunContext, default_value: bool) -> bool {
    context.insecure_env_var().unwrap_or(default_value)
}

fn ip_resolve(context: &RunContext, default_value: Option<IpResolve>) -> Option<IpResolve> {
    if let Some(ipv6) = context.ipv6_env_var() {
        if ipv6 {
            Some(IpResolve::IpV6)
        } else {
            Some(IpResolve::IpV4)
        }
    } else if let Some(ipv4) = context.ipv4_env_var() {
        if ipv4 {
            Some(IpResolve::IpV4)
        } else {
            Some(IpResolve::IpV6)
        }
    } else {
        default_value
    }
}

fn jobs(
    context: &RunContext,
    default_value: Option<usize>,
) -> Result<Option<usize>, CliOptionsError> {
    match context.jobs_env_var() {
        Some(jobs) => jobs
            .parse::<usize>()
            .map(Some)
            .map_err(|e| err_from(e, HURL_JOBS)),
        None => Ok(default_value),
    }
}

fn limit_rate(
    context: &RunContext,
    default_value: Option<BytesPerSec>,
) -> Result<Option<BytesPerSec>, CliOptionsError> {
    match context.limit_rate_env_var() {
        Some(limit_rate) => limit_rate
            .parse::<u64>()
            .map(BytesPerSec)
            .map(Some)
            .map_err(|e| err_from(e, HURL_LIMIT_RATE)),
        None => Ok(default_value),
    }
}

fn max_filesize(
    context: &RunContext,
    default_value: Option<u64>,
) -> Result<Option<u64>, CliOptionsError> {
    match context.max_filesize_env_var() {
        Some(max_filesize) => max_filesize
            .parse::<u64>()
            .map(Some)
            .map_err(|e| err_from(e, HURL_MAX_FILESIZE)),
        None => Ok(default_value),
    }
}

fn max_redirect(context: &RunContext, default_value: Count) -> Result<Count, CliOptionsError> {
    match context.max_redirs_env_var() {
        Some(max_redirs) => max_redirs
            .parse::<i32>()
            .map_err(|e| err_from(e, HURL_MAX_REDIRS))
            .and_then(|n| Count::try_from(n).map_err(|e| err_from(&e, HURL_MAX_REDIRS))),
        None => Ok(default_value),
    }
}

fn no_assert(context: &RunContext, default_value: bool) -> bool {
    context.no_assert_env_var().unwrap_or(default_value)
}

fn no_cookie_store(context: &RunContext, default_value: bool) -> bool {
    context.no_cookie_store_env_var().unwrap_or(default_value)
}

fn output_type(context: &RunContext, default_value: OutputType) -> OutputType {
    match (context.no_output_env_var(), context.test_env_var()) {
        (Some(true), _) => OutputType::NoOutput,
        (_, Some(true)) => OutputType::NoOutput,
        _ => default_value,
    }
}

fn parallel(context: &RunContext, default_value: bool) -> bool {
    if let Some(true) = context.test_env_var() {
        true
    } else {
        default_value
    }
}

fn pretty(context: &RunContext, default_value: PrettyMode) -> PrettyMode {
    if let Some(true) = context.pretty_env_var() {
        return PrettyMode::Force;
    }
    if let Some(true) = context.no_pretty_env_var() {
        return PrettyMode::None;
    }
    default_value
}

fn progress_bar(context: &RunContext, default_value: bool) -> bool {
    // The progress bar is automatically displayed for test mode when stderr is a TTY and not running in CI.
    if let Some(true) = context.test_env_var() {
        if context.is_stderr_term() && !context.is_ci_env_var() {
            return true;
        }
    }
    default_value
}

fn retry(
    context: &RunContext,
    default_value: Option<Count>,
) -> Result<Option<Count>, CliOptionsError> {
    match context.retry_env_var() {
        Some(retry) => retry
            .parse::<i32>()
            .map_err(|e| err_from(e, HURL_RETRY))
            .and_then(|n| Count::try_from(n).map_err(|e| err_from(&e, HURL_RETRY)))
            .map(Some),
        None => Ok(default_value),
    }
}

fn retry_interval(
    context: &RunContext,
    default_value: Duration,
) -> Result<Duration, CliOptionsError> {
    match context.retry_interval_env_var() {
        Some(retry_interval) => {
            duration::duration_from_str(retry_interval, DurationUnit::MilliSecond)
                .map_err(|e| err_from_cli_err(e, HURL_RETRY_INTERVAL))
        }
        None => Ok(default_value),
    }
}

fn test(context: &RunContext, default_value: bool) -> bool {
    context.test_env_var().unwrap_or(default_value)
}

fn timeout(context: &RunContext, default_value: Duration) -> Result<Duration, CliOptionsError> {
    match context.max_time_env_var() {
        Some(max_time) => duration::duration_from_str(max_time, DurationUnit::Second)
            .map_err(|e| err_from_cli_err(e, HURL_MAX_TIME)),
        None => Ok(default_value),
    }
}

fn user_agent(context: &RunContext, default_value: Option<String>) -> Option<String> {
    context
        .user_agent_env_var()
        .map(|s| s.to_string())
        .or(default_value)
}

fn verbosity(
    context: &RunContext,
    default_value: Option<Verbosity>,
) -> Result<Option<Verbosity>, CliOptionsError> {
    let verbosity = if let Some(true) = context.verbose_env_var() {
        Some(Verbosity::Verbose)
    } else if let Some(true) = context.very_verbose_env_var() {
        Some(Verbosity::Debug)
    } else if let Some(verbosity) = context.verbosity_env_var() {
        let verbosity =
            Verbosity::from_str(verbosity).map_err(|e| err_from_cli_err(e, HURL_VERBOSITY))?;
        Some(verbosity)
    } else {
        default_value
    };
    Ok(verbosity)
}

/// Parses Hurl configuration defined in environment variables.
pub fn parse_env_vars(
    context: &RunContext,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
    let color_stdout = color(context, default_options.color_stdout);
    let color_stderr = color(context, default_options.color_stderr);
    let compressed = compressed(context, default_options.compressed);
    let connect_timeout = connect_timeout(context, default_options.connect_timeout)?;
    let continue_on_error = continue_on_error(context, default_options.continue_on_error);
    let delay = delay(context, default_options.delay)?;
    let error_format = error_format(context, default_options.error_format)?;
    let headers = headers(context, default_options.headers)?;
    let http_version = http_version(context, default_options.http_version);
    let ip_resolve = ip_resolve(context, default_options.ip_resolve);
    let no_assert = no_assert(context, default_options.no_assert);
    let no_cookie_store = no_cookie_store(context, default_options.no_cookie_store);
    let output_type = output_type(context, default_options.output_type);
    let follow_location = follow_location(context, default_options.follow_location)?;
    let follow_location_trusted =
        follow_location_trusted(context, default_options.follow_location_trusted);
    let insecure = insecure(context, default_options.insecure);
    let jobs = jobs(context, default_options.jobs)?;
    let limit_rate = limit_rate(context, default_options.limit_rate)?;
    let max_filesize = max_filesize(context, default_options.max_filesize)?;
    let max_redirect = max_redirect(context, default_options.max_redirect)?;
    let parallel = parallel(context, default_options.parallel);
    let pretty = pretty(context, default_options.pretty);
    let progress_bar = progress_bar(context, default_options.progress_bar);
    let retry = retry(context, default_options.retry)?;
    let retry_interval = retry_interval(context, default_options.retry_interval)?;
    let secrets = secrets(context, default_options.secrets)?;
    let timeout = timeout(context, default_options.timeout)?;
    let user_agent = user_agent(context, default_options.user_agent);
    let variables = variables(context, default_options.variables)?;
    let verbosity = verbosity(context, default_options.verbosity)?;
    let test = test(context, default_options.test);

    Ok(CliOptions {
        color_stdout,
        color_stderr,
        compressed,
        connect_timeout,
        continue_on_error,
        delay,
        error_format,
        headers,
        http_version,
        follow_location,
        follow_location_trusted,
        insecure,
        ip_resolve,
        jobs,
        limit_rate,
        max_filesize,
        max_redirect,
        no_assert,
        no_cookie_store,
        output_type,
        parallel,
        pretty,
        progress_bar,
        retry,
        retry_interval,
        secrets,
        test,
        timeout,
        user_agent,
        variables,
        verbosity,
        ..default_options
    })
}

/// Parses Hurl variables configured in environment variables, given a set of existing variables
/// `default_variables`.
///
/// Variables can be set with `HURL_VARIABLE_foo`.
fn variables(
    context: &RunContext,
    default_variables: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, CliOptionsError> {
    let mut variables = default_variables;

    // Variables are typed, based on their values.
    let type_kind = TypeKind::Inferred;

    // Insert environment variables `HURL_VARIABLE_foo`
    for (env_name, env_value) in context.var_env_vars() {
        let value = variables::parse_value(env_value, type_kind)?;
        variables.insert(env_name.to_string(), value);
    }

    Ok(variables)
}

/// Parses Hurl secrets configured in environment variables, given a set of existing secrets
/// `default_secrets`.
///
/// Secrets can be set with `HURL_SECRET_foo`.
fn secrets(
    context: &RunContext,
    default_secrets: HashMap<String, String>,
) -> Result<HashMap<String, String>, CliOptionsError> {
    let mut secrets = default_secrets;

    // Secrets are always parsed as string.
    let type_kind = TypeKind::String;

    // Insert environment secrets `HURL_SECRET_foo`
    for (env_name, env_value) in context.secret_env_vars() {
        let value = variables::parse_value(env_value, type_kind)?;
        secret::add_secret(&mut secrets, env_name.to_string(), value)?;
    }
    Ok(secrets)
}

fn err_from<E: fmt::Display>(error: E, env: &'static str) -> CliOptionsError {
    let message = format!("{error} ({env} environment variable)");
    CliOptionsError::Error(message)
}

fn err_from_cli_err(error: CliOptionsError, env: &'static str) -> CliOptionsError {
    match error {
        CliOptionsError::DisplayHelp(_) => error,
        CliOptionsError::DisplayVersion(_) => error,
        CliOptionsError::NoInput(_) => error,
        CliOptionsError::Error(message) => {
            let message = format!("{message} ({env} environment variable)");
            CliOptionsError::Error(message)
        }
        CliOptionsError::InvalidInputFile(_) => error,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_env_vars, CliOptions, RunContext};
    use hurl::runner::{Number, Value};
    use std::collections::HashMap;

    #[test]
    fn test_options_variables_override_by_env_vars() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        // Default configuration of Hurl run.
        let mut options = CliOptions::default();
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), Value::String("zzz".to_string()));
        variables.insert("foo".to_string(), Value::String("FOO".to_string()));
        options.variables = variables;

        // Overrides Hurl run variables with env vars.
        let env_vars_override = HashMap::from([
            ("FOO".to_string(), "xxx".to_string()),
            ("HURL_VARIABLE_foo".to_string(), "48".to_string()),
            ("HURL_VARIABLE_bar".to_string(), "BAR".to_string()),
            ("HURL_baz".to_string(), "abcd".to_string()),
            ("NOT_A_VARIABLE".to_string(), "bar".to_string()),
        ]);
        let ctx = RunContext::new(env_vars_override, stdin_term, stdout_term, stderr_term);

        let updated_options = parse_env_vars(&ctx, options).unwrap();
        assert_eq!(updated_options.variables.len(), 3);
        assert_eq!(
            updated_options.variables["foo"],
            Value::Number(Number::Integer(48))
        );
        assert_eq!(
            updated_options.variables["var1"],
            Value::String("zzz".to_string())
        );
        assert_eq!(
            updated_options.variables["bar"],
            Value::String("BAR".to_string())
        );
    }

    #[test]
    fn test_options_secrets_override_by_env_vars() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        // Default configuration of Hurl run.
        let mut options = CliOptions::default();
        let mut secrets = HashMap::new();
        secrets.insert("secret1".to_string(), "SECRET1".to_string());
        options.secrets = secrets;

        // Overrides Hurl run secrets with env vars.
        let env_vars_override = HashMap::from([
            ("QUX".to_string(), "qux".to_string()),
            ("HURL_SECRET_secret2".to_string(), "SECRET2".to_string()),
            ("HURL_VARIABLE_bar".to_string(), "BAR".to_string()),
            ("HURL_SECRET_secret3".to_string(), "SECRET3".to_string()),
        ]);
        let ctx = RunContext::new(env_vars_override, stdin_term, stdout_term, stderr_term);

        let updated_options = parse_env_vars(&ctx, options).unwrap();
        assert_eq!(updated_options.variables.len(), 1);
        assert_eq!(
            updated_options.variables["bar"],
            Value::String("BAR".to_string())
        );
        assert_eq!(updated_options.secrets.len(), 3);
        assert_eq!(updated_options.secrets["secret1"], "SECRET1".to_string(),);
        assert_eq!(updated_options.secrets["secret2"], "SECRET2".to_string(),);
        assert_eq!(updated_options.secrets["secret3"], "SECRET3".to_string(),);
    }
}
