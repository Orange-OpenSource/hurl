#!/bin/bash
set -Eeuo pipefail

export XDG_CONFIG_HOME="$(dirname "$0")"
hurl tests_ok/config_file.hurl
echo
hurl --repeat 1 tests_ok/config_file.hurl
