#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/missing_value_config
export XDG_CONFIG_HOME
hurl tests_failed/config_file/config_file.hurl
