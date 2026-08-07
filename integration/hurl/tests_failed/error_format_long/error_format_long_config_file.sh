#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME

hurl --continue-on-error tests_failed/error_format_long/error_format_long.hurl
