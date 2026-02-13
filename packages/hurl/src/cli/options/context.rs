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

use super::CliOptions;

/// Represents the context in which is executed Hurl: the env variables, whether standard
/// input is a terminal or not (when pipe or redirected to a file for instance), whether standard
/// error is a terminal or not, whether Hurl is executed in a CI/CD environment, whether users has
/// disallowed ANSI code color etc...
pub struct RunContext {
    /// All the environment variables.
    env_vars: HashMap<String, String>,
    /// Is standard input a terminal or not?
    stdin_term: bool,
    /// Is standard output a terminal or not?
    stdout_term: bool,
    /// Is standard error a terminal or not?
    stderr_term: bool,
    /// Path to the config file if any.
    config_file: Option<PathBuf>,
}

const LEGACY_VARIABLE_PREFIX: &str = "HURL_";
const VARIABLE_PREFIX: &str = "HURL_VARIABLE_";
const SECRET_PREFIX: &str = "HURL_SECRET_";

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

        RunContext {
            env_vars,
            stdin_term,
            stdout_term,
            stderr_term,
            config_file,
        }
    }

    /// Returns `true` if ANSI escape codes are enabled, `false` otherwise.
    pub fn use_color_env_var(&self) -> Option<bool> {
        // According to the NO_COLOR spec, any presence of the variable should disable color, but to
        // maintain backward compatibility with code < 7.1.0, we check that the NO_COLOR env is at
        // least not empty.
        if let Some(v) = self.env_vars.get("NO_COLOR") {
            if !v.is_empty() {
                Some(false)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns `true` if the context is run from a CI context (like GitHub Actions, GitLab CI/CD etc...)
    /// `false` otherwise.
    pub fn is_ci_env_var(&self) -> bool {
        // Code borrowed from <https://github.com/rust-lang/cargo/blob/master/crates/cargo-util/src/lib.rs>
        self.env_vars.contains_key("CI") || self.env_vars.contains_key("TF_BUILD")
    }

    /// Returns the map of Hurl variables injected by environment variables.
    ///
    /// Environment variables are prefixed with `HURL_VARIABLE_` and returned values have their name
    /// stripped of this prefix.
    pub fn var_env_vars(&self) -> HashMap<&str, &str> {
        self.env_vars
            .iter()
            .filter_map(|(name, value)| {
                name.strip_prefix(VARIABLE_PREFIX)
                    .filter(|n| !n.is_empty())
                    .map(|stripped| (stripped, value.as_str()))
            })
            .collect()
    }

    /// Returns the map of legacy Hurl variables injected by environment variables.
    ///
    /// Environment variables are prefixed with `HURL_` and returned values have their name
    /// stripped of this prefix.
    pub fn legacy_var_env_vars(&self) -> HashMap<&str, &str> {
        self.env_vars
            .iter()
            .filter_map(|(name, value)| {
                name.strip_prefix(LEGACY_VARIABLE_PREFIX)
                    // Not a new variable
                    .filter(|_| !name.starts_with(VARIABLE_PREFIX))
                    // Not a secret
                    .filter(|_| !name.starts_with(SECRET_PREFIX))
                    .filter(|n| !n.is_empty())
                    .map(|stripped| (stripped, value.as_str()))
            })
            .collect()
    }

    /// Returns the map of Hurl secrets injected by environment variables.
    ///
    /// Environment variables are prefixed with `HURL_SECRET_` and returned values have their name
    /// stripped of this prefix.
    pub fn secret_env_vars(&self) -> HashMap<&str, &str> {
        self.env_vars
            .iter()
            .filter_map(|(name, value)| {
                name.strip_prefix(SECRET_PREFIX)
                    .filter(|n| !n.is_empty())
                    .map(|stripped| (stripped, value.as_str()))
            })
            .collect()
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

    /// Returns the config file path if any.
    pub fn config_file_path(&self) -> Option<&Path> {
        self.config_file.as_deref()
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
    options.color = context.is_stdout_term();
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
        assert!(ctx.use_color_env_var().is_none());
    }

    #[test]
    fn context_has_env_var_color() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        let env_vars = HashMap::from([("NO_COLOR".to_string(), "1".to_string())]);

        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);
        assert_eq!(ctx.use_color_env_var(), Some(false));
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
        assert!(ctx.legacy_var_env_vars().is_empty());
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
        ]);

        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);

        assert_eq!(ctx.var_env_vars().len(), 3);
        assert_eq!(ctx.var_env_vars()["foo"], "true");
        assert_eq!(ctx.var_env_vars()["id"], "1234");
        assert_eq!(ctx.var_env_vars()["FOO"], "def");
        assert_eq!(ctx.legacy_var_env_vars().len(), 1);
        assert_eq!(ctx.legacy_var_env_vars()["VARIABLE"], "1234");
        assert!(ctx.secret_env_vars().is_empty());
    }

    #[test]
    fn legacy_variables_from_env() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        let env_vars = HashMap::from([
            ("FOO".to_string(), "xxx".to_string()),
            ("BAR".to_string(), "yyy".to_string()),
            ("BAZ".to_string(), "yyy".to_string()),
            ("HURL_VARIABLE_bar".to_string(), "def".to_string()),
            ("HURL_foo".to_string(), "true".to_string()),
            ("HURL_id".to_string(), "1234".to_string()),
            ("BAZ".to_string(), "yyy".to_string()),
            ("HURL_".to_string(), "1234".to_string()),
            ("HURL_".to_string(), "abcd".to_string()),
            ("HURL_FOO".to_string(), "def".to_string()),
        ]);

        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);

        assert_eq!(ctx.var_env_vars().len(), 1);
        assert_eq!(ctx.var_env_vars()["bar"], "def");
        assert_eq!(ctx.legacy_var_env_vars().len(), 3);
        assert_eq!(ctx.legacy_var_env_vars()["foo"], "true");
        assert_eq!(ctx.legacy_var_env_vars()["id"], "1234");
        assert_eq!(ctx.legacy_var_env_vars()["FOO"], "def");
        assert!(ctx.secret_env_vars().is_empty());
    }

    #[test]
    fn legacy_secrets_from_env() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        let env_vars = HashMap::from([
            ("FOO".to_string(), "xxx".to_string()),
            ("HURL_SECRET".to_string(), "48".to_string()),
            ("HURL_SECRET_".to_string(), "48".to_string()),
            ("HURL_SECRET_abcd".to_string(), "1234".to_string()),
            ("HURL_SECRET_ABCD".to_string(), "5678".to_string()),
            ("BAR".to_string(), "bar".to_string()),
        ]);

        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);

        assert!(ctx.var_env_vars().is_empty());
        assert_eq!(ctx.legacy_var_env_vars().len(), 1);
        assert_eq!(ctx.legacy_var_env_vars()["SECRET"], "48");
        assert_eq!(ctx.secret_env_vars().len(), 2);
        assert_eq!(ctx.secret_env_vars()["abcd"], "1234");
        assert_eq!(ctx.secret_env_vars()["ABCD"], "5678");
    }
}
