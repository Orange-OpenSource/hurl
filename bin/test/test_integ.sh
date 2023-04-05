#!/bin/bash
set -Eeuo pipefail
set -x

echo "----- integration tests -----"

# hurl infos
command -v hurl || (echo "ERROR - hurl not found" ; exit 1)
command -v hurlfmt || (echo "ERROR - hurlfmt not found" ; exit 1)
hurl --version
hurlfmt --version

# integration tests
cd integration
./integration.py
./test_curl_commands.sh 
./test_html_output.py tests_ok/*.html tests_failed/*.html
#./report.py TODO: review test report

