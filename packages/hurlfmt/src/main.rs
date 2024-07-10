/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
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
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use hurl_core::parser;
use hurlfmt::cli::options::{InputFormat, OptionsError, OutputFormat};
use hurlfmt::cli::Logger;
use hurlfmt::{cli, curl, format, linter};

const EXIT_OK: i32 = 0;
const EXIT_ERROR: i32 = 1;
const EXIT_INVALID_INPUT: i32 = 2;

/// Executes `hurlfmt` entry point.
fn main() {
    init_colored();

    let opts = match cli::options::parse() {
        Ok(v) => v,
        Err(e) => match e {
            OptionsError::Info(message) => {
                print!("{message}");
                process::exit(EXIT_OK);
            }
            OptionsError::Error(message) => {
                eprintln!("{message}");
                process::exit(EXIT_ERROR);
            }
        },
    };

    let mut logger = Logger::new(opts.color);
    let mut output_all = String::new();

    for input_file in &opts.input_files {
        match cli::read_to_string(input_file) {
            Ok(content) => {
                // parse input
                let input = match opts.input_format {
                    InputFormat::Hurl => content.to_string(),
                    InputFormat::Curl => match curl::parse(&content) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("{}", e);
                            process::exit(EXIT_INVALID_INPUT);
                        }
                    },
                };
                let input_path = Path::new(input_file).to_path_buf();
                let lines: Vec<&str> = regex::Regex::new(r"\n|\r\n")
                    .unwrap()
                    .split(&input)
                    .collect();
                let lines: Vec<String> = lines.iter().map(|s| (*s).to_string()).collect();
                let log_linter_error =
                    cli::make_logger_linter_error(lines, opts.color, Some(input_path));

                match parser::parse_hurl_file(&input) {
                    Err(e) => {
                        logger.error_parsing_rich(&content, input_file, &e);
                        process::exit(EXIT_INVALID_INPUT);
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
                            if opts.in_place {
                                let output_file = Some(Path::new(input_file).to_path_buf());
                                write_output(&output, output_file.clone());
                            } else {
                                output_all.push_str(&output);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                logger.error(&format!(
                    "Input file {} can not be read - {}",
                    input_file, e.message
                ));
                process::exit(EXIT_INVALID_INPUT);
            }
        }
    }
    if !opts.in_place {
        write_output(&output_all, opts.output_file);
    }
}

#[cfg(target_family = "unix")]
pub fn init_colored() {
    colored::control::set_override(true);
}

#[cfg(target_family = "windows")]
pub fn init_colored() {
    colored::control::set_override(true);
    colored::control::set_virtual_terminal(true).expect("set virtual terminal");
}

fn write_output(content: &str, filename: Option<PathBuf>) {
    let content = if !content.ends_with('\n') {
        format!("{content}\n")
    } else {
        content.to_string()
    };
    let bytes = content.into_bytes();
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
                    process::exit(1);
                }
                Ok(file) => file,
            };
            file.write_all(bytes.as_slice())
                .expect("writing bytes to file");
        }
    }
}
