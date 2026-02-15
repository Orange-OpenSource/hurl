#!/bin/bash
set -Eeuo pipefail

# FIXME: We simulate CI in order to disable progress bar (we don't have --no-progress-bar)
export CI=1
hurl --no-color --test --report-tap tests_failed/parse_error_tap/parse_error_tap.tap tests_ok/hello/hello.hurl
