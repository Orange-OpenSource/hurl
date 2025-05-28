#!/bin/bash
set -Eeuo pipefail

hurl --verbose --json tests_ok/retry/retry_option.hurl
