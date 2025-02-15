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

use hurl_core::input::{Input, InputKind};
use hurl_core::text;
use hurlfmt::cli::options::{InputFormat, OptionsError, OutputFormat};
use hurlfmt::cli::Logger;
use hurlfmt::command::check::CheckError;
use hurlfmt::command::export::ExportError;
use hurlfmt::command::format::FormatError;
use hurlfmt::{cli, command};

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

    if opts.check {
        process_check_command(&opts.input_files, opts.output_file, &logger);
    } else if opts.in_place {
        process_format_command(&opts.input_files, &logger);
    } else {
        process_export_command(
            &opts.input_files,
            opts.output_file,
            &logger,
            &opts.input_format,
            &opts.output_format,
            opts.standalone,
            opts.color,
        );
    }
}

fn process_check_command(input_files: &[Input], output_file: Option<PathBuf>, logger: &Logger) {
    let errors = command::check::run(input_files);
    if errors.is_empty() {
        process::exit(EXIT_OK);
    } else {
        let mut count = 0;
        let mut invalid_input = false;
        let mut output_all = String::new();

        for e in &errors {
            match e {
                CheckError::IO(filename) => {
                    logger.error(&format!("Input file {filename} can not be read"));
                    invalid_input = true;
                }
                CheckError::Parse {
                    content,
                    input_file,
                    error,
                } => {
                    logger.error_parsing(content, input_file, error);
                    invalid_input = true;
                }
                CheckError::Unformatted(filename) => {
                    output_all.push_str(&format!("would reformat: {}\n", filename));
                    count += 1;
                }
            }
        }
        if count > 0 {
            output_all.push_str(&format!(
                "{count} file{} would be reformatted",
                if count > 1 { "s" } else { "" }
            ));
        }
        write_output(&output_all, output_file, logger);
        if invalid_input {
            process::exit(EXIT_INVALID_INPUT);
        } else {
            process::exit(EXIT_LINT_ISSUE);
        }
    }
}

fn process_format_command(input_files: &[Input], logger: &Logger) {
    let mut input_files2 = vec![];
    for input_file in input_files {
        if let InputKind::File(path) = input_file.kind() {
            input_files2.push(path.clone());
        } else {
            logger.error("Standard input can be formatted in place!");
            process::exit(EXIT_INVALID_INPUT);
        }
    }

    let errors = command::format::run(&input_files2);
    if errors.is_empty() {
        process::exit(EXIT_OK);
    } else {
        for e in &errors {
            match e {
                FormatError::IO(filename) => {
                    logger.error(&format!("Input file {filename} can not be read"));
                }
                FormatError::Parse {
                    content,
                    input_file,
                    error,
                } => {
                    logger.error_parsing(content, input_file, error);
                }
            }
        }
        process::exit(EXIT_INVALID_INPUT);
    }
}

fn process_export_command(
    input_files: &[Input],
    output_file: Option<PathBuf>,
    logger: &Logger,
    input_format: &InputFormat,
    output_format: &OutputFormat,
    standalone: bool,
    color: bool,
) {
    let mut error = false;
    let mut output_all = String::new();
    let results = command::export::run(input_files, input_format, output_format, standalone, color);
    for result in &results {
        match result {
            Ok(output) => output_all.push_str(output),
            Err(e) => {
                error = true;
                match e {
                    ExportError::IO { filename, message } => {
                        logger.error(&format!(
                            "Input file {filename} can not be read - {message}"
                        ));
                        error = true;
                    }
                    ExportError::Parse {
                        content,
                        input_file,
                        error,
                    } => {
                        logger.error_parsing(content, input_file, error);
                    }
                    ExportError::Curl(s) => logger.error(&format!("error curl {s} d")),
                }
            }
        }
    }
    write_output(&output_all, output_file, logger);

    if error {
        process::exit(EXIT_INVALID_INPUT);
    } else {
        process::exit(EXIT_OK);
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
