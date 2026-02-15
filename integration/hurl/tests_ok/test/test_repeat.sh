#!/bin/bash
set -Eeuo pipefail

# FIXME: We simulate CI in order to disable progress bar (we don't have --no-progress-bar)
export CI=1

hurl --no-color --test --repeat 100 tests_ok/test/test_repeat.hurl
