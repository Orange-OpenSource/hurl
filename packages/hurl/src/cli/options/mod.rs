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
mod args;
mod commands;
mod config_file;
mod context;
mod duration;
mod env_vars;
mod error;
mod secret;
mod variables;
mod variables_file;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

use hurl::http;
use hurl::http::RequestedHttpVersion;
use hurl::pretty::PrettyMode;
use hurl::runner::Output;
use hurl::util::logger;
use hurl::util::logger::{LoggerOptions, LoggerOptionsBuilder};
use hurl::util::path::ContextDir;
use hurl_core::input::{Input, InputKind};
use hurl_core::types::{BytesPerSec, Count};

pub use crate::cli::options::context::RunContext;
use crate::runner::{RunnerOptions, RunnerOptionsBuilder, Value};
pub use error::CliOptionsError;

/// Represents the list of all options that can be used in Hurl command line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliOptions {
    pub aws_sigv4: Option<String>,
    pub cacert_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
    pub color_stdout: bool,
    pub color_stderr: bool,
    pub compressed: bool,
    pub connect_timeout: Duration,
    pub connects_to: Vec<String>,
    pub continue_on_error: bool,
    pub cookie_input_file: Option<String>,
    pub cookie_output_file: Option<PathBuf>,
    pub curl_file: Option<PathBuf>,
    pub delay: Duration,
    pub digest: bool,
    pub error_format: ErrorFormat,
    pub file_root: Option<String>,
    pub follow_location: bool,
    pub follow_location_trusted: bool,
    pub from_entry: Option<usize>,
    pub headers: Vec<String>,
    pub html_dir: Option<PathBuf>,
    pub http_version: Option<HttpVersion>,
    pub ignore_asserts: bool,
    pub include: bool,
    pub input_files: Vec<Input>,
    pub insecure: bool,
    pub ip_resolve: Option<IpResolve>,
    pub jobs: Option<usize>,
    pub json_report_dir: Option<PathBuf>,
    pub junit_file: Option<PathBuf>,
    pub limit_rate: Option<BytesPerSec>,
    pub max_filesize: Option<u64>,
    pub max_redirect: Count,
    pub negotiate: bool,
    pub netrc: bool,
    pub netrc_file: Option<String>,
    pub netrc_optional: bool,
    pub no_cookie_store: bool,
    pub no_proxy: Option<String>,
    pub ntlm: bool,
    pub output: Option<Output>,
    pub output_type: OutputType,
    pub parallel: bool,
    pub path_as_is: bool,
    pub pinned_pub_key: Option<String>,
    pub pretty: PrettyMode,
    pub progress_bar: bool,
    pub proxy: Option<String>,
    pub repeat: Option<Count>,
    pub resolves: Vec<String>,
    pub retry: Option<Count>,
    pub retry_interval: Duration,
    pub secrets: HashMap<String, String>,
    pub ssl_no_revoke: bool,
    pub tap_file: Option<PathBuf>,
    pub test: bool,
    pub timeout: Duration,
    pub to_entry: Option<usize>,
    pub unix_socket: Option<String>,
    pub user: Option<String>,
    pub user_agent: Option<String>,
    pub variables: HashMap<String, Value>,
    pub verbosity: Option<Verbosity>,
}

/// Log verbosity level
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Verbosity {
    Brief,
    Verbose,
    Debug,
}

impl FromStr for Verbosity {
    type Err = CliOptionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "brief" => Ok(Verbosity::Brief),
            "verbose" => Ok(Verbosity::Verbose),
            "debug" => Ok(Verbosity::Debug),
            _ => {
                let message = format!(
                    "invalid value '{s}' for verbosity [possible values: brief, verbose, debug]"
                );
                Err(CliOptionsError::Error(message))
            }
        }
    }
}

/// Error format: long or rich.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorFormat {
    Short,
    Long,
}

impl From<ErrorFormat> for logger::ErrorFormat {
    fn from(value: ErrorFormat) -> Self {
        match value {
            ErrorFormat::Short => logger::ErrorFormat::Short,
            ErrorFormat::Long => logger::ErrorFormat::Long,
        }
    }
}

/// Requested HTTP version.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HttpVersion {
    V10,
    V11,
    V2,
    V3,
}

