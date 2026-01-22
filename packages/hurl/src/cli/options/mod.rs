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
mod commands;
mod context;
mod duration;
mod error;
mod matches;
mod variables;
mod variables_file;

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::ArgMatches;
pub use error::CliOptionsError;
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

/// Represents the list of all options that can be used in Hurl command line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliOptions {
    pub aws_sigv4: Option<String>,
    pub cacert_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
    pub color: bool,
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
    let styles = Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Cyan.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default());

    let mut command = clap::Command::new("hurl")
        .version(get_version())
        .disable_colored_help(true)
        .styles(styles)
        .about("Hurl, run and test HTTP requests with plain text")
        // HTTP options
        .arg(commands::aws_sigv4())
        .arg(commands::cacert_file())
        .arg(commands::client_cert_file())
        .arg(commands::compressed())
        .arg(commands::connect_timeout())
        .arg(commands::connect_to())
        .arg(commands::digest())
        .arg(commands::header())
        .arg(commands::http10())
        .arg(commands::http11())
        .arg(commands::http2())
        .arg(commands::http3())
        .arg(commands::input_files())
        .arg(commands::insecure())
        .arg(commands::ipv4())
        .arg(commands::ipv6())
        .arg(commands::client_key_file())
        .arg(commands::limit_rate())
        .arg(commands::follow_location())
        .arg(commands::follow_location_trusted())
        .arg(commands::max_filesize())
        .arg(commands::max_redirects())
        .arg(commands::max_time())
        .arg(commands::negotiate())
        .arg(commands::no_cookie_store())
        .arg(commands::no_proxy())
        .arg(commands::ntlm())
        .arg(commands::path_as_is())
        .arg(commands::pinned_pub_key())
        .arg(commands::proxy())
        .arg(commands::resolve())
        .arg(commands::ssl_no_revoke())
        .arg(commands::unix_socket())
        .arg(commands::user())
        .arg(commands::user_agent())
        // Output options
        .arg(commands::color())
        .arg(commands::curl())
        .arg(commands::error_format())
        .arg(commands::include())
        .arg(commands::json())
        .arg(commands::no_color())
        .arg(commands::no_output())
        .arg(commands::no_pretty())
        .arg(commands::output())
        .arg(commands::pretty())
        .arg(commands::progress_bar())
        .arg(commands::verbose())
        .arg(commands::very_verbose())
        .arg(commands::verbosity())
        // Run options
        .arg(commands::continue_on_error())
        .arg(commands::delay())
        .arg(commands::from_entry())
        .arg(commands::ignore_asserts())
        .arg(commands::jobs())
        .arg(commands::parallel())
        .arg(commands::repeat())
        .arg(commands::retry())
        .arg(commands::retry_interval())
        .arg(commands::secret())
        .arg(commands::secrets_file())
        .arg(commands::test())
        .arg(commands::to_entry())
        .arg(commands::variable())
        .arg(commands::variables_file())
        // Report options
        .arg(commands::report_html())
        .arg(commands::report_json())
        .arg(commands::report_junit())
        .arg(commands::report_tap())
        // Other options
        .arg(commands::cookies_input_file())
        .arg(commands::cookies_output_file())
        .arg(commands::file_root())
        .arg(commands::glob())
        .arg(commands::netrc())
        .arg(commands::netrc_file())
        .arg(commands::netrc_optional());

    let arg_matches = command.try_get_matches_from_mut(env::args_os());
    let arg_matches = match arg_matches {
        Ok(args) => args,
        Err(error) => return Err(CliOptionsError::from_clap(error, context.is_with_color())),
    };

    // If we've no file input (either from the standard input or from the command line arguments),
    // we just print help and exit.
    if !matches::has_input_files(&arg_matches, context) {
        let help = if context.is_with_color() {
            command.render_help().ansi().to_string()
        } else {
            command.render_help().to_string()
        };
        return Err(CliOptionsError::NoInput(help));
    }

    let options = CliOptions::default();
    // TODO: let options = parse_config_file(arg_matches, options)?;
    let options = parse_matches(&arg_matches, context, options)?;
    if options.input_files.is_empty() {
        return Err(CliOptionsError::Error(
            "No input files provided".to_string(),
        ));
    }

    Ok(options)
}

