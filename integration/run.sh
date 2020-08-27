#!/bin/bash
set -u
set -e

for hurl_file in "$@"; do
    echo "$hurl_file";
    set +e
    hurl "$hurl_file" --color 2>/tmp/test.stderr >/tmp/test.stdout
    EXITCODE_ACTUAL=$?
    set -e

    EXITCODE_EXPECTED=$(cat "${hurl_file%.*}.exit")
    if [ "$EXITCODE_ACTUAL" != "$EXITCODE_EXPECTED" ]; then
        echo "ERROR Exit Code"
        echo "  Expected: $EXITCODE_EXPECTED"
        echo "  Actual: $EXITCODE_ACTUAL"

        # log unexpected error
        if [ "$EXITCODE_ACTUAL" != 0 ]; then
            cat /tmp/test.stderr
        fi

        exit 1
    fi

    if [ "$EXITCODE_ACTUAL" == 0 ]; then
        expected=$(cat "${hurl_file%.*}.out")
        actual=$(cat /tmp/test.stdout)
        if [ "$actual" != "$expected" ]; then
            diff  <(echo "$actual" ) <(echo "$expected")
            exit 1
        fi
    else
        STDERR_EXPECTED=$(cat "${hurl_file%.*}.err")
        STDERR_ACTUAL=$(cat /tmp/test.stderr)
        if [ "$STDERR_ACTUAL" != "$STDERR_EXPECTED" ]; then
            echo "ERROR StandardError"
            echo "  Expected: $STDERR_EXPECTED"
            echo "  Actual: $STDERR_ACTUAL"
            exit 1
    fi
    fi



done





