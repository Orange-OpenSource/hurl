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
use std::num::ParseIntError;
use std::str::FromStr;

use super::variables::TypeKind;
use super::{
    context::HURL_CONNECT_TIMEOUT, context::HURL_ERROR_FORMAT, context::HURL_HEADER,
    context::HURL_MAX_TIME, context::HURL_VERBOSITY, duration, secret, variables, CliOptions,
    CliOptionsError, ErrorFormat, HttpVersion, IpResolve, RunContext, Verbosity,
};
use crate::cli::options::context::{
    HURL_DELAY, HURL_JOBS, HURL_LIMIT_RATE, HURL_MAX_FILESIZE, HURL_MAX_REDIRS,
};
use hurl::runner::Value;
use hurl_core::types::{BytesPerSec, Count, DurationUnit};

/// Parses Hurl configuration defined in environment variables.
pub fn parse_env_vars(
    context: &RunContext,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
    let mut options = default_options;
    if let Some(color) = context.use_color_env_var() {
        options.color_stdout = color;
        options.color_stderr = color;
    }
    if let Some(timeout) = context.connect_timeout_env_var() {
        options.connect_timeout = duration::duration_from_str(timeout, DurationUnit::Second)
            .map_err(|e| with_env_var(e, HURL_CONNECT_TIMEOUT))?;
    }
    if let Some(continue_on_error) = context.continue_on_error_env_var() {
        options.continue_on_error = continue_on_error;
    }
    if let Some(delay) = context.delay_env_var() {
        options.delay = duration::duration_from_str(delay, DurationUnit::MilliSecond)
            .map_err(|e| with_env_var(e, HURL_DELAY))?;
    }
    if let Some(error_format) = context.error_format_env_var() {
        let error_format =
            ErrorFormat::from_str(error_format).map_err(|e| with_env_var(e, HURL_ERROR_FORMAT))?;
        options.error_format = error_format;
    }
    if let Some(header) = context.header_env_var() {
        let headers = header.split("|").map(|h| h.to_string()).collect::<Vec<_>>();
        for h in &headers {
            if !h.contains(':') {
                let msg = "Invalid header <{h}> missing `:`".to_string();
                return Err(with_env_var(CliOptionsError::Error(msg), HURL_HEADER));
            }
        }
        options.headers.extend(headers);
    }
    if let Some(http3) = context.http3_env_var() {
        if http3 {
            options.http_version = Some(HttpVersion::V3);
        } else {
            options.http_version = Some(HttpVersion::V2);
        }
    }
    if let Some(http2) = context.http2_env_var() {
        if http2 {
            options.http_version = Some(HttpVersion::V2);
        } else {
            options.http_version = Some(HttpVersion::V11);
        }
    }
    if let Some(http11) = context.http11_env_var() {
        if http11 {
            options.http_version = Some(HttpVersion::V11);
        } else {
            options.http_version = Some(HttpVersion::V10);
        }
    }
    if let Some(true) = context.http10_env_var() {
        options.http_version = Some(HttpVersion::V10);
    }
    if let Some(ipv4) = context.ipv4_env_var() {
        if ipv4 {
            options.ip_resolve = Some(IpResolve::IpV4);
        } else {
            options.ip_resolve = Some(IpResolve::IpV6);
        }
    }
    if let Some(no_assert) = context.no_assert_env_var() {
        options.no_assert = no_assert;
    }
    if let Some(follow_location) = context.follow_location_env_var() {
        options.follow_location = follow_location;
    }
    if let Some(follow_location_trusted) = context.follow_location_trusted_env_var() {
        options.follow_location_trusted = follow_location_trusted;
        if follow_location_trusted {
            options.follow_location = true;
        }
    }
    if let Some(insecure) = context.insecure_env_var() {
        options.insecure = insecure;
    }
    if let Some(ipv6) = context.ipv6_env_var() {
        if ipv6 {
            options.ip_resolve = Some(IpResolve::IpV6);
        } else {
            options.ip_resolve = Some(IpResolve::IpV4);
        }
    }
    if let Some(jobs) = context.jobs_env_var() {
        let jobs = jobs
            .parse::<u32>()
            .map_err(|e| from_parse_err(e, HURL_JOBS))?;
        options.jobs = Some(jobs as usize);
    }
    if let Some(limit_rate) = context.limit_rate_env_var() {
        let limit_rate = limit_rate
            .parse::<u64>()
            .map_err(|e| from_parse_err(e, HURL_LIMIT_RATE))?;
        options.limit_rate = Some(BytesPerSec(limit_rate));
    }
    if let Some(max_filesize) = context.max_filesize_env_var() {
        let max_filesize = max_filesize
            .parse::<u64>()
            .map_err(|e| from_parse_err(e, HURL_MAX_FILESIZE))?;
        options.max_filesize = Some(max_filesize);
    }
    if let Some(max_redirs) = context.max_redirs_env_var() {
        let max_redirs = max_redirs
            .parse::<i32>()
            .map_err(|e| from_parse_err(e, HURL_MAX_REDIRS))?;
        options.max_redirect = match max_redirs {
            -1 => Count::Infinite,
            n => Count::Finite(n as usize),
        };
    }
    if let Some(user_agent) = context.user_agent_env_var() {
        options.user_agent = Some(user_agent.to_string());
    }
    options.variables = parse_variables(context, options.variables)?;
    options.secrets = parse_secrets(context, options.secrets)?;
    if let Some(true) = context.verbose_env_var() {
        options.verbosity = Some(Verbosity::Verbose);
    } else if let Some(true) = context.very_verbose_env_var() {
        options.verbosity = Some(Verbosity::Debug);
    } else if let Some(verbosity) = context.verbosity_env_var() {
        let verbosity =
            Verbosity::from_str(verbosity).map_err(|e| with_env_var(e, HURL_VERBOSITY))?;
        options.verbosity = Some(verbosity);
    }
    if let Some(timeout) = context.max_time_env_var() {
        options.timeout = duration::duration_from_str(timeout, DurationUnit::Second)
            .map_err(|e| with_env_var(e, HURL_MAX_TIME))?;
    }
    Ok(options)
}

/// Parses Hurl variables configured in environment variables, given a set of existing variables
/// `default_variables`.
///
/// Variables can be set with `HURL_VARIABLE_foo` and `HURL_foo` (legacy syntax).
fn parse_variables(
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
fn parse_secrets(
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

fn from_parse_err(error: ParseIntError, env: &'static str) -> CliOptionsError {
    let message = format!("{} ({env} environment variable)", error);
    CliOptionsError::Error(message)
}

fn with_env_var(error: CliOptionsError, env: &'static str) -> CliOptionsError {
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
