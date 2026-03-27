#!/bin/bash
set -Eeuo pipefail

export HURL_LOCATION=1
hurl tests_ok/follow_redirect/follow_redirect.hurl
