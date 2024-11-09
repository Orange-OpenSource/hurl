#!/usr/bin/env python3
"""Build Hurl Man File.

This script creates Hurl man file from a Markdown source.

This tool takes the Hurl man Markdown source file as a first argument.

Examples:
    $ python3 bin/release/gen_manpage.py docs/manual/hurl.md > docs/manual/hurl.1
    $ python3 bin/release/gen_manpage.py docs/manual/hurlfmt.md > docs/manual/hurlfmt.1

"""

import re
import sys
from datetime import date
from typing import Optional


def header(version: str, today: date) -> str:
    today_formatted = today.strftime("%d %b %Y")
    return f'.TH hurl 1 "{today_formatted}" "hurl {version}" " Hurl Manual"'


def version() -> Optional[str]:
    p = re.compile('version = "(.*)"')
    for line in open("packages/hurl/Cargo.toml", "r").readlines():
        m = p.match(line)
        if m:
            return m.group(1)
    return None


def process_code_block(s: str) -> str:
    output = ""
    indent = False
    for line in s.split("\n"):
        if indent and line.startswith("```"):
            indent = False
        elif not indent and line.startswith("```"):
            indent = True
        else:
            if line != "":
                if indent:
                    output += "    "
                output += line
            output += "\n"

    return output


def convert_md(s) -> str:
    p = re.compile(r"^###\s+(.*)")
    s = p.sub('.IP "\\1"', s)

    p = re.compile(r"^##")
    s = p.sub(".SH", s)

    p = re.compile(r"\*\*(.*)\*\*\s+")
    s = p.sub(".B \\1\n", s)

    # Remove link Text
    p = re.compile(r"\[`?(.*?)`?\]\(.*?\)")
    s = p.sub("\\\\fI\\1\\\\fP", s)

    # Remove local anchor
    p = re.compile(r"{#.*}")
    s = p.sub("", s)
    return s


def main():
    input_file = sys.argv[1]
    data = open(input_file).readlines()
    print(header(version(), date.today()))

    s = "".join([convert_md(line) for line in data])

    s = process_code_block(s)
    print(s)


if __name__ == "__main__":
    main()
