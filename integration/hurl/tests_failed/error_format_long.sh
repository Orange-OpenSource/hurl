#!/bin/bash
set -Eeuo pipefail

hurl --error-format long --continue-on-error tests_failed/error_format_long.hurl
