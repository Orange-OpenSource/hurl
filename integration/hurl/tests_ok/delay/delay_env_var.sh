#!/bin/bash
set -Eeuo pipefail

export HURL_DELAY=1000
hurl tests_ok/delay/delay.hurl
export HURL_DELAY=1s
hurl tests_ok/delay/delay.hurl
