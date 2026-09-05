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

use super::variables::TypeKind;
use super::{
    BoolOpt, CliOptions, CliOptionsError, ErrorFormat, HttpVersion, IpResolve, OutputType,
    Verbosity, duration, secret, variables,
};

/// Contains all env vars at the start of the execution of the program.
pub struct EnvVars {
    /// All the environment variables.
    all_env_vars: HashMap<String, String>,

    /// The environment variables that have `HURL_` prefix (and that could be used by Hurl)
    hurl_env_vars: HashMap<String, String>,
}

/// All the supported env vars.
const HURL_PREFIX: &str = "HURL_";

const HURL_COLOR: &str = "HURL_COLOR";
const HURL_COMPRESSED: &str = "HURL_COMPRESSED";
const HURL_FAIL_WITH_BODY: &str = "HURL_FAIL_WITH_BODY";
const HURL_CONNECT_TIMEOUT: &str = "HURL_CONNECT_TIMEOUT";
const HURL_CONTINUE_ON_ERROR: &str = "HURL_CONTINUE_ON_ERROR";
const HURL_DELAY: &str = "HURL_DELAY";
const HURL_ERROR_FORMAT: &str = "HURL_ERROR_FORMAT";
const HURL_LOCATION: &str = "HURL_LOCATION";
const HURL_LOCATION_TRUSTED: &str = "HURL_LOCATION_TRUSTED";
const HURL_INSECURE: &str = "HURL_INSECURE";
const HURL_IPV4: &str = "HURL_IPV4";
const HURL_IPV6: &str = "HURL_IPV6";
const HURL_JOBS: &str = "HURL_JOBS";
const HURL_HEADER: &str = "HURL_HEADER";
const HURL_HTTP10: &str = "HURL_HTTP10";
const HURL_HTTP11: &str = "HURL_HTTP11";
const HURL_HTTP2: &str = "HURL_HTTP2";
const HURL_HTTP2_PRIOR_KNOWLEDGE: &str = "HURL_HTTP2_PRIOR_KNOWLEDGE";
const HURL_HTTP3: &str = "HURL_HTTP3";
const HURL_LIMIT_RATE: &str = "HURL_LIMIT_RATE";
const HURL_MAX_FILESIZE: &str = "HURL_MAX_FILESIZE";
const HURL_MAX_REDIRS: &str = "HURL_MAX_REDIRS";
const HURL_MAX_TIME: &str = "HURL_MAX_TIME";
const HURL_NO_ASSERT: &str = "HURL_NO_ASSERT";
const HURL_NO_COLOR: &str = "HURL_NO_COLOR";
const HURL_NO_COOKIE_STORE: &str = "HURL_NO_COOKIE_STORE";
const HURL_NO_HEADER: &str = "HURL_NO_HEADER";
const HURL_NO_JSONPATH_COERCION: &str = "HURL_NO_JSONPATH_COERCION";
const HURL_NO_OUTPUT: &str = "HURL_NO_OUTPUT";
const HURL_NO_PRETTY: &str = "HURL_NO_PRETTY";
const HURL_PARALLEL: &str = "HURL_PARALLEL";
const HURL_PRETTY: &str = "HURL_PRETTY";
const HURL_PROGRESS_BAR: &str = "HURL_PROGRESS_BAR";
const HURL_PROXY_HEADER: &str = "HURL_PROXY_HEADER";
const HURL_RETRY: &str = "HURL_RETRY";
const HURL_RETRY_INTERVAL: &str = "HURL_RETRY_INTERVAL";
const HURL_SECRET_PREFIX: &str = "HURL_SECRET_";
const HURL_TEST: &str = "HURL_TEST";
const HURL_USER: &str = "HURL_USER";
const HURL_USER_AGENT: &str = "HURL_USER_AGENT";
const HURL_VARIABLE_PREFIX: &str = "HURL_VARIABLE_";
const HURL_VERBOSE: &str = "HURL_VERBOSE";
const HURL_VERBOSITY: &str = "HURL_VERBOSITY";
const HURL_VERY_VERBOSE: &str = "HURL_VERY_VERBOSE";
const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";
const HOME: &str = "HOME";

