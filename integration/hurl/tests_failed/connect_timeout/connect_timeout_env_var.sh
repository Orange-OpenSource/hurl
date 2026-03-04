#!/bin/bash
set -Eeuo pipefail

export HURL_CONNECT_TIMEOUT=500ms
hurl tests_failed/connect_timeout/connect_timeout.hurl
