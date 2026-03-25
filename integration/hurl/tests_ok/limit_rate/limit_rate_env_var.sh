#!/bin/bash
set -Eeuo pipefail

export HURL_LIMIT_RATE=2000000
hurl --no-output tests_ok/limit_rate/limit_rate.hurl
