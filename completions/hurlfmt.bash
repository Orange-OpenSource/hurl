# hurlfmt(1) completion                            -*- shell-script -*-
_hurlfmt()
{
    local cur prev words cword
    _init_completion || return

    if [[ $cur == -* ]]; then
        COMPREPLY=($(compgen -W '--check --color --in-place --in --no-color --output --out --standalone --help --version' -- "$cur"))
        return
    fi
 
} &&
    complete -F _hurlfmt hurlfmt
# ex: filetype=sh

