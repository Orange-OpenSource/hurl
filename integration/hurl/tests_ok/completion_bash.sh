#!/bin/bash
set -Eeuo pipefail

# shellcheck source=/dev/null
source ../../completions/hurl.bash

test () {
    unset COMPREPLY
    export COMP_WORDS=( hurl "$@" )
    export COMP_CWORD=$(( ${#COMP_WORDS[@]} -1 ))   # last parameter
    _hurl 
    echo "${COMPREPLY[*]}"
}

test --ver
test --verb
test --verbose tests_ok/completion_b
