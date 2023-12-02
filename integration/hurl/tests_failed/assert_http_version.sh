#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/assert_http_version.hurl
