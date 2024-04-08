#!/bin/bash
set -Eeuo pipefail

hurl --parallel --json \
  tests_ok/parallel_a.hurl \
  tests_ok/parallel_b.hurl \
  tests_ok/parallel_c.hurl \
  tests_ok/parallel_d.hurl \
  tests_ok/parallel_e.hurl \
  tests_ok/parallel_f.hurl \
  tests_ok/parallel_g.hurl
