#!/bin/bash
set -Eeuo pipefail


set +e
XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME
hurl tests_failed/timeout/timeout.hurl
