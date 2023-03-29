#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/max_redirect.hurl --location --max-redirs 5
