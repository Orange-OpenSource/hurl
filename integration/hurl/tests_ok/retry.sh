#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/retry.hurl --retry 10 --retry-interval 100 --verbose --json
