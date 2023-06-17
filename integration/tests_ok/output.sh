#!/bin/bash
set -Eeuo pipefail
rm -f tests_ok/output.bin
hurl --output tests_ok/output.bin tests_ok/output.hurl
cat tests_ok/output.bin
