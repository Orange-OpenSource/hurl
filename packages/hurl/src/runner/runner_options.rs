/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
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
use crate::cli;
use crate::cli::CliOptions;
use crate::http::ContextDir;
use crate::runner::Verbosity;
use hurl_core::ast::Entry;
use std::path::Path;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerOptions {
    pub cacert_file: Option<String>,
    pub compressed: bool,
    pub connect_timeout: Duration,
    pub context_dir: ContextDir,
    pub cookie_input_file: Option<String>,
    pub fail_fast: bool,
    pub follow_location: bool,
    pub ignore_asserts: bool,
    pub insecure: bool,
    pub max_redirect: Option<usize>,
    pub no_proxy: Option<String>,
    pub post_entry: Option<fn() -> bool>,
    pub pre_entry: Option<fn(Entry) -> bool>,
    pub proxy: Option<String>,
    pub retry: bool,
    pub retry_interval: Duration,
    pub retry_max_count: Option<usize>,
    pub timeout: Duration,
    pub to_entry: Option<usize>,
    pub user: Option<String>,
    pub user_agent: Option<String>,
    pub verbosity: Option<Verbosity>,
    pub very_verbose: bool, // If true, log body response in verbose mode.
}

impl Default for RunnerOptions {
    fn default() -> Self {
        RunnerOptions {
            cacert_file: None,
            compressed: false,
            connect_timeout: Duration::from_secs(300),
            context_dir: Default::default(),
            cookie_input_file: None,
            fail_fast: false,
            follow_location: false,
            ignore_asserts: false,
            insecure: false,
            max_redirect: Some(50),
            no_proxy: None,
            post_entry: None,
            pre_entry: None,
            proxy: None,
            retry: false,
            retry_interval: Duration::from_millis(1000),
            retry_max_count: Some(10),
            timeout: Duration::from_secs(300),
            to_entry: None,
            user: None,
            user_agent: None,
            verbosity: None,
            very_verbose: false,
        }
    }
}

impl RunnerOptions {
    pub fn from(filename: &str, current_dir: &Path, cli_options: &CliOptions) -> Self {
        let cacert_file = cli_options.cacert_file.clone();
        let follow_location = cli_options.follow_location;
        let verbosity = match (cli_options.verbose, cli_options.very_verbose) {
            (true, true) => Some(Verbosity::VeryVerbose),
            (true, _) => Some(Verbosity::Verbose),
            _ => None,
        };
        let insecure = cli_options.insecure;
        let max_redirect = cli_options.max_redirect;
        let proxy = cli_options.proxy.clone();
        let no_proxy = cli_options.no_proxy.clone();
        let cookie_input_file = cli_options.cookie_input_file.clone();
        let timeout = cli_options.timeout;
        let connect_timeout = cli_options.connect_timeout;
        let user = cli_options.user.clone();
        let user_agent = cli_options.user_agent.clone();
        let compressed = cli_options.compressed;
        let file_root = match cli_options.file_root {
            Some(ref filename) => Path::new(filename),
            None => {
                if filename == "-" {
                    current_dir
                } else {
                    let path = Path::new(filename);
                    path.parent().unwrap()
                }
            }
        };
        let context_dir = ContextDir::new(current_dir, file_root);
        let pre_entry = if cli_options.interactive {
            Some(cli::interactive::pre_entry as fn(Entry) -> bool)
        } else {
            None
        };
        let post_entry = if cli_options.interactive {
            Some(cli::interactive::post_entry as fn() -> bool)
        } else {
            None
        };
        let fail_fast = cli_options.fail_fast;
        let to_entry = cli_options.to_entry;
        let retry = cli_options.retry;
        let retry_interval = cli_options.retry_interval;
        let retry_max_count = cli_options.retry_max_count;
        let ignore_asserts = cli_options.ignore_asserts;
        let very_verbose = cli_options.very_verbose;
        RunnerOptions {
            cacert_file,
            compressed,
            connect_timeout,
            context_dir,
            cookie_input_file,
            fail_fast,
            follow_location,
            ignore_asserts,
            insecure,
            max_redirect,
            no_proxy,
            post_entry,
            pre_entry,
            proxy,
            retry,
            retry_interval,
            retry_max_count,
            timeout,
            to_entry,
            user,
            user_agent,
            verbosity,
            very_verbose,
        }
    }
}
