#!/bin/bash
# Export AST to json
set -u
set -eo pipefail
for hurl_file in "$@"; do
    json_file="${hurl_file%.*}.json"
    cmd="hurlfmt --output json $hurl_file"
    echo "$cmd"

    $cmd >/tmp/test.stdout
    expected=$(cat "$json_file")
    actual=$(cat /tmp/test.stdout)
    if [ "$actual" != "$expected" ]; then
        diff  <(echo "$actual" ) <(echo "$expected")
        exit 1
    fi
done


