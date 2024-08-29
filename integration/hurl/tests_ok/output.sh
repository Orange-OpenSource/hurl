#!/bin/bash
set -Eeuo pipefail

rm -f build/output.bin

hurl --output build/output.bin tests_ok/output.hurl
cat build/output.bin
