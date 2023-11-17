#!/usr/bin/env python3
import sys
from option import Option

"""
Generate options for man
"""
from typing import *
import sys


def generate_man(options: List[Option]):
    s = ""
    for option in options:
        if not option.deprecated:
            s += generate_man_option(option)
            s += "\n\n"
    return s


def generate_man_option(option: Option):
    s = "###"
    if option.short:
        s += " -%s," % option.short
    s += " --%s" % option.long
    if option.value:
        s += " <%s>" % option.value
    s += " {#%s}" % option.long.replace(".", "")
    s += "\n\n"
    s += option.description
    return s


def main():
    # Parse all options file given at the command line
    if len(sys.argv) < 2:
        print("usage: generate_nab.py OPTION_FILE1 OPTION_FILE2 ...")
        sys.exit(1)
    options = sorted(
        [Option.parse_file(filename) for filename in sys.argv[1:]],
        key=lambda option: option.long,
    )
    print(generate_man(options))


if __name__ == "__main__":
    main()
