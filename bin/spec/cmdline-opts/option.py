#!/usr/bin/env python3
from typing import *


class Option:
    def __init__(
        self,
        name,
        long,
        short,
        value,
        value_default,
        value_parser,
        help,
        conflict,
        append,
        deprecated,
        description,
    ):
        self.name = name
        self.long = long
        self.short = short
        self.value = value
        self.value_default = value_default
        self.value_parser = value_parser
        self.help = help
        self.conflict = conflict
        self.append = append
        self.deprecated = deprecated
        self.description = description

    def __eq__(self, other):
        if not isinstance(other, Option):
            return False
        return (
            self.name == other.name
            and self.long == other.long
            and self.short == other.short
            and self.value == other.value
            and self.value_default == other.value_default
            and self.value_parser == other.value_parser
            and self.help == other.help
            and self.conflict == other.conflict
            and self.append == other.append
            and self.deprecated == other.deprecated
            and self.description == other.description
        )

    def __str__(self):
        s = "name: " + self.name
        s += "\nlong: " + self.long
        if self.short is not None:
            s += "\nshort: " + self.short
        if self.value is not None:
            s += "\nvalue: " + self.value
        if self.value_default is not None:
            s += "\nvalue_default: " + self.value_default
        if self.value_parser is not None:
            s += "\nvalue_parser: " + self.value_parser
        if self.help is not None:
            s += "\nhelp: " + self.help
        if self.conflict is not None:
            s += "\nconflict: " + self.conflict
        if self.append:
            s += "\nmulti: append"
        if self.deprecated:
            s += "\ndeprecated: true"
        s += "\n---"
        s += "\n" + self.description
        return s

    def __hash__(self):
        return hash(
            (
                self.name,
                self.long,
                self.short,
                self.value,
                self.value_default,
                self.value_parser,
                self.help,
                self.conflict,
                self.append,
                self.description,
            )
        )

    @staticmethod
    def parse(s):
        name = None
        long = None
        short = None
        value = None
        value_default = None
        value_parser = None
        help = None
        conflict = None
        append = False
        deprecated = False
        description = ""
        in_description = False

        for line in s.split("\n"):
            if line.startswith("---"):
                in_description = True
            elif in_description:
                description += line + "\n"
            else:
                key, v = parse_key_value(line)
                if key == "name":
                    name = v
                elif key == "long":
                    long = v
                elif key == "short":
                    short = v
                elif key == "value":
                    value = v
                elif key == "value_default":
                    value_default = v
                elif key == "value_parser":
                    value_parser = v
                elif key == "help":
                    help = v
                elif key == "conflict":
                    conflict = v
                elif key == "multi":
                    if v == "append":
                        append = True
                elif key == "deprecated":
                    if v == "true":
                        deprecated = True
                    elif v == "false":
                        deprecated = False
                    else:
                        raise Exception(
                            "Expected true or false for deprecated attribute"
                        )
                else:
                    raise Exception("Invalid attribute " + key)

        if name is None:
            raise Exception("missing name attribute")

        if long is None:
            raise Exception("missing long attribute")

        return Option(
            name,
            long,
            short,
            value,
            value_default,
            value_parser,
            help,
            conflict,
            append,
            deprecated,
            description.strip(),
        )

    @staticmethod
    def parse_file(filename):
        import sys

        sys.stderr.write("Parsing " + filename + "\n")
        s = open(filename).read()
        return Option.parse(s)


def parse_key_value(s) -> tuple[str, str]:
    if ":" not in s:
        raise Exception("Expecting key value")
    index = s.index(":")
    key = s[:index].strip()
    value = s[index + 1 :].strip()
    return key, value
