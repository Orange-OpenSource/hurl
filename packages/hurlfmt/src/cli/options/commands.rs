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
use clap::ArgAction;

pub fn check() -> clap::Arg {
    clap::Arg::new("check")
        .long("check")
        .help("Run in 'check' mode")
        .action(ArgAction::SetTrue)
        .conflicts_with("format")
        .conflicts_with("output")
}

pub fn color() -> clap::Arg {
    clap::Arg::new("color")
        .long("color")
        .help("Colorize Output")
        .action(ArgAction::SetTrue)
        .conflicts_with("no_color")
        .conflicts_with("in_place")
}

pub fn format() -> clap::Arg {
    clap::Arg::new("format")
        .long("format")
        .value_name("FORMAT")
        .help("Specify output format: hurl, json or html (DEPRECATED)")
        .conflicts_with("check")
        .default_value("hurl")
        .num_args(1)
}

pub fn in_place() -> clap::Arg {
    clap::Arg::new("in_place")
        .long("in-place")
        .help("Modify file in place")
        .action(ArgAction::SetTrue)
        .conflicts_with("output")
        .conflicts_with("color")
}

pub fn input_files() -> clap::Arg {
    clap::Arg::new("input_files")
        .help("Sets the input file to use")
        .required(false)
        .index(1)
        .num_args(1..)
}

pub fn input_format() -> clap::Arg {
    clap::Arg::new("input_format")
        .long("in")
        .value_name("FORMAT")
        .help("Specify input format: hurl or curl")
        .conflicts_with("check")
        .default_value("hurl")
        .num_args(1)
}
pub fn no_color() -> clap::Arg {
    clap::Arg::new("no_color")
        .long("no-color")
        .help("Do not colorize output")
        .action(ArgAction::SetTrue)
        .conflicts_with("color")
}

pub fn output() -> clap::Arg {
    clap::Arg::new("output")
        .short('o')
        .long("output")
        .value_name("FILE")
        .help("Write to FILE instead of stdout")
        .num_args(1)
}

pub fn output_format() -> clap::Arg {
    clap::Arg::new("output_format")
        .long("out")
        .value_name("FORMAT")
        .help("Specify output format: hurl, json or html")
        .conflicts_with("check")
        .default_value("hurl")
        .num_args(1)
}

pub fn standalone() -> clap::Arg {
    clap::Arg::new("standalone")
        .long("standalone")
        .help("Standalone Html")
        .action(ArgAction::SetTrue)
}
