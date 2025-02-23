#!/bin/bash
set -Eeuo pipefail

# shellcheck source=/dev/null
source ../../completions/hurl.bash

export COMP_WORDS=(curl --ver)
export COMP_CWORD=1
_hurl 
echo "${COMPREPLY[*]}"

export COMP_WORDS=(curl --verb)
export COMP_CWORD=1
_hurl 
echo "${COMPREPLY[*]}"
