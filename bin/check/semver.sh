#!/bin/bash
set -Eeuo pipefail

result=/tmp/output.txt
cargo semver-checks > "${result}" 2>&1 && exit_code=0 || exit_code=$?
cat "${result}"
if [[ ${exit_code} -gt 0 ]] ; then
    if grep -i "unsupported rustdoc format" "${result}" >/dev/null 2>&1 ; then
        echo "> Allowing failure because actual rustdoc format is not supported by cargo-semver-checks for now"
        exit 0
    else
        exit "${exit_code}"
    fi
fi