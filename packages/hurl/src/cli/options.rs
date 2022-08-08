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

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Duration;

use atty::Stream;
use clap::{AppSettings, ArgAction, ArgMatches, Command};

use crate::cli;
use crate::cli::CliError;
use crate::http::ClientOptions;
use crate::runner::Value;
use crate::util::logger::BaseLogger;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliOptions {
    pub cacert_file: Option<String>,
    pub color: bool,
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
    pub progress: bool,
    pub proxy: Option<String>,
    pub summary: bool,
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
    Command::new("hurl")
        .about("Run hurl FILE(s) or standard input")
        .setting(AppSettings::DeriveDisplayOrder)
        .disable_colored_help(true)
        .version(version)
        .arg(
            clap::Arg::new("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .multiple_values(true),
        )
        .arg(
            clap::Arg::new("cacert_file")
                .long("cacert")
                .value_name("FILE")
                .help("CA certificate to verify peer against (PEM format)"),
        )
        .arg(
            clap::Arg::new("color")
                .long("color")
                .conflicts_with("no_color")
                .help("Colorize Output"),
        )
        .arg(
            clap::Arg::new("compressed")
                .long("compressed")
                .help("Request compressed response (using deflate or gzip)"),
        )
        .arg(
            clap::Arg::new("connect_timeout")
                .long("connect-timeout")
                .value_name("SECONDS")
                .help("Maximum time allowed for connection"),
        )
        .arg(
            clap::Arg::new("cookies_input_file")
                .short('b')
                .long("cookie")
                .value_name("FILE")
                .help("Read cookies from FILE"),
        )
        .arg(
            clap::Arg::new("cookies_output_file")
                .short('c')
                .long("cookie-jar")
                .value_name("FILE")
                .help("Write cookies to FILE after running the session (only for one session)"),
        )
        .arg(
            clap::Arg::new("fail_at_end")
                .long("fail-at-end")
                .help("Fail at end")
        )
        .arg(
            clap::Arg::new("file_root")
                .long("file-root")
                .value_name("DIR")
                .help("set root filesystem to import file in hurl (default is current directory)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("follow_location")
                .short('L')
                .long("location")
                .help("Follow redirects"),
        )
        .arg(
            clap::Arg::new("glob")
                .long("glob")
                .value_name("GLOB")
                .action(ArgAction::Append)
                .number_of_values(1)
                .help("Specify input files that match the given blob. Multiple glob flags may be used."),
        )
        .arg(
            clap::Arg::new("include")
                .short('i')
                .long("include")
                .help("Include the HTTP headers in the output"),
        )
        .arg(
            clap::Arg::new("ignore_asserts")
                .long("ignore-asserts")
                .help("Ignore asserts defined in the Hurl file."),
        )
        .arg(
            clap::Arg::new("insecure")
                .short('k')
                .long("insecure")
                .help("Allow insecure SSL connections"),
        )
        .arg(
            clap::Arg::new("interactive")
                .long("interactive")
                .conflicts_with("to_entry")
                .help("Turn on interactive mode"),
        )
        .arg(
            clap::Arg::new("json")
                .long("json")
                .conflicts_with("no_output")
                .help("Output each hurl file result to JSON"),
        )
        .arg(
            clap::Arg::new("max_redirects")
                .long("max-redirs")
                .value_name("NUM")
                .allow_hyphen_values(true)
                .help("Maximum number of redirects allowed"),
        )
        .arg(
            clap::Arg::new("max_time")
                .long("max-time")
                .short('m')
                .value_name("NUM")
                .allow_hyphen_values(true)
                .help("Maximum time allowed for the transfer"),
        )
        .arg(
            clap::Arg::new("no_color")
                .long("no-color")
                .conflicts_with("color")
                .help("Do not colorize Output"),
        )
        .arg(
            clap::Arg::new("no_output")
                .long("no-output")
                .conflicts_with("json")
                .help("Suppress output. By default, Hurl outputs the body of the last response."),
        )
        .arg(
            clap::Arg::new("noproxy")
                .long("noproxy")
                .value_name("HOST(S)")
                .help("List of hosts which do not use proxy")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Write to FILE instead of stdout"),
        )
        .arg(
            clap::Arg::new("progress")
                .long("progress")
                .help("Print filename and status for each test (stderr)"),
        )
        .arg(
            clap::Arg::new("proxy")
                .short('x')
                .long("proxy")
                .value_name("[PROTOCOL://]HOST[:PORT]")
                .help("Use proxy on given protocol/host/port"),
        )
        .arg(
            clap::Arg::new("junit")
                .long("report-junit")
                .value_name("FILE")
                .help("Write a Junit XML report to the given file")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("report_html")
                .long("report-html")
                .value_name("DIR")
                .help("Generate html report to dir")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("summary")
                .long("summary")
                .help("Print test metrics at the end of the run (stderr)"),
        )
        .arg(
            clap::Arg::new("test")
                .long("test")
                .help("Activate test mode; equals --no-output --progress --summary"),
        )
        .arg(
            clap::Arg::new("to_entry")
                .long("to-entry")
                .value_name("ENTRY_NUMBER")
                .conflicts_with("interactive")
                .help("Execute hurl file to ENTRY_NUMBER (starting at 1)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("user")
                .short('u')
                .long("user")
                .value_name("user:password")
                .help("Add basic Authentication header to each request.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("user_agent")
                .short('A')
                .long("user-agent")
                .value_name("name")
                .help("Specify the User-Agent string to send to the HTTP server.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("variable")
                .long("variable")
                .value_name("NAME=VALUE")
                .action(ArgAction::Append)
                .number_of_values(1)
                .help("Define a variable")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("variables_file")
                .long("variables-file")
                .value_name("FILE")
                .help("Define a properties file in which you define your variables")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Turn on verbose output"),
        )
        .arg(
            clap::Arg::new("very_verbose")
                .long("very-verbose")
                .help("Turn on verbose output, uncluding HTTP response"),
        )
}

pub fn parse_options(matches: &ArgMatches, logger: &BaseLogger) -> Result<CliOptions, CliError> {
    let cacert_file = match get_string(matches, "cacert_file") {
        None => None,
        Some(filename) => {
            if !Path::new(&filename).is_file() {
                let message = format!("File {} does not exist", filename);
                return Err(CliError { message });
            } else {
                Some(filename)
            }
        }
    };
    let color = output_color(matches);
    let compressed = has_flag(matches, "compressed");
    let connect_timeout = match get_string(matches, "connect_timeout") {
        None => ClientOptions::default().connect_timeout,
        Some(s) => match s.parse::<u64>() {
            Ok(n) => Duration::from_secs(n),
            Err(_) => {
                return Err(CliError {
                    message: "connect-timeout option can not be parsed".to_string(),
                });
            }
        },
    };
    let cookie_input_file = get_string(matches, "cookies_input_file");
    let cookie_output_file = get_string(matches, "cookies_output_file");
    let fail_fast = !has_flag(matches, "fail_at_end");
    let file_root = get_string(matches, "file_root");
    let follow_location = has_flag(matches, "follow_location");
    let glob_files = match_glob_files(matches)?;
    let report_html = get_string(matches, "report_html");
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
    let junit_file = get_string(matches, "junit");
    let max_redirect = match get_string(matches, "max_redirects").as_deref() {
        None => Some(50),
        Some("-1") => None,
        Some(s) => match s.parse::<usize>() {
            Ok(x) => Some(x),
            Err(_) => {
                return Err(CliError {
                    message: "max_redirs option can not be parsed".to_string(),
                });
            }
        },
    };
    let no_proxy = get_string(matches, "proxy");
    let output = get_string(matches, "output");
    let test = has_flag(matches, "test");
    let output_type = if has_flag(matches, "json") {
        OutputType::Json
    } else if has_flag(matches, "no_output") || test {
        OutputType::NoOutput
    } else {
        OutputType::ResponseBody
    };
    let progress = has_flag(matches, "progress");
    if progress {
        logger.warning(
            "--progress option is deprecated and will be removed soon. Please use --test or --json",
        );
    }
    let progress = progress || test;
    let proxy = get_string(matches, "proxy");
    let summary = has_flag(matches, "summary");
    if summary {
        logger.warning(
            "--summary option is deprecated and will be removed soon. Please use --test or --json",
        );
    }
    let summary = summary || test;
    let timeout = match get_string(matches, "max_time") {
        None => ClientOptions::default().timeout,
        Some(s) => match s.parse::<u64>() {
            Ok(n) => Duration::from_secs(n),
            Err(_) => {
                return Err(CliError {
                    message: "max_time option can not be parsed".to_string(),
                });
            }
        },
    };
    let to_entry = to_entry(matches)?;
    let user = get_string(matches, "user");
    let user_agent = get_string(matches, "user_agent");
    let variables = variables(matches.clone())?;
    let very_verbose = has_flag(matches, "very_verbose");
    let verbose = has_flag(matches, "verbose") || has_flag(matches, "interactive") || very_verbose;

    Ok(CliOptions {
        cacert_file,
        color,
        compressed,
        connect_timeout,
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
        progress,
        proxy,
        summary,
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

fn to_entry(matches: &ArgMatches) -> Result<Option<usize>, CliError> {
    match get_string(matches, "to_entry") {
        Some(value) => match value.parse() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(CliError {
                message: "Invalid value for option --to-entry - must be a positive integer!"
                    .to_string(),
            }),
        },
        None => Ok(None),
    }
}

fn variables(matches: ArgMatches) -> Result<HashMap<String, Value>, CliError> {
    let mut variables = HashMap::new();

    // use environment variables prefix by HURL_
    for (env_name, env_value) in env::vars() {
        if let Some(name) = env_name.strip_prefix("HURL_") {
            let value = cli::parse_variable_value(env_value.as_str())?;
            variables.insert(name.to_string(), value);
        }
    }

    if let Some(filename) = get_string(&matches, "variables_file") {
        let path = Path::new(&filename);
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
                        message: format!("Can not parse line {} of {}", index + 1, path.display()),
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

    if let Some(input) = get_strings(&matches, "variable") {
        for s in input {
            let (name, value) = cli::parse_variable(&s)?;
            variables.insert(name.to_string(), value);
        }
    }

    Ok(variables)
}

///
/// Returns a list of path names that match `matches`.
///
/// # Arguments
/// * `matches` - A pattern to be matched
///
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

fn get_string(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).map(|x| x.to_string())
}

pub fn get_strings(matches: &ArgMatches, name: &str) -> Option<Vec<String>> {
    matches
        .get_many::<String>(name)
        .map(|v| v.map(|x| x.to_string()).collect())
}

pub fn has_flag(matches: &ArgMatches, name: &str) -> bool {
    matches.contains_id(name)
}
