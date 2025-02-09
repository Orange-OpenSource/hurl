#!/bin/bash
set -Eeuo pipefail
hurl --continue-on-error tests_failed/assert_http_version.hurl
