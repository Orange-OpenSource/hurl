#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME

hurl tests_failed/fail_with_body/fail_with_body.hurl
