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

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Duration;

use atty::Stream;
use clap::{value_parser, ArgAction, ArgMatches, Command};
use hurl::runner::RunnerOptionsBuilder;
use hurl_core::ast::Entry;

use crate::cli;
use crate::cli::CliError;
use crate::http::{ClientOptions, ContextDir};
use crate::runner::{RunnerOptions, Value, Verbosity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliOptions {
    pub cacert_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
    pub color: bool,
    pub connects_to: Vec<String>,
    pub compressed: bool,
    pub connect_timeout: Duration,
    pub cookie_input_file: Option<String>,
    pub cookie_output_file: Option<String>,
    pub fail_fast: bool,
    pub file_root: Option<String>,
    pub follow_location: bool,
    pub glob_files: Vec<String>,
    pub html_dir: Option<PathBuf>,
    pub ignore_asserts: bool,
    pub include: bool,
    pub insecure: bool,
    pub interactive: bool,
    pub junit_file: Option<String>,
    pub max_redirect: Option<usize>,
    pub no_proxy: Option<String>,
    pub output: Option<String>,
    pub output_type: OutputType,
    pub proxy: Option<String>,
    pub resolves: Vec<String>,
    pub retry: bool,
    pub retry_interval: Duration,
    pub retry_max_count: Option<usize>,
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
pub enum OutputType {
    ResponseBody,
    Json,
    NoOutput,
}

pub fn app(version: &str) -> Command {
    let ClientOptions {
        connect_timeout: default_connect_timeout,
        max_redirect: default_max_redirect,
        retry_max_count: default_retry_max_count,
        timeout: default_timeout,
        ..
    } = ClientOptions::default();

    let default_connect_timeout = default_connect_timeout.as_secs();
    let default_max_redirect = default_max_redirect.unwrap();
    let default_timeout = default_timeout.as_secs();
    let default_retry_max_count = default_retry_max_count.unwrap();

    Command::new("hurl")
        .about("Run Hurl file(s) or standard input")
        .disable_colored_help(true)
        .version(version.to_string())
        .arg(
            clap::Arg::new("FILE")
                .help("Sets the input file to use")
                .required(false)
                .num_args(1..)
        )
        .arg(
            clap::Arg::new("cacert_file")
                .long("cacert")
                .value_name("FILE")
                .help("CA certificate to verify peer against (PEM format)")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("client_cert_file")
                .short('E')
                .long("cert")
                .value_name("CERTIFICATE[:PASSWORD]")
                .help("Client certificate file and password")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("client_key_file")
                .long("key")
                .value_name("KEY")
                .help("Private key file name")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("color")
                .long("color")
                .help("Colorize output")
                .action(ArgAction::SetTrue)
                .conflicts_with("no_color")
        )
        .arg(
            clap::Arg::new("compressed")
                .long("compressed")
                .help("Request compressed response (using deflate or gzip)")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("connect_timeout")
                .long("connect-timeout")
                .value_name("SECONDS")
                .help("Maximum time allowed for connection")
                .default_value(default_connect_timeout.to_string())
                .value_parser(value_parser!(u64))
                .num_args(1)
        )
        .arg(
            clap::Arg::new("connect_to")
                .long("connect-to")
                .value_name("HOST1:PORT1:HOST2:PORT2")
                .help("For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead")
                .action(ArgAction::Append)
                .number_of_values(1)
                .num_args(1)
        )
        .arg(
            clap::Arg::new("cookies_input_file")
                .short('b')
                .long("cookie")
                .value_name("FILE")
                .help("Read cookies from FILE")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("cookies_output_file")
                .short('c')
                .long("cookie-jar")
                .value_name("FILE")
                .help("Write cookies to FILE after running the session (only for one session)")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("fail_at_end")
                .long("fail-at-end")
                .help("Fail at end")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("file_root")
                .long("file-root")
                .value_name("DIR")
                .help("Set root filesystem to import files (default is current directory)")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("follow_location")
                .short('L')
                .long("location")
                .help("Follow redirects")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("glob")
                .long("glob")
                .value_name("GLOB")
                .help("Specify input files that match the given GLOB. Multiple glob flags may be used")
                .action(ArgAction::Append)
                .number_of_values(1)
        )
        .arg(
            clap::Arg::new("include")
                .short('i')
                .long("include")
                .help("Include the HTTP headers in the output")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("ignore_asserts")
                .long("ignore-asserts")
                .help("Ignore asserts defined in the Hurl file")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("insecure")
                .short('k')
                .long("insecure")
                .help("Allow insecure SSL connections")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("interactive")
                .long("interactive")
                .help("Turn on interactive mode")
                .conflicts_with("to_entry")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("json")
                .long("json")
                .help("Output each Hurl file result to JSON")
                .conflicts_with("no_output")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("max_redirects")
                .long("max-redirs")
                .value_name("NUM")
                .help("Maximum number of redirects allowed, -1 for unlimited redirects")
                .default_value(default_max_redirect.to_string())
                .allow_hyphen_values(true)
                .value_parser(value_parser!(i32).range(-1..))
                .num_args(1)
        )
        .arg(
            clap::Arg::new("max_time")
                .long("max-time")
                .short('m')
                .value_name("SECONDS")
                .help("Maximum time allowed for the transfer")
                .default_value(default_timeout.to_string())
                .allow_hyphen_values(true)
                .value_parser(value_parser!(u64))
                .num_args(1)
        )
        .arg(
            clap::Arg::new("no_color")
                .long("no-color")
                .help("Do not colorize output")
                .conflicts_with("color")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("no_output")
                .long("no-output")
                .help("Suppress output. By default, Hurl outputs the body of the last response")
                .conflicts_with("json")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("noproxy")
                .long("noproxy")
                .value_name("HOST(S)")
                .help("List of hosts which do not use proxy")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Write to FILE instead of stdout")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("proxy")
                .short('x')
                .long("proxy")
                .value_name("[PROTOCOL://]HOST[:PORT]")
                .help("Use proxy on given protocol/host/port")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("junit")
                .long("report-junit")
                .value_name("FILE")
                .help("Write a Junit XML report to FILE")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("report_html")
                .long("report-html")
                .value_name("DIR")
                .help("Generate HTML report to DIR")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("resolve")
                .long("resolve")
                .value_name("HOST:PORT:ADDR")
                .help("Provide a custom address for a specific host and port pair")
                .action(ArgAction::Append)
                .number_of_values(1)
                .num_args(1)
        )
        .arg(
            clap::Arg::new("retry")
                .long("retry")
                .help("Retry requests on errors")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("retry_interval")
                .long("retry-interval")
                .value_name("MILLISECONDS")
                .help("Interval in milliseconds before a retry")
                .value_parser(value_parser!(u64))
                .default_value("1000")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("retry_max_count")
                .long("retry-max-count")
                .value_name("NUM")
                .help("Maximum number of retries, -1 for unlimited retries")
                .default_value(default_retry_max_count.to_string())
                .allow_hyphen_values(true)
                .value_parser(value_parser!(i32).range(-1..))
                .num_args(1)
        )
        .arg(
            clap::Arg::new("ssl_no_revoke")
                .long("ssl-no-revoke")
                .help("(Windows) This option tells Hurl to disable certificate revocation checks. WARNING: this option loosens the SSL security, and by using this flag you ask for exactly that.")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("test")
                .long("test")
                .help("Activate test mode")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("to_entry")
                .long("to-entry")
                .value_name("ENTRY_NUMBER")
                .help("Execute Hurl file to ENTRY_NUMBER (starting at 1)")
                .conflicts_with("interactive")
                .allow_hyphen_values(true)
                .value_parser(value_parser!(u32).range(1..))
                .num_args(1)
        )
        .arg(
            clap::Arg::new("user")
                .short('u')
                .long("user")
                .value_name("USER:PASSWORD")
                .help("Add basic Authentication header to each request")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("user_agent")
                .short('A')
                .long("user-agent")
                .value_name("NAME")
                .help("Specify the User-Agent string to send to the HTTP server")
                .num_args(1)
        )
        .arg(
            clap::Arg::new("variable")
                .long("variable")
                .value_name("NAME=VALUE")
                .help("Define a variable")
                .action(ArgAction::Append)
                .number_of_values(1)
                .num_args(1)
        )
        .arg(
            clap::Arg::new("variables_file")
                .long("variables-file")
                .value_name("FILE")
                .help("Define a properties file in which you define your variables")
                .action(ArgAction::Append)
                .num_args(1)
        )
        .arg(
            clap::Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Turn on verbose output")
                .action(ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("very_verbose")
                .long("very-verbose")
                .help("Turn on verbose output, including HTTP response and libcurl logs")
                .action(ArgAction::SetTrue)
        )
}

/// Parses command line options `matches`.
// FIXME: disallow this clippy warning because of https://github.com/rust-lang/rust-clippy/issues/9766
#[allow(clippy::manual_filter)]
pub fn parse_options(matches: &ArgMatches) -> Result<CliOptions, CliError> {
    let cacert_file = match get::<String>(matches, "cacert_file") {
        None => None,
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                return Err(CliError { message });
            } else {
                Some(filename)
            }
        }
    };
    let client_cert_file = match get::<String>(matches, "client_cert_file") {
        None => None,
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                return Err(CliError { message });
            } else {
                Some(filename)
            }
        }
    };
    let client_key_file = match get::<String>(matches, "client_key_file") {
        None => None,
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {filename} does not exist");
                return Err(CliError { message });
            } else {
                Some(filename)
            }
        }
    };
    let color = output_color(matches);
    let compressed = has_flag(matches, "compressed");
    let connect_timeout = get::<u64>(matches, "connect_timeout").unwrap();
    let connect_timeout = Duration::from_secs(connect_timeout);
    let connects_to = get_strings(matches, "connect_to").unwrap_or_default();
    let cookie_input_file = get::<String>(matches, "cookies_input_file");
    let cookie_output_file = get::<String>(matches, "cookies_output_file");
    let fail_fast = !has_flag(matches, "fail_at_end");
    let file_root = get::<String>(matches, "file_root");
    let follow_location = has_flag(matches, "follow_location");
    let glob_files = match_glob_files(matches)?;
    let report_html = get::<String>(matches, "report_html");
    let html_dir = if let Some(dir) = report_html {
        let path = Path::new(&dir);
        if !path.exists() {
            match std::fs::create_dir(path) {
                Err(_) => {
                    return Err(CliError {
                        message: format!("Html dir {} can not be created", path.display()),
                    });
                }
                Ok(_) => Some(path.to_path_buf()),
            }
        } else if path.is_dir() {
            Some(path.to_path_buf())
        } else {
            return Err(CliError {
                message: format!("{} is not a valid directory", path.display()),
            });
        }
    } else {
        None
    };
    let ignore_asserts = has_flag(matches, "ignore_asserts");
    let include = has_flag(matches, "include");
    let insecure = has_flag(matches, "insecure");
    let interactive = has_flag(matches, "interactive");
    let junit_file = get::<String>(matches, "junit");
    let max_redirect = get::<i32>(matches, "max_redirects").unwrap();
    let max_redirect = match max_redirect {
        m if m == -1 => None,
        m => Some(m as usize),
    };
    let no_proxy = get::<String>(matches, "proxy");
    let output = get::<String>(matches, "output");
    let test = has_flag(matches, "test");
    let output_type = if has_flag(matches, "json") {
        OutputType::Json
    } else if has_flag(matches, "no_output") || test {
        OutputType::NoOutput
    } else {
        OutputType::ResponseBody
    };
    let proxy = get::<String>(matches, "proxy");
    let resolves = get_strings(matches, "resolve").unwrap_or_default();
    let retry = has_flag(matches, "retry");
    let retry_interval = get::<u64>(matches, "retry_interval").unwrap();
    let retry_interval = Duration::from_millis(retry_interval);
    let retry_max_count = get::<i32>(matches, "retry_max_count").unwrap();
    let retry_max_count = match retry_max_count {
        r if r == -1 => None,
        r => Some(r as usize),
    };
    let ssl_no_revoke = has_flag(matches, "ssl_no_revoke");
    let timeout = get::<u64>(matches, "max_time").unwrap();
    let timeout = Duration::from_secs(timeout);
    let to_entry = get::<u32>(matches, "to_entry").map(|x| x as usize);
    let user = get::<String>(matches, "user");
    let user_agent = get::<String>(matches, "user_agent");
    let variables = variables(matches)?;
    let very_verbose = has_flag(matches, "very_verbose");
    let verbose = has_flag(matches, "verbose") || has_flag(matches, "interactive") || very_verbose;

    Ok(CliOptions {
        cacert_file,
        client_cert_file,
        client_key_file,
        color,
        compressed,
        connect_timeout,
        connects_to,
        cookie_input_file,
        cookie_output_file,
        fail_fast,
        file_root,
        follow_location,
        glob_files,
        html_dir,
        ignore_asserts,
        include,
        insecure,
        interactive,
        junit_file,
        max_redirect,
        no_proxy,
        output,
        output_type,
        proxy,
        resolves,
        retry,
        retry_interval,
        retry_max_count,
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

/// Returns true if Hurl output uses ANSI code and false otherwise.
pub fn output_color(matches: &ArgMatches) -> bool {
    if has_flag(matches, "color") {
        return true;
    }
    if has_flag(matches, "no_color") {
        return false;
    }
    if let Ok(v) = env::var("NO_COLOR") {
        if !v.is_empty() {
            return false;
        }
    }
    atty::is(Stream::Stdout)
}

/// Returns a map of variables from the command line options `matches`.
fn variables(matches: &ArgMatches) -> Result<HashMap<String, Value>, CliError> {
    let mut variables = HashMap::new();

    // Use environment variables prefix by HURL_
    for (env_name, env_value) in env::vars() {
        if let Some(name) = env_name.strip_prefix("HURL_") {
            let value = cli::parse_variable_value(env_value.as_str())?;
            variables.insert(name.to_string(), value);
        }
    }

    if let Some(filenames) = get_strings(matches, "variables_file") {
        for f in filenames.iter() {
            let path = Path::new(&f);
            if !path.exists() {
                return Err(CliError {
                    message: format!("Properties file {} does not exist", path.display()),
                });
            }

            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                let line = match line {
                    Ok(s) => s,
                    Err(_) => {
                        return Err(CliError {
                            message: format!(
                                "Can not parse line {} of {}",
                                index + 1,
                                path.display()
                            ),
                        });
                    }
                };
                let line = line.trim();
                if line.starts_with('#') || line.is_empty() {
                    continue;
                }
                let (name, value) = cli::parse_variable(line)?;
                variables.insert(name.to_string(), value);
            }
        }
    }

    if let Some(input) = get_strings(matches, "variable") {
        for s in input {
            let (name, value) = cli::parse_variable(&s)?;
            variables.insert(name.to_string(), value);
        }
    }

    Ok(variables)
}

/// Returns a list of path names from the command line options `matches`.
fn match_glob_files(matches: &ArgMatches) -> Result<Vec<String>, CliError> {
    let mut filenames = vec![];
    if let Some(exprs) = get_strings(matches, "glob") {
        for expr in exprs {
            let paths = match glob::glob(&expr) {
                Ok(paths) => paths,
                Err(_) => {
                    return Err(CliError {
                        message: "Failed to read glob pattern".to_string(),
                    })
                }
            };
            for entry in paths {
                match entry {
                    Ok(path) => match path.into_os_string().into_string() {
                        Ok(filename) => filenames.push(filename),
                        Err(_) => {
                            return Err(CliError {
                                message: "Failed to read glob pattern".to_string(),
                            })
                        }
                    },
                    Err(_) => {
                        return Err(CliError {
                            message: "Failed to read glob pattern".to_string(),
                        })
                    }
                }
            }
        }
    }
    Ok(filenames)
}

/// Returns an optional value of type `T` from the command line `matches` given the option `name`.
fn get<T: Clone + Send + Sync + 'static>(matches: &ArgMatches, name: &str) -> Option<T> {
    matches.get_one::<T>(name).cloned()
}

