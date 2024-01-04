#!/bin/bash
set -Eeuo pipefail
hurl --json tests_failed/assert_status.hurl
