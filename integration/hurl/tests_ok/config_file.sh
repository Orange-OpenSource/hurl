#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME="$(dirname "$0")"
hurl tests_ok/config_file.hurl

