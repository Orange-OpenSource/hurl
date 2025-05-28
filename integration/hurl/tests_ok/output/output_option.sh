#!/bin/bash
set -Eeuo pipefail
rm -f build/output_request_1.bin
rm -f build/output_request_2.bin
hurl --no-output --file-root build tests_ok/output_option.hurl
cat build/output_request_1.bin build/output_request_2.bin
