#!/usr/bin/env python3
"""Generate options for man pages."""

import sys
from functools import cmp_to_key
from typing import List, Optional

from option import Option, OptionGroup


def generate_man(filenames: List[str]) -> str:
    """Parse option files and return a man page string with all options grouped and sorted.

    Args:
        filenames: List of option file paths to parse.

    Returns:
        A man page formatted string with all options.
    """
    options = [Option.parse_file(filename) for filename in filenames]

    # Group options
    groups = {}
    for option in options:
        name = option.help_heading
        group = groups.get(option.help_heading, OptionGroup(name=name, options=[]))
        groups[name] = group
        group.options.append(option)

    # Sort groups
    sorted_names = sorted(groups.keys(), key=cmp_to_key(cmp_group))
    sorted_groups = {k: groups[k] for k in sorted_names}

    # Sort options in group and display group
    man = ""
    for name, group in sorted_groups.items():
        group.options = sorted(group.options, key=lambda o: o.long)
        man += generate_man_group(group)
    return man


def generate_man_group(option_group: OptionGroup) -> str:
    """Return a man page string for a single option group.

    Args:
        option_group: The option group to render.

    Returns:
        A man page formatted string for the group, excluding deprecated and experimental options.
    """
    s = ""
    if option_group.name:
        s += f"### {option_group.name}"
        s += "\n\n"
    for option in option_group.options:
        if not option.deprecated and not option.experimental:
            s += generate_man_option(option)
            s += "\n\n"
    return s


def generate_man_option(option: Option) -> str:
    """Return a man page string for a single option.

    Args:
        option: The option to render.

    Returns:
        A man page formatted string for the option, including its name, value placeholder,
        description, environment variables, and cli-only notice if applicable.
    """
    s = "####"
    if option.short:
        s += " -%s," % option.short
    s += " --%s" % option.long
    if option.value:
        s += " <%s>" % option.value
    s += " {#%s}" % option.long.replace(".", "")
    s += "\n\n"
    s += option.description
    if len(option.env_vars) > 0:
        s += "\n\n"
        s += "Environment variables: " + ", ".join(option.env_vars)
    if option.cli_only:
        s += "\n\n"
        s += "This is a cli-only option."
    return s


def cmp_group(a: Optional[str], b: Optional[str]) -> int:
    """Compare two group of options"""

    def get(elems, value, default):
        try:
            return elems.index(value)
        except ValueError:
            return default

    all_groups = [
        None,
        "HTTP options",
        "Output options",
        "Run options",
        "Report options",
        "Other options",
    ]
    index_a = get(all_groups, a, 0)
    index_b = get(all_groups, b, 0)
    if index_a > index_b:
        return 1
    elif index_a < index_b:
        return -1
    else:
        return 0


def main():
    """Entry point: parse option files from command-line arguments and print the man page."""
    # Parse all options file given at the command line
    if len(sys.argv) < 2:
        print("usage: generate_man.py OPTION_FILE1 OPTION_FILE2 ...")
        sys.exit(1)
    s = generate_man(filenames=sys.argv[1:])
    print(s)


if __name__ == "__main__":
    main()