impl EnvVars {
    /// Captures all the environment vars, they will be seen as non-mutable for the execution.
    pub fn new(env_vars: HashMap<String, String>) -> Self {
        let hurl_env_vars = env_vars
            .iter()
            .filter(|(k, _v)| k.starts_with(HURL_PREFIX))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<HashMap<_, _>>();

        EnvVars {
            all_env_vars: env_vars,
            hurl_env_vars,
        }
    }

    pub fn home(&self) -> Option<&str> {
        self.all_env_vars.get(HOME).map(|v| v.as_str())
    }

    pub fn xdg_config_home(&self) -> Option<&str> {
        self.all_env_vars.get(XDG_CONFIG_HOME).map(|v| v.as_str())
    }

    /// Returns the env var for compressed response.
    pub fn compressed(&self) -> Option<bool> {
        self.get_bool(HURL_COMPRESSED)
    }

    /// Returns the env var for fail with body.
    pub fn fail_with_body(&self) -> Option<bool> {
        self.get_bool(HURL_FAIL_WITH_BODY)
    }

    /// Returns the env var for connect timeout duration.
    pub fn connect_timeout(&self) -> Option<&str> {
        self.hurl_env_vars
            .get(HURL_CONNECT_TIMEOUT)
            .map(|v| v.as_str())
    }

    /// Returns the env var for continue on error.
    pub fn continue_on_error(&self) -> Option<bool> {
        self.get_bool(HURL_CONTINUE_ON_ERROR)
    }

