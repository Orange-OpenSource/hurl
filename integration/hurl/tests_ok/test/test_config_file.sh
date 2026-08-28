#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME
# We're using --jobs 1 to fix the standard error order.
hurl --jobs 1 --glob "tests_ok/test/test.*.hurl"
