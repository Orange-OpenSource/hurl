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
extern crate clap;

use std::fs;
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;
use std::process;

use atty::Stream;

use hurl_core::parser;
use hurlfmt::cli;
use hurlfmt::format;
use hurlfmt::linter::Lintable;

fn main() {
    let app = clap::App::new("hurlfmt")
        // .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about("Format hurl FILE")
        .arg(
            clap::Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("check")
                .long("check")
                .conflicts_with("output")
                .help("Run in 'check' mode"),
        )
        .arg(
            clap::Arg::with_name("color")
                .long("color")
                .conflicts_with("no_color")
                .conflicts_with("in_place")
                .help("Colorize Output"),
        )
        .arg(
            clap::Arg::with_name("in_place")
                .long("in-place")
                .conflicts_with("output")
                .help("Modify file in place"),
        )
        .arg(
            clap::Arg::with_name("no_color")
                .long("no-color")
                .conflicts_with("color")
                .help("Do not colorize Output"),
        )
        .arg(
            clap::Arg::with_name("no_format")
                .long("no-format")
                .help("Do not format Output"),
        )
        .arg(
            clap::Arg::with_name("output")
                .long("output")
                .conflicts_with("check")
                .value_name("FORMAT")
                .help("Output to a specified format: text (default), json or html"),
        )
        .arg(
            clap::Arg::with_name("standalone")
                .long("standalone")
                .conflicts_with("ast_output")
                .help("Standalone Html"),
        );

    let matches = app.clone().get_matches();

    // Additional checks
    if matches.is_present("standalone") && matches.value_of("output") != Some("html") {
        eprintln!("use --standalone option only with html output");
        std::process::exit(1);
    }

    let output_color = if matches.is_present("color") {
        true
    } else if matches.is_present("no_color") {
        false
    } else {
        atty::is(Stream::Stdout)
    };

    let log_error_message = cli::make_logger_error_message(output_color);

    let filename = match matches.value_of("INPUT") {
        None => "-",
        Some("-") => "-",
        Some(v) => v,
    };

    if filename == "-" && atty::is(Stream::Stdin) {
        if app.clone().print_help().is_err() {
            panic!("panic during printing help");
        }
        println!();
        std::process::exit(1);
    } else if filename != "-" && !Path::new(filename).exists() {
        eprintln!("Input file {} does not exit!", filename);
        std::process::exit(1);
    };

    if matches.is_present("in_place") && filename == "-" {
        log_error_message(false, "You can not use inplace with standard input stream!");
        std::process::exit(1);
    };

    let contents = if filename == "-" {
        let mut contents = String::new();
        io::stdin()
            .read_to_string(&mut contents)
            .expect("Something went wrong reading standard input");
        contents
    } else {
        fs::read_to_string(filename).expect("Something went wrong reading the file")
    };

    let lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(&contents)
        .collect();

    let lines: Vec<String> = lines.iter().map(|s| (*s).to_string()).collect();
    let optional_filename = if filename == "" {
        None
    } else {
        Some(filename.to_string())
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
            if matches.is_present("check") {
                for e in hurl_file.errors() {
                    log_linter_error(&e, true);
                }
                std::process::exit(1);
            } else {
                match matches.value_of("output").unwrap_or("text") {
                    "text" => {
                        let hurl_file = if matches.is_present("no_format") {
                            hurl_file
                        } else {
                            hurl_file.lint()
                        };
                        if matches.is_present("in_place") {
                            match fs::File::create(filename) {
                                Ok(mut f) => {
                                    let s = format::format_text(hurl_file, false);
                                    f.write_all(s.as_bytes()).unwrap();
                                }
                                Err(_) => {
                                    eprintln!("Error opening file {} in write mode", filename)
                                }
                            };
                        } else {
                            print!("{}", format::format_text(hurl_file, output_color));
                        }
                    }
                    "json" => println!("{}", format::format_json(hurl_file)),
                    "html" => {
                        let standalone = matches.is_present("standalone");
                        println!("{}", format::format_html(hurl_file, standalone));
                    }
                    "ast" => println!("{:#?}", hurl_file),
                    _ => {
                        eprintln!("Invalid output option - expecting text, html or json");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}
