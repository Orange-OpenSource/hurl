#!/bin/bash
set -Eeuo pipefail

hurl --retry 5 --retry-interval 100 --verbose tests_failed/retry/retry.hurl
