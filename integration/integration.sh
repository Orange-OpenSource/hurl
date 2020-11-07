#!/bin/bash
set -e
cd "$(dirname "$0")"

# Static Analysis
./hurl_echo tests/*.hurl tests_error_lint/*.hurl
./export_json.sh tests/*.hurl

./lint.sh tests_error_lint/*.hurl
./generate_html

# Dynamic
./run.sh tests/*.hurl tests_error_parser/*.hurl
#./hurl_output.sh output/*.command

# Generate a unique html report
# Each hurl file will be run successively, result is appended to the same json file
set +e
rm -rf report/*
find tests -name "*.hurl" | sort | while read -r hurl_file; do
    rm -f report/html/*
    options=("--append" "--json report/tests.json" "--html report/html" "--output /dev/null")
    if test -f "${hurl_file%.*}.options"; then
        options+=("$(cat "${hurl_file%.*}.options")")
    fi
    cmd="hurl $hurl_file ${options[*]}"
    #echo "$cmd"
    echo "$cmd" | sh
done

echo "test integration ok!"
