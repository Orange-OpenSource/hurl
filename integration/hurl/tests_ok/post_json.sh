#!/bin/bash
set -Eeuo pipefail
hurl --variable age=30 --variable strict=true --variable string_variable=\\ tests_ok/post_json.hurl
