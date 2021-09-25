/*
 * hurl (https://hurl.dev)
 * Copyright (C) 2020 Orange
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
use crate::cli::CliError;
use crate::http::ClientOptions;
use crate::runner::Value;
use atty::Stream;
use clap::{AppSettings, ArgMatches};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliOptions {
    pub color: bool,
    pub compressed: bool,
    pub connect_timeout: Duration,
    pub cookie_input_file: Option<String>,
    pub cookie_output_file: Option<String>,
    pub fail_fast: bool,
    pub file_root: Option<String>,
    pub follow_location: bool,
    pub html_dir: Option<PathBuf>,
    pub include: bool,
    pub insecure: bool,
    pub interactive: bool,
    pub json_file: Option<PathBuf>,
    pub max_redirect: Option<usize>,
    pub no_proxy: Option<String>,
    pub output: Option<String>,
    pub progress: bool,
    pub proxy: Option<String>,
    pub summary: bool,
    pub timeout: Duration,
    pub to_entry: Option<usize>,
    pub user: Option<String>,
    pub variables: HashMap<String, Value>,
    pub verbose: bool,
}

pub fn app() -> clap::App<'static, 'static> {
    clap::App::new("hurl")
        .version(clap::crate_version!())
        .about("Run hurl FILE(s) or standard input")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .arg(
            clap::Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .multiple(true),
        )
        .arg(
            clap::Arg::with_name("color")
                .long("color")
                .conflicts_with("no-color")
                .help("Colorize Output"),
        )
        .arg(
            clap::Arg::with_name("compressed")
                .long("compressed")
                .help("Request compressed response (using deflate or gzip)"),
        )
        .arg(
            clap::Arg::with_name("connect_timeout")
                .long("connect-timeout")
                .value_name("SECONDS")
                .help("Maximum time allowed for connection"),
        )
        .arg(
            clap::Arg::with_name("cookies_input_file")
                .short("b")
                .long("cookie")
                .value_name("FILE")
                .help("Read cookies from FILE"),
        )
        .arg(
            clap::Arg::with_name("cookies_output_file")
                .short("c")
                .long("cookie-jar")
                .value_name("FILE")
                .help("Write cookies to FILE after running the session (only for one session)"),
        )
        .arg(
            clap::Arg::with_name("fail_at_end")
                .long("fail-at-end")
                .help("Fail at end")
                .takes_value(false),
        )
        .arg(
            clap::Arg::with_name("file_root")
                .long("file-root")
                .value_name("DIR")
                .help("set root filesystem to import file in hurl (default is current directory)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("follow_location")
                .short("L")
                .long("location")
                .help("Follow redirects"),
        )
        .arg(
            clap::Arg::with_name("html")
                .long("html")
                .value_name("DIR")
                .help("Generate html report to dir")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("include")
                .short("i")
                .long("include")
                .help("Include the HTTP headers in the output"),
        )
        .arg(
            clap::Arg::with_name("insecure")
                .short("k")
                .long("insecure")
                .help("Allow insecure SSL connections"),
        )
        .arg(
            clap::Arg::with_name("interactive")
                .long("interactive")
                .conflicts_with("to_entry")
                .help("Turn on interactive mode"),
        )
        .arg(
            clap::Arg::with_name("json")
                .long("json")
                .value_name("FILE")
                .help("Write full session(s) to json file")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("max_time")
                .long("max-time")
                .short("m")
                .value_name("NUM")
                .allow_hyphen_values(true)
                .help("Maximum time allowed for the transfer"),
        )
        .arg(
            clap::Arg::with_name("max_redirects")
                .long("max-redirs")
                .value_name("NUM")
                .allow_hyphen_values(true)
                .help("Maximum number of redirects allowed"),
        )
        .arg(
            clap::Arg::with_name("no_color")
                .long("no-color")
                .conflicts_with("color")
                .help("Do not colorize Output"),
        )
        .arg(
            clap::Arg::with_name("noproxy")
                .long("noproxy")
                .value_name("HOST(S)")
                .help("List of hosts which do not use proxy")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Write to FILE instead of stdout"),
        )
        .arg(
            clap::Arg::with_name("progress")
                .long("progress")
                .help("Print filename and status for each test"),
        )
        .arg(
            clap::Arg::with_name("proxy")
                .short("x")
                .long("proxy")
                .value_name("[PROTOCOL://]HOST[:PORT]")
                .help("Use proxy on given protocol/host/port"),
        )
        .arg(
            clap::Arg::with_name("summary")
                .long("summary")
                .help("Print test metrics at the end of the run"),
        )
        .arg(
            clap::Arg::with_name("test")
                .long("test")
                .help("Activate test mode; equals --output /dev/null --progress --summary"),
        )
        .arg(
            clap::Arg::with_name("to_entry")
                .long("to-entry")
                .value_name("ENTRY_NUMBER")
                .conflicts_with("interactive")
                .help("Execute hurl file to ENTRY_NUMBER (starting at 1)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("user")
                .short("u")
                .long("user")
                .value_name("user:password")
                .help("Add basic Authentication header to each request.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("variable")
                .long("variable")
                .value_name("NAME=VALUE")
                .multiple(true)
                .number_of_values(1)
                .help("Define a variable")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("variables_file")
                .long("variables-file")
                .value_name("FILE")
                .help("Define a properties file in which you define your variables")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Turn on verbose output"),
        )
}

pub fn parse_options(matches: ArgMatches) -> Result<CliOptions, CliError> {
    let color = output_color(matches.clone());
    let compressed = matches.is_present("compressed");
    let connect_timeout = match matches.value_of("connect_timeout") {
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
    let cookie_input_file = matches
        .value_of("cookies_input_file")
        .map(|x| x.to_string());
    let cookie_output_file = matches
        .value_of("cookies_output_file")
        .map(|x| x.to_string());
    let fail_fast = !matches.is_present("fail_at_end");
    let file_root = matches.value_of("file_root").map(|value| value.to_string());
    let follow_location = matches.is_present("follow_location");
    let html_dir = if let Some(dir) = matches.value_of("html") {
        let path = Path::new(dir);
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
    let include = matches.is_present("include");
    let insecure = matches.is_present("insecure");
    let interactive = matches.is_present("interactive");
    let json_file = if let Some(filename) = matches.value_of("json") {
        let path = Path::new(filename);
        Some(path.to_path_buf())
    } else {
        None
    };
    let max_redirect = match matches.value_of("max_redirects") {
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
    let no_proxy = matches.value_of("proxy").map(|x| x.to_string());
    let output = if let Some(filename) = matches.value_of("output") {
        Some(filename.to_string())
    } else if matches.is_present("test") {
        Some("/dev/null".to_string())
    } else {
        None
    };
    let progress = matches.is_present("progress") || matches.is_present("test");
    let proxy = matches.value_of("proxy").map(|x| x.to_string());
    let summary = matches.is_present("summary") || matches.is_present("test");
    let timeout = match matches.value_of("max_time") {
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
    let to_entry = to_entry(matches.clone())?;
    let user = matches.value_of("user").map(|x| x.to_string());
    let variables = variables(matches.clone())?;
    let verbose = matches.is_present("verbose") || matches.is_present("interactive");

    Ok(CliOptions {
        color,
        compressed,
        connect_timeout,
        cookie_input_file,
        cookie_output_file,
        fail_fast,
        file_root,
        follow_location,
        html_dir,
        include,
        insecure,
        interactive,
        json_file,
        max_redirect,
        no_proxy,
        output,
        progress,
        proxy,
        summary,
        timeout,
        to_entry,
        user,
        variables,
        verbose,
    })
}

pub fn output_color(matches: ArgMatches) -> bool {
    if matches.is_present("color") {
        true
    } else if matches.is_present("no_color") {
        false
    } else {
        atty::is(Stream::Stdout)
    }
}

fn to_entry(matches: ArgMatches) -> Result<Option<usize>, CliError> {
    match matches.value_of("to_entry") {
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

    if let Some(filename) = matches.value_of("variables_file") {
        let path = std::path::Path::new(filename);
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

    if matches.is_present("variable") {
        let input: Vec<_> = matches.values_of("variable").unwrap().collect();
        for s in input {
            let (name, value) = cli::parse_variable(s)?;
            variables.insert(name.to_string(), value);
        }
    }

    Ok(variables)
}
