#!/bin/bash
set -Eeuo pipefail

# GitHub runners only support IPV6 on macOS so we skip other OS (for the moment).
if [[ "$(uname -s)" = "Linux" ]]; then
  exit 255
fi

hurl --very-verbose tests_ok/ip.hurl 2>&1 | grep 'Connected to google.com' | sed 's/^** //'
