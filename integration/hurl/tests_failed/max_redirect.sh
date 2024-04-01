#!/bin/bash
set -Eeuo pipefail
hurl --location --max-redirs 5 tests_failed/max_redirect.hurl
