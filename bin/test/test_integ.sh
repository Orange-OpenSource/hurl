#!/bin/sh
set -e

echo "----- integration tests -----"

# hurl infos
command -v hurl
command -v hurlfmt
hurl --version
hurlfmt --version

# integration tests
cd integration || exit
./integration.py
./test_curl_commands.sh "$(find ./tests_ok ./tests_failed -maxdepth 1 -type f -name '*.curl' ! -name '*windows*')"
./test_html_output.py tests_ok/*.html tests_failed/*.html
./ad_hoc.sh
./report.py
