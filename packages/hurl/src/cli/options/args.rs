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
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::ArgMatches;
use hurl::pretty::PrettyMode;
use hurl::runner::Value;
use hurl_core::input::Input;
use hurl_core::types::{BytesPerSec, Count, DurationUnit};

use super::context::RunContext;
use super::variables::TypeKind;
use super::variables_file::VariablesFile;
use super::{commands, get_version, secret, CliOptions};
use super::{duration, variables, CliOptionsError, ErrorFormat, HttpVersion, IpResolve, Output};
use super::{OutputType, Verbosity};

/// Parses the command line arguments given a `context` and default options values.
///
/// Returns a new [`CliOptions`].
pub fn parse_cli_args(
    context: &RunContext,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
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

    let arg_matches = command.try_get_matches_from_mut(std::env::args_os());
    let arg_matches = match arg_matches {
        Ok(args) => args,
        Err(error) => return Err(CliOptionsError::from_clap(error, default_options.color)),
    };

    // If we've no file input (either from the standard input or from the command line arguments),
    // we just print help and exit.
    if !has_input_files(&arg_matches, context) {
        let help = if default_options.color {
            command.render_help().ansi().to_string()
        } else {
            command.render_help().to_string()
        };
        return Err(CliOptionsError::NoInput(help));
    }

    let options = parse_arg_matches(&arg_matches, context, default_options)?;
    if options.input_files.is_empty() {
        return Err(CliOptionsError::Error(
            "No input files provided".to_string(),
        ));
    }
    Ok(options)
}

