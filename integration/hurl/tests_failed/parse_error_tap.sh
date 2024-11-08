#!/bin/bash
set -Eeuo pipefail

hurl --test --report-tap tests_failed/parse_error_tap.tap tests_ok/hello.hurl
