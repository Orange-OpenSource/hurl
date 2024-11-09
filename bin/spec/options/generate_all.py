#!/usr/bin/env python3
import glob
import re
import sys
from typing import List

import generate_completion
import generate_man
import generate_source
from option import Option


def get_option_files(dir) -> List[str]:
    return sorted(glob.glob(dir + "/*.option"))


def format_option_file(option_files: List[str]):
    for option_file in option_files:
        option = Option.parse_file(option_file)
        sys.stderr.write("Format " + option_file + "\n")
        open(option_file, "w").write(str(option) + "\n")


def generate_source_file(option_files: List[str], output_file: str):
    options = sorted(
        [Option.parse_file(option_file) for option_file in option_files],
        key=lambda option: option.name,
    )
    src = generate_source.generate_source(options)
    sys.stderr.write("Generate " + output_file + "\n")
    open(output_file, "w").write(src + "\n")


def update_man(option_files: List[str], output_file):
    sys.stderr.write("Update " + output_file + "\n")
    options = sorted(
        [Option.parse_file(option_file) for option_file in option_files],
        key=lambda option: option.long,
    )
    current_man = open(output_file).read()
    result = re.search(
        r"## OPTIONS[^#]*(###.*)### -h, --help", current_man, re.MULTILINE | re.DOTALL
    )
    if result is None:
        raise Exception("Options can not been found in current man " + output_file)

    existing_options_str = result.group(1)
    new_options_str = generate_man.generate_man(options)
    new_man = current_man.replace(existing_options_str, new_options_str)
    open(output_file, "w").write(new_man)


def generate_completion_files(name: str, option_files: List[str]):
    options = sorted(
        [Option.parse_file(option_file) for option_file in option_files],
        key=lambda option: option.name,
    )

    output_file = "completions/" + name + ".bash"
    src = generate_completion.generate_bash_completion(name, options)
    sys.stderr.write("Generate " + output_file + "\n")
    open(output_file, "w").write(src + "\n")

    output_file = "completions/_" + name
    src = generate_completion.generate_zsh_completion(name, options)
    sys.stderr.write("Generate " + output_file + "\n")
    open(output_file, "w").write(src + "\n")

    output_file = "completions/" + name + ".fish"
    src = generate_completion.generate_fish_completion(name, options)
    sys.stderr.write("Generate " + output_file + "\n")
    open(output_file, "w").write(src + "\n")

    output_file = "completions/_" + name + ".ps1"
    src = generate_completion.generate_powershell_completion(name, options)
    sys.stderr.write("Generate " + output_file + "\n")
    open(output_file, "w").write(src + "\n")


def main():
    option_files_hurl = get_option_files("docs/spec/options/hurl")
    option_files_hurlfmt = get_option_files("docs/spec/options/hurlfmt")

    # Format option files
    format_option_file(option_files_hurl)
    format_option_file(option_files_hurlfmt)

    # Generate Source files
    generate_source_file(option_files_hurl, "packages/hurl/src/cli/options/commands.rs")
    generate_source_file(
        option_files_hurlfmt, "packages/hurlfmt/src/cli/options/commands.rs"
    )

    # Update Man
    update_man(option_files_hurl, "docs/manual/hurl.md")
    update_man(option_files_hurlfmt, "docs/manual/hurlfmt.md")

    # Generate completion files
    generate_completion_files("hurl", option_files_hurl)
    generate_completion_files("hurlfmt", option_files_hurlfmt)


if __name__ == "__main__":
    main()
