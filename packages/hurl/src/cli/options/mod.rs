/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
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
mod commands;
mod matches;
mod variables;

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use atty::Stream;
use clap::ArgMatches;
use hurl::libcurl_version_info;
use hurl::util::logger::{LoggerOptions, LoggerOptionsBuilder, Verbosity};
use hurl::util::path::ContextDir;
use hurl_core::ast::{Entry, Retry};

use crate::cli;
use crate::runner::{RunnerOptions, RunnerOptionsBuilder, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    pub cacert_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
    pub color: bool,
    pub compressed: bool,
    pub connect_timeout: Duration,
    pub connects_to: Vec<String>,
    pub cookie_input_file: Option<String>,
    pub cookie_output_file: Option<String>,
    pub error_format: ErrorFormat,
    pub fail_fast: bool,
    pub file_root: Option<String>,
    pub follow_location: bool,
    pub html_dir: Option<PathBuf>,
    pub ignore_asserts: bool,
    pub include: bool,
    pub input_files: Vec<String>,
    pub insecure: bool,
    pub interactive: bool,
    pub junit_file: Option<String>,
    pub max_redirect: Option<usize>,
    pub no_proxy: Option<String>,
    pub output: Option<String>,
    pub output_type: OutputType,
    pub path_as_is: bool,
    pub progress_bar: bool,
    pub proxy: Option<String>,
    pub resolves: Vec<String>,
    pub retry: Retry,
    pub retry_interval: Duration,
    pub ssl_no_revoke: bool,
    pub test: bool,
    pub timeout: Duration,
    pub to_entry: Option<usize>,
    pub user: Option<String>,
    pub user_agent: Option<String>,
    pub variables: HashMap<String, Value>,
    pub verbose: bool,
    pub very_verbose: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionsError {
    Info(String),
    Error(String),
}

impl From<clap::Error> for OptionsError {
    fn from(error: clap::Error) -> Self {
        match error.kind() {
            clap::error::ErrorKind::DisplayVersion => OptionsError::Info(error.to_string()),
            clap::error::ErrorKind::DisplayHelp => OptionsError::Info(error.to_string()),
            _ => OptionsError::Error(error.to_string()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorFormat {
    Short,
    Long,
}

impl From<ErrorFormat> for hurl::util::logger::ErrorFormat {
    fn from(value: ErrorFormat) -> Self {
        match value {
            ErrorFormat::Short => hurl::util::logger::ErrorFormat::Short,
            ErrorFormat::Long => hurl::util::logger::ErrorFormat::Long,
        }
    }
}

fn get_version() -> String {
    let libcurl_version = libcurl_version_info();
    format!(
        "{} {}\nFeatures (libcurl):  {}\nFeatures (built-in): brotli",
        clap::crate_version!(),
        libcurl_version.libraries.join(" "),
        libcurl_version.features.join(" ")
    )
}

pub fn parse() -> Result<Options, OptionsError> {
    let mut command = clap::Command::new("hurl")
        .version(get_version())
        .disable_colored_help(true)
        .about("Run Hurl file(s) or standard input")
        .arg(commands::cacert_file())
        .arg(commands::client_cert_file())
        .arg(commands::client_key_file())
        .arg(commands::color())
        .arg(commands::compressed())
        .arg(commands::connect_timeout())
        .arg(commands::connect_to())
        .arg(commands::cookies_input_file())
        .arg(commands::cookies_output_file())
        .arg(commands::error_format())
        .arg(commands::fail_at_end())
        .arg(commands::file_root())
        .arg(commands::follow_location())
        .arg(commands::glob())
        .arg(commands::ignore_asserts())
        .arg(commands::include())
        .arg(commands::input_files())
        .arg(commands::insecure())
        .arg(commands::interactive())
        .arg(commands::json())
        .arg(commands::max_redirects())
        .arg(commands::max_time())
        .arg(commands::no_color())
        .arg(commands::no_output())
        .arg(commands::noproxy())
        .arg(commands::output())
        .arg(commands::path_as_is())
        .arg(commands::proxy())
        .arg(commands::report_html())
        .arg(commands::report_junit())
        .arg(commands::resolve())
        .arg(commands::retry())
        .arg(commands::retry_interval())
        .arg(commands::ssl_no_revoke())
        .arg(commands::test())
        .arg(commands::to_entry())
        .arg(commands::user_agent())
        .arg(commands::user())
        .arg(commands::variable())
        .arg(commands::variables_file())
        .arg(commands::verbose())
        .arg(commands::very_verbose());

    let arg_matches = command.try_get_matches_from_mut(env::args_os())?;
    let opts = parse_matches(&arg_matches)?;

    // If we've no file input (either from the standard input or from the command line arguments),
    // we just print help and exit.
    if opts.input_files.is_empty() && atty::is(Stream::Stdin) {
        let help = command.render_help().to_string();
        return Err(OptionsError::Error(help));
    }

    if opts.cookie_output_file.is_some() && opts.input_files.len() > 1 {
        return Err(OptionsError::Error(
            "Only save cookies for a unique session".to_string(),
        ));
    }
    Ok(opts)
}

fn parse_matches(arg_matches: &ArgMatches) -> Result<Options, OptionsError> {
    let cacert_file = matches::cacert_file(arg_matches)?;
    let client_cert_file = matches::client_cert_file(arg_matches)?;
    let client_key_file = matches::client_key_file(arg_matches)?;
    let color = matches::color(arg_matches);
    let compressed = matches::compressed(arg_matches);
    let connect_timeout = matches::connect_timeout(arg_matches);
    let connects_to = matches::connects_to(arg_matches);
    let cookie_input_file = matches::cookie_input_file(arg_matches);
    let cookie_output_file = matches::cookie_output_file(arg_matches);
    let error_format = matches::error_format(arg_matches);
    let fail_fast = matches::fail_fast(arg_matches);
    let file_root = matches::file_root(arg_matches);
    let follow_location = matches::follow_location(arg_matches);
    let html_dir = matches::html_dir(arg_matches)?;
    let ignore_asserts = matches::ignore_asserts(arg_matches);
    let include = matches::include(arg_matches);
    let input_files = matches::input_files(arg_matches)?;
    let insecure = matches::insecure(arg_matches);
    let interactive = matches::interactive(arg_matches);
    let junit_file = matches::junit_file(arg_matches);
    let max_redirect = matches::max_redirect(arg_matches);
    let no_proxy = matches::no_proxy(arg_matches);
    let progress_bar = matches::progress_bar(arg_matches);
    let path_as_is = matches::path_as_is(arg_matches);
    let proxy = matches::proxy(arg_matches);
    let output = matches::output(arg_matches);
    let output_type = matches::output_type(arg_matches);
    let resolves = matches::resolves(arg_matches);
    let retry = matches::retry(arg_matches);
    let retry_interval = matches::retry_interval(arg_matches);
    let ssl_no_revoke = matches::ssl_no_revoke(arg_matches);
    let test = matches::test(arg_matches);
    let timeout = matches::timeout(arg_matches);
    let to_entry = matches::to_entry(arg_matches);
    let user = matches::user(arg_matches);
    let user_agent = matches::user_agent(arg_matches);
    let variables = matches::variables(arg_matches)?;
    let verbose = matches::verbose(arg_matches);
    let very_verbose = matches::very_verbose(arg_matches);
    Ok(Options {
        cacert_file,
        client_cert_file,
        client_key_file,
        color,
        compressed,
        connect_timeout,
        connects_to,
        cookie_input_file,
        cookie_output_file,
        error_format,
        fail_fast,
        file_root,
        follow_location,
        html_dir,
        ignore_asserts,
        include,
        input_files,
        insecure,
        interactive,
        junit_file,
        max_redirect,
        no_proxy,
        path_as_is,
        progress_bar,
        proxy,
        output,
        output_type,
        resolves,
        retry,
        retry_interval,
        ssl_no_revoke,
        test,
        timeout,
        to_entry,
        user,
        user_agent,
        variables,
        verbose,
        very_verbose,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputType {
    ResponseBody,
    Json,
    NoOutput,
}

impl Options {
    pub fn to_runner_options(&self, filename: &str, current_dir: &Path) -> RunnerOptions {
        let cacert_file = self.cacert_file.clone();
        let client_cert_file = self.client_cert_file.clone();
        let client_key_file = self.client_key_file.clone();
        let connects_to = self.connects_to.clone();
        let follow_location = self.follow_location;
        let insecure = self.insecure;
        let max_redirect = self.max_redirect;
        let path_as_is = self.path_as_is;
        let proxy = self.proxy.clone();
        let no_proxy = self.no_proxy.clone();
        let cookie_input_file = self.cookie_input_file.clone();
        let timeout = self.timeout;
        let connect_timeout = self.connect_timeout;
        let user = self.user.clone();
        let user_agent = self.user_agent.clone();
        let compressed = self.compressed;
        let file_root = match self.file_root {
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
        let pre_entry = if self.interactive {
            Some(cli::interactive::pre_entry as fn(Entry) -> bool)
        } else {
            None
        };
        let post_entry = if self.interactive {
            Some(cli::interactive::post_entry as fn() -> bool)
        } else {
            None
        };
        let fail_fast = self.fail_fast;
        let to_entry = self.to_entry;
        let resolves = self.resolves.clone();
        let retry = self.retry;
        let retry_interval = self.retry_interval;
        let ignore_asserts = self.ignore_asserts;
        let ssl_no_revoke = self.ssl_no_revoke;

        RunnerOptionsBuilder::new()
            .cacert_file(cacert_file)
            .client_cert_file(client_cert_file)
            .client_key_file(client_key_file)
            .compressed(compressed)
            .connect_timeout(connect_timeout)
            .connects_to(&connects_to)
            .context_dir(&context_dir)
            .cookie_input_file(cookie_input_file)
            .fail_fast(fail_fast)
            .follow_location(follow_location)
            .ignore_asserts(ignore_asserts)
            .insecure(insecure)
            .max_redirect(max_redirect)
            .no_proxy(no_proxy)
            .path_as_is(path_as_is)
            .post_entry(post_entry)
            .pre_entry(pre_entry)
            .proxy(proxy)
            .resolves(&resolves)
            .retry(retry)
            .retry_interval(retry_interval)
            .ssl_no_revoke(ssl_no_revoke)
            .timeout(timeout)
            .to_entry(to_entry)
            .user(user)
            .user_agent(user_agent)
            .build()
    }

    pub fn to_logger_options(&self, filename: &str) -> LoggerOptions {
        let verbosity = Verbosity::from(self.verbose, self.very_verbose);
        LoggerOptionsBuilder::new()
            .color(self.color)
            .error_format(self.error_format.clone().into())
            .filename(filename)
            .progress_bar(self.progress_bar)
            .test(self.test)
            .verbosity(verbosity)
            .build()
    }
}
