#!/bin/bash
set -Eeuo pipefail

hurl --no-color --retry 10 --retry-interval 100 --verbose --json tests_ok/retry/retry.hurl
