#!/bin/bash
set -e
cd "$(dirname "$0")"

# Static Analysis
./hurl_echo tests/*.hurl tests_error_lint/*.hurl
./lint.sh tests_error_lint/*.hurl
./generate_html

# Dynamic
./run.sh tests/*.hurl tests_error_parser/*.hurl
#./hurl_output.sh output/*.command

set +e
rm -rf report/*
hurl --json report/tests.json --html report/html --output /dev/null tests/*.hurl

echo "test integration ok!"
