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

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use atty::Stream;
use chrono::{DateTime, Local};
use clap::{AppSettings, ArgMatches};

use hurl::cli;
use hurl::core::common::FormatError;
use hurl::format;
use hurl::html;
use hurl::http;
use hurl::parser;
use hurl::runner;
use hurl::runner::core::*;
use hurl::runner::log_deserialize;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CLIOptions {
    pub verbose: bool,
    pub color: bool,
    pub fail_fast: bool,
    pub insecure: bool,
    pub variables: HashMap<String, String>,
    pub to_entry: Option<usize>,
    pub follow_location: bool,
    pub max_redirect: Option<usize>,
    pub proxy: Option<String>,
    pub no_proxy: Option<String>,
    pub cookie_input_file: Option<String>,
    pub timeout: Duration,
    pub connect_timeout: Duration,
}

fn execute(
    filename: &str,
    contents: String,
    current_dir: &Path,
    file_root: Option<String>,
    cli_options: CLIOptions,
    logger: format::logger::Logger,
) -> HurlResult {
    match parser::parse_hurl_file(contents.as_str()) {
        Err(e) => {
            let error = hurl::format::error::Error {
                source_info: e.source_info(),
                description: e.description(),
                fixme: e.fixme(),
                lines: vec![],
                filename: "".to_string(),
                warning: false,
                color: logger.color,
            };
            logger.error(&error);
            std::process::exit(2);
        }
        Ok(hurl_file) => {
            logger.verbose(format!("fail fast: {}", cli_options.fail_fast).as_str());
            logger.verbose(format!("insecure: {}", cli_options.insecure).as_str());
            logger.verbose(format!("follow redirect: {}", cli_options.follow_location).as_str());
            if let Some(n) = cli_options.max_redirect {
                logger.verbose(format!("max redirect: {}", n).as_str());
            }
            if let Some(proxy) = cli_options.proxy.clone() {
                logger.verbose(format!("proxy: {}", proxy).as_str());
            }

            if !cli_options.variables.is_empty() {
                logger.verbose("variables:");
                for (name, value) in cli_options.variables.clone() {
                    logger.verbose(format!("    {}={}", name, value).as_str());
                }
            }

            if let Some(to_entry) = cli_options.to_entry {
                if to_entry < hurl_file.entries.len() {
                    logger.verbose(
                        format!(
                            "executing {}/{} entries",
                            to_entry.to_string(),
                            hurl_file.entries.len()
                        )
                        .as_str(),
                    );
                } else {
                    logger.verbose("executing all entries");
                }
            }

            let follow_location = cli_options.follow_location;
            let verbose = cli_options.verbose;
            let insecure = cli_options.insecure;
            let max_redirect = cli_options.max_redirect;
            let proxy = cli_options.proxy;
            let no_proxy = cli_options.no_proxy;
            let cookie_input_file = cli_options.cookie_input_file;

            let timeout = cli_options.timeout;
            let connect_timeout = cli_options.connect_timeout;
            let options = http::ClientOptions {
                follow_location,
                max_redirect,
                cookie_input_file,
                proxy,
                no_proxy,
                verbose,
                insecure,
                timeout,
                connect_timeout,
            };
            let mut client = http::Client::init(options);

            let context_dir = match file_root {
                None => {
                    if filename == "-" {
                        current_dir.to_str().unwrap().to_string()
                    } else {
                        let path = Path::new(filename);
                        let parent = path.parent();
                        parent.unwrap().to_str().unwrap().to_string()
                    }
                }
                Some(filename) => filename,
            };

            let options = RunnerOptions {
                fail_fast: cli_options.fail_fast,
                variables: cli_options.variables,
                to_entry: cli_options.to_entry,
            };
            runner::file::run(
                hurl_file,
                &mut client,
                filename.to_string(),
                context_dir,
                options,
                logger,
            )
        }
    }
}

fn output_color(matches: ArgMatches) -> bool {
    if matches.is_present("color") {
        true
    } else if matches.is_present("no_color") {
        false
    } else {
        atty::is(Stream::Stdout)
    }
}

