#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
hurl --version | grep Features | grep -q HTTP3
if [ $? -eq 1 ]; then
  exit 255
fi
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config_3
export XDG_CONFIG_HOME

hurl tests_ok/http_version/http_version_3.hurl
