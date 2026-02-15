#!/bin/bash
set -Eeuo pipefail

hurl --no-color --json tests_failed/assert_status/assert_status.hurl
