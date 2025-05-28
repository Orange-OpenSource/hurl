#!/bin/bash
set -Eeuo pipefail

hurl --repeat 4 \
  tests_ok/repeat/repeat_a.hurl \
  tests_ok/repeat/repeat_b.hurl \
  tests_ok/repeat/repeat_c.hurl