fn to_entry(matches: ArgMatches, logger: format::logger::Logger) -> Option<usize> {
    match matches.value_of("to_entry") {
        Some(value) => match value.parse() {
            Ok(v) => Some(v),
            Err(_) => {
                logger.error_message(
                    "Invalid value for option --to-entry - must be a positive integer!".to_string(),
                );
                std::process::exit(1);
            }
        },
        None => None,
    }
}

fn json_file(
    matches: ArgMatches,
    logger: format::logger::Logger,
) -> (Vec<HurlResult>, Option<std::path::PathBuf>) {
    if let Some(filename) = matches.value_of("json") {
        let path = Path::new(filename);

        let results = if matches.is_present("append") && std::path::Path::new(&path).exists() {
            logger.verbose(format!("Appending session to {}", path.display()).as_str());

            let data = fs::read_to_string(path).unwrap();
            let v: serde_json::Value = match serde_json::from_str(data.as_str()) {
                Ok(v) => v,
                Err(_) => {
                    logger.error_message(format!(
                        "The file {} is not a valid json file",
                        path.display()
                    ));
                    std::process::exit(127);
                }
            };
            match log_deserialize::parse_results(v) {
                Err(msg) => {
                    logger
                        .error_message(format!("Existing Hurl json can not be parsed! -  {}", msg));
                    std::process::exit(127);
                }
                Ok(results) => results,
            }
        } else {
            if matches.is_present("verbose") {
                logger.error_message(format!("* Writing session to {}", path.display()));
            }
            vec![]
        };
        (results, Some(path.to_path_buf()))
    } else {
        (vec![], None)
    }
}

fn html_report(matches: ArgMatches, logger: format::logger::Logger) -> Option<std::path::PathBuf> {
    if let Some(dir) = matches.value_of("html_report") {
        let path = Path::new(dir);
        if std::path::Path::new(&path).exists() {
            if !path
                .read_dir()
                .map(|mut i| i.next().is_none())
                .unwrap_or(false)
            {
                logger.error_message(format!(
                    "Html dir {} already exists and is not empty",
                    path.display()
                ));
                std::process::exit(127)
            }
            Some(path.to_path_buf())
        } else {
            match std::fs::create_dir(path) {
                Err(_) => {
                    logger.error_message(format!("Html dir {} can not be created", path.display()));
                    std::process::exit(127)
                }
                Ok(_) => Some(path.to_path_buf()),
            }
        }
    } else {
        None
    }
}

fn variables(matches: ArgMatches, logger: format::logger::Logger) -> HashMap<String, String> {
    let mut variables = HashMap::new();
    if matches.is_present("variable") {
        let input: Vec<_> = matches.values_of("variable").unwrap().collect();
        for s in input {
            match s.find('=') {
                None => {
                    logger.error_message(format!("Missing variable value for {}!", s));
                    std::process::exit(1);
                }
                Some(index) => {
                    let (name, value) = s.split_at(index);
                    if variables.contains_key(name) {
                        std::process::exit(1);
                    }
                    variables.insert(name.to_string(), value[1..].to_string());
                }
            };
        }
    }
    variables
}

fn app() -> clap::App<'static, 'static> {
    clap::App::new("hurl")
        //.author(clap::crate_authors!())
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
            clap::Arg::with_name("append")
                .long("append")
                .help("Append sessions to json output"),
        )
        .arg(
            clap::Arg::with_name("color")
                .long("color")
                .conflicts_with("no-color")
                .help("Colorize Output"),
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
            clap::Arg::with_name("html_report")
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
                .help("Allow insecure SSl connections"),
        )
        .arg(
            clap::Arg::with_name("json")
                .long("json")
                .value_name("FILE")
                .help("Write full session(s) to json file")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("follow_location")
                .short("L")
                .long("location")
                .help("Follow redirects"),
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
            clap::Arg::with_name("proxy")
                .short("x")
                .long("proxy")
                .value_name("[PROTOCOL://]HOST[:PORT]")
                .help("Use proxy on given protocol/host/port"),
        )
        .arg(
            clap::Arg::with_name("to_entry")
                .long("to-entry")
                .value_name("ENTRY_NUMBER")
                .help("Execute hurl file to ENTRY_NUMBER (starting at 1)")
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
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Turn on verbose output"),
        )
}

