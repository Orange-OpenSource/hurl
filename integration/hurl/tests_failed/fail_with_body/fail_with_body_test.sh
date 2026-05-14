#!/bin/bash
set -Eeuo pipefail

hurl --test --fail-with-body tests_failed/fail_with_body/fail_with_body.hurl
