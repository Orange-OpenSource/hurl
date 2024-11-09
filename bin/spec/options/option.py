#!/usr/bin/env python3
from dataclasses import dataclass
from typing import Optional


@dataclass
class Option:
    name: str
    long: str
    description: str
    short: Optional[str] = None
    value: Optional[str] = None
    value_default: Optional[str] = None
    value_parser: Optional[str] = None
    help: Optional[str] = None
    help_heading: Optional[str] = None
    conflict: Optional[str] = None
    append: bool = False
    cli_only: bool = False
    deprecated: bool = False
    experimental: bool = False

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
        if self.help_heading is not None:
            s += "\nhelp_heading: " + self.help_heading
        if self.conflict is not None:
            s += "\nconflict: " + " ".join(self.conflict)
        if self.append:
            s += "\nmulti: append"
        if self.cli_only:
            s += "\ncli_only: true"
        if self.deprecated:
            s += "\ndeprecated: true"
        if self.experimental:
            s += "\nexperimental: true"
        s += "\n---"
        s += "\n" + self.description
        return s

    @staticmethod
    def parse(s) -> "Option":
        name = None
        long = None
        short = None
        value = None
        value_default = None
        value_parser = None
        help = None
        help_heading = None
        conflict = None
        append = False
        cli_only = False
        deprecated = False
        description = ""
        experimental = False
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
                    if help.endswith("."):
                        raise Exception(f"{name}: help should not end with period")
                elif key == "help_heading":
                    help_heading = v
                elif key == "conflict":
                    conflict = [a.strip() for a in v.split(" ")]
                elif key == "multi":
                    if v == "append":
                        append = True
                elif key == "cli_only":
                    if v == "true":
                        cli_only = True
                    elif v == "false":
                        cli_only = False
                    else:
                        raise Exception(
                            f"{name}: Expected true or false for cli attribute"
                        )
                elif key == "deprecated":
                    if v == "true":
                        deprecated = True
                    elif v == "false":
                        deprecated = False
                    else:
                        raise Exception(
                            f"{name}: Expected true or false for deprecated attribute"
                        )
                elif key == "experimental":
                    if v == "true":
                        experimental = True
                    elif v == "false":
                        experimental = False
                    else:
                        raise Exception(
                            f"{name}: Expected true or false for experimental attribute"
                        )
                else:
                    raise Exception(f"{name}: Invalid attribute " + key)

        if name is None:
            raise Exception("missing name attribute")

        if long is None:
            raise Exception(f"{name}: missing long attribute")

        return Option(
            name=name,
            long=long,
            short=short,
            value=value,
            value_default=value_default,
            value_parser=value_parser,
            help=help,
            help_heading=help_heading,
            conflict=conflict,
            append=append,
            cli_only=cli_only,
            deprecated=deprecated,
            experimental=experimental,
            description=description.strip(),
        )

    @staticmethod
    def parse_file(filename: str) -> "Option":
        # sys.stderr.write("Parsing " + filename + "\n")
        s = open(filename).read()
        return Option.parse(s)


def parse_key_value(s: str) -> tuple[str, str]:
    if ":" not in s:
        raise Exception("Expecting key value")
    index = s.index(":")
    key = s[:index].strip()
    value = s[index + 1 :].strip()
    return key, value
