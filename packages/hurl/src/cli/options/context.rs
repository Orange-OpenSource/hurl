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
use std::path::{Path, PathBuf};

use super::CliOptions;
use crate::cli::options::env_vars::EnvVars;
use hurl::pretty::PrettyMode;

/// Represents the context in which is executed Hurl: the env variables, whether standard
/// input is a terminal or not (when pipe or redirected to a file for instance), whether standard
/// error is a terminal or not, whether Hurl is executed in a CI/CD environment, whether users has
/// disallowed ANSI code color etc...
pub struct RunContext {
    /// Is standard input a terminal or not?
    stdin_term: bool,
    /// Is standard output a terminal or not?
    stdout_term: bool,
    /// Is standard error a terminal or not?
    stderr_term: bool,
    /// Path to the config file if any.
    config_file: Option<PathBuf>,
}

impl RunContext {
    /// Creates a new context. The environment is captured and will be seen as non-mutable for the
    /// execution with this context.
    pub fn new(env_vars: &EnvVars, stdin_term: bool, stdout_term: bool, stderr_term: bool) -> Self {
        let config_file = get_config_file(env_vars);
        RunContext {
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
}

/// Get config file path if any
/// In order of precedence
/// 1. from `XDG_CONFIG_HOME/hurl/config` if `XDG_CONFIG_HOME` is set
/// 2. from `$HOME/.config/hurl/config` if $HOME is set
fn get_config_file(env_vars: &EnvVars) -> Option<PathBuf> {
    get_config_dir(env_vars).map(|config_dir| config_dir.join("hurl").join("config"))
}

fn get_config_dir(env_vars: &EnvVars) -> Option<PathBuf> {
    if let Some(config_dir) = env_vars.xdg_config_home() {
        Some(Path::new(config_dir).to_path_buf())
    } else {
        env_vars
            .home()
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
