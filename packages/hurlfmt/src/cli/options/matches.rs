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
use crate::cli::options::OutputFormat;
use atty::Stream;
use clap::ArgMatches;
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
        atty::is(Stream::Stdout)
    }
}

pub fn output_format(arg_matches: &ArgMatches) -> Result<Option<OutputFormat>, OptionsError> {
    if has_flag(arg_matches, "no_format") {
        Ok(None)
    } else {
        match get_string(arg_matches, "format").unwrap().as_str() {
            "text" => Ok(Some(OutputFormat::Hurl)),
            "json" => Ok(Some(OutputFormat::Json)),
            "html" => Ok(Some(OutputFormat::Html)),
            v => Err(OptionsError::Error(format!("Invalid output format {v}"))),
        }
    }
}

pub fn in_place(arg_matches: &ArgMatches) -> Result<bool, OptionsError> {
    if has_flag(arg_matches, "in_place") {
        if get_string(arg_matches, "format") != Some("text".to_string()) {
            Err(OptionsError::Error(
                "You can use --in-place only text format!".to_string(),
            ))
        } else if get_string(arg_matches, "input_file").is_none() {
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

pub fn input_file(arg_matches: &ArgMatches) -> Result<Option<PathBuf>, OptionsError> {
    match get_string(arg_matches, "INPUT") {
        None => Ok(None),
        Some(s) => {
            let path = Path::new(&s);
            if path.exists() {
                Ok(Some(path.to_path_buf()))
            } else {
                Err(OptionsError::Error(format!(
                    "input file {} does not exist",
                    path.display()
                )))
            }
        }
    }
}

pub fn output_file(arg_matches: &ArgMatches) -> Option<PathBuf> {
    get_string(arg_matches, "output").map(|s| Path::new(&s).to_path_buf())
}

pub fn standalone(arg_matches: &ArgMatches) -> Result<bool, OptionsError> {
    if has_flag(arg_matches, "standalone") {
        if get_string(arg_matches, "format") != Some("html".to_string()) {
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