/// Parse command line arguments from `arg_matches`
/// given a run `context` and `default_options`.
fn parse_arg_matches(
    arg_matches: &ArgMatches,
    context: &RunContext,
    default_options: CliOptions,
) -> Result<CliOptions, CliOptionsError> {
    let aws_sigv4 = aws_sigv4(arg_matches, default_options.aws_sigv4);
    let cacert_file = cacert_file(arg_matches, default_options.cacert_file)?;
    let client_cert_file = client_cert_file(arg_matches, default_options.client_cert_file)?;
    let client_key_file = client_key_file(arg_matches, default_options.client_key_file)?;
    let color = color(arg_matches, default_options.color);
    let compressed = compressed(arg_matches, default_options.compressed);
    let connect_timeout = connect_timeout(arg_matches, default_options.connect_timeout)?;
    let connects_to = connects_to(arg_matches, default_options.connects_to);
    let continue_on_error = continue_on_error(arg_matches, default_options.continue_on_error);
    let cookie_input_file = cookie_input_file(arg_matches, default_options.cookie_input_file);
    let cookie_output_file = cookie_output_file(arg_matches, default_options.cookie_output_file);
    let curl_file = curl_file(arg_matches, default_options.curl_file);
    let delay = delay(arg_matches, default_options.delay)?;
    let digest = digest(arg_matches, default_options.digest);
    let error_format = error_format(arg_matches, default_options.error_format);
    let file_root = file_root(arg_matches, default_options.file_root);
    let (follow_location, follow_location_trusted) = follow_location(
        arg_matches,
        (
            default_options.follow_location,
            default_options.follow_location_trusted,
        ),
    );
    let from_entry = from_entry(arg_matches, default_options.from_entry);
    let headers = headers(arg_matches, default_options.headers);
    let html_dir = html_dir(arg_matches, default_options.html_dir)?;
    let http_version = http_version(arg_matches, default_options.http_version);
    let ignore_asserts = ignore_asserts(arg_matches, default_options.ignore_asserts);
    let include = include(arg_matches, default_options.include);
    let input_files = input_files(arg_matches, context)?;
    let insecure = insecure(arg_matches, default_options.insecure);
    let ip_resolve = ip_resolve(arg_matches, default_options.ip_resolve);
    let jobs = jobs(arg_matches, default_options.jobs);
    let json_report_dir = json_report_dir(arg_matches, default_options.json_report_dir)?;
    let junit_file = junit_file(arg_matches, default_options.junit_file);
    let limit_rate = limit_rate(arg_matches, default_options.limit_rate);
    let max_filesize = max_filesize(arg_matches, default_options.max_filesize);
    let max_redirect = max_redirect(arg_matches, default_options.max_redirect);
    let negotiate = negotiate(arg_matches, default_options.negotiate);
    let netrc = netrc(arg_matches, default_options.netrc);
    let netrc_file = netrc_file(arg_matches, default_options.netrc_file)?;
    let netrc_optional = netrc_optional(arg_matches, default_options.netrc_optional);
    let no_cookie_store = no_cookie_store(arg_matches, default_options.no_cookie_store);
    let no_proxy = no_proxy(arg_matches, default_options.no_proxy);
    let ntlm = ntlm(arg_matches, default_options.ntlm);
    let parallel = parallel(arg_matches, default_options.parallel);
    let path_as_is = path_as_is(arg_matches, default_options.path_as_is);
    let pinned_pub_key = pinned_pub_key(arg_matches, default_options.pinned_pub_key);
    let progress_bar = progress_bar(arg_matches, context, default_options.progress_bar);
    let pretty = pretty(arg_matches, default_options.pretty);
    let proxy = proxy(arg_matches, default_options.proxy);
    let output = output(arg_matches, default_options.output);
    let output_type = output_type(arg_matches, default_options.output_type);
    let repeat = repeat(arg_matches, default_options.repeat);
    let resolves = resolves(arg_matches, default_options.resolves);
    let retry = retry(arg_matches, default_options.retry);
    let retry_interval = retry_interval(arg_matches, default_options.retry_interval)?;
    let secrets = secret(arg_matches, default_options.secrets)?;
    let ssl_no_revoke = ssl_no_revoke(arg_matches, default_options.ssl_no_revoke);
    let tap_file = tap_file(arg_matches, default_options.tap_file);
    let test = test(arg_matches, default_options.test);
    let timeout = timeout(arg_matches, default_options.timeout)?;
    let to_entry = to_entry(arg_matches, default_options.to_entry);
    let unix_socket = unix_socket(arg_matches, default_options.unix_socket);
    let user = user(arg_matches, default_options.user);
    let user_agent = user_agent(arg_matches, default_options.user_agent);
    let variables = variables(arg_matches, default_options.variables)?;

    let verbose = verbose(
        arg_matches,
        default_options.verbosity == Some(Verbosity::Verbose),
    );
    let very_verbose = very_verbose(
        arg_matches,
        default_options.verbosity == Some(Verbosity::Debug),
    );
    // --verbose and --very-verbose flags are prioritized over --verbosity enum.
    let verbosity = if verbose {
        Some(Verbosity::Verbose)
    } else if very_verbose {
        Some(Verbosity::Debug)
    } else {
        verbosity(arg_matches, default_options.verbosity)
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

fn cacert_file(
    arg_matches: &ArgMatches,
    default_value: Option<String>,
) -> Result<Option<String>, CliOptionsError> {
    match get_string(arg_matches, "cacert_file") {
        None => Ok(default_value),
        Some(filename) => {
            let path = Path::new(&filename);
            if path.exists() {
                Ok(Some(filename))
            } else {
                Err(CliOptionsError::Error(format!(
                    "Input file {} does not exist",
                    path.display()
                )))
            }
        }
    }
}

fn aws_sigv4(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "aws_sigv4").or(default_value)
}

fn client_cert_file(
    arg_matches: &ArgMatches,
    default_value: Option<String>,
) -> Result<Option<String>, CliOptionsError> {
    match get::<String>(arg_matches, "client_cert_file") {
        None => Ok(default_value),
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                Err(CliOptionsError::Error(message))
            } else {
                Ok(Some(filename))
            }
        }
    }
}

fn client_key_file(
    arg_matches: &ArgMatches,
    default_value: Option<String>,
) -> Result<Option<String>, CliOptionsError> {
    match get::<String>(arg_matches, "client_key_file") {
        None => Ok(default_value),
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                Err(CliOptionsError::Error(message))
            } else {
                Ok(Some(filename))
            }
        }
    }
}

/// Returns true if Hurl output uses ANSI code and false otherwise.
fn color(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "color") {
        return true;
    }
    if has_flag(arg_matches, "no_color") {
        return false;
    }
    default_value
}

fn compressed(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "compressed") {
        true
    } else {
        default_value
    }
}

fn connect_timeout(
    arg_matches: &ArgMatches,
    default_value: Duration,
) -> Result<Duration, CliOptionsError> {
    match get::<String>(arg_matches, "connect_timeout") {
        Some(s) => get_duration(&s, DurationUnit::Second),
        None => Ok(default_value),
    }
}

fn connects_to(arg_matches: &ArgMatches, default_value: Vec<String>) -> Vec<String> {
    get_strings(arg_matches, "connect_to").unwrap_or(default_value)
}

