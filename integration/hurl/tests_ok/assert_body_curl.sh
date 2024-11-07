#!/bin/bash
set -Eeuo pipefail

hurl --curl build/assert_body.curl --no-output tests_ok/assert_body.hurl

cat build/assert_body.curl
