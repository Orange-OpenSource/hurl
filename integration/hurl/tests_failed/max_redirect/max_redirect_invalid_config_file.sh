#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/invalid_config
export XDG_CONFIG_HOME

hurl --location tests_failed/max_redirect/max_redirect.hurl
