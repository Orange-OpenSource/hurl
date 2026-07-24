#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config_11
export XDG_CONFIG_HOME

hurl tests_ok/http_version/http_version_11.hurl
