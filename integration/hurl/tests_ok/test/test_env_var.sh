#!/bin/bash
set -Eeuo pipefail

export HURL_TEST=1
# We're using --jobs 1 to fix the standard error order.
hurl --jobs 1 --glob "tests_ok/test/test.*.hurl"
