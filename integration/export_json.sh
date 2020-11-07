#!/bin/bash
set -u
set -e
for hurl_file in "$@"; do
    json_file="${hurl_file%.*}.json"
    cmd="hurlfmt --json $hurl_file"
    echo "$cmd"

    $cmd 2>/tmp/test.stderr >/tmp/test.stdout
    expected=$(cat "$json_file")
    actual=$(cat /tmp/test.stdout)
    if [ "$actual" != "$expected" ]; then
        diff  <(echo "$actual" ) <(echo "$expected")
        exit 1
    fi
done