    /// Returns the env var for delay duration.
    pub fn delay(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_DELAY).map(|v| v.as_str())
    }

    /// Returns the env var for error format.
    pub fn error_format(&self) -> Option<&str> {
        self.hurl_env_vars
            .get(HURL_ERROR_FORMAT)
            .map(|v| v.as_str())
    }

    /// Returns the Hurl headers injected by environment variables.
    pub fn headers(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_HEADER).map(|v| v.as_str())
    }

    /// Returns the env var for using HTTP/1.0.
    pub fn http10(&self) -> Option<bool> {
        self.get_bool(HURL_HTTP10)
    }

    /// Returns the env var for using HTTP/1.1.
    pub fn http11(&self) -> Option<bool> {
        self.get_bool(HURL_HTTP11)
    }

    /// Returns the env var for using HTTP/2.
    pub fn http2(&self) -> Option<bool> {
        self.get_bool(HURL_HTTP2)
    }

    /// Returns the env var for using HTTP/2 with prior knowledge.
    pub fn http2_prior_knowledge(&self) -> Option<bool> {
        self.get_bool(HURL_HTTP2_PRIOR_KNOWLEDGE)
    }

    /// Returns the env var for using HTTP/3.
    pub fn http3(&self) -> Option<bool> {
        self.get_bool(HURL_HTTP3)
    }

    /// Returns the env var for following redirects.
    pub fn follow_location(&self) -> Option<bool> {
        self.get_bool(HURL_LOCATION)
    }

    /// Returns the env var for following redirects with trusted location.
    pub fn follow_location_trusted(&self) -> Option<bool> {
        self.get_bool(HURL_LOCATION_TRUSTED)
    }

    /// Returns the env var for allowing insecure transfers.
    pub fn insecure(&self) -> Option<bool> {
        self.get_bool(HURL_INSECURE)
    }

    /// Returns the env var for IPv4 resolution.
    pub fn ipv4(&self) -> Option<bool> {
        self.get_bool(HURL_IPV4)
    }

    /// Returns the env var for IPv6 resolution.
    pub fn ipv6(&self) -> Option<bool> {
        self.get_bool(HURL_IPV6)
    }

    /// Returns `true` if the context is run from a CI context (like GitHub Actions, GitLab CI/CD etc...)
    /// `false` otherwise.
    pub fn is_ci(&self) -> bool {
        // Code borrowed from <https://github.com/rust-lang/cargo/blob/master/crates/cargo-util/src/lib.rs>
        self.all_env_vars.contains_key("CI") || self.all_env_vars.contains_key("TF_BUILD")
    }

    /// Returns the env var for maximum number of parallel jobs.
    pub fn jobs(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_JOBS).map(|v| v.as_str())
    }

    /// Returns the env var for transfer rate limit rating.
    pub fn limit_rate(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_LIMIT_RATE).map(|v| v.as_str())
    }

    /// Returns the env var for maximum file size to download.
    pub fn max_filesize(&self) -> Option<&str> {
        self.hurl_env_vars
            .get(HURL_MAX_FILESIZE)
            .map(|v| v.as_str())
    }

    /// Returns the env var for maximum number of redirects.
    pub fn max_redirs(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_MAX_REDIRS).map(|v| v.as_str())
    }

    /// Returns the env var for max time duration.
    pub fn max_time(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_MAX_TIME).map(|v| v.as_str())
    }

    /// Returns the env var for parallel mode.
    pub fn parallel(&self) -> Option<bool> {
        self.get_bool(HURL_PARALLEL)
    }

    /// Returns the env var for max time duration.
    pub fn progress_bar(&self) -> Option<bool> {
        self.get_bool(HURL_PROGRESS_BAR)
    }

    /// Returns the env var for the user authentication.
    pub fn user(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_USER).map(|v| v.as_str())
    }

    /// Returns the env var for the User-Agent string.
    pub fn user_agent(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_USER_AGENT).map(|v| v.as_str())
    }

    /// Returns the env var for ignoring asserts.
    pub fn no_assert(&self) -> Option<bool> {
        self.get_bool(HURL_NO_ASSERT)
    }

    /// Returns the env var for disabling cookie store.
    pub fn no_cookie_store(&self) -> Option<bool> {
        self.get_bool(HURL_NO_COOKIE_STORE)
    }

    /// Returns the env var for headers to remove from requests.
    pub fn no_header(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_NO_HEADER).map(|v| v.as_str())
    }

    /// Returns the env var for disabling JSONPath coercion.
    pub fn no_jsonpath_coercion(&self) -> Option<bool> {
        self.get_bool(HURL_NO_JSONPATH_COERCION)
    }

    /// Returns the env var for suppressing output.
    pub fn no_output(&self) -> Option<bool> {
        self.get_bool(HURL_NO_OUTPUT)
    }

    /// Returns the env var for disabling pretty output.
    pub fn no_pretty(&self) -> Option<bool> {
        self.get_bool(HURL_NO_PRETTY)
    }

    /// Returns the env var for enabling pretty output.
    pub fn pretty(&self) -> Option<bool> {
        self.get_bool(HURL_PRETTY)
    }

    /// Returns the Hurl proxy headers injected by environment variables.
    pub fn proxy_headers(&self) -> Option<&str> {
        self.hurl_env_vars
            .get(HURL_PROXY_HEADER)
            .map(|v| v.as_str())
    }

    /// Returns the env var for retry count.
    pub fn retry(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_RETRY).map(|v| v.as_str())
    }

    /// Returns the env var for retry interval duration.
    pub fn retry_interval(&self) -> Option<&str> {
        self.hurl_env_vars
            .get(HURL_RETRY_INTERVAL)
            .map(|v| v.as_str())
    }

    /// Returns the map of Hurl secrets injected by environment variables.
    ///
    /// Environment variables are prefixed with `HURL_SECRET_` and returned values have their name
    /// stripped of this prefix.
    pub fn secrets(&self) -> HashMap<&str, &str> {
        self.hurl_env_vars
            .iter()
            .filter_map(|(name, value)| {
                name.strip_prefix(HURL_SECRET_PREFIX)
                    .filter(|n| !n.is_empty())
                    .map(|stripped| (stripped, value.as_str()))
            })
            .collect()
    }

    /// Returns `Some(true)` if color is set through env, `Some(false)` if color is disable through env,
    /// `None` otherwise.
    pub fn color(&self) -> Option<bool> {
        self.get_bool(HURL_COLOR)
    }

    /// Returns `Some(true)` if no color is set through env, `Some(false)` if no color is disable through env,
    /// `None` otherwise.
    pub fn no_color(&self) -> Option<bool> {
        if let Some(v) = self.all_env_vars.get("NO_COLOR") {
            // According to the NO_COLOR spec, any presence of the variable should disable color, but to
            // maintain backward compatibility with code < 7.1.0, we check that the NO_COLOR env is at
            // least not empty.
            if !v.is_empty() { Some(true) } else { None }
        } else {
            self.get_bool(HURL_NO_COLOR)
        }
    }

    /// Returns `Some(true)` if test mode is set through env, `Some(false)` if test mode is disable through env,
    /// `None` otherwise.
    pub fn test(&self) -> Option<bool> {
        self.get_bool(HURL_TEST)
    }

    /// Returns the map of Hurl variables injected by environment variables.
    ///
    /// Environment variables are prefixed with `HURL_VARIABLE_` and returned values have their name
    /// stripped of this prefix.
    pub fn variables(&self) -> HashMap<&str, &str> {
        self.hurl_env_vars
            .iter()
            .filter_map(|(name, value)| {
                name.strip_prefix(HURL_VARIABLE_PREFIX)
                    .filter(|n| !n.is_empty())
                    .map(|stripped| (stripped, value.as_str()))
            })
            .collect()
    }

    pub fn verbose(&self) -> Option<bool> {
        self.get_bool(HURL_VERBOSE)
    }

    pub fn verbosity(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_VERBOSITY).map(|v| v.as_str())
    }

    pub fn very_verbose(&self) -> Option<bool> {
        self.get_bool(HURL_VERY_VERBOSE)
    }

    fn get_bool(&self, name: &'static str) -> Option<bool> {
        self.hurl_env_vars
            .get(name)
            .map(|s| s.as_str())
            .map(|v| v.to_ascii_lowercase())
            .and_then(|v| match v.as_str() {
                "1" | "true" => Some(true),
                "0" | "false" => Some(false),
                _ => None,
            })
    }
}

