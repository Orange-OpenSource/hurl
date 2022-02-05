#!/usr/bin/env python3
import sys
import re
from datetime import date


def header(version, date):
    return '.TH hurl 1 "%s" "hurl %s" " Hurl Manual"' % (
        date.strftime("%d %b %Y"),
        version,
    )


def version():
    p = re.compile('version = "(.*)"')
    for line in open("packages/hurl/Cargo.toml", "r").readlines():
        m = p.match(line)
        if m:
            return m.group(1)
    return None


def process_code_block(s):
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
    # p.sub('\\\\f[C]\\1\\\\f[R]', s)


def convert_md(s):

    p = re.compile("^###\s+(.*)")
    s = p.sub('.IP "\\1"', s)

    p = re.compile("^##")
    s = p.sub(".SH", s)

    p = re.compile("\*\*(.*)\*\*\s+")
    s = p.sub(".B \\1\n", s)

    # Remove link Text
    p = re.compile("\[(.*)\]\(.*\)")
    s = p.sub("\\\\fI\\1\\\\fP", s)

    # Remove local anchor
    p = re.compile("{#.*}")
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
