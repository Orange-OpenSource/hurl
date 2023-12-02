#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/assert_match_utf8.hurl --json