pub fn unwrap_or_exit<T>(result: Result<T, cli::Error>, logger: format::logger::Logger) -> T {
    match result {
        Ok(v) => v,
        Err(e) => {
            logger.error_message(e.message);
            std::process::exit(127);
        }
    }
}

fn parse_options(
    matches: ArgMatches,
    logger: format::logger::Logger,
) -> Result<CLIOptions, cli::Error> {
    let verbose = matches.is_present("verbose");
    let color = output_color(matches.clone());
    let fail_fast = !matches.is_present("fail_at_end");
    let variables = variables(matches.clone(), logger.clone());
    let to_entry = to_entry(matches.clone(), logger);
    let proxy = matches.value_of("proxy").map(|x| x.to_string());
    let no_proxy = matches.value_of("proxy").map(|x| x.to_string());
    let insecure = matches.is_present("insecure");
    let follow_location = matches.is_present("follow_location");
    let cookie_input_file = matches.value_of("cookie_input_file").map(|x| x.to_string());
    let max_redirect = match matches.value_of("max_redirects") {
        None => Some(50),
        Some("-1") => None,
        Some(s) => match s.parse::<usize>() {
            Ok(x) => Some(x),
            Err(_) => {
                return Err(cli::Error {
                    message: "max_redirs option can not be parsed".to_string(),
                });
            }
        },
    };

    let timeout = match matches.value_of("max_time") {
        None => Duration::from_secs(0),
        Some(s) => match s.parse::<u64>() {
            Ok(n) => Duration::from_secs(n),
            Err(_) => {
                return Err(cli::Error {
                    message: "max_time option can not be parsed".to_string(),
                });
            }
        },
    };

    let connect_timeout = match matches.value_of("connect_timeout") {
        None => Duration::from_secs(300),
        Some(s) => match s.parse::<u64>() {
            Ok(n) => Duration::from_secs(n),
            Err(_) => {
                return Err(cli::Error {
                    message: "connect-timeout option can not be parsed".to_string(),
                });
            }
        },
    };

    Ok(CLIOptions {
        verbose,
        color,
        fail_fast,
        insecure,
        variables,
        to_entry,
        follow_location,
        max_redirect,
        proxy,
        no_proxy,
        cookie_input_file,
        timeout,
        connect_timeout,
    })
}

