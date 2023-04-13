#!/bin/bash
set -Eeuo pipefail
hurl --fail-at-end tests_failed/fail_at_end.hurl
