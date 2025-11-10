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
use std::collections::HashMap;

/// Represents the context in which is executed Hurl: the env variables, whether standard
/// input is a terminal or not (when pipe or redirected to a file for instance), whether standard
/// error is a terminal or not, whether Hurl is executed in a CI/CD environment, whether users has
/// disallowed ANSI code color etc...
pub struct RunContext {
    /// Are we allowed to ise ANSI escaoe codes or not.
    with_color: bool,
    /// All the environment variables.
    env_vars: HashMap<String, String>,
    /// Whether we're running in a Continuous Integration environment or not.
    ci: bool,
    /// Is standard input a terminal or not?
    stdin_term: bool,
    /// Is standard output a terminal or not?
    stdout_term: bool,
    /// Is standard error a terminal or not?
    stderr_term: bool,
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
        // Code borrowed from <https://github.com/rust-lang/cargo/blob/master/crates/cargo-util/src/lib.rs>
        let ci = env_vars.contains_key("CI") || env_vars.contains_key("TF_BUILD");

        // According to the NO_COLOR spec, any presence of the variable should disable color, but to
        // maintain backward compatibility with code < 7.1.0, we check that the NO_COLOR env is at
        // least not empty.
        let with_color = if let Some(v) = env_vars.get("NO_COLOR") {
            if !v.is_empty() {
                false
            } else {
                stdout_term
            }
        } else {
            stdout_term
        };

        RunContext {
            with_color,
            env_vars,
            ci,
            stdin_term,
            stdout_term,
            stderr_term,
        }
    }

    /// Returns `true` if ANSI escape codes are authorized, `false` otherwise.
    pub fn is_with_color(&self) -> bool {
        self.with_color
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

    /// Returns `true` if the context is run from a CI context (like GitHub Actions, GitLab CI/CD etc...)
    /// `false` otherwise.
    pub fn is_ci(&self) -> bool {
        self.ci
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
}

#[cfg(test)]
mod tests {
    use crate::cli::options::context::RunContext;
    use std::collections::HashMap;

    #[test]
    fn context_is_colored() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        let env_vars = HashMap::from([("A".to_string(), "B".to_string())]);

        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);
        assert!(ctx.is_with_color());
    }

    #[test]
    fn context_respect_no_color() {
        let stdin_term = true;
        let stdout_term = true;
        let stderr_term = true;

        let env_vars = HashMap::from([("NO_COLOR".to_string(), "1".to_string())]);

        let ctx = RunContext::new(env_vars, stdin_term, stdout_term, stderr_term);
        assert!(!ctx.is_with_color());
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
