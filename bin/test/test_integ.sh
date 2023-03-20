#!/bin/bash
set -Eeuo pipefail

echo "----- integration tests -----"

# hurl infos
command -v hurl
command -v hurlfmt
hurl --version
hurlfmt --version

# integration tests
cd integration || exit
./integration.py
./test_curl_commands.sh 
./test_html_output.py tests_ok/*.html tests_failed/*.html
./report.py

