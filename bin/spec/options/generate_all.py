#!/usr/bin/env python3
from typing import List
import glob
from option import Option
import generate_source


def get_option_files(dir) -> List[str]:
    return sorted(glob.glob(dir + "/*.option"))


def format_option_file(option_files: List[str]):
    for option_file in option_files:
        option = Option.parse_file(option_file)
    open(option_file, "w").write(str(option) + "\n")


def generate_source_file(option_files: List[str], output_file: str):
    options = sorted(
        [Option.parse_file(option_file) for option_file in option_files],
        key=lambda option: option.name,
    )
    src = generate_source.generate_source(options)
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


if __name__ == "__main__":
    main()