fn compressed(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.compressed().unwrap_or(default_value)
}

fn fail_with_body(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.fail_with_body().unwrap_or(default_value)
}

fn color(env_vars: &EnvVars, default_value: bool) -> bool {
    if let Some(no_color) = env_vars.no_color() {
        return !no_color;
    }
    if let Some(color) = env_vars.color() {
        return color;
    }
    default_value
}

fn connect_timeout(
    env_vars: &EnvVars,
    default_value: Duration,
) -> Result<Duration, CliOptionsError> {
    match env_vars.connect_timeout() {
        Some(timeout) => duration::duration_from_str(timeout, DurationUnit::Second)
            .map_err(|e| err_from_cli_err(e, HURL_CONNECT_TIMEOUT)),
        None => Ok(default_value),
    }
}

fn continue_on_error(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.continue_on_error().unwrap_or(default_value)
}

fn delay(env_vars: &EnvVars, default_value: Duration) -> Result<Duration, CliOptionsError> {
    match env_vars.delay() {
        Some(delay) => duration::duration_from_str(delay, DurationUnit::MilliSecond)
            .map_err(|e| err_from_cli_err(e, HURL_DELAY)),
        None => Ok(default_value),
    }
}

fn error_format(
    env_vars: &EnvVars,
    default_value: ErrorFormat,
) -> Result<ErrorFormat, CliOptionsError> {
    match env_vars.error_format() {
        Some(error_format) => {
            ErrorFormat::from_str(error_format).map_err(|e| err_from_cli_err(e, HURL_ERROR_FORMAT))
        }
        None => Ok(default_value),
    }
}

fn follow_location(env_vars: &EnvVars, default_value: bool) -> Result<bool, CliOptionsError> {
    let value = match (
        env_vars.follow_location(),
        env_vars.follow_location_trusted(),
    ) {
        (Some(true), _) => true,
        (Some(false), Some(true)) => {
            let error = format!(
                "Invalid environment variables configuration {} {}",
                HURL_LOCATION, HURL_LOCATION_TRUSTED
            );
            return Err(CliOptionsError::Error(error));
        }
        (Some(false), _) => false,
        (None, Some(true)) => true,
        (None, _) => default_value,
    };
    Ok(value)
}

fn follow_location_trusted(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.follow_location_trusted().unwrap_or(default_value)
}

