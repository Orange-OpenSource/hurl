#!/bin/bash
set -Eeuo pipefail

set +e
out=$(hurl --json tests_failed/assert_value_error.hurl)
exit_code="$?"
echo "$out" | jq
exit "$exit_code"

