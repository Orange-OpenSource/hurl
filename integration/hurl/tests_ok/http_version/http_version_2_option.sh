#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
hurl --version | grep Features | grep -q HTTP2
if [ $? -eq 1 ]; then
  exit 255
fi
set -Eeuo pipefail

hurl tests_ok/http_version/http_version_2_option.hurl
