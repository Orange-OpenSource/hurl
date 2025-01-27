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

mod commands;
mod matches;

use std::env;
use std::path::PathBuf;

use clap::ArgMatches;
use hurl_core::input::Input;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    pub check: bool,
    pub color: bool,
    pub in_place: bool,
    pub input_files: Vec<Input>,
    pub input_format: InputFormat,
    pub output_file: Option<PathBuf>,
    pub output_format: OutputFormat,
    pub standalone: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputFormat {
    Curl,
    Hurl,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Hurl,
    Json,
    Html,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionsError {
    Info(String),
    Error(String),
}

impl From<clap::Error> for OptionsError {
    fn from(error: clap::Error) -> Self {
        match error.kind() {
            clap::error::ErrorKind::DisplayVersion => OptionsError::Info(error.to_string()),
            clap::error::ErrorKind::DisplayHelp => OptionsError::Info(error.to_string()),
            _ => OptionsError::Error(error.to_string()),
        }
    }
}

pub fn parse() -> Result<Options, OptionsError> {
    let mut command = clap::Command::new("hurlfmt")
        .version(clap::crate_version!())
        .disable_colored_help(true)
        .about("Format Hurl files")
        .arg(commands::check())
        .arg(commands::color())
        .arg(commands::in_place())
        .arg(commands::input_files())
        .arg(commands::input_format())
        .arg(commands::no_color())
        .arg(commands::output())
        .arg(commands::output_format())
        .arg(commands::standalone());

    let arg_matches = command.try_get_matches_from_mut(env::args_os())?;
    let opts = parse_matches(&arg_matches)?;

    if opts.input_files.is_empty() {
        let help = command.render_help().to_string();
        return Err(OptionsError::Error(help));
    }
    Ok(opts)
}

fn parse_matches(arg_matches: &ArgMatches) -> Result<Options, OptionsError> {
    let check = matches::check(arg_matches);
    let color = matches::color(arg_matches);
    let in_place = matches::in_place(arg_matches)?;
    let input_files = matches::input_files(arg_matches)?;
    let input_format = matches::input_format(arg_matches)?;
    let output_file = matches::output_file(arg_matches);
    let output_format = matches::output_format(arg_matches)?;
    let standalone = matches::standalone(arg_matches)?;
    Ok(Options {
        check,
        color,
        in_place,
        input_files,
        input_format,
        output_file,
        output_format,
        standalone,
    })
}
