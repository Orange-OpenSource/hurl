#!/bin/bash
set -Eeuo pipefail

hurl --no-color --location --max-redirs 5 tests_failed/max_redirect/max_redirect.hurl
