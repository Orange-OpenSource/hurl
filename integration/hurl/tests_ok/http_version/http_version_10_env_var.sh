#!/bin/bash
set -Eeuo pipefail

export HURL_HTTP10=1
hurl tests_ok/http_version/http_version_10.hurl
