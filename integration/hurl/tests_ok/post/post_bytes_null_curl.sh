#!/bin/bash
set -Eeuo pipefail

hurl --curl build/post_bytes_null.curl --no-output tests_ok/post/post_bytes_null.hurl

cat build/post_bytes_null.curl
