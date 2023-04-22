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
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;

use hurl_core::parser;
use hurlfmt::cli::options::{InputFormat, OptionsError, OutputFormat};
use hurlfmt::{cli, curl, format, linter};

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
    let opts = match cli::options::parse() {
        Ok(v) => v,
        Err(e) => match e {
            OptionsError::Info(message) => {
                eprintln!("{message}");
                process::exit(0);
            }
            OptionsError::Error(message) => {
                eprintln!("{message}");
                process::exit(1);
            }
        },
    };

    init_colored();

    let log_error_message = cli::make_logger_error_message(opts.color);
    let contents = if let Some(filename) = &opts.input_file {
        match cli::read_to_string(&filename.display().to_string()) {
            Ok(s) => s,
            Err(e) => {
                log_error_message(
                    false,
                    format!(
                        "Input file {} can not be read - {}",
                        filename.display(),
                        e.message
                    )
                    .as_str(),
                );
                process::exit(2);
            }
        }
    } else {
        let mut contents = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut contents) {
            log_error_message(
                false,
                format!("Input stream can not be read - {e}").as_str(),
            );
            process::exit(2);
        }
        contents
    };

    let input = match opts.input_format {
        InputFormat::Hurl => contents,
        InputFormat::Curl => match curl::parse(&contents) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(2);
            }
        },
    };

    let lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
        .unwrap()
        .split(&input)
        .collect();
    let lines: Vec<String> = lines.iter().map(|s| (*s).to_string()).collect();
    let log_parser_error =
        cli::make_logger_parser_error(lines.clone(), opts.color, opts.input_file.clone());
    let log_linter_error =
        cli::make_logger_linter_error(lines, opts.color, opts.input_file.clone());

    match parser::parse_hurl_file(&input) {
        Err(e) => {
            log_parser_error(&e, false);
            process::exit(2);
        }
        Ok(hurl_file) => {
            if opts.check {
                for e in linter::check_hurl_file(&hurl_file).iter() {
                    log_linter_error(e, true);
                }
                process::exit(1);
            } else {
                let output = match opts.output_format {
                    OutputFormat::Hurl => {
                        let hurl_file = linter::lint_hurl_file(&hurl_file);
                        format::format_text(hurl_file, opts.color)
                    }
                    OutputFormat::Json => format::format_json(&hurl_file),
                    OutputFormat::Html => {
                        hurl_core::format::format_html(&hurl_file, opts.standalone)
                    }
                };
                let output = if !output.ends_with('\n') {
                    format!("{output}\n")
                } else {
                    output
                };
                write_output(output.into_bytes(), opts.output_file);
            }
        }
    }
}

fn write_output(bytes: Vec<u8>, filename: Option<PathBuf>) {
    match filename {
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            handle
                .write_all(bytes.as_slice())
                .expect("writing bytes to console");
        }
        Some(path_buf) => {
            let mut file = match std::fs::File::create(&path_buf) {
                Err(why) => {
                    eprintln!("Issue writing to {}: {:?}", path_buf.display(), why);
                    std::process::exit(1);
                }
                Ok(file) => file,
            };
            file.write_all(bytes.as_slice())
                .expect("writing bytes to file");
        }
    }
}