fn continue_on_error(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "continue_on_error") {
        true
    } else {
        default_value
    }
}

fn cookie_input_file(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "cookies_input_file").or(default_value)
}

fn cookie_output_file(arg_matches: &ArgMatches, default_value: Option<PathBuf>) -> Option<PathBuf> {
    get::<String>(arg_matches, "cookies_output_file")
        .map(PathBuf::from)
        .or(default_value)
}

fn curl_file(arg_matches: &ArgMatches, default_value: Option<PathBuf>) -> Option<PathBuf> {
    get::<String>(arg_matches, "curl")
        .map(PathBuf::from)
        .or(default_value)
}

fn delay(arg_matches: &ArgMatches, default_value: Duration) -> Result<Duration, CliOptionsError> {
    match get::<String>(arg_matches, "delay") {
        Some(s) => get_duration(&s, DurationUnit::MilliSecond),
        None => Ok(default_value),
    }
}

fn digest(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "digest") {
        true
    } else {
        default_value
    }
}

fn error_format(arg_matches: &ArgMatches, default_value: ErrorFormat) -> ErrorFormat {
    match get::<String>(arg_matches, "error_format") {
        Some(error_format) => match error_format.as_str() {
            "long" => ErrorFormat::Long,
            "short" => ErrorFormat::Short,
            _ => ErrorFormat::Short,
        },
        None => default_value,
    }
}

fn file_root(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "file_root").or(default_value)
}

fn follow_location(arg_matches: &ArgMatches, default_value: (bool, bool)) -> (bool, bool) {
    let follow_location = has_flag(arg_matches, "follow_location")
        || has_flag(arg_matches, "follow_location_trusted");
    let follow_location_trusted = has_flag(arg_matches, "follow_location_trusted");
    if follow_location || follow_location_trusted {
        (follow_location, follow_location_trusted)
    } else {
        default_value
    }
}

fn from_entry(arg_matches: &ArgMatches, default_value: Option<usize>) -> Option<usize> {
    get::<u32>(arg_matches, "from_entry")
        .map(|x| x as usize)
        .or(default_value)
}

fn headers(arg_matches: &ArgMatches, default_value: Vec<String>) -> Vec<String> {
    get_strings(arg_matches, "header").unwrap_or(default_value)
}

fn html_dir(
    arg_matches: &ArgMatches,
    default_value: Option<PathBuf>,
) -> Result<Option<PathBuf>, CliOptionsError> {
    if let Some(dir) = get::<String>(arg_matches, "report_html") {
        let path = Path::new(&dir);
        if !path.exists() {
            match fs::create_dir_all(path) {
                Err(_) => Err(CliOptionsError::Error(format!(
                    "HTML dir {} can not be created",
                    path.display()
                ))),
                Ok(_) => Ok(Some(path.to_path_buf())),
            }
        } else if path.is_dir() {
            Ok(Some(path.to_path_buf()))
        } else {
            Err(CliOptionsError::Error(format!(
                "{} is not a valid directory",
                path.display()
            )))
        }
    } else {
        Ok(default_value)
    }
}

fn http_version(
    arg_matches: &ArgMatches,
    default_value: Option<HttpVersion>,
) -> Option<HttpVersion> {
    if has_flag(arg_matches, "http3") {
        Some(HttpVersion::V3)
    } else if has_flag(arg_matches, "http2") {
        Some(HttpVersion::V2)
    } else if has_flag(arg_matches, "http11") {
        Some(HttpVersion::V11)
    } else if has_flag(arg_matches, "http10") {
        Some(HttpVersion::V10)
    } else {
        default_value
    }
}

fn ignore_asserts(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "ignore_asserts") {
        true
    } else {
        default_value
    }
}

fn include(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "include") {
        true
    } else {
        default_value
    }
}

/// Returns true if we have at least one input files.
/// The input file can be a file, the standard input, or a glob (even a glob returns empty results).
fn has_input_files(arg_matches: &ArgMatches, context: &RunContext) -> bool {
    get_strings(arg_matches, "input_files").is_some()
        || get_strings(arg_matches, "glob").is_some()
        || !context.is_stdin_term()
}

