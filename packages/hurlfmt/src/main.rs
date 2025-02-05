/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2025 Orange
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
use std::path::PathBuf;
use std::process;

use hurl_core::input::InputKind;
use hurl_core::{parser, text};
use hurlfmt::cli::options::{InputFormat, OptionsError, OutputFormat};
use hurlfmt::cli::Logger;
use hurlfmt::{cli, curl, format, linter};

const EXIT_OK: i32 = 0;
const EXIT_ERROR: i32 = 1;
const EXIT_INVALID_INPUT: i32 = 2;
const EXIT_LINT_ISSUE: i32 = 3;

/// Executes `hurlfmt` entry point.
fn main() {
    text::init_crate_colored();

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

    let logger = Logger::new(opts.color);
    let mut output_all = String::new();

    for input_file in &opts.input_files {
        // Get content of the input
        let content = match input_file.read_to_string() {
            Ok(c) => c,
            Err(e) => {
                logger.error(&format!(
                    "Input file {} can not be read - {e}",
                    &input_file.to_string()
                ));
                process::exit(EXIT_INVALID_INPUT);
            }
        };

        // Parse input curl or Hurl file
        let input = match opts.input_format {
            InputFormat::Hurl => content.to_string(),
            InputFormat::Curl => match curl::parse(&content) {
                Ok(s) => s,
                Err(e) => {
                    logger.error(&e.to_string());
                    process::exit(EXIT_INVALID_INPUT);
                }
            },
        };

        // Parse Hurl content
        let hurl_file = match parser::parse_hurl_file(&input) {
            Ok(h) => h,
            Err(e) => {
                logger.error_parsing(&content, input_file, &e);
                process::exit(EXIT_INVALID_INPUT);
            }
        };

        // Only checks
        if opts.check {
            let lints = linter::check_hurl_file(&hurl_file);
            for e in lints.iter() {
                logger.warn_lint(&content, input_file, e);
            }
            if lints.is_empty() {
                process::exit(EXIT_OK);
            } else {
                process::exit(EXIT_LINT_ISSUE);
            }
        }

        // Output files
        let output = match opts.output_format {
            OutputFormat::Hurl => {
                let hurl_file = linter::lint_hurl_file(&hurl_file);
                format::format_text(&hurl_file, opts.color)
            }
            OutputFormat::Json => format::format_json(&hurl_file),
            OutputFormat::Html => hurl_core::format::format_html(&hurl_file, opts.standalone),
        };
        if opts.in_place {
            let InputKind::File(path) = input_file.kind() else {
                unreachable!("--in-place and standard input have been filtered in args parsing")
            };
            write_output(&output, Some(path.clone()), &logger);
        } else {
            output_all.push_str(&output);
        }
    }
    if !opts.in_place {
        write_output(&output_all, opts.output_file, &logger);
    }
}

fn write_output(content: &str, filename: Option<PathBuf>, logger: &Logger) {
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

            if let Err(why) = handle.write_all(bytes.as_slice()) {
                logger.error(&format!("Issue writing to stdout: {why}"));
                process::exit(EXIT_ERROR);
            }
        }
        Some(path_buf) => {
            let mut file = match std::fs::File::create(&path_buf) {
                Err(why) => {
                    eprintln!("Issue writing to {}: {:?}", path_buf.display(), why);
                    process::exit(EXIT_ERROR);
                }
                Ok(file) => file,
            };
            file.write_all(bytes.as_slice())
                .expect("writing bytes to file");
        }
    }
}
