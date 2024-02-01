#!/usr/bin/env python3
"""
Generate Completion files
"""
from typing import *
from option import Option


def generate_bash_completion(name: str, options: List[Option]):
    available_options = ["--" + option.long for option in options] + [
        "--help",
        "--version",
    ]
    return (
        "# "
        + name
        + """(1) completion                            -*- shell-script -*-
_"""
        + name
        + """()
{
    local cur prev words cword
    _init_completion || return

    if [[ $cur == -* ]]; then
        COMPREPLY=($(compgen -W '"""
        + " ".join(available_options)
        + """' -- "$cur"))
        return
    fi
 
} &&
    complete -F _"""
        + name
        + " "
        + name
        + """
# ex: filetype=sh
"""
    )
