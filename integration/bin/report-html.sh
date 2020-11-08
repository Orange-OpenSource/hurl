#!/bin/bash
# Generate a unique html report
# Each hurl file will be run successively, result is appended to the same json file
set +e
rm -rf report/*
mkdir -p report/html
find tests -name "*.hurl" | sort | while read -r hurl_file; do
    rm -f report/html/*   # TODO to be removed
    options=("--append" "--json report/tests.json" "--html report/html" "--output /dev/null")
    if test -f "${hurl_file%.*}.options"; then
        options+=("$(cat "${hurl_file%.*}.options")")
    fi
    cmd="hurl $hurl_file ${options[*]}"
    echo "$cmd"
    $cmd >/tmp/test.stdout 2>/tmp/test.stderr
    exit_code=$?

    if [ "$exit_code" -gt 4 ]; then
	echo "unexpected exit code $exit_code"
	exit 1
    fi


done
