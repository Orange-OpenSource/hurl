#!/bin/bash
# echo hurl file
# The file is parsed and output exactly as the input
#
set -eo pipefail

for hurl_file in "$@"; do
    cmd="hurlfmt --no-format $hurl_file"
    echo "$cmd"
    $cmd 2>/tmp/test.stderr >/tmp/test.stdout

    expected=$(cat "$hurl_file")
    actual=$(cat "/tmp/test.stdout")
    if [ "$actual" != "$expected" ]; then
	echo "=> Difference!"
        diff "/tmp/test.stdout" "$hurl_file"
	exit &
    fi
done
