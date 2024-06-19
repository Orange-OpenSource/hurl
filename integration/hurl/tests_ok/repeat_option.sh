#!/bin/bash
set -Eeuo pipefail

# We're deactivating output here because we explicitly enable output per request
# to control the number of repetition for each request.
hurl --no-output tests_ok/repeat_option.hurl
