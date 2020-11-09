#!/bin/bash
# run hurl files
set -u
set -eo pipefail

for hurl_file in "$@"; do
    set +e

    options=("")
    if test -f "${hurl_file%.*}.options"; then
        options+=("$(cat "${hurl_file%.*}.options")")
    fi

    cmd="hurl $hurl_file ${options[*]}"
    echo "$cmd"

    $cmd 2>/tmp/test.stderr >/tmp/test.stdout
    exit_code=$?
    set -eo pipefail

    exit_code_expected=$(cat "${hurl_file%.*}.exit")
    if [ "$exit_code" != "$exit_code_expected" ]; then
        echo "ERROR Exit Code"
        echo "  Expected: $exit_code_expected"
        echo "  Actual: $exit_code"

        cat /tmp/test.stderr
	exit 1
    fi

    if test -f "${hurl_file%.*}.out"; then
        expected=$(cat "${hurl_file%.*}.out")
        actual=$(cat /tmp/test.stdout)
        if [ "$actual" != "$expected" ]; then
	    echo "Diff in standard output"
            diff  <(echo "$actual" ) <(echo "$expected")
            exit 1
        fi
    fi

    if test -f "${hurl_file%.*}.err"; then
        expected=$(cat "${hurl_file%.*}.err")
        actual=$(cat /tmp/test.stderr)
        if [ "$actual" != "$expected" ]; then
	    echo "Diff in standard error"
            diff  <(echo "$actual" ) <(echo "$expected")
            exit 1
        fi
    fi



done





