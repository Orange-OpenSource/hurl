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


def generate_zsh_completion(name: str, options: List[Option]):
    return (
        """#compdef """
        + name
        + """

autoload -U is-at-least

_"""
        + name
        + """() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \\\n\
    """
        + """\n    """.join(
            [argument for option in options for argument in zsh_option(option)]
        )
        + """
    '--help[Print help]' \
    '--version[Print version]' \
    '*:file:_files' \\
    && ret=0
}

(( $+functions[_"""
        + name
        + """_commands] )) ||
_"""
        + name
        + """_commands() {
    local commands; commands=()
    _describe -t commands '"""
        + name
        + """ commands' commands "$@"
}

if [ "$funcstack[1]" = "_"""
        + name
        + """" ]; then
    _"""
        + name
        + """ "$@"
else
    compdef _"""
        + name
        + " "
        + name
        + """
fi"""
    )


def zsh_option(option: Option):
    arguments = []
    help = option.help.replace("[", r"\[").replace("]", r"\]")
    if option.value == "FILE":
        action = "_files"
    else:
        action = ""
    if option.append:
        cardinality = "*"
    else:
        cardinality = ""
    if option.short:
        arguments.append(f"'{cardinality}-{option.short}[{help}]::{action}' \\")
    if option.long:
        arguments.append(f"'{cardinality}--{option.long}[{help}]::{action}' \\")
    return arguments


def generate_fish_completion(name: str, options: List[Option]):
    return (
        "\n".join([fish_option(name, option) for option in options])
        + """
complete -c """
        + name
        + """ -l help -d 'Print help'
complete -c """
        + name
        + """ -l version -d 'Print version'
"""
    )


def fish_option(name: str, option: Option):
    return "complete -c " + name + " -l " + option.long + " -d '" + option.help + "'"


def generate_powershell_completion(name: str, options: List[Option]):
    return (
        """using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName '"""
        + name
        + """' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        '"""
        + name
        + """'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        '"""
        + name
        + """'
         {"""
        + "\n            ".join(
            [
                "[CompletionResult]::new('--"
                + option.long
                + "', '"
                + option.long
                + "', [CompletionResultType]::ParameterName, '"
                + option.help
                + "')"
                for option in options
            ]
        )
        + """
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')"""
        + """
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}    
    """
    )
