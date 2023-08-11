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
use super::OptionsError;
use crate::cli::options::{InputFormat, OutputFormat};
use clap::parser::ValueSource;
use clap::ArgMatches;
use std::io;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

pub fn check(arg_matches: &ArgMatches) -> bool {
    has_flag(arg_matches, "check")
}

pub fn color(arg_matches: &ArgMatches) -> bool {
    if has_flag(arg_matches, "color") {
        true
    } else if has_flag(arg_matches, "no_color") || has_flag(arg_matches, "in_place") {
        false
    } else {
        io::stdout().is_terminal()
    }
}

pub fn input_format(arg_matches: &ArgMatches) -> Result<InputFormat, OptionsError> {
    match get_string(arg_matches, "input_format").unwrap().as_str() {
        "hurl" => Ok(InputFormat::Hurl),
        "curl" => Ok(InputFormat::Curl),
        v => Err(OptionsError::Error(format!("Invalid input format {v}"))),
    }
}

pub fn output_format(arg_matches: &ArgMatches) -> Result<OutputFormat, OptionsError> {
    // Deprecated --format option
    if arg_matches.value_source("format") == Some(ValueSource::CommandLine) {
        eprintln!("--format is deprecated. use --out instead.");
        return match get_string(arg_matches, "format").unwrap().as_str() {
            "hurl" => Ok(OutputFormat::Hurl),
            "json" => Ok(OutputFormat::Json),
            "html" => Ok(OutputFormat::Html),
            v => Err(OptionsError::Error(format!("Invalid output format {v}"))),
        };
    }

    match get_string(arg_matches, "output_format").unwrap().as_str() {
        "hurl" => Ok(OutputFormat::Hurl),
        "json" => Ok(OutputFormat::Json),
        "html" => Ok(OutputFormat::Html),
        v => Err(OptionsError::Error(format!("Invalid output format {v}"))),
    }
}

pub fn in_place(arg_matches: &ArgMatches) -> Result<bool, OptionsError> {
    if has_flag(arg_matches, "in_place") {
        if get_string(arg_matches, "format") != Some("hurl".to_string()) {
            Err(OptionsError::Error(
                "You can use --in-place only hurl format!".to_string(),
            ))
        } else if get_string(arg_matches, "input_files").is_none() {
            Err(OptionsError::Error(
                "You can not use --in-place with standard input stream!".to_string(),
            ))
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

/// Returns the input files from the positional arguments and input stream
pub fn input_files(arg_matches: &ArgMatches) -> Result<Vec<String>, OptionsError> {
    let mut files = vec![];
    if let Some(filenames) = get_strings(arg_matches, "input_files") {
        for filename in filenames {
            let path = Path::new(&filename);
            if path.exists() {
                files.push(filename);
            } else {
                return Err(OptionsError::Error(format!(
                    "hurl: cannot access '{}': No such file or directory",
                    path.display()
                )));
            }
        }
    }
    if files.is_empty() && !io::stdin().is_terminal() {
        files.push("-".to_string());
    }
    Ok(files)
}

pub fn output_file(arg_matches: &ArgMatches) -> Option<PathBuf> {
    get_string(arg_matches, "output").map(|s| Path::new(&s).to_path_buf())
}

pub fn standalone(arg_matches: &ArgMatches) -> Result<bool, OptionsError> {
    if has_flag(arg_matches, "standalone") {
        if get_string(arg_matches, "output_format") != Some("html".to_string()) {
            Err(OptionsError::Error(
                "use --standalone option only with html output".to_string(),
            ))
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

fn has_flag(matches: &ArgMatches, name: &str) -> bool {
    matches.get_one::<bool>(name) == Some(&true)
}

pub fn get_string(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).map(|x| x.to_string())
}

/// Returns an optional list of `String` from the command line `matches` given the option `name`.
pub fn get_strings(matches: &ArgMatches, name: &str) -> Option<Vec<String>> {
    matches
        .get_many::<String>(name)
        .map(|v| v.map(|x| x.to_string()).collect())
}