/// Returns an optional list of `String` from the command line `matches` given the option `name`.
pub fn get_strings(matches: &ArgMatches, name: &str) -> Option<Vec<String>> {
    matches
        .get_many::<String>(name)
        .map(|v| v.map(|x| x.to_string()).collect())
}

/// Returns true if the command line options `matches` has a given flag `name`.
pub fn has_flag(matches: &ArgMatches, name: &str) -> bool {
    matches.get_one::<bool>(name) == Some(&true)
}

impl CliOptions {
    pub fn to(&self, filename: &str, current_dir: &Path) -> RunnerOptions {
        let cacert_file = self.cacert_file.clone();
        let client_cert_file = self.client_cert_file.clone();
        let client_key_file = self.client_key_file.clone();
        let connects_to = self.connects_to.clone();
        let follow_location = self.follow_location;
        let verbosity = match (self.verbose, self.very_verbose) {
            (true, true) => Some(Verbosity::VeryVerbose),
            (true, _) => Some(Verbosity::Verbose),
            _ => None,
        };
        let insecure = self.insecure;
        let max_redirect = self.max_redirect;
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
        let retry_max_count = self.retry_max_count;
        let ignore_asserts = self.ignore_asserts;
        let ssl_no_revoke = self.ssl_no_revoke;

        let mut bd = RunnerOptionsBuilder::new();
        bd.cacert_file(cacert_file)
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
            .post_entry(post_entry)
            .pre_entry(pre_entry)
            .proxy(proxy)
            .resolves(&resolves)
            .retry(retry)
            .retry_interval(retry_interval)
            .retry_max_count(retry_max_count)
            .ssl_no_revoke(ssl_no_revoke)
            .timeout(timeout)
            .to_entry(to_entry)
            .user(user)
            .user_agent(user_agent)
            .verbosity(verbosity)
            .build()
    }
}
