#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
features=$(hurl --version | grep Features)
if echo "$features" | grep -q HTTP3 then
  exit 255
fi
set -Eeuo pipefail

hurl --http3 tests_failed/http_version_not_supported.hurl
