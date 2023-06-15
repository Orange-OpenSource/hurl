#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/retry_option.hurl --verbose --json
