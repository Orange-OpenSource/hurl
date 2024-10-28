#!/bin/bash
set -Eeuo pipefail

hurl --no-output tests_ok/limit_rate_option.hurl
