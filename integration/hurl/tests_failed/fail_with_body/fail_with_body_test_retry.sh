#!/bin/bash
set -Eeuo pipefail

hurl --retry 4 --retry-interval 100ms --test --fail-with-body tests_failed/fail_with_body/fail_with_body.hurl
