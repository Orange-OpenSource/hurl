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
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;
use std::process;

use atty::Stream;

use hurl_core::parser;
use hurlfmt::cli;
use hurlfmt::format;
use hurlfmt::linter::Lintable;

#[cfg(target_family = "unix")]
pub fn init_colored() {
    colored::control::set_override(true);
}

#[cfg(target_family = "windows")]
pub fn init_colored() {
    colored::control::set_override(true);
    colored::control::set_virtual_terminal(true).expect("set virtual terminal");
}

fn main() {
    let app = clap::Command::new("hurlfmt")
        .version(clap::crate_version!())
        .disable_colored_help(true)
        .about("Format hurl FILE")
        .arg(
            clap::Arg::new("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        )
        .arg(
            clap::Arg::new("check")
                .long("check")
                .conflicts_with("format")
                .conflicts_with("output")
                .help("Run in 'check' mode"),
        )
        .arg(
            clap::Arg::new("color")
                .long("color")
                .conflicts_with("no_color")
                .conflicts_with("in_place")
                .help("Colorize Output"),
        )
        .arg(
            clap::Arg::new("format")
                .long("format")
                .conflicts_with("check")
                .value_name("FORMAT")
                .default_value("text")
                .help("Specify output format: text (default), json or html"),
        )
        .arg(
            clap::Arg::new("in_place")
                .long("in-place")
                .conflicts_with("output")
                .conflicts_with("color")
                .help("Modify file in place"),
        )
        .arg(
            clap::Arg::new("no_color")
                .long("no-color")
                .conflicts_with("color")
                .help("Do not colorize output"),
        )
        .arg(
            clap::Arg::new("no_format")
                .long("no-format")
                .help("Do not format output"),
        )
        .arg(
            clap::Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Write to FILE instead of stdout"),
        )
        .arg(
            clap::Arg::new("standalone")
                .long("standalone")
                .help("Standalone Html"),
        );

    let matches = app.clone().get_matches();
    init_colored();

    // Additional checks
    if cli::has_flag(&matches, "standalone")
        && cli::get_string(&matches, "format") != Some("html".to_string())
    {
        eprintln!("use --standalone option only with html output");
        process::exit(1);
    }

    let output_color = if cli::has_flag(&matches, "color") {
        true
    } else if cli::has_flag(&matches, "no_color") || cli::has_flag(&matches, "in_place") {
        false
    } else {
        atty::is(Stream::Stdout)
    };

    let log_error_message = cli::make_logger_error_message(output_color);

    let filename = match cli::get_string(&matches, "INPUT") {
        None => "-".to_string(),
        Some(v) => v,
    };

    if filename == "-" && atty::is(Stream::Stdin) {
        if app.clone().print_help().is_err() {
            panic!("panic during printing help");
        }
        println!();
        process::exit(1);
    } else if filename != "-" && !Path::new(&filename).exists() {
        eprintln!("Input file {} does not exit!", filename);
        process::exit(1);
    };

    if cli::has_flag(&matches, "in_place") {
        if filename == "-" {
            log_error_message(
                true,
                "You can not use --in-place with standard input stream!",
            );
            process::exit(1);
        };
        if cli::get_string(&matches, "format") != Some("text".to_string()) {
            log_error_message(true, "You can use --in-place only text format!");
            process::exit(1);
        };
    }

    let contents = if filename == "-" {
        let mut contents = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut contents) {
            log_error_message(
                false,
                format!("Input stream can not be read - {}", e).as_str(),
            );
            process::exit(2);
        }
        contents
    } else {
        match cli::read_to_string(&filename) {
            Ok(s) => s,
            Err(e) => {
                log_error_message(
                    false,
                    format!("Input stream can not be read - {}", e.message).as_str(),
                );
                process::exit(2);
            }
        }
    };

    let lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(&contents)
        .collect();

    let lines: Vec<String> = lines.iter().map(|s| (*s).to_string()).collect();
    let optional_filename = if filename.is_empty() {
        None
    } else {
        Some(filename.to_string())
    };

    let output_file = if cli::has_flag(&matches, "in_place") {
        Some(filename)
    } else {
        cli::get_string(&matches, "output")
    };

    let log_parser_error =
        cli::make_logger_parser_error(lines.clone(), output_color, optional_filename.clone());
    let log_linter_error = cli::make_logger_linter_error(lines, output_color, optional_filename);
    match parser::parse_hurl_file(contents.as_str()) {
        Err(e) => {
            log_parser_error(&e, false);
            process::exit(2);
        }
        Ok(hurl_file) => {
            if cli::has_flag(&matches, "check") {
                for e in hurl_file.errors() {
                    log_linter_error(&e, true);
                }
                process::exit(1);
            } else {
                let output = match cli::get_string(&matches, "format").unwrap().as_str() {
                    "text" => {
                        let hurl_file = if cli::has_flag(&matches, "no_format") {
                            hurl_file
                        } else {
                            hurl_file.lint()
                        };
                        format::format_text(hurl_file, output_color)
                    }
                    "json" => format::format_json(hurl_file),
                    "html" => {
                        let standalone = cli::has_flag(&matches, "standalone");
                        hurl_core::format::format_html(hurl_file, standalone)
                    }
                    "ast" => format!("{:#?}", hurl_file),
                    _ => {
                        eprintln!("Invalid output option - expecting text, html or json");
                        process::exit(1);
                    }
                };
                write_output(output.into_bytes(), output_file.as_deref());
            }
        }
    }
}

fn write_output(bytes: Vec<u8>, filename: Option<&str>) {
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
                    eprintln!("Issue writing to {}: {:?}", path.display(), why);
                    std::process::exit(1);
                }
                Ok(file) => file,
            };
            file.write_all(bytes.as_slice())
                .expect("writing bytes to file");
        }
    }
}
