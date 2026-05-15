#!/bin/bash
set -Eeuo pipefail

hurl --fail-with-body tests_failed/fail_with_body/fail_with_body_unauthorized.hurl