/// Returns the input files from the positional arguments and the glob options
fn input_files(
    arg_matches: &ArgMatches,
    context: &RunContext,
) -> Result<Vec<Input>, CliOptionsError> {
    let mut files = vec![];
    if let Some(filenames) = get_strings(arg_matches, "input_files") {
        for filename in &filenames {
            let filename = Path::new(filename);
            if !filename.exists() {
                return Err(CliOptionsError::InvalidInputFile(filename.to_path_buf()));
            }
            if filename.is_file() {
                let file = Input::from(filename);
                files.push(file);
            } else if filename.is_dir() {
                walks_hurl_files(filename, &mut files)?;
            }
        }
    }
    for filename in glob_files(arg_matches)? {
        files.push(filename);
    }
    if files.is_empty() && !context.is_stdin_term() {
        let input = match Input::from_stdin() {
            Ok(input) => input,
            Err(err) => return Err(CliOptionsError::Error(err.to_string())),
        };
        files.push(input);
    }
    Ok(files)
}

/// Walks recursively a directory from `dir` and push Hurl files to `files`.
fn walks_hurl_files(dir: &Path, files: &mut Vec<Input>) -> Result<(), CliOptionsError> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Err(CliOptionsError::InvalidInputFile(dir.to_path_buf()));
    };
    for entry in entries {
        let Ok(entry) = entry else {
            return Err(CliOptionsError::InvalidInputFile(dir.to_path_buf()));
        };
        let path = entry.path();
        if path.is_dir() {
            walks_hurl_files(&path, files)?;
        } else if entry.path().extension() == Some("hurl".as_ref()) {
            files.push(Input::from(entry.path()));
        }
    }
    Ok(())
}

fn insecure(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "insecure") {
        true
    } else {
        default_value
    }
}

fn ip_resolve(arg_matches: &ArgMatches, default_value: Option<IpResolve>) -> Option<IpResolve> {
    if has_flag(arg_matches, "ipv6") {
        Some(IpResolve::IpV6)
    } else if has_flag(arg_matches, "ipv4") {
        Some(IpResolve::IpV4)
    } else {
        default_value
    }
}

fn junit_file(arg_matches: &ArgMatches, default_value: Option<PathBuf>) -> Option<PathBuf> {
    get::<String>(arg_matches, "report_junit")
        .map(PathBuf::from)
        .or(default_value)
}

fn limit_rate(arg_matches: &ArgMatches, default_value: Option<BytesPerSec>) -> Option<BytesPerSec> {
    get::<u64>(arg_matches, "limit_rate")
        .map(BytesPerSec)
        .or(default_value)
}

fn max_filesize(arg_matches: &ArgMatches, default_value: Option<u64>) -> Option<u64> {
    get::<u64>(arg_matches, "max_filesize").or(default_value)
}

fn max_redirect(arg_matches: &ArgMatches, default_value: Count) -> Count {
    match get::<i32>(arg_matches, "max_redirects") {
        Some(-1) => Count::Infinite,
        Some(m) => Count::Finite(m as usize),
        None => default_value,
    }
}

fn jobs(arg_matches: &ArgMatches, default_value: Option<usize>) -> Option<usize> {
    get::<u32>(arg_matches, "jobs")
        .map(|m| m as usize)
        .or(default_value)
}

fn json_report_dir(
    arg_matches: &ArgMatches,
    default_value: Option<PathBuf>,
) -> Result<Option<PathBuf>, CliOptionsError> {
    if let Some(dir) = get::<String>(arg_matches, "report_json") {
        let path = Path::new(&dir);
        if !path.exists() {
            match fs::create_dir_all(path) {
                Err(_) => Err(CliOptionsError::Error(format!(
                    "JSON dir {} can not be created",
                    path.display()
                ))),
                Ok(_) => Ok(Some(path.to_path_buf())),
            }
        } else if path.is_dir() {
            Ok(Some(path.to_path_buf()))
        } else {
            Err(CliOptionsError::Error(format!(
                "{} is not a valid directory",
                path.display()
            )))
        }
    } else {
        Ok(default_value)
    }
}

fn negotiate(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "negotiate") {
        true
    } else {
        default_value
    }
}

fn netrc(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "netrc") {
        true
    } else {
        default_value
    }
}

fn netrc_file(
    arg_matches: &ArgMatches,
    default_value: Option<String>,
) -> Result<Option<String>, CliOptionsError> {
    match get::<String>(arg_matches, "netrc_file") {
        None => Ok(default_value),
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                Err(CliOptionsError::Error(message))
            } else {
                Ok(Some(filename))
            }
        }
    }
}

fn netrc_optional(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "netrc_optional") {
        true
    } else {
        default_value
    }
}

fn no_cookie_store(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "no_cookie_store") {
        true
    } else {
        default_value
    }
}