/// Parse command line arguments from `arg_matches`
/// given a run `context` and `default_options`.
fn parse_matches(
    arg_matches: &ArgMatches,
    context: &RunContext,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
    let aws_sigv4 = matches::aws_sigv4(arg_matches, default_options.aws_sigv4);
    let cacert_file = matches::cacert_file(arg_matches, default_options.cacert_file)?;
    let client_cert_file =
        matches::client_cert_file(arg_matches, default_options.client_cert_file)?;
    let client_key_file = matches::client_key_file(arg_matches, default_options.client_key_file)?;
    let color = matches::color(arg_matches, context, default_options.color);
    let compressed = matches::compressed(arg_matches, default_options.compressed);
    let connect_timeout = matches::connect_timeout(arg_matches, default_options.connect_timeout)?;
    let connects_to = matches::connects_to(arg_matches, default_options.connects_to);
    let continue_on_error =
        matches::continue_on_error(arg_matches, default_options.continue_on_error);
    let cookie_input_file =
        matches::cookie_input_file(arg_matches, default_options.cookie_input_file);
    let cookie_output_file =
        matches::cookie_output_file(arg_matches, default_options.cookie_output_file);
    let curl_file = matches::curl_file(arg_matches, default_options.curl_file);
    let delay = matches::delay(arg_matches, default_options.delay)?;
    let digest = matches::digest(arg_matches, default_options.digest);
    let error_format = matches::error_format(arg_matches, default_options.error_format);
    let file_root = matches::file_root(arg_matches, default_options.file_root);
    let (follow_location, follow_location_trusted) = matches::follow_location(
        arg_matches,
        (
            default_options.follow_location,
            default_options.follow_location_trusted,
        ),
    );
    let from_entry = matches::from_entry(arg_matches, default_options.from_entry);
    let headers = matches::headers(arg_matches, default_options.headers);
    let html_dir = matches::html_dir(arg_matches, default_options.html_dir)?;
    let http_version = matches::http_version(arg_matches, default_options.http_version);
    let ignore_asserts = matches::ignore_asserts(arg_matches, default_options.ignore_asserts);
    let include = matches::include(arg_matches, default_options.include);
    let input_files = matches::input_files(arg_matches, context)?;
    let insecure = matches::insecure(arg_matches, default_options.insecure);
    let ip_resolve = matches::ip_resolve(arg_matches, default_options.ip_resolve);
    let jobs = matches::jobs(arg_matches, default_options.jobs);
    let json_report_dir = matches::json_report_dir(arg_matches, default_options.json_report_dir)?;
    let junit_file = matches::junit_file(arg_matches, default_options.junit_file);
    let limit_rate = matches::limit_rate(arg_matches, default_options.limit_rate);
    let max_filesize = matches::max_filesize(arg_matches, default_options.max_filesize);
    let max_redirect = matches::max_redirect(arg_matches, default_options.max_redirect);
    let negotiate = matches::negotiate(arg_matches, default_options.negotiate);
    let netrc = matches::netrc(arg_matches, default_options.netrc);
    let netrc_file = matches::netrc_file(arg_matches, default_options.netrc_file)?;
    let netrc_optional = matches::netrc_optional(arg_matches, default_options.netrc_optional);
    let no_cookie_store = matches::no_cookie_store(arg_matches, default_options.no_cookie_store);
    let no_proxy = matches::no_proxy(arg_matches, default_options.no_proxy);
    let ntlm = matches::ntlm(arg_matches, default_options.ntlm);
    let parallel = matches::parallel(arg_matches, default_options.parallel);
    let path_as_is = matches::path_as_is(arg_matches, default_options.path_as_is);
    let pinned_pub_key = matches::pinned_pub_key(arg_matches, default_options.pinned_pub_key);
    let progress_bar = matches::progress_bar(arg_matches, context, default_options.progress_bar);
    let pretty = matches::pretty(arg_matches, context, default_options.pretty);
    let proxy = matches::proxy(arg_matches, default_options.proxy);
    let output = matches::output(arg_matches, default_options.output);
    let output_type = matches::output_type(arg_matches, default_options.output_type);
    let repeat = matches::repeat(arg_matches, default_options.repeat);
    let resolves = matches::resolves(arg_matches, default_options.resolves);
    let retry = matches::retry(arg_matches, default_options.retry);
    let retry_interval = matches::retry_interval(arg_matches, default_options.retry_interval)?;
    let secrets = matches::secret(arg_matches, context, default_options.secrets)?;
    let ssl_no_revoke = matches::ssl_no_revoke(arg_matches, default_options.ssl_no_revoke);
    let tap_file = matches::tap_file(arg_matches, default_options.tap_file);
    let test = matches::test(arg_matches, default_options.test);
    let timeout = matches::timeout(arg_matches, default_options.timeout)?;
    let to_entry = matches::to_entry(arg_matches, default_options.to_entry);
    let unix_socket = matches::unix_socket(arg_matches, default_options.unix_socket);
    let user = matches::user(arg_matches, default_options.user);
    let user_agent = matches::user_agent(arg_matches, default_options.user_agent);
    let variables = matches::variables(arg_matches, context, default_options.variables)?;

    let verbose = matches::verbose(
        arg_matches,
        default_options.verbosity == Some(Verbosity::Verbose),
    );
    let very_verbose = matches::very_verbose(
        arg_matches,
        default_options.verbosity == Some(Verbosity::Debug),
    );
    // --verbose and --very-verbose flags are prioritized over --verbosity enum.
    let verbosity = if verbose {
        Some(Verbosity::Verbose)
    } else if very_verbose {
        Some(Verbosity::Debug)
    } else {
        matches::verbosity(arg_matches, default_options.verbosity)
    };

    Ok(CliOptions {
        aws_sigv4,
        cacert_file,
        client_cert_file,
        client_key_file,
        color,
        compressed,
        connect_timeout,
        connects_to,
        continue_on_error,
        cookie_input_file,
        cookie_output_file,
        curl_file,
        delay,
        digest,
        error_format,
        file_root,
        follow_location,
        follow_location_trusted,
        from_entry,
        headers,
        html_dir,
        http_version,
        ignore_asserts,
        include,
        input_files,
        insecure,
        ip_resolve,
        json_report_dir,
        junit_file,
        limit_rate,
        max_filesize,
        max_redirect,
        negotiate,
        netrc,
        netrc_file,
        netrc_optional,
        no_cookie_store,
        no_proxy,
        ntlm,
        path_as_is,
        pinned_pub_key,
        parallel,
        pretty,
        progress_bar,
        proxy,
        output,
        output_type,
        repeat,
        resolves,
        retry,
        retry_interval,
        secrets,
        ssl_no_revoke,
        tap_file,
        test,
        timeout,
        to_entry,
        unix_socket,
        user,
        user_agent,
        variables,
        verbosity,
        jobs,
    })
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
            color: false,
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
            .color(self.color)
            .error_format(self.error_format.into())
            .verbosity(verbosity)
            .build()
    }
}
