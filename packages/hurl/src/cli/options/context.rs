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
use std::path::{Path, PathBuf};

use hurl::pretty::PrettyMode;

use super::CliOptions;

/// Represents the context in which is executed Hurl: the env variables, whether standard
/// input is a terminal or not (when pipe or redirected to a file for instance), whether standard
/// error is a terminal or not, whether Hurl is executed in a CI/CD environment, whether users has
/// disallowed ANSI code color etc...
pub struct RunContext {
    /// All the environment variables.
    env_vars: HashMap<String, String>,

    /// The environment variables that have `HURL_` prefix (and that could be used by Hurl)
    hurl_env_vars: HashMap<String, String>,

    /// Is standard input a terminal or not?
    stdin_term: bool,
    /// Is standard output a terminal or not?
    stdout_term: bool,
    /// Is standard error a terminal or not?
    stderr_term: bool,
    /// Path to the config file if any.
    config_file: Option<PathBuf>,
}

/// All the supported Hurl env vars.
const HURL_PREFIX: &str = "HURL_";

pub const HURL_COLOR: &str = "HURL_COLOR";
pub const HURL_COMPRESSED: &str = "HURL_COMPRESSED";
pub const HURL_CONNECT_TIMEOUT: &str = "HURL_CONNECT_TIMEOUT";
pub const HURL_CONTINUE_ON_ERROR: &str = "HURL_CONTINUE_ON_ERROR";
pub const HURL_DELAY: &str = "HURL_DELAY";
pub const HURL_ERROR_FORMAT: &str = "HURL_ERROR_FORMAT";
pub const HURL_FOLLOW_LOCATION: &str = "HURL_LOCATION";
pub const HURL_FOLLOW_LOCATION_TRUSTED: &str = "HURL_LOCATION_TRUSTED";
pub const HURL_INSECURE: &str = "HURL_INSECURE";
pub const HURL_IPV4: &str = "HURL_IPV4";
pub const HURL_IPV6: &str = "HURL_IPV6";
pub const HURL_JOBS: &str = "HURL_JOBS";
pub const HURL_HEADER: &str = "HURL_HEADER";
pub const HURL_HTTP10: &str = "HURL_HTTP10";
pub const HURL_HTTP11: &str = "HURL_HTTP11";
pub const HURL_HTTP2: &str = "HURL_HTTP2";
pub const HURL_HTTP3: &str = "HURL_HTTP3";
pub const HURL_LIMIT_RATE: &str = "HURL_LIMIT_RATE";
pub const HURL_MAX_FILESIZE: &str = "HURL_MAX_FILESIZE";
pub const HURL_MAX_REDIRS: &str = "HURL_MAX_REDIRS";
pub const HURL_MAX_TIME: &str = "HURL_MAX_TIME";
pub const HURL_NO_ASSERT: &str = "HURL_NO_ASSERT";
pub const HURL_NO_COLOR: &str = "HURL_NO_COLOR";
pub const HURL_NO_OUTPUT: &str = "HURL_NO_OUTPUT";
pub const HURL_SECRET_PREFIX: &str = "HURL_SECRET_";
pub const HURL_TEST: &str = "HURL_TEST";
pub const HURL_USER_AGENT: &str = "HURL_USER_AGENT";
pub const HURL_VARIABLE_PREFIX: &str = "HURL_VARIABLE_";
pub const HURL_VERBOSE: &str = "HURL_VERBOSE";
pub const HURL_VERBOSITY: &str = "HURL_VERBOSITY";
pub const HURL_VERY_VERBOSE: &str = "HURL_VERY_VERBOSE";

impl RunContext {
    /// Creates a new context. The environment is captured and will be seen as non-mutable for the
    /// execution with this context.
    pub fn new(
        env_vars: HashMap<String, String>,
        stdin_term: bool,
        stdout_term: bool,
        stderr_term: bool,
    ) -> Self {
        let config_file = get_config_file(&env_vars);
        let hurl_env_vars = env_vars
            .iter()
            .filter(|(k, _v)| k.starts_with(HURL_PREFIX))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<HashMap<_, _>>();

        RunContext {
            env_vars,
            hurl_env_vars,
            stdin_term,
            stdout_term,
            stderr_term,
            config_file,
        }
    }

    /// Returns the config file path if any.
    pub fn config_file_path(&self) -> Option<&Path> {
        self.config_file.as_deref()
    }

    /// Checks if standard input is a terminal.
    pub fn is_stdin_term(&self) -> bool {
        self.stdin_term
    }

    /// Checks if standard output is a terminal.
    pub fn is_stdout_term(&self) -> bool {
        self.stdout_term
    }

    /// Checks if standard error is a terminal.
    pub fn is_stderr_term(&self) -> bool {
        self.stderr_term
    }

