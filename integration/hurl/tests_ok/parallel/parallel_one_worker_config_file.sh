#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME

hurl --parallel \
  tests_ok/parallel/parallel_a.hurl \
  tests_ok/parallel/parallel_b.hurl \
  tests_ok/parallel/parallel_c.hurl \
  tests_ok/parallel/parallel_d.hurl \
  tests_ok/parallel/parallel_e.hurl \
  tests_ok/parallel/parallel_f.hurl \
  tests_ok/parallel/parallel_g.hurl
