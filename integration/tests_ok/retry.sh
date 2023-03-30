#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/retry.hurl --retry --retry-interval 100 --verbose --json
