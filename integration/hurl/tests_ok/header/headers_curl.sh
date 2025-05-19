#!/bin/bash
set -Eeuo pipefail

hurl --verbose --no-output --curl build/headers.curl tests_ok/header/headers.hurl

cat build/headers.curl
