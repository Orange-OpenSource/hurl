#!/bin/bash
set -Eeuo pipefail
hurl --fail-at-end tests_failed/option_fail_at_end_not_last_ko.hurl