    /// Returns the env var for compressed response.
    pub fn compressed_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_COMPRESSED)
    }

    /// Returns the env var for connect timeout duration.
    pub fn connect_timeout_env_var(&self) -> Option<&str> {
        self.hurl_env_vars
            .get(HURL_CONNECT_TIMEOUT)
            .map(|v| v.as_str())
    }

    /// Retunrs the env var for continue on error.
    pub fn continue_on_error_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_CONTINUE_ON_ERROR)
    }

    /// Returns the env var for delay duration.
    pub fn delay_env_var(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_DELAY).map(|v| v.as_str())
    }

    /// Returns the env var for error format.
    pub fn error_format_env_var(&self) -> Option<&str> {
        self.hurl_env_vars
            .get(HURL_ERROR_FORMAT)
            .map(|v| v.as_str())
    }

    /// Returns the map of Hurl headers injected by environment variables.
    ///
    /// Environment variables are prefixed with `HURL_VARIABLE_` and returned values have their name
    /// stripped of this prefix.
    pub fn header_env_var(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_HEADER).map(|v| v.as_str())
    }

    /// Returns the env var for using HTTP/1.0.
    pub fn http10_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_HTTP10)
    }

    /// Returns the env var for using HTTP/1.1.
    pub fn http11_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_HTTP11)
    }

    /// Returns the env var for using HTTP/2.
    pub fn http2_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_HTTP2)
    }

    /// Returns the env var for using HTTP/3.
    pub fn http3_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_HTTP3)
    }

    /// Returns the env var for following redirects.
    pub fn follow_location_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_FOLLOW_LOCATION)
    }

    /// Returns the env var for following redirects with trusted location.
    pub fn follow_location_trusted_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_FOLLOW_LOCATION_TRUSTED)
    }

    /// Returns the env var for allowing insecure transfers.
    pub fn insecure_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_INSECURE)
    }
    /// Returns the env var for IPv4 resolution.
    pub fn ipv4_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_IPV4)
    }

    /// Returns the env var for IPv6 resolution.
    pub fn ipv6_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_IPV6)
    }

    /// Returns `true` if the context is run from a CI context (like GitHub Actions, GitLab CI/CD etc...)
    /// `false` otherwise.
    pub fn is_ci_env_var(&self) -> bool {
        // Code borrowed from <https://github.com/rust-lang/cargo/blob/master/crates/cargo-util/src/lib.rs>
        self.env_vars.contains_key("CI") || self.env_vars.contains_key("TF_BUILD")
    }

    /// Returns the env var for maximum number of parallel jobs.
    pub fn jobs_env_var(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_JOBS).map(|v| v.as_str())
    }

    /// Returns the env var for transfer rate limit rating.
    pub fn limit_rate_env_var(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_LIMIT_RATE).map(|v| v.as_str())
    }

    /// Returns the env var for maximum file size to download.
    pub fn max_filesize_env_var(&self) -> Option<&str> {
        self.hurl_env_vars
            .get(HURL_MAX_FILESIZE)
            .map(|v| v.as_str())
    }

    /// Returns the env var for maximum number of redirects.
    pub fn max_redirs_env_var(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_MAX_REDIRS).map(|v| v.as_str())
    }

    /// Returns the env var for max time duration.
    pub fn max_time_env_var(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_MAX_TIME).map(|v| v.as_str())
    }

    /// Returns the env var for the User-Agent string.
    pub fn user_agent_env_var(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_USER_AGENT).map(|v| v.as_str())
    }

    /// Returns the env var for suppressing output.
    pub fn no_output_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_NO_OUTPUT)
    }

    /// Returns the env var for ignoring asserts.
    pub fn no_assert_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_NO_ASSERT)
    }

    /// Returns the map of Hurl secrets injected by environment variables.
    ///
    /// Environment variables are prefixed with `HURL_SECRET_` and returned values have their name
    /// stripped of this prefix.
    pub fn secret_env_vars(&self) -> HashMap<&str, &str> {
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
    pub fn color_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_COLOR)
    }

    /// Returns `Some(true)` if no color is set through env, `Some(false)` if no color is disable through env,
    /// `None` otherwise.
    pub fn no_color_env_var(&self) -> Option<bool> {
        if let Some(v) = self.env_vars.get("NO_COLOR") {
            // According to the NO_COLOR spec, any presence of the variable should disable color, but to
            // maintain backward compatibility with code < 7.1.0, we check that the NO_COLOR env is at
            // least not empty.
            if !v.is_empty() {
                Some(true)
            } else {
                None
            }
        } else {
            self.get_env_var_bool(HURL_NO_COLOR)
        }
    }

    /// Returns `Some(true)` if test mode is set through env, `Some(false)` if test mode is disable through env,
    /// `None` otherwise.
    pub fn test_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_TEST)
    }

    /// Returns the map of Hurl variables injected by environment variables.
    ///
    /// Environment variables are prefixed with `HURL_VARIABLE_` and returned values have their name
    /// stripped of this prefix.
    pub fn var_env_vars(&self) -> HashMap<&str, &str> {
        self.hurl_env_vars
            .iter()
            .filter_map(|(name, value)| {
                name.strip_prefix(HURL_VARIABLE_PREFIX)
                    .filter(|n| !n.is_empty())
                    .map(|stripped| (stripped, value.as_str()))
            })
            .collect()
    }

    pub fn verbose_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_VERBOSE)
    }
    pub fn verbosity_env_var(&self) -> Option<&str> {
        self.hurl_env_vars.get(HURL_VERBOSITY).map(|v| v.as_str())
    }

    pub fn very_verbose_env_var(&self) -> Option<bool> {
        self.get_env_var_bool(HURL_VERY_VERBOSE)
    }

    fn get_env_var_bool(&self, name: &'static str) -> Option<bool> {
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

/// Get config file path if any
/// In order of precedence
/// 1. from `XDG_CONFIG_HOME/hurl/config` if `XDG_CONFIG_HOME` is set
/// 2. from `$HOME/.config/hurl/config` if $HOME is set
fn get_config_file(env_vars: &HashMap<String, String>) -> Option<PathBuf> {
    get_config_dir(env_vars).map(|config_dir| config_dir.join("hurl").join("config"))
}

fn get_config_dir(env_vars: &HashMap<String, String>) -> Option<PathBuf> {
    if let Some(config_dir) = env_vars.get("XDG_CONFIG_HOME") {
        Some(Path::new(config_dir).to_path_buf())
    } else {
        env_vars
            .get("HOME")
            .map(|home_dir| Path::new(home_dir).join("config").to_path_buf())
    }
}

/// Take a [`CliOptions`] and apply modification from runtime context.
///
/// This method configures the options with values that are inferred from the runtime context
/// such as: do we use color or not etc...
pub fn init_options(context: &RunContext, default_options: CliOptions) -> CliOptions {
    let mut options = default_options;
    options.color_stdout = context.is_stdout_term();
    options.color_stderr = context.is_stderr_term();
    options.pretty = if context.is_stdout_term() {
        PrettyMode::Automatic
    } else {
        PrettyMode::None
    };
    options
}

#[cfg(test)]
mod tests {
    use crate::cli::options::context::RunContext;
    use std::collections::HashMap;

    #[test]
    fn context_has_no_env_var_color() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        let env_vars = HashMap::from([("A".to_string(), "B".to_string())]);
        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);
        assert!(ctx.color_env_var().is_none());
    }

    #[test]
    fn context_has_color_env_var() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

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
            let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);
            assert_eq!(
                ctx.color_env_var(),
                expected,
                "test env var {}={}",
                name,
                value
            );
        }
    }

    #[test]
    fn context_has_no_color_env_var() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

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
            let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);
            assert_eq!(
                ctx.no_color_env_var(),
                expected,
                "test env var {}={}",
                name,
                value
            );
        }
    }

    #[test]
    fn empty_variables_secrets_from_env() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        let env_vars = HashMap::from([
            ("FOO".to_string(), "xxx".to_string()),
            ("BAR".to_string(), "yyy".to_string()),
            ("BAZ".to_string(), "yyy".to_string()),
        ]);

        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);

        assert!(ctx.var_env_vars().is_empty());
        assert!(ctx.secret_env_vars().is_empty());
    }

    #[test]
    fn variables_from_env() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        let env_vars = HashMap::from([
            ("FOO".to_string(), "xxx".to_string()),
            ("BAR".to_string(), "yyy".to_string()),
            ("BAZ".to_string(), "yyy".to_string()),
            ("HURL_VARIABLE_foo".to_string(), "true".to_string()),
            ("HURL_VARIABLE_id".to_string(), "1234".to_string()),
            ("BAZ".to_string(), "yyy".to_string()),
            ("HURL_VARIABLE".to_string(), "1234".to_string()),
            ("HURL_VARIABLE_".to_string(), "abcd".to_string()),
            ("HURL_VARIABLE_FOO".to_string(), "def".to_string()),
            ("HURL_COLOR".to_string(), "1".to_string()),
            ("HURL_NO_COLOR".to_string(), "1".to_string()),
        ]);

        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);

        assert_eq!(ctx.var_env_vars().len(), 3);
        assert_eq!(ctx.var_env_vars()["foo"], "true");
        assert_eq!(ctx.var_env_vars()["id"], "1234");
        assert_eq!(ctx.var_env_vars()["FOO"], "def");
        assert!(ctx.secret_env_vars().is_empty());
    }
}
