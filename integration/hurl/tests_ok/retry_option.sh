#!/bin/bash
set -Eeuo pipefail

hurl --verbose --json tests_ok/retry_option.hurl
