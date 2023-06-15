#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/retry_option.hurl --verbose
