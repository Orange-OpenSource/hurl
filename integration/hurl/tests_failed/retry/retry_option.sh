#!/bin/bash
set -Eeuo pipefail

hurl --no-color --verbose tests_failed/retry/retry_option.hurl
