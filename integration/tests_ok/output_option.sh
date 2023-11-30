#!/bin/bash
set -Eeuo pipefail
rm -f build/output_request_1.bin
hurl --no-output tests_ok/output_option.hurl
cat build/output_request_1.bin
