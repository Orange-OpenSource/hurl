#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/retry_until_200.hurl --verbose
