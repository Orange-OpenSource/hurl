#!/bin/bash
set -Eeuo pipefail

export HURL_RETRY=10
export HURL_RETRY_INTERVAL=100ms
hurl --verbose --json tests_ok/retry/retry.hurl
