#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")
export XDG_CONFIG_HOME
hurl tests_failed/config_file/config_file.hurl
