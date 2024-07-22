#!/bin/bash
set -Eeuo pipefail
rm -f build/repeat/tap.txt

hurl --repeat 4 --parallel --report-tap build/repeat/tap.txt --no-output \
  tests_ok/repeat_a.hurl \
  tests_ok/repeat_b.hurl \
  tests_ok/repeat_c.hurl

cat build/repeat/tap.txt
