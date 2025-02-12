#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/test_retry_reset.hurl
hurl --test-retry 5 --test-retry-interval 100 --verbose --json tests_ok/test_retry.hurl
