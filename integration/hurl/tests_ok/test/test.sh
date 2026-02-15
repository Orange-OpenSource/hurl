#!/bin/bash
set -Eeuo pipefail

# FIXME: We simulate CI in order to disable progress bar (we don't have --no-progress-bar)
export CI=1

# We're using --jobs 1 to fix the standard error order.
hurl --no-color --test --jobs 1 --glob "tests_ok/test/test.*.hurl"
