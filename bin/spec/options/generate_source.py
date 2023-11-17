#!/usr/bin/env python3
import sys
from option import Option
import sys

"""
Generate source file for clap
"""


COPYRIGHT = """/*
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
 */"""
from typing import *
import sys


def generate_source(options: List[Option]):
    s = COPYRIGHT
    s += "\n" + "// Generated - Do not modify"
    s += "\nuse clap::{value_parser, ArgAction};"
    s += """\n\npub fn input_files() -> clap::Arg {
    clap::Arg::new("input_files")
        .value_name("FILES")
        .help("Set the input file to use")
        .required(false)
        .index(1)
        .num_args(1..)
}"""

    for option in options:
        s += "\n\n" + generate_source_option(option)
    return s


def generate_source_option(option: Option):
    s = f"pub fn {option.name}() -> clap::Arg {{"
    s += f'\n    clap::Arg::new("{option.name}")'
    s += f'\n        .long("{option.long}")'
    if option.short is not None:
        s += f"\n        .short('{option.short}')"
    if option.value is not None:
        s += f'\n        .value_name("{option.value}")'
    if option.value_default is not None:
        s += f'\n        .default_value("{option.value_default}")'
    if option.value_parser is not None:
        s += f"\n        .value_parser({option.value_parser})"
        if "-1" in option.value_parser:
            s += f"\n        .allow_hyphen_values(true)"
    s += f'\n        .help("{option.help}")'
    if option.conflict is not None:
        s += f'\n        .conflicts_with("{option.conflict}")'
    if option.value is not None:
        s += f"\n        .num_args(1)"
    else:
        s += f"\n        .action(ArgAction::SetTrue)"
    if option.append:
        s += f"\n        .action(ArgAction::Append)"
    if option.deprecated:
        s += f"\n        .hide(true)"
    s += "\n}"
    return s


def main():
    # Parse all options file given at the command line
    if len(sys.argv) < 2:
        print("usage: generate_source.py OPTION_FILE1 OPTION_FILE2 ...")
        sys.exit(1)
    options = sorted(
        [Option.parse_file(filename) for filename in sys.argv[1:]],
        key=lambda option: option.name,
    )
    print(generate_source(options))


if __name__ == "__main__":
    main()
