#!/bin/bash
set -Eeuo pipefail
set +e
hurl --no-color tests_failed/timeout.hurl --max-time 1     # Default unit for max-time in seconds
hurl --no-color tests_failed/timeout.hurl --max-time 1s
hurl --no-color tests_failed/timeout.hurl --max-time 500ms