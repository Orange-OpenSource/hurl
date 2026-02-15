#!/bin/bash
set -Eeuo pipefail

hurl --no-color --continue-on-error tests_failed/assert_http_version/assert_http_version.hurl
