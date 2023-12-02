#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/retry.hurl --retry 5 --retry-interval 100 --verbose