fn no_proxy(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "no_proxy").or(default_value)
}

fn ntlm(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "ntlm") {
        true
    } else {
        default_value
    }
}

fn output(arg_matches: &ArgMatches, default_value: Option<Output>) -> Option<Output> {
    get::<String>(arg_matches, "output")
        .map(|filename| Output::new(&filename))
        .or(default_value)
}

fn output_type(arg_matches: &ArgMatches, default_value: OutputType) -> OutputType {
    if has_flag(arg_matches, "json") {
        OutputType::Json
    } else if has_flag(arg_matches, "no_output") || test(arg_matches, false) {
        OutputType::NoOutput
    } else {
        default_value
    }
}

fn parallel(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "parallel") || has_flag(arg_matches, "test") {
        true
    } else {
        default_value
    }
}

fn path_as_is(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "path_as_is") {
        true
    } else {
        default_value
    }
}

fn pinned_pub_key(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "pinned_pub_key").or(default_value)
}

fn pretty(arg_matches: &ArgMatches, default_value: PrettyMode) -> PrettyMode {
    if has_flag(arg_matches, "pretty") {
        return PrettyMode::Force;
    }
    if has_flag(arg_matches, "no_pretty") {
        return PrettyMode::None;
    }
    default_value
}

fn progress_bar(arg_matches: &ArgMatches, context: &RunContext, default_value: bool) -> bool {
    if has_flag(arg_matches, "progress_bar") {
        return true;
    }
    // The progress bar is automatically displayed for test mode when stderr is a TTY and not running in CI.
    if has_flag(arg_matches, "test") && context.is_stderr_term() && !context.is_ci_env_var() {
        return true;
    }
    default_value
}

fn proxy(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "proxy").or(default_value)
}

fn repeat(arg_matches: &ArgMatches, default_value: Option<Count>) -> Option<Count> {
    match get::<i32>(arg_matches, "repeat") {
        Some(-1) => Some(Count::Infinite),
        Some(n) => Some(Count::Finite(n as usize)),
        None => default_value,
    }
}

fn resolves(arg_matches: &ArgMatches, default_value: Vec<String>) -> Vec<String> {
    get_strings(arg_matches, "resolve").unwrap_or(default_value)
}

fn retry(arg_matches: &ArgMatches, default_value: Option<Count>) -> Option<Count> {
    match get::<i32>(arg_matches, "retry") {
        Some(-1) => Some(Count::Infinite),
        Some(r) => Some(Count::Finite(r as usize)),
        None => default_value,
    }
}

fn retry_interval(
    arg_matches: &ArgMatches,
    default_value: Duration,
) -> Result<Duration, CliOptionsError> {
    match get::<String>(arg_matches, "retry_interval") {
        Some(s) => get_duration(&s, DurationUnit::MilliSecond),
        None => Ok(default_value),
    }
}

fn secret(
    matches: &ArgMatches,
    default_value: HashMap<String, String>,
) -> Result<HashMap<String, String>, CliOptionsError> {
    let mut all_secrets = default_value;

    // Secrets are always parsed as string.
    let type_kind = TypeKind::String;

    // Add secrets from files:
    if let Some(filenames) = get_strings(matches, "secrets_file") {
        for f in &filenames {
            let filename = Path::new(f);
            let vars = VariablesFile::open(filename, type_kind)?;
            for var in vars {
                let (name, value) = var?;
                secret::add_secret(&mut all_secrets, name, value)?;
            }
        }
    }

    // Finally, add single secrets.
    if let Some(secrets) = get_strings(matches, "secret") {
        for s in secrets {
            let (name, value) = variables::parse(&s, type_kind)?;
            secret::add_secret(&mut all_secrets, name, value)?;
        }
    }
    Ok(all_secrets)
}

fn ssl_no_revoke(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "ssl_no_revoke") {
        true
    } else {
        default_value
    }
}

fn tap_file(arg_matches: &ArgMatches, default_value: Option<PathBuf>) -> Option<PathBuf> {
    get::<String>(arg_matches, "report_tap")
        .map(PathBuf::from)
        .or(default_value)
}

fn test(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "test") {
        true
    } else {
        default_value
    }
}

fn timeout(arg_matches: &ArgMatches, default_value: Duration) -> Result<Duration, CliOptionsError> {
    match get::<String>(arg_matches, "max_time") {
        Some(s) => get_duration(&s, DurationUnit::Second),
        None => Ok(default_value),
    }
}

