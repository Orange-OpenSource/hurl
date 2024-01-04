#!/bin/bash
set -Eeuo pipefail
hurl --json tests_failed/assert_value_error.hurl