fn headers(env_vars: &EnvVars, default_value: Vec<String>) -> Result<Vec<String>, CliOptionsError> {
    let mut all_headers = default_value;
    if let Some(headers) = env_vars.headers() {
        let headers = headers
            .split("|")
            .map(|h| h.to_string())
            .collect::<Vec<_>>();
        for h in &headers {
            if !h.contains(':') {
                let msg = format!("Invalid header <{h}>, missing `:`");
                return Err(err_from_cli_err(CliOptionsError::Error(msg), HURL_HEADER));
            }
        }
        all_headers.extend(headers);
    }
    Ok(all_headers)
}

fn http_version(env_vars: &EnvVars, default_value: Option<HttpVersion>) -> Option<HttpVersion> {
    if let Some(http3) = env_vars.http3() {
        if http3 {
            Some(HttpVersion::V3)
        } else {
            Some(HttpVersion::V2)
        }
    } else if let Some(http2_prior_knowledge) = env_vars.http2_prior_knowledge() {
        if http2_prior_knowledge {
            Some(HttpVersion::V2PriorKnowledge)
        } else {
            Some(HttpVersion::V11)
        }
    } else if let Some(http2) = env_vars.http2() {
        if http2 {
            Some(HttpVersion::V2)
        } else {
            Some(HttpVersion::V11)
        }
    } else if let Some(http11) = env_vars.http11() {
        if http11 {
            Some(HttpVersion::V11)
        } else {
            Some(HttpVersion::V10)
        }
    } else if let Some(true) = env_vars.http10() {
        Some(HttpVersion::V10)
    } else {
        default_value
    }
}

fn insecure(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.insecure().unwrap_or(default_value)
}

