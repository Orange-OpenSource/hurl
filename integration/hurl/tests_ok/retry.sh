#!/bin/bash
set -Eeuo pipefail

hurl --retry 10 --retry-interval 100 --verbose --json tests_ok/retry.hurl
