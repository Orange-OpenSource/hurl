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

/// Represents the context in whin is executed Hurl: the env variables, whether standard
/// input is a terminal or not (when pipe or redirected to a file for instance), whether standard
/// error is a terminal or not, whether Hurl is executed in a CI/CD environment, whether users has
/// disallowed ANSI code color etc...
pub struct RunContext {
    with_color: bool,
    env_vars: Vec<(String, String)>,
    /// Whether this running in a Continuous Integration environment.
    ci: bool,
    /// Is standard input a terminal or not?
    stdin_term: bool,
    /// Is standard output a terminal or not?
    stdout_term: bool,
    /// Is standard error a terminal or not?
    stderr_term: bool,
}

impl RunContext {
    /// Creates a new context. The environment is captured and will be seen as non mutable for the
    /// execution with this context.
    pub fn new(
        with_color: bool,
        env_vars: Vec<(String, String)>,
        stdin_term: bool,
        stdout_term: bool,
        stderr_term: bool,
    ) -> Self {
        // Code borrowed from <https://github.com/rust-lang/cargo/blob/master/crates/cargo-util/src/lib.rs>
        let ci = env_vars
            .iter()
            .any(|(name, _)| name == "CI" || name == "TF_BUILD");

        RunContext {
            with_color,
            env_vars,
            ci,
            stdin_term,
            stdout_term,
            stderr_term,
        }
    }

    pub fn is_with_color(&self) -> bool {
        self.with_color
    }

    pub fn env_vars(&self) -> &[(String, String)] {
        &self.env_vars
    }

    pub fn is_ci(&self) -> bool {
        self.ci
    }

    pub fn is_stdin_term(&self) -> bool {
        self.stdin_term
    }

    pub fn is_stdout_term(&self) -> bool {
        self.stdout_term
    }

    pub fn is_stderr_term(&self) -> bool {
        self.stderr_term
    }
}
