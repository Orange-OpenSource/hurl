#!/bin/bash
set -Eeuo pipefail

hurl --fail-with-body tests_pty/fail_with_body/fail_with_body.hurl
