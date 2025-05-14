#!/bin/bash
set -Eeuo pipefail

hurl --parallel --jobs 1 \
  tests_ok/parallel/parallel_a.hurl \
  tests_ok/parallel/parallel_b.hurl \
  tests_ok/parallel/parallel_c.hurl \
  tests_ok/parallel/parallel_d.hurl \
  tests_ok/parallel/parallel_e.hurl \
  tests_ok/parallel/parallel_f.hurl \
  tests_ok/parallel/parallel_g.hurl

