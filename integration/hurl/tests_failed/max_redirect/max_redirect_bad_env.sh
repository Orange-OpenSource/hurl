#!/bin/bash
set -Eeuo pipefail

export HURL_MAX_REDIRS=-5
hurl tests_failed/max_redirect/max_redirect.hurl
