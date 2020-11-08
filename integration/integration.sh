#!/bin/bash
set -e
cd "$(dirname "$0")"

# Static run (without server)
bin/echo.sh           tests/*.hurl tests_error_lint/*.hurl
bin/format_to_json.sh tests/*.hurl
bin/format_to_html.sh tests/*.hurl
bin/lint.sh           tests_error_lint/*.hurl
bin/hurl.sh           tests_error_parser/*.hurl

# Dynamic run (with server)
bin/hurl.sh tests/*.hurl
bin/report-html.sh

echo "test integration ok!"
