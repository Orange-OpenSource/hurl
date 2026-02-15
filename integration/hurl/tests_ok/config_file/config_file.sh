#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")
export XDG_CONFIG_HOME
hurl --no-color tests_ok/config_file/config_file.hurl
