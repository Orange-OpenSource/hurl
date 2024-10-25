#!/bin/bash
set -Eeuo pipefail

hurl --no-output --limit-rate 2000000 tests_ok/limit_rate.hurl
