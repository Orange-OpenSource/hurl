#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME
export HURL_HEADER="header-e: corge|header-f: grault"
hurl \
  --header 'header-b:baz' \
  --header 'header-c:qux' \
  tests_ok/add_header/add_header.hurl