fn main() -> Result<(), cli::Error> {
    let app = app();
    let matches = app.clone().get_matches();

    let mut filenames = match matches.values_of("INPUT") {
        None => vec![],
        Some(v) => v.collect(),
    };

    if filenames.is_empty() && atty::is(Stream::Stdin) {
        if app.clone().print_help().is_err() {
            std::process::exit(1);
        } else {
        }
    } else if filenames.is_empty() {
        filenames.push("-");
    }

    let current_dir_buf = std::env::current_dir().unwrap();
    let current_dir = current_dir_buf.as_path();

    let file_root = match matches.value_of("file_root") {
        Some(value) => Some(value.to_string()),
        _ => None,
    };

    let logger = format::logger::Logger {
        filename: None,
        lines: vec![],
        verbose: matches.is_present("verbose"),
        color: output_color(matches.clone()),
    };

    let (mut hurl_results, json_file) = json_file(matches.clone(), logger.clone());
    let html_report = html_report(matches.clone(), logger.clone());
    let cookies_output_file = match matches.value_of("cookies_output_file") {
        None => None,
        Some(filename) => {
            let filename = unwrap_or_exit(
                cli::options::cookies_output_file(filename.to_string(), filenames.len()),
                logger.clone(),
            );
            Some(filename)
        }
    };

    let cli_options = unwrap_or_exit(
        parse_options(matches.clone(), logger.clone()),
        logger.clone(),
    );

    for filename in filenames {
        let contents = if filename == "-" {
            let mut contents = String::new();
            io::stdin()
                .read_to_string(&mut contents)
                .expect("Something went wrong reading standard input");
            contents
        } else {
            if !Path::new(filename).exists() {
                logger.error_message(format!("Input file {} does not exit!", filename));
                std::process::exit(1);
            }
            fs::read_to_string(filename).expect("Something went wrong reading the file")
        };
        let lines: Vec<String> = regex::Regex::new(r"\n|\r\n")
            .unwrap()
            .split(&contents)
            .map(|l| l.to_string())
            .collect();
        let logger = format::logger::Logger {
            filename: Some(filename.to_string()),
            lines: lines.clone(),
            verbose: cli_options.verbose,
            color: cli_options.color,
        };

        let hurl_result = execute(
            filename,
            contents,
            current_dir,
            file_root.clone(),
            cli_options.clone(),
            logger.clone(),
        );

        if hurl_result.errors().is_empty() {
            // default
            // last entry + response + body
            if let Some(entry_result) = hurl_result.entries.last() {
                if let Some(response) = entry_result.response.clone() {
                    if matches.is_present("include") {
                        logger.info(
                            format!(
                                "HTTP/{} {}",
                                response.version.to_string(),
                                response.status.to_string()
                            )
                            .as_str(),
                        );
                        for header in response.headers.clone() {
                            logger.info(format!("{}: {}", header.name, header.value).as_str());
                        }
                        logger.info("");
                    }

                    write_output(response.body, matches.value_of("output"), logger.clone());
                } else {
                    logger.warning_message("no response has been received".to_string());
                }
            } else {
                logger.warning_message(format!(
                    "warning: no entry have been executed {}",
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!("for file {}", filename)
                    }
                ));
            };
        }

        hurl_results.push(hurl_result.clone());
    }

    if let Some(file_path) = json_file {
        let mut file = match std::fs::File::create(&file_path) {
            Err(why) => {
                logger.error_message(format!(
                    "Issue writing to {}: {:?}",
                    file_path.display(),
                    why
                ));
                std::process::exit(127)
            }
            Ok(file) => file,
        };
        let serialized = serde_json::to_string_pretty(&hurl_results).unwrap();
        if let Err(why) = file.write_all(serialized.as_bytes()) {
            logger.error_message(format!(
                "Issue writing to {}: {:?}",
                file_path.display(),
                why
            ));
            std::process::exit(127)
        }
    }

    if let Some(dir_path) = html_report {
        logger.verbose(format!("Writing html report to {}", dir_path.display()).as_str());
        write_html_report(dir_path, hurl_results.clone(), logger.clone());
    }

    if let Some(file_path) = cookies_output_file {
        logger.verbose(format!("Writing cookies to {}", file_path.display()).as_str());
        write_cookies_file(file_path, hurl_results.clone(), logger);
    }

    std::process::exit(exit_code(hurl_results));
}

fn exit_code(hurl_results: Vec<HurlResult>) -> i32 {
    let mut count_errors_runner = 0;
    let mut count_errors_assert = 0;
    for hurl_result in hurl_results.clone() {
        let runner_errors: Vec<runner::core::Error> = hurl_result
            .clone()
            .errors()
            .iter()
            .filter(|e| !e.assert)
            .cloned()
            .collect();

        if hurl_result.clone().errors().is_empty() {
        } else if runner_errors.is_empty() {
            count_errors_assert += 1;
        } else {
            count_errors_runner += 1;
        }
    }
    if count_errors_runner > 0 {
        3
    } else if count_errors_assert > 0 {
        4
    } else {
        0
    }
}

fn write_output(bytes: Vec<u8>, filename: Option<&str>, logger: format::logger::Logger) {
    match filename {
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            handle
                .write_all(bytes.as_slice())
                .expect("writing bytes to console");
        }
        Some(filename) => {
            let path = Path::new(filename);
            let mut file = match std::fs::File::create(&path) {
                Err(why) => {
                    logger.error_message(format!("Issue writing to {}: {:?}", path.display(), why));
                    std::process::exit(127)
                }
                Ok(file) => file,
            };
            file.write_all(bytes.as_slice())
                .expect("writing bytes to file");
        }
    }
}

fn write_cookies_file(
    file_path: PathBuf,
    hurl_results: Vec<HurlResult>,
    logger: format::logger::Logger,
) {
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            logger.error_message(format!(
                "Issue writing to {}: {:?}",
                file_path.display(),
                why
            ));
            std::process::exit(127)
        }
        Ok(file) => file,
    };
    let mut s = r#"# Netscape HTTP Cookie File
# This file was generated by hurl

