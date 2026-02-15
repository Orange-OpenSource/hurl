#!/bin/bash
set -Eeuo pipefail

set +e
out=$(hurl --json --no-color tests_failed/assert_value_error/assert_value_error.hurl)
exit_code="$?"
echo "$out" | jq --monochrome-output
exit "$exit_code"

