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
