#!/bin/bash
set -Eeuo pipefail

set +e
hurl --max-time 1 tests_failed/timeout/timeout.hurl     # Default unit for max-time in seconds
hurl --max-time 1s tests_failed/timeout/timeout.hurl
hurl --max-time 500ms tests_failed/timeout/timeout.hurl
