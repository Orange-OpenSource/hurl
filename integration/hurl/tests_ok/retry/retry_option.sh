#!/bin/bash
set -Eeuo pipefail

hurl --no-color --verbose --json tests_ok/retry/retry_option.hurl