"#
    .to_string();
    match hurl_results.first() {
        None => {
            logger.error_message("Issue fetching results".to_string());
            std::process::exit(127)
        }
        Some(result) => {
            for cookie in result.cookies.clone() {
                s.push_str(cookie.to_string().as_str());
                s.push('\n');
            }
        }
    }

    if let Err(why) = file.write_all(s.as_bytes()) {
        logger.error_message(format!(
            "Issue writing to {}: {:?}",
            file_path.display(),
            why
        ));
        std::process::exit(127)
    }
}

fn write_html_report(
    dir_path: PathBuf,
    hurl_results: Vec<HurlResult>,
    logger: format::logger::Logger,
) {
    //let now: DateTime<Utc> = Utc::now();
    let now: DateTime<Local> = Local::now();
    let html = create_html_index(now.to_rfc2822(), hurl_results);
    let s = html.render();

    let file_path = dir_path.join("index.html");
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            logger.error_message(format!(
                "Issue writing to {}: {:?}",
                file_path.display(),
                why
            ));
            std::process::exit(127)
        }
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(s.as_bytes()) {
        logger.error_message(format!(
            "Issue writing to {}: {:?}",
            file_path.display(),
            why
        ));
        std::process::exit(127)
    }

    let file_path = dir_path.join("report.css");
    let mut file = match std::fs::File::create(&file_path) {
        Err(why) => {
            logger.error_message(format!(
                "Issue writing to {}: {:?}",
                file_path.display(),
                why
            ));
            std::process::exit(127)
        }
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(include_bytes!("report.css")) {
        logger.error_message(format!(
            "Issue writing to {}: {:?}",
            file_path.display(),
            why
        ));
        std::process::exit(127)
    }
}

fn create_html_index(now: String, hurl_results: Vec<HurlResult>) -> html::ast::Html {
    let head = html::ast::Head {
        title: "Hurl Report".to_string(),
        stylesheet: Some("report.css".to_string()),
    };

    let body = html::ast::Body {
        children: vec![
            html::ast::Element::NodeElement {
                name: "h2".to_string(),
                attributes: vec![],
                children: vec![html::ast::Element::TextElement("Hurl Report".to_string())],
            },
            html::ast::Element::NodeElement {
                name: "div".to_string(),
                attributes: vec![html::ast::Attribute::Class("date".to_string())],
                children: vec![html::ast::Element::TextElement(now)],
            },
            html::ast::Element::NodeElement {
                name: "table".to_string(),
                attributes: vec![],
                children: vec![
                    create_html_table_header(),
                    create_html_table_body(hurl_results),
                ],
            },
        ],
    };
    html::ast::Html { head, body }
}

fn create_html_table_header() -> html::ast::Element {
    html::ast::Element::NodeElement {
        name: "thead".to_string(),
        attributes: vec![],
        children: vec![html::ast::Element::NodeElement {
            name: "tr".to_string(),
            attributes: vec![],
            children: vec![
                html::ast::Element::NodeElement {
                    name: "td".to_string(),
                    attributes: vec![],
                    children: vec![html::ast::Element::TextElement("filename".to_string())],
                },
                html::ast::Element::NodeElement {
                    name: "td".to_string(),
                    attributes: vec![],
                    children: vec![html::ast::Element::TextElement("duration".to_string())],
                },
            ],
        }],
    }
}

fn create_html_table_body(hurl_results: Vec<HurlResult>) -> html::ast::Element {
    let children = hurl_results
        .iter()
        .map(|result| create_html_result(result.clone()))
        .collect();

    html::ast::Element::NodeElement {
        name: "tbody".to_string(),
        attributes: vec![],
        children,
    }
}

fn create_html_result(result: HurlResult) -> html::ast::Element {
    let status = if result.success {
        "success".to_string()
    } else {
        "failure".to_string()
    };
    html::ast::Element::NodeElement {
        name: "tr".to_string(),
        attributes: vec![],
        children: vec![
            html::ast::Element::NodeElement {
                name: "td".to_string(),
                attributes: vec![html::ast::Attribute::Class(status)],
                children: vec![html::ast::Element::TextElement(result.filename.clone())],
            },
            html::ast::Element::NodeElement {
                name: "td".to_string(),
                attributes: vec![],
                children: vec![html::ast::Element::TextElement(format!(
                    "{}s",
                    result.time_in_ms as f64 / 1000.0
                ))],
            },
        ],
    }
}
