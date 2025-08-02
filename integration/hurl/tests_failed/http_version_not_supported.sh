#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
hurl --version | grep Features | grep -q HTTP3
exit_code=$?
if [[ $exit_code -eq 0 ]] ; then
  exit 255
fi
set -Eeuo pipefail

hurl --http3 tests_failed/http_version_not_supported.hurl