fn ip_resolve(env_vars: &EnvVars, default_value: Option<IpResolve>) -> Option<IpResolve> {
    if let Some(ipv6) = env_vars.ipv6() {
        if ipv6 {
            Some(IpResolve::IpV6)
        } else {
            Some(IpResolve::IpV4)
        }
    } else if let Some(ipv4) = env_vars.ipv4() {
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
    env_vars: &EnvVars,
    default_value: Option<usize>,
) -> Result<Option<usize>, CliOptionsError> {
    match env_vars.jobs() {
        Some(jobs) => jobs
            .parse::<usize>()
            .map(Some)
            .map_err(|e| err_from(e, HURL_JOBS)),
        None => Ok(default_value),
    }
}

fn limit_rate(
    env_vars: &EnvVars,
    default_value: Option<BytesPerSec>,
) -> Result<Option<BytesPerSec>, CliOptionsError> {
    match env_vars.limit_rate() {
        Some(limit_rate) => limit_rate
            .parse::<u64>()
            .map(BytesPerSec)
            .map(Some)
            .map_err(|e| err_from(e, HURL_LIMIT_RATE)),
        None => Ok(default_value),
    }
}

fn max_filesize(
    env_vars: &EnvVars,
    default_value: Option<u64>,
) -> Result<Option<u64>, CliOptionsError> {
    match env_vars.max_filesize() {
        Some(max_filesize) => max_filesize
            .parse::<u64>()
            .map(Some)
            .map_err(|e| err_from(e, HURL_MAX_FILESIZE)),
        None => Ok(default_value),
    }
}

fn max_redirect(env_vars: &EnvVars, default_value: Count) -> Result<Count, CliOptionsError> {
    match env_vars.max_redirs() {
        Some(max_redirs) => max_redirs
            .parse::<i32>()
            .map_err(|e| err_from(e, HURL_MAX_REDIRS))
            .and_then(|n| Count::try_from(n).map_err(|e| err_from(&e, HURL_MAX_REDIRS))),
        None => Ok(default_value),
    }
}

fn no_assert(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.no_assert().unwrap_or(default_value)
}

fn no_cookie_store(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.no_cookie_store().unwrap_or(default_value)
}

fn no_headers(
    env_vars: &EnvVars,
    default_value: Vec<String>,
) -> Result<Vec<String>, CliOptionsError> {
    let mut all_no_headers = default_value;
    if let Some(no_header) = env_vars.no_header() {
        let no_headers = no_header
            .split("|")
            .map(|h| h.trim().to_string())
            .collect::<Vec<_>>();
        for h in &no_headers {
            if h.is_empty() {
                let msg = "Missing header name".to_string();
                return Err(err_from_cli_err(
                    CliOptionsError::Error(msg),
                    HURL_NO_HEADER,
                ));
            }
        }
        all_no_headers.extend(no_headers);
    }
    Ok(all_no_headers)
}

fn proxy_headers(
    env_vars: &EnvVars,
    default_value: Vec<String>,
) -> Result<Vec<String>, CliOptionsError> {
    let mut all_proxy_headers = default_value;
    if let Some(proxy_headers) = env_vars.proxy_headers() {
        let proxy_headers = proxy_headers
            .split("|")
            .map(|h| h.to_string())
            .collect::<Vec<_>>();
        for h in &proxy_headers {
            if !h.contains(':') {
                let msg = format!("Invalid proxy header <{h}>, missing `:`");
                return Err(err_from_cli_err(
                    CliOptionsError::Error(msg),
                    HURL_PROXY_HEADER,
                ));
            }
        }
        all_proxy_headers.extend(proxy_headers);
    }
    Ok(all_proxy_headers)
}

fn no_jsonpath_coercion(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.no_jsonpath_coercion().unwrap_or(default_value)
}

fn output_type(env_vars: &EnvVars, default_value: OutputType) -> OutputType {
    if let Some(true) = env_vars.no_output() {
        OutputType::NoOutput
    } else if let Some(true) = env_vars.test() {
        OutputType::NoOutput
    } else {
        default_value
    }
}

fn parallel(env_vars: &EnvVars, default_value: BoolOpt) -> BoolOpt {
    if let Some(true) = env_vars.parallel() {
        BoolOpt::Set(true)
    } else {
        default_value
    }
}

fn pretty(env_vars: &EnvVars, default_value: PrettyMode) -> PrettyMode {
    if let Some(true) = env_vars.pretty() {
        return PrettyMode::Force;
    }
    if let Some(true) = env_vars.no_pretty() {
        return PrettyMode::None;
    }
    default_value
}

fn progress_bar(env_vars: &EnvVars, default_value: BoolOpt) -> BoolOpt {
    if let Some(true) = env_vars.progress_bar() {
        BoolOpt::Set(true)
    } else {
        default_value
    }
}

fn retry(
    env_vars: &EnvVars,
    default_value: Option<Count>,
) -> Result<Option<Count>, CliOptionsError> {
    match env_vars.retry() {
        Some(retry) => retry
            .parse::<i32>()
            .map_err(|e| err_from(e, HURL_RETRY))
            .and_then(|n| Count::try_from(n).map_err(|e| err_from(&e, HURL_RETRY)))
            .map(Some),
        None => Ok(default_value),
    }
}

fn retry_interval(
    env_vars: &EnvVars,
    default_value: Duration,
) -> Result<Duration, CliOptionsError> {
    match env_vars.retry_interval() {
        Some(retry_interval) => {
            duration::duration_from_str(retry_interval, DurationUnit::MilliSecond)
                .map_err(|e| err_from_cli_err(e, HURL_RETRY_INTERVAL))
        }
        None => Ok(default_value),
    }
}

fn test(env_vars: &EnvVars, default_value: bool) -> bool {
    env_vars.test().unwrap_or(default_value)
}

fn timeout(env_vars: &EnvVars, default_value: Duration) -> Result<Duration, CliOptionsError> {
    match env_vars.max_time() {
        Some(max_time) => duration::duration_from_str(max_time, DurationUnit::Second)
            .map_err(|e| err_from_cli_err(e, HURL_MAX_TIME)),
        None => Ok(default_value),
    }
}

fn user(env_vars: &EnvVars, default_value: Option<String>) -> Option<String> {
    env_vars.user().map(|s| s.to_string()).or(default_value)
}

fn user_agent(env_vars: &EnvVars, default_value: Option<String>) -> Option<String> {
    env_vars
        .user_agent()
        .map(|s| s.to_string())
        .or(default_value)
}

fn verbosity(
    env_vars: &EnvVars,
    default_value: Option<Verbosity>,
) -> Result<Option<Verbosity>, CliOptionsError> {
    let verbosity = if let Some(true) = env_vars.verbose() {
        Some(Verbosity::Verbose)
    } else if let Some(true) = env_vars.very_verbose() {
        Some(Verbosity::Debug)
    } else if let Some(verbosity) = env_vars.verbosity() {
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
    env_vars: &EnvVars,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
    let color_stdout = color(env_vars, default_options.color_stdout);
    let color_stderr = color(env_vars, default_options.color_stderr);
    let compressed = compressed(env_vars, default_options.compressed);
    let connect_timeout = connect_timeout(env_vars, default_options.connect_timeout)?;
    let fail_with_body = fail_with_body(env_vars, default_options.fail_with_body);
    let continue_on_error = continue_on_error(env_vars, default_options.continue_on_error);
    let delay = delay(env_vars, default_options.delay)?;
    let error_format = error_format(env_vars, default_options.error_format)?;
    let headers = headers(env_vars, default_options.headers)?;
    let http_version = http_version(env_vars, default_options.http_version);
    let ip_resolve = ip_resolve(env_vars, default_options.ip_resolve);
    let no_assert = no_assert(env_vars, default_options.no_assert);
    let no_cookie_store = no_cookie_store(env_vars, default_options.no_cookie_store);
    let no_headers = no_headers(env_vars, default_options.no_headers)?;
    let no_jsonpath_coercion = no_jsonpath_coercion(env_vars, default_options.no_jsonpath_coercion);
    let output_type = output_type(env_vars, default_options.output_type);
    let follow_location = follow_location(env_vars, default_options.follow_location)?;
    let follow_location_trusted =
        follow_location_trusted(env_vars, default_options.follow_location_trusted);
    let insecure = insecure(env_vars, default_options.insecure);
    let jobs = jobs(env_vars, default_options.jobs)?;
    let limit_rate = limit_rate(env_vars, default_options.limit_rate)?;
    let max_filesize = max_filesize(env_vars, default_options.max_filesize)?;
    let max_redirect = max_redirect(env_vars, default_options.max_redirect)?;
    let parallel = parallel(env_vars, default_options.parallel);
    let pretty = pretty(env_vars, default_options.pretty);
    let progress_bar = progress_bar(env_vars, default_options.progress_bar);
    let proxy_headers = proxy_headers(env_vars, default_options.proxy_headers)?;
    let retry = retry(env_vars, default_options.retry)?;
    let retry_interval = retry_interval(env_vars, default_options.retry_interval)?;
    let secrets = secrets(env_vars, default_options.secrets)?;
    let timeout = timeout(env_vars, default_options.timeout)?;
    let user = user(env_vars, default_options.user);
    let user_agent = user_agent(env_vars, default_options.user_agent);
    let variables = variables(env_vars, default_options.variables)?;
    let verbosity = verbosity(env_vars, default_options.verbosity)?;
    let test = test(env_vars, default_options.test);

    Ok(CliOptions {
        color_stdout,
        color_stderr,
        compressed,
        connect_timeout,
        continue_on_error,
        delay,
        error_format,
        fail_with_body,
        follow_location_trusted,
        follow_location,
        headers,
        http_version,
        insecure,
        ip_resolve,
        jobs,
        limit_rate,
        max_filesize,
        max_redirect,
        no_assert,
        no_cookie_store,
        no_headers,
        no_jsonpath_coercion,
        output_type,
        parallel,
        pretty,
        progress_bar,
        proxy_headers,
        retry,
        retry_interval,
        secrets,
        test,
        timeout,
        user,
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
    env_vars: &EnvVars,
    default_variables: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, CliOptionsError> {
    let mut variables = default_variables;

    // Variables are typed, based on their values.
    let type_kind = TypeKind::Inferred;

    // Insert environment variables `HURL_VARIABLE_foo`
    for (env_name, env_value) in env_vars.variables() {
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
    env_vars: &EnvVars,
    default_secrets: HashMap<String, String>,
) -> Result<HashMap<String, String>, CliOptionsError> {
    let mut secrets = default_secrets;

    // Secrets are always parsed as string.
    let type_kind = TypeKind::String;

    // Insert environment secrets `HURL_SECRET_foo`
    for (env_name, env_value) in env_vars.secrets() {
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
    use super::{CliOptions, EnvVars, parse_env_vars};
    use hurl::runner::{Number, Value};
    use std::collections::HashMap;

    #[test]
    fn env_vars_has_no_env_var_color() {
        let env_vars = HashMap::from([("A".to_string(), "B".to_string())]);
        let env_vars = EnvVars::new(env_vars);
        assert!(env_vars.color().is_none());
    }

    #[test]
    fn env_vars_has_color() {
        let data = [
            ("HURL_COLOR", "0", Some(false)),
            ("HURL_COLOR", "1", Some(true)),
            ("HURL_COLOR", "true", Some(true)),
            ("HURL_COLOR", "TRUE", Some(true)),
            ("HURL_COLOR", "false", Some(false)),
            ("HURL_COLOR", "FALSE", Some(false)),
        ];

        for (name, value, expected) in data {
            let env_vars = HashMap::from([(name.to_string(), value.to_string())]);
            let env_vars = EnvVars::new(env_vars);
            assert_eq!(
                env_vars.color(),
                expected,
                "test env var {}={}",
                name,
                value
            );
        }
    }

    #[test]
    fn env_vars_has_no_color() {
        let data = [
            ("NO_COLOR", "0", Some(true)),
            ("NO_COLOR", "1", Some(true)),
            ("NO_COLOR", "true", Some(true)),
            ("NO_COLOR", "TRUE", Some(true)),
            ("NO_COLOR", "false", Some(true)),
            ("NO_COLOR", "FALSE", Some(true)),
            ("HURL_NO_COLOR", "0", Some(false)),
            ("HURL_NO_COLOR", "1", Some(true)),
            ("HURL_NO_COLOR", "true", Some(true)),
            ("HURL_NO_COLOR", "TRUE", Some(true)),
            ("HURL_NO_COLOR", "false", Some(false)),
            ("HURL_NO_COLOR", "FALSE", Some(false)),
        ];

        for (name, value, expected) in data {
            let env_vars = HashMap::from([(name.to_string(), value.to_string())]);
            let env_vars = EnvVars::new(env_vars);
            assert_eq!(
                env_vars.no_color(),
                expected,
                "test env var {}={}",
                name,
                value
            );
        }
    }

    #[test]
    fn empty_variables_secrets_from_env() {
        let env_vars = HashMap::from([
            ("FOO".to_string(), "xxx".to_string()),
            ("BAR".to_string(), "yyy".to_string()),
            ("BAZ".to_string(), "yyy".to_string()),
        ]);

        let env_vars = EnvVars::new(env_vars);

        assert!(env_vars.variables().is_empty());
        assert!(env_vars.secrets().is_empty());
    }

    #[test]
    fn variables_from_env() {
        let env_vars = HashMap::from([
            ("FOO".to_string(), "xxx".to_string()),
            ("BAR".to_string(), "yyy".to_string()),
            ("BAZ".to_string(), "yyy".to_string()),
            ("HURL_VARIABLE_foo".to_string(), "true".to_string()),
            ("HURL_VARIABLE_id".to_string(), "1234".to_string()),
            ("HURL_VARIABLE".to_string(), "1234".to_string()),
            ("HURL_VARIABLE_".to_string(), "abcd".to_string()),
            ("HURL_VARIABLE_FOO".to_string(), "def".to_string()),
            ("HURL_COLOR".to_string(), "1".to_string()),
            ("HURL_NO_COLOR".to_string(), "1".to_string()),
        ]);

        let env_vars = EnvVars::new(env_vars);

        assert_eq!(env_vars.variables().len(), 3);
        assert_eq!(env_vars.variables()["foo"], "true");
        assert_eq!(env_vars.variables()["id"], "1234");
        assert_eq!(env_vars.variables()["FOO"], "def");
        assert!(env_vars.secrets().is_empty());
    }

    #[test]
    fn test_options_variables_override_by_env_vars() {
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
        let env_vars = EnvVars::new(env_vars_override);

        let updated_options = parse_env_vars(&env_vars, options).unwrap();
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
        let env_vars = EnvVars::new(env_vars_override);

        let updated_options = parse_env_vars(&env_vars, options).unwrap();
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
