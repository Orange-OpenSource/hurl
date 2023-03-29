#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/assert_value_error.hurl --json
