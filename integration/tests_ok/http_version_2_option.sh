#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
curl --version | grep Features | grep --quiet HTTP2
if [ $? -eq 1 ]; then
  exit 0
fi
set -Eeuo pipefail

hurl tests_ok/http_version_2_option.hurl