fn to_entry(arg_matches: &ArgMatches, default_value: Option<usize>) -> Option<usize> {
    get::<u32>(arg_matches, "to_entry")
        .map(|x| x as usize)
        .or(default_value)
}

fn unix_socket(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "unix_socket").or(default_value)
}

fn user(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "user").or(default_value)
}

fn user_agent(arg_matches: &ArgMatches, default_value: Option<String>) -> Option<String> {
    get::<String>(arg_matches, "user_agent").or(default_value)
}

/// Returns a map of variables from the command line options `matches`.
fn variables(
    matches: &ArgMatches,
    default_value: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, CliOptionsError> {
    let mut variables = default_value;

    // Variables are typed, based on their values.
    let type_kind = TypeKind::Inferred;

    // Add variables from files:
    if let Some(filenames) = get_strings(matches, "variables_file") {
        for f in &filenames {
            let filename = Path::new(f);
            let vars = VariablesFile::open(filename, type_kind)?;
            for var in vars {
                let (name, value) = var?;
                variables.insert(name.to_string(), value);
            }
        }
    }

    // Add single variables from command line.
    if let Some(input) = get_strings(matches, "variable") {
        for s in input {
            let (name, value) = variables::parse(&s, type_kind)?;
            variables.insert(name.to_string(), value);
        }
    }

    Ok(variables)
}

fn verbosity(arg_matches: &ArgMatches, default_value: Option<Verbosity>) -> Option<Verbosity> {
    match get::<String>(arg_matches, "verbosity") {
        Some(value) => Some(match value.as_str() {
            "brief" => Verbosity::Brief,
            "verbose" => Verbosity::Verbose,
            "debug" => Verbosity::Debug,
            _ => unreachable!(),
        }),
        None => default_value,
    }
}

fn verbose(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "verbose") {
        true
    } else {
        default_value
    }
}

fn very_verbose(arg_matches: &ArgMatches, default_value: bool) -> bool {
    if has_flag(arg_matches, "very_verbose") {
        true
    } else {
        default_value
    }
}

/// Returns a list of path names from the command line options `matches`.
fn glob_files(matches: &ArgMatches) -> Result<Vec<Input>, CliOptionsError> {
    let mut all_files = vec![];
    if let Some(exprs) = get_strings(matches, "glob") {
        for expr in exprs {
            let paths = match glob::glob(&expr) {
                Ok(paths) => paths,
                Err(_) => {
                    return Err(CliOptionsError::Error(
                        "Failed to read glob pattern".to_string(),
                    ))
                }
            };
            let mut files = vec![];
            for entry in paths {
                match entry {
                    Ok(path) => files.push(Input::from(path)),
                    Err(_) => {
                        return Err(CliOptionsError::Error(
                            "Failed to read glob pattern".to_string(),
                        ))
                    }
                }
            }
            if files.is_empty() {
                return Err(CliOptionsError::InvalidInputFile(PathBuf::from(&expr)));
            }
            all_files.extend(files);
        }
    }
    Ok(all_files)
}

/// Returns an optional value of type `T` from the command line `matches` given the option `name`.
fn get<T: Clone + Send + Sync + 'static>(matches: &ArgMatches, name: &str) -> Option<T> {
    matches.get_one::<T>(name).cloned()
}

fn has_flag(matches: &ArgMatches, name: &str) -> bool {
    matches.get_one::<bool>(name) == Some(&true)
}

fn get_string(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).map(|x| x.to_string())
}

/// Returns an optional list of `String` from the command line `matches` given the option `name`.
fn get_strings(matches: &ArgMatches, name: &str) -> Option<Vec<String>> {
    matches
        .get_many::<String>(name)
        .map(|v| v.map(|x| x.to_string()).collect())
}

/// Get duration from input string `s` and `default_unit`
fn get_duration(s: &str, default_unit: DurationUnit) -> Result<Duration, CliOptionsError> {
    let duration = duration::parse(s).map_err(CliOptionsError::Error)?;
    let unit = duration.unit.unwrap_or(default_unit);
    let millis = match unit {
        DurationUnit::MilliSecond => duration.value.as_u64(),
        DurationUnit::Second => duration.value.as_u64() * 1000,
        DurationUnit::Minute => duration.value.as_u64() * 1000 * 60,
        DurationUnit::Hour => duration.value.as_u64() * 1000 * 60 * 60,
    };
    Ok(Duration::from_millis(millis))
}
