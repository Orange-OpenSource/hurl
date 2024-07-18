#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
curl --version | grep Features | grep -q HTTP3
if [ $? -eq 1 ]; then
  exit 255
fi
set -Eeuo pipefail

hurl tests_ok/http_version_3_option.hurl
