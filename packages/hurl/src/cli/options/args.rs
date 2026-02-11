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
use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::ArgMatches;

use super::{commands, get_version, matches, CliOptions, CliOptionsError, RunContext, Verbosity};

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
    if !matches::has_input_files(&arg_matches, context) {
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
    let aws_sigv4 = matches::aws_sigv4(arg_matches, default_options.aws_sigv4);
    let cacert_file = matches::cacert_file(arg_matches, default_options.cacert_file)?;
    let client_cert_file =
        matches::client_cert_file(arg_matches, default_options.client_cert_file)?;
    let client_key_file = matches::client_key_file(arg_matches, default_options.client_key_file)?;
    let color = matches::color(arg_matches, default_options.color);
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
    let secrets = matches::secret(arg_matches, default_options.secrets)?;
    let ssl_no_revoke = matches::ssl_no_revoke(arg_matches, default_options.ssl_no_revoke);
    let tap_file = matches::tap_file(arg_matches, default_options.tap_file);
    let test = matches::test(arg_matches, default_options.test);
    let timeout = matches::timeout(arg_matches, default_options.timeout)?;
    let to_entry = matches::to_entry(arg_matches, default_options.to_entry);
    let unix_socket = matches::unix_socket(arg_matches, default_options.unix_socket);
    let user = matches::user(arg_matches, default_options.user);
    let user_agent = matches::user_agent(arg_matches, default_options.user_agent);
    let variables = matches::variables(arg_matches, default_options.variables)?;

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
