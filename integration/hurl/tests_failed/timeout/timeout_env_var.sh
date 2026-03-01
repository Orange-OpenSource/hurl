#!/bin/bash
set -Eeuo pipefail

set +e
export HURL_MAX_TIME=1
hurl tests_failed/timeout/timeout.hurl
export HURL_MAX_TIME=1s
hurl tests_failed/timeout/timeout.hurl
export HURL_MAX_TIME=500ms
hurl tests_failed/timeout/timeout.hurl
