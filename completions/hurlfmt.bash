# hurlfmt(1) completion                            -*- shell-script -*-
_hurlfmt()
{
    cur="${COMP_WORDS[COMP_CWORD]}"

    if [[ $cur == -* ]]; then
        COMPREPLY=($(compgen -W '--check --color --in-place --in --no-color --output --out --standalone --help --version' -- "$cur"))
        return
    fi
    # Generate filenames by default
    COMPREPLY=($(compgen -f "$cur" | sort))
} &&
    complete -F _hurlfmt hurlfmt
# ex: filetype=sh