impl From<HttpVersion> for RequestedHttpVersion {
    fn from(value: HttpVersion) -> Self {
        match value {
            HttpVersion::V10 => RequestedHttpVersion::Http10,
            HttpVersion::V11 => RequestedHttpVersion::Http11,
            HttpVersion::V2 => RequestedHttpVersion::Http2,
            HttpVersion::V3 => RequestedHttpVersion::Http3,
        }
    }
}

/// IP protocol used.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IpResolve {
    IpV4,
    IpV6,
}

impl From<IpResolve> for http::IpResolve {
    fn from(value: IpResolve) -> Self {
        match value {
            IpResolve::IpV4 => http::IpResolve::IpV4,
            IpResolve::IpV6 => http::IpResolve::IpV6,
        }
    }
}

fn get_version() -> String {
    let libcurl_version = http::libcurl_version_info();
    let pkg_version = env!("CARGO_PKG_VERSION");
    format!(
        "{} ({}) {}\nFeatures (libcurl):  {}\nFeatures (built-in): brotli",
        pkg_version,
        libcurl_version.host,
        libcurl_version.libraries.join(" "),
        libcurl_version.features.join(" ")
    )
}

/// Parse the Hurl CLI options and returns a [`CliOptions`] result, given a run `context`
/// (environment variables).
pub fn parse(context: &RunContext) -> Result<CliOptions, CliOptionsError> {
    let options = CliOptions::default();
    let options = context::init_options(context, options);
    let options = config_file::parse_config_file(context.config_file_path(), options)?;
    let options = env_vars::parse_env_vars(context, options)?;
    let options = args::parse_cli_args(context, options)?;
    Ok(options)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputType {
    /// The last HTTP response body of a Hurl file is outputted on standard output.
    ResponseBody,
    /// The whole Hurl file run is exported in a structured JSON export on standard output.
    Json,
    /// Nothing is outputted on standard output when a Hurl file run is completed.
    NoOutput,
}

impl Default for CliOptions {
    fn default() -> Self {
        CliOptions {
            aws_sigv4: None,
            cacert_file: None,
            client_cert_file: None,
            client_key_file: None,
            color_stdout: false,
            color_stderr: false,
            compressed: false,
            connect_timeout: Duration::from_secs(300),
            connects_to: Vec::new(),
            continue_on_error: false,
            cookie_input_file: None,
            cookie_output_file: None,
            curl_file: None,
            delay: Duration::from_millis(0),
            digest: false,
            error_format: ErrorFormat::Short,
            file_root: None,
            follow_location: false,
            follow_location_trusted: false,
            from_entry: None,
            headers: Vec::new(),
            html_dir: None,
            http_version: None,
            ignore_asserts: false,
            include: false,
            input_files: Vec::new(),
            insecure: false,
            ip_resolve: None,
            jobs: None,
            json_report_dir: None,
            junit_file: None,
            limit_rate: None,
            max_filesize: None,
            max_redirect: Count::Finite(50),
            negotiate: false,
            netrc: false,
            netrc_file: None,
            netrc_optional: false,
            no_cookie_store: false,
            no_proxy: None,
            ntlm: false,
            output: None,
            output_type: OutputType::ResponseBody,
            parallel: false,
            path_as_is: false,
            pinned_pub_key: None,
            pretty: PrettyMode::None,
            progress_bar: false,
            proxy: None,
            repeat: None,
            resolves: Vec::new(),
            retry: None,
            retry_interval: Duration::from_millis(1000),
            secrets: HashMap::new(),
            ssl_no_revoke: false,
            tap_file: None,
            test: false,
            timeout: Duration::from_secs(300),
            to_entry: None,
            unix_socket: None,
            user: None,
            user_agent: None,
            variables: HashMap::new(),
            verbosity: None,
        }
    }
}

impl CliOptions {
    /// Converts this instance of [`CliOptions`] to an instance of [`RunnerOptions`]
    pub fn to_runner_options(&self, filename: &Input, current_dir: &Path) -> RunnerOptions {
        let aws_sigv4 = self.aws_sigv4.clone();
        let cacert_file = self.cacert_file.clone();
        let client_cert_file = self.client_cert_file.clone();
        let client_key_file = self.client_key_file.clone();
        let compressed = self.compressed;
        let connect_timeout = self.connect_timeout;
        let connects_to = self.connects_to.clone();
        let file_root = match &self.file_root {
            Some(f) => Path::new(f),
            None => match filename.kind() {
                InputKind::File(path) => path.parent().unwrap(),
                InputKind::Stdin(_) => current_dir,
            },
        };
        let context_dir = ContextDir::new(current_dir, file_root);
        let continue_on_error = self.continue_on_error;
        let cookie_input_file = self.cookie_input_file.clone();
        let delay = self.delay;
        let digest = self.digest;
        let follow_location = self.follow_location;
        let follow_location_trusted = self.follow_location_trusted;
        let from_entry = self.from_entry;
        let headers = &self.headers;
        let http_version = match self.http_version {
            Some(version) => version.into(),
            None => RequestedHttpVersion::default(),
        };
        let ignore_asserts = self.ignore_asserts;
        let insecure = self.insecure;
        let ip_resolve = match self.ip_resolve {
            Some(ip) => ip.into(),
            None => http::IpResolve::default(),
        };
        let max_filesize = self.max_filesize;
        // Like curl, we don't differentiate upload and download limit rate, we have
        // only one option.
        let max_recv_speed = self.limit_rate;
        let max_send_speed = self.limit_rate;
        let max_redirect = self.max_redirect;
        let netrc = self.netrc;
        let netrc_file = self.netrc_file.clone();
        let netrc_optional = self.netrc_optional;
        let no_proxy = self.no_proxy.clone();
        let output = self.output.clone();
        let path_as_is = self.path_as_is;
        let pinned_pub_key = self.pinned_pub_key.clone();
        let proxy = self.proxy.clone();
        let resolves = self.resolves.clone();
        let retry = self.retry;
        let retry_interval = self.retry_interval;
        let ssl_no_revoke = self.ssl_no_revoke;
        let negotiate = self.negotiate;
        let ntlm = self.ntlm;
        let timeout = self.timeout;
        let to_entry = self.to_entry;
        let unix_socket = self.unix_socket.clone();
        let use_cookie_store = !self.no_cookie_store;
        let user = self.user.clone();
        let user_agent = self.user_agent.clone();

        RunnerOptionsBuilder::new()
            .aws_sigv4(aws_sigv4)
            .cacert_file(cacert_file)
            .client_cert_file(client_cert_file)
            .client_key_file(client_key_file)
            .delay(delay)
            .digest(digest)
            .compressed(compressed)
            .connect_timeout(connect_timeout)
            .connects_to(&connects_to)
            .continue_on_error(continue_on_error)
            .context_dir(&context_dir)
            .cookie_input_file(cookie_input_file)
            .follow_location(follow_location)
            .follow_location_trusted(follow_location_trusted)
            .from_entry(from_entry)
            .headers(headers)
            .http_version(http_version)
            .ignore_asserts(ignore_asserts)
            .insecure(insecure)
            .ip_resolve(ip_resolve)
            .max_filesize(max_filesize)
            .max_recv_speed(max_recv_speed)
            .max_redirect(max_redirect)
            .max_send_speed(max_send_speed)
            .negotiate(negotiate)
            .netrc(netrc)
            .netrc_file(netrc_file)
            .netrc_optional(netrc_optional)
            .no_proxy(no_proxy)
            .ntlm(ntlm)
            .output(output)
            .path_as_is(path_as_is)
            .pinned_pub_key(pinned_pub_key)
            .proxy(proxy)
            .resolves(&resolves)
            .retry(retry)
            .retry_interval(retry_interval)
            .ssl_no_revoke(ssl_no_revoke)
            .timeout(timeout)
            .to_entry(to_entry)
            .unix_socket(unix_socket)
            .use_cookie_store(use_cookie_store)
            .user(user)
            .user_agent(user_agent)
            .build()
    }

    /// Converts this instance of [`ClipOptions`] to an instance of [`LoggerOptions`]
    pub fn to_logger_options(&self) -> LoggerOptions {
        let verbosity = self.verbosity.map(|v| match v {
            Verbosity::Brief => logger::Verbosity::LowVerbose,
            Verbosity::Verbose => logger::Verbosity::Verbose,
            Verbosity::Debug => logger::Verbosity::VeryVerbose,
        });
        LoggerOptionsBuilder::new()
            .color(self.color_stderr)
            .error_format(self.error_format.into())
            .verbosity(verbosity)
            .build()
    }
}
