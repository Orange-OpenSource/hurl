#!/bin/bash
set -Eeuo pipefail

hurl --location \
  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' \
  --header 'Cookie: fruit=lemon' \
  tests_ok/follow_redirect/follow_redirect_leak.hurl
