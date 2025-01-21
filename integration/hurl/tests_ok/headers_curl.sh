#!/bin/bash
set -Eeuo pipefail
hurl --verbose --no-output --curl build/headers.curl tests_ok/headers.hurl

cat build/headers.curl
