#!/bin/bash
set -eo pipefail

for hurl_file in "$@"; do
    html_file="${hurl_file%.*}".html
    cmd="hurlfmt --html $hurl_file"
    echo "$cmd"
    $cmd 2>/tmp/test.stderr >/tmp/test.stdout

    expected=$(cat "$html_file")
    actual=$(cat /tmp/test.stdout)
    if [ "$actual" != "$expected" ]; then
        diff  <(echo "$actual" ) <(echo "$expected")
        exit 1
    fi

done


